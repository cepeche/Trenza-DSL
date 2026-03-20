#!/usr/bin/env bash
# smoke-test.sh — Verifica los flujos críticos contra el NAS real
#
# Uso:   bash tests/smoke/smoke-test.sh [BASE_URL]
# Ej:    bash tests/smoke/smoke-test.sh http://192.168.1.71:8080/apps/cronometro/api/api
#
# Prerequisitos:
#   - Estar en la red local (192.168.1.71 accesible)
#   - python3 disponible (para parsear JSON; viene por defecto en WSL Ubuntu)
#
# Codigos de salida:
#   0  Todos los tests pasaron
#   1  Al menos un test falló
#   2  El NAS no responde (health check inicial)

set -uo pipefail

BASE="${1:-http://192.168.1.71:8080/apps/cronometro/api/api}"
PASSED=0
FAILED=0
declare -a ERRORS=()

# ─── Colores ──────────────────────────────────────────────────────────────────
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# ─── Utilidades ───────────────────────────────────────────────────────────────

assert() {
    local description="$1"
    local condition="$2"
    if eval "$condition" 2>/dev/null; then
        echo -e "  ${GREEN}PASS${NC}: $description"
        (( PASSED++ )) || true
    else
        echo -e "  ${RED}FAIL${NC}: $description"
        ERRORS+=("$description")
        (( FAILED++ )) || true
    fi
}

api_get() {
    curl -s --max-time 10 --fail-with-body "$BASE/$1" 2>/dev/null || echo '{}'
}

api_post() {
    curl -s --max-time 10 --fail-with-body \
         -X POST "$BASE/$1" \
         -H "Content-Type: application/json" \
         -d "$2" 2>/dev/null || echo '{}'
}

# Extrae campo de JSON usando python3 (sin jq)
# Uso: json_get "$JSON" "campo.subcampo"
json_get() {
    local json="$1"
    local keys="$2"
    echo "$json" | python3 -c "
import sys, json as j
try:
    d = j.loads(sys.stdin.read())
    for k in '$keys'.split('.'):
        if isinstance(d, list):
            d = d[int(k)]
        elif isinstance(d, dict):
            d = d.get(k)
        else:
            d = None
        if d is None:
            break
    print('' if d is None else str(d))
except Exception:
    print('')
" 2>/dev/null
}

# Devuelve el código HTTP de una petición (para tests de errores esperados)
http_code() {
    curl -s -o /dev/null -w "%{http_code}" --max-time 10 \
         -X POST "$BASE/$1" \
         -H "Content-Type: application/json" \
         -d "${2:-{}}" 2>/dev/null
}

section() {
    echo ""
    echo -e "${YELLOW}▶ $1${NC}"
}

# ─── SMOKE-000: Verificar conectividad básica ─────────────────────────────────
section "SMOKE-000: Conectividad con el NAS"
RESP=$(curl -s --max-time 5 "$BASE/health" 2>/dev/null)
if [[ -z "$RESP" || "$RESP" == '{}' ]]; then
    echo -e "${RED}ERROR: El NAS no responde en $BASE${NC}"
    echo "Verifica que estás en la red local y el NAS está encendido."
    exit 2
fi
assert "health status=ok"               "[[ \"$(json_get "$RESP" status)\" == 'ok' ]]"
assert "health version presente"        "[[ -n \"$(json_get "$RESP" version)\" ]]"
assert "health timestamp numérico"      "[[ \"$(json_get "$RESP" timestamp)\" =~ ^[0-9]+$ ]]"

# ─── SMOKE-001: Actividades ───────────────────────────────────────────────────
section "SMOKE-001: GET /actividades"
RESP=$(api_get "actividades")
assert "Respuesta es array JSON"        "echo \"\$RESP\" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert isinstance(d,list)'"
assert "Campo 'nombre' presente"        "[[ -n \"$(json_get "$RESP" 0.nombre)\" ]]"
assert "Campo 'color' presente"         "[[ -n \"$(json_get "$RESP" 0.color)\" ]]"
assert "Campo 'permanente' presente"    "[[ \"$(json_get "$RESP" 0.permanente)\" != '' ]]"
assert "Al menos una actividad activa"  "[[ \"$(json_get "$RESP" 0.id)\" != '' ]]"

# Capturar primera tarea válida para los tests de sesiones
TAREA_ID=$(echo "$RESP" | python3 -c "
import sys, json
acts = json.load(sys.stdin)
print(acts[0]['id'] if acts else '')
" 2>/dev/null)

# ─── SMOKE-002: Tipos de tarea ────────────────────────────────────────────────
section "SMOKE-002: GET /tipos-tarea"
RESP=$(api_get "tipos-tarea")
assert "Respuesta es array JSON"               "echo \"\$RESP\" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert isinstance(d,list)'"
assert "Campo 'usos_7d' presente"              "[[ \"$(json_get "$RESP" 0.usos_7d)\" != '' ]]"
assert "Campo 'actividades_permitidas' presente" "[[ \"$(json_get "$RESP" 0.actividades_permitidas)\" != '' ]]"
assert "Campo 'icono' presente"                "[[ -n \"$(json_get "$RESP" 0.icono)\" ]]"
assert "Al menos un tipo de tarea"             "[[ \"$(json_get "$RESP" 0.id)\" != '' ]]"

# Obtener ID de tarea válida (tipo_id + actividad_id) para tests de sesiones
TAREA_ID=$(echo "$RESP" | python3 -c "
import sys, json
tipos = json.load(sys.stdin)
for t in tipos:
    perms = t.get('actividades_permitidas', [])
    if isinstance(perms, str):
        import json as j2; perms = j2.loads(perms)
    if perms:
        print(f\"{t['id']}_{perms[0]}\")
        break
" 2>/dev/null)
assert "Al menos una tarea expandida obtenida" "[[ -n \"$TAREA_ID\" ]]"

# ─── SMOKE-003: Sesión activa (estado inicial) ────────────────────────────────
section "SMOKE-003: GET /sesiones?action=activa"
RESP=$(api_get "sesiones?action=activa")
assert "Respuesta válida (null o con server_time)" \
    "[[ \"\$RESP\" == 'null' || -n \"$(json_get "$RESP" server_time)\" ]]"

# Limpiar: si hay sesión activa, detenerla antes de los tests de sesiones
if [[ "$RESP" != "null" ]]; then
    echo -e "  ${YELLOW}[INFO]${NC} Había sesión activa, deteniéndola para tests limpios..."
    api_post "sesiones?action=detener" '{}' > /dev/null
fi

# ─── SMOKE-004: Iniciar sesión normal ────────────────────────────────────────
section "SMOKE-004: POST /sesiones?action=iniciar (modo normal)"
BODY="{\"tarea_id\": \"$TAREA_ID\", \"notas\": \"smoke test\"}"
RESP=$(api_post "sesiones?action=iniciar" "$BODY")
assert "success=true"                        "[[ \"$(json_get "$RESP" success)\" == 'True' ]]"
assert "data.id empieza con 'ses_'"          "[[ \"$(json_get "$RESP" data.id)\" == ses_* ]]"
assert "data.inicio es numérico"             "[[ \"$(json_get "$RESP" data.inicio)\" =~ ^[0-9]+$ ]]"
assert "data.server_time presente"           "[[ \"$(json_get "$RESP" data.server_time)\" =~ ^[0-9]+$ ]]"
assert "data.minutos_aplicados=0 (sin retro)" "[[ \"$(json_get "$RESP" data.minutos_aplicados)\" == '0' ]]"
assert "data.notas guardadas"                "[[ \"$(json_get "$RESP" data.tipo_tarea_nombre)\" != '' ]]"

SES_ID_1=$(json_get "$RESP" data.id)

# ─── SMOKE-005: Sesión activa visible tras iniciar ───────────────────────────
section "SMOKE-005: Sesión activa visible"
RESP=$(api_get "sesiones?action=activa")
assert "No es null"                          "[[ \"\$RESP\" != 'null' ]]"
assert "tarea_id coincide"                   "[[ \"$(json_get "$RESP" tarea_id)\" == \"$TAREA_ID\" ]]"
assert "notas = 'smoke test'"                "[[ \"$(json_get "$RESP" notas)\" == 'smoke test' ]]"
assert "server_time presente"                "[[ \"$(json_get "$RESP" server_time)\" =~ ^[0-9]+$ ]]"

# ─── SMOKE-006: Iniciar segunda sesión (cierra la anterior) ──────────────────
section "SMOKE-006: Iniciar segunda sesión (cierra la primera)"
sleep 2
RESP=$(api_post "sesiones?action=iniciar" "{\"tarea_id\": \"$TAREA_ID\"}")
assert "success=true"                        "[[ \"$(json_get "$RESP" success)\" == 'True' ]]"
SES_ID_2=$(json_get "$RESP" data.id)
assert "Nuevo id distinto al anterior"       "[[ \"$SES_ID_2\" != \"$SES_ID_1\" && -n \"$SES_ID_2\" ]]"

# ─── SMOKE-007: Inicio retroactivo (sin sesión previa) ───────────────────────
section "SMOKE-007: Inicio retroactivo de 5 minutos"
# Primero cerrar la sesión activa actual
api_post "sesiones?action=detener" '{}' > /dev/null
sleep 1

RESP=$(api_post "sesiones?action=iniciar" \
    "{\"tarea_id\": \"$TAREA_ID\", \"minutos_retroactivos\": 5}")
assert "success=true"                        "[[ \"$(json_get "$RESP" success)\" == 'True' ]]"
MIN_APLIC=$(json_get "$RESP" data.minutos_aplicados)
assert "minutos_aplicados=5 (sin sesión previa)" "[[ \"$MIN_APLIC\" == '5' ]]"

# El inicio debe estar ~300s antes que el server_time
INICIO_TS=$(json_get "$RESP" data.inicio)
SERVER_TS=$(json_get "$RESP" data.server_time)
DIFF=$(( SERVER_TS - INICIO_TS ))
assert "inicio ~300s antes del server_time (±30s)"  "[[ $DIFF -ge 270 && $DIFF -le 330 ]]"

SES_ID_3=$(json_get "$RESP" data.id)

# ─── SMOKE-008: Inicio retroactivo recortado (con sesión previa corta) ────────
section "SMOKE-008: Retroactivo recortado por sesión previa"
sleep 2  # la sesión actual lleva ~2s → pedir 10 min debe recortarse

RESP=$(api_post "sesiones?action=iniciar" \
    "{\"tarea_id\": \"$TAREA_ID\", \"minutos_retroactivos\": 10}")
assert "success=true"                        "[[ \"$(json_get "$RESP" success)\" == 'True' ]]"
MIN_APLIC=$(json_get "$RESP" data.minutos_aplicados)
assert "minutos_aplicados < 10 (fue recortado)" "[[ \"$MIN_APLIC\" -lt 10 ]]"

SES_ID_4=$(json_get "$RESP" data.id)

# ─── SMOKE-009: Modo sustituir ────────────────────────────────────────────────
section "SMOKE-009: Modo 'sustituir' (actualiza sesión activa)"
sleep 2
RESP=$(api_post "sesiones?action=iniciar" \
    "{\"tarea_id\": \"$TAREA_ID\", \"sustituir\": true, \"notas\": \"sustituida por smoke\"}")
assert "success=true"                         "[[ \"$(json_get "$RESP" success)\" == 'True' ]]"
assert "message contiene 'sustituida'"        "[[ \"$(json_get "$RESP" message)\" == *'sustituida'* ]]"
assert "id igual al de la sesión activa previa" "[[ \"$(json_get "$RESP" data.id)\" == \"$SES_ID_4\" ]]"
assert "minutos_aplicados es null"            "[[ \"$(json_get "$RESP" data.minutos_aplicados)\" == '' ]]"

# ─── SMOKE-010: Detener sesión activa ────────────────────────────────────────
section "SMOKE-010: POST /sesiones?action=detener"
sleep 2
RESP=$(api_post "sesiones?action=detener" '{}')
assert "success=true"                         "[[ \"$(json_get "$RESP" success)\" == 'True' ]]"
DUR=$(json_get "$RESP" data.duracion)
assert "duración > 0"                         "[[ \"$DUR\" =~ ^[0-9]+$ && $DUR -gt 0 ]]"

# Verificar que ya no hay sesión activa
RESP2=$(api_get "sesiones?action=activa")
assert "Sesión activa = null tras detener"    "[[ \"$RESP2\" == 'null' ]]"

# ─── SMOKE-011: Detener sin sesión activa → 404 ──────────────────────────────
section "SMOKE-011: Detener sin sesión activa → HTTP 404"
CODE=$(http_code "sesiones?action=detener" '{}')
assert "HTTP 404 cuando no hay sesión activa" "[[ \"$CODE\" == '404' ]]"

# ─── SMOKE-012: Sesiones del día ─────────────────────────────────────────────
section "SMOKE-012: GET /sesiones?fecha=hoy"
RESP=$(api_get "sesiones?fecha=hoy")
assert "Respuesta es array JSON"              "echo \"\$RESP\" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert isinstance(d,list)'"
assert "Hay sesiones registradas hoy"         "[[ \"$(json_get "$RESP" 0.id)\" != '' ]]"

# ─── SMOKE-013: Estadísticas del día ─────────────────────────────────────────
section "SMOKE-013: GET /sesiones?action=estadisticas"
RESP=$(api_get "sesiones?action=estadisticas&fecha=hoy")
assert "campo tiempo_total presente"          "[[ \"$(json_get "$RESP" tiempo_total)\" != '' ]]"
assert "campo por_tipo_tarea es array"        "echo \"\$RESP\" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert isinstance(d.get(\"por_tipo_tarea\"),list)'"
assert "campo por_tarea presente"             "[[ \"$(json_get "$RESP" por_tarea)\" != '' ]]"
assert "tiempo_total > 0 (hemos registrado sesiones)" \
    "[[ $(json_get "$RESP" tiempo_total) -gt 0 ]]"

# ─── SMOKE-014: Acumulado por actividad ──────────────────────────────────────
section "SMOKE-014: GET /sesiones?action=acumulado"
RESP=$(api_get "sesiones?action=acumulado")
assert "campo server_time presente"           "[[ -n \"$(json_get "$RESP" server_time)\" ]]"
assert "campo actividades presente"           "echo \"\$RESP\" | python3 -c 'import sys,json; d=json.load(sys.stdin); assert \"actividades\" in d'"

# ─── SMOKE-015: Tarea inexistente → 404 ──────────────────────────────────────
section "SMOKE-015: Iniciar con tarea inexistente → HTTP 404"
CODE=$(http_code "sesiones?action=iniciar" '{"tarea_id":"tarea_que_no_existe_jamas"}')
assert "HTTP 404 para tarea inexistente"      "[[ \"$CODE\" == '404' ]]"

# ─── RESUMEN ──────────────────────────────────────────────────────────────────
echo ""
echo "════════════════════════════════════"
TOTAL=$(( PASSED + FAILED ))
echo -e "  Total: $TOTAL  |  ${GREEN}Passed: $PASSED${NC}  |  ${RED}Failed: $FAILED${NC}"
echo "════════════════════════════════════"

if [[ $FAILED -gt 0 ]]; then
    echo ""
    echo -e "${RED}Tests fallidos:${NC}"
    for err in "${ERRORS[@]}"; do
        echo "  - $err"
    done
    exit 1
fi

echo -e "\n${GREEN}Todos los smoke tests pasaron.${NC}\n"
