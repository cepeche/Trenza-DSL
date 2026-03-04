#!/bin/bash

# Script de recuperación tras reinicio/corte del NAS WD My Cloud EX2 Ultra
#
# Qué hace:
#   1. Restaura la clave SSH en authorized_keys (si se perdió)
#   2. Reconfigura Apache (puerto 8080 + acceso LAN)
#   3. Despliega los archivos de la app (frontend + backend)
#   4. Inicializa la BD si no existe (nunca sobreescribe datos)
#   5. Verifica que todo responde correctamente
#
# Uso:
#   bash scripts/recuperar-nas.sh              → pide contraseña si la clave no está
#   bash scripts/recuperar-nas.sh --solo-apache → solo reconfigura Apache (sin deploy)
#
# Ejecutar desde la raíz del proyecto en WSL.

set -e  # salir si cualquier comando falla

NAS_HOST="192.168.1.71"
NAS_USER="sshd"
NAS_PATH="/var/www/apps/cronometro"

# La clave maestra está en Windows (persiste entre sesiones WSL).
# SSH requiere permisos 600, que /mnt/c/ no puede tener → copiamos a /tmp.
WIN_SSH_KEY="/mnt/c/Users/cpcxb/.ssh/id_nas"
NAS_SSH_KEY="/tmp/id_nas_deploy"
if [ -f "$WIN_SSH_KEY" ]; then
    cp "$WIN_SSH_KEY" "$NAS_SSH_KEY"
    chmod 600 "$NAS_SSH_KEY"
else
    echo "❌ Clave SSH no encontrada en $WIN_SSH_KEY"
    exit 1
fi
SOLO_APACHE=false

# Parsear argumentos
for arg in "$@"; do
    case $arg in
        --solo-apache) SOLO_APACHE=true ;;
    esac
done

# ─── Colores ──────────────────────────────────────────────────────────────────
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'
ok()   { echo -e "${GREEN}✅ $1${NC}"; }
err()  { echo -e "${RED}❌ $1${NC}"; exit 1; }
warn() { echo -e "${YELLOW}⚠️  $1${NC}"; }
info() { echo -e "   $1"; }

# ─── Wrappers SSH/SCP (sin contraseña una vez restaurada la clave) ────────────
SSH_OPTS="-o StrictHostKeyChecking=no -o ConnectTimeout=10 -o LogLevel=ERROR"
SSH_KEY_OPT="-i $NAS_SSH_KEY"
SSH="ssh $SSH_KEY_OPT $SSH_OPTS $NAS_USER@$NAS_HOST"
SCP="scp $SSH_KEY_OPT $SSH_OPTS"

echo ""
echo "╔══════════════════════════════════════════╗"
echo "║   Recuperación NAS — Mi Cronómetro PSP   ║"
echo "╚══════════════════════════════════════════╝"
echo ""

# ─── Verificaciones previas ───────────────────────────────────────────────────
if [ ! -f "README.md" ]; then
    err "Ejecuta este script desde la raíz del proyecto"
fi

if [ ! -f "$NAS_SSH_KEY" ]; then
    err "Clave SSH no encontrada en $NAS_SSH_KEY\n   Genera una nueva con: ssh-keygen -t ed25519 -f ~/.ssh/id_nas"
fi

# ─── PASO 1: Restaurar clave SSH ──────────────────────────────────────────────
echo "🔑 Paso 1: Verificando autenticación SSH..."

if ssh $SSH_KEY_OPT $SSH_OPTS -o BatchMode=yes -o PasswordAuthentication=no \
       $NAS_USER@$NAS_HOST "exit" 2>/dev/null; then
    ok "Clave SSH ya está en el NAS (no es necesario restaurarla)"
else
    warn "Clave SSH no encontrada en el NAS. Restaurando..."
    info "→ Se pedirá la contraseña de '$NAS_USER' UNA ÚLTIMA VEZ"
    echo ""
    if ssh-copy-id -i "$NAS_SSH_KEY.pub" $NAS_USER@$NAS_HOST; then
        echo ""
        ok "Clave SSH restaurada en el NAS"
    else
        err "No se pudo restaurar la clave SSH"
    fi
fi
echo ""

# ─── PASO 2: Reconfigurar Apache ──────────────────────────────────────────────
echo "🌐 Paso 2: Verificando configuración de Apache..."

APACHE_STATUS=$($SSH "
    LISTEN_OK=false
    DIR_OK=false
    LOC_OK=false

    grep -q 'Listen 8080' /usr/local/apache2/conf/extra/ports.conf && LISTEN_OK=true
    grep -q 'cronometro' /usr/local/apache2/conf/httpd.conf && DIR_OK=true && LOC_OK=true

    echo \"\${LISTEN_OK}|\${DIR_OK}|\${LOC_OK}\"
")

LISTEN_OK=$(echo $APACHE_STATUS | cut -d'|' -f1)
DIR_OK=$(echo $APACHE_STATUS | cut -d'|' -f2)
LOC_OK=$(echo $APACHE_STATUS | cut -d'|' -f3)

APACHE_CHANGED=false

if [ "$LISTEN_OK" != "true" ]; then
    warn "ports.conf: falta Listen 8080 → añadiendo..."
    $SSH "echo 'Listen 8080' >> /usr/local/apache2/conf/extra/ports.conf"
    APACHE_CHANGED=true
else
    info "ports.conf: Listen 8080 presente"
fi

if [ "$DIR_OK" != "true" ] || [ "$LOC_OK" != "true" ]; then
    warn "httpd.conf: faltan bloques Directory/Location → añadiendo..."
    $SSH "cat >> /usr/local/apache2/conf/httpd.conf << 'EOF'

<Directory \"/var/www/apps/cronometro\">
    Options +FollowSymLinks -Indexes
    AllowOverride All
    AuthType None
    <RequireAll>
        Require ip 192.168.1
    </RequireAll>
</Directory>

<Location \"/apps/cronometro\">
    AuthType None
    <RequireAll>
        Require ip 192.168.1
    </RequireAll>
</Location>
EOF"
    APACHE_CHANGED=true
else
    info "httpd.conf: bloques Directory y Location presentes"
fi

if [ "$APACHE_CHANGED" = "true" ]; then
    info "→ Verificando sintaxis..."
    SYNTAX=$($SSH "httpd -f /usr/local/apache2/conf/httpd.conf -t 2>&1 | tail -1")
    if echo "$SYNTAX" | grep -q "Syntax OK"; then
        info "→ Recargando Apache..."
        $SSH "httpd -f /usr/local/apache2/conf/httpd.conf -k graceful 2>/dev/null; sleep 1"
        ok "Apache reconfigurado y recargado"
    else
        err "Error de sintaxis en httpd.conf: $SYNTAX"
    fi
else
    ok "Apache ya estaba correctamente configurado"
fi

# Verificar que escucha en 8080
LISTENING=$($SSH "netstat -tlnp 2>/dev/null | grep ':8080'" || true)
if [ -n "$LISTENING" ]; then
    ok "Apache escuchando en 0.0.0.0:8080"
else
    err "Apache NO está escuchando en :8080 tras la reconfiguración"
fi
echo ""

# ─── PASO 3: Deploy de archivos ───────────────────────────────────────────────
if [ "$SOLO_APACHE" = "true" ]; then
    echo "ℹ️  Modo --solo-apache: omitiendo deploy de archivos"
    echo ""
else
    echo "📁 Paso 3: Verificando estructura de archivos..."

    ARCHIVOS_OK=$($SSH "
        [ -f $NAS_PATH/www/index.html ] && \
        [ -f $NAS_PATH/api/index.php ] && \
        [ -f $NAS_PATH/api/.htaccess ] && \
        [ -f $NAS_PATH/api/db/database.php ] && \
        [ -f $NAS_PATH/api/api/sesiones.php ] && \
        echo 'ok' || echo 'incompleto'
    ")

    if [ "$ARCHIVOS_OK" = "ok" ]; then
        ok "Archivos de la app presentes en el NAS"
    else
        warn "Archivos incompletos → desplegando..."

        # Crear directorios
        $SSH "
            mkdir -p $NAS_PATH/www/css $NAS_PATH/www/js
            mkdir -p $NAS_PATH/api/db $NAS_PATH/api/api
            mkdir -p $NAS_PATH/data $NAS_PATH/logs
        "

        # Frontend
        info "→ Subiendo frontend..."
        $SCP frontend/index.html         "$NAS_USER@$NAS_HOST:$NAS_PATH/www/index.html"
        $SCP frontend/css/styles.css     "$NAS_USER@$NAS_HOST:$NAS_PATH/www/css/styles.css"
        $SCP frontend/js/api-client.js   "$NAS_USER@$NAS_HOST:$NAS_PATH/www/js/api-client.js"
        $SCP frontend/js/app.js          "$NAS_USER@$NAS_HOST:$NAS_PATH/www/js/app.js"

        # Backend
        info "→ Subiendo backend..."
        $SCP backend/index.php           "$NAS_USER@$NAS_HOST:$NAS_PATH/api/index.php"
        $SCP backend/config.php          "$NAS_USER@$NAS_HOST:$NAS_PATH/api/config.php"
        $SCP backend/db/database.php     "$NAS_USER@$NAS_HOST:$NAS_PATH/api/db/database.php"
        $SCP backend/db/schema.sql       "$NAS_USER@$NAS_HOST:$NAS_PATH/api/db/schema.sql"
        $SCP backend/api/actividades.php "$NAS_USER@$NAS_HOST:$NAS_PATH/api/api/actividades.php"
        $SCP backend/api/tipos-tarea.php "$NAS_USER@$NAS_HOST:$NAS_PATH/api/api/tipos-tarea.php"
        $SCP backend/api/sesiones.php    "$NAS_USER@$NAS_HOST:$NAS_PATH/api/api/sesiones.php"

        # .htaccess para el router PHP
        $SSH "cat > $NAS_PATH/api/.htaccess << 'EOF'
RewriteEngine On
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_FILENAME} !-d
RewriteRule ^ index.php [QSA,L]
EOF"

        # Permisos
        $SSH "
            chmod -R 755 $NAS_PATH/www/
            chmod -R 755 $NAS_PATH/api/
            chmod -R 777 $NAS_PATH/data/
            chmod -R 755 $NAS_PATH/logs/
        "
        ok "Archivos desplegados"
    fi
    echo ""

    # ─── PASO 4: Base de datos ─────────────────────────────────────────────────
    echo "🗄️  Paso 4: Verificando base de datos..."

    DB_PERSISTENT="/mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db"
    DB_STATUS=$($SSH "
        if [ ! -f $DB_PERSISTENT ]; then
            echo 'no_existe'
        else
            SIZE=\$(stat -c%s $DB_PERSISTENT 2>/dev/null || echo 0)
            echo \"\$SIZE\"
        fi
    ")

    if [ "$DB_STATUS" = "no_existe" ] || [ "$DB_STATUS" = "0" ]; then
        warn "BD no existe o está vacía → inicializando schema..."

        # Crear script PHP temporal y ejecutarlo
        cat > /tmp/init_schema_recovery.php << 'PHPEOF'
<?php
$dbPath = '/mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db';
$schemaPath = '/var/www/apps/cronometro/api/db/schema.sql';
try {
    $db = new PDO("sqlite:$dbPath");
    $db->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
    // Solo inicializar si no hay tablas
    $tables = $db->query("SELECT count(*) FROM sqlite_master WHERE type='table'")->fetchColumn();
    if ($tables == 0) {
        $schema = file_get_contents($schemaPath);
        $db->exec($schema);
        $acts  = $db->query("SELECT COUNT(*) FROM actividades")->fetchColumn();
        $tipos = $db->query("SELECT COUNT(*) FROM tipos_tarea")->fetchColumn();
        $tar   = $db->query("SELECT COUNT(*) FROM tareas")->fetchColumn();
        echo "INIT_OK:$acts:$tipos:$tar";
    } else {
        $acts  = $db->query("SELECT COUNT(*) FROM actividades")->fetchColumn();
        echo "YA_EXISTE:$acts";
    }
    chmod($dbPath, 0666);
} catch (Exception $e) {
    echo "ERROR:" . $e->getMessage();
    exit(1);
}
PHPEOF
        $SCP /tmp/init_schema_recovery.php "$NAS_USER@$NAS_HOST:/tmp/init_schema_recovery.php"
        RESULT=$($SSH "php /tmp/init_schema_recovery.php; rm /tmp/init_schema_recovery.php")
        rm -f /tmp/init_schema_recovery.php

        if echo "$RESULT" | grep -q "^INIT_OK"; then
            ACTS=$(echo $RESULT | cut -d':' -f2)
            TIPOS=$(echo $RESULT | cut -d':' -f3)
            TAR=$(echo $RESULT | cut -d':' -f4)
            ok "BD inicializada ($ACTS actividades, $TIPOS tipos, $TAR tareas)"
        elif echo "$RESULT" | grep -q "^YA_EXISTE"; then
            ok "BD ya tenía datos (no se tocó)"
        else
            err "Error al inicializar BD: $RESULT"
        fi
    else
        SIZE_KB=$(( DB_STATUS / 1024 ))
        ok "BD existente (${SIZE_KB}KB) — datos preservados"
    fi
    echo ""
fi

# ─── PASO 5: Sincronización horaria (NTP) ────────────────────────────────────
echo "🕐 Paso 5: Verificando sincronización horaria..."

NTP_CRON_OK=$($SSH "crontab -l 2>/dev/null | grep -q sntp && echo 'ok' || echo 'no'")

if [ "$NTP_CRON_OK" = "ok" ]; then
    info "Cron de NTP ya configurado"
else
    warn "No hay cron de NTP → configurando sincronización horaria..."
    # Sincronizar ahora y añadir cron para cada hora
    $SSH "
        /usr/sbin/sntp -s pool.ntp.org 2>/dev/null || true
        (crontab -l 2>/dev/null; echo '0 * * * * /usr/sbin/sntp -s pool.ntp.org > /dev/null 2>&1') | crontab -
    "
    ok "NTP configurado (sync cada hora con pool.ntp.org)"
fi

# Mostrar desfase actual
DESFASE=$($SSH "date +%s" 2>/dev/null)
LOCAL=$(date +%s)
DIFF=$(( DESFASE - LOCAL ))
if [ "${DIFF#-}" -le 2 ]; then
    ok "Reloj del NAS sincronizado (desfase: ${DIFF}s)"
else
    warn "Desfase de ${DIFF}s — se corregirá en la próxima sincronización"
fi
echo ""

# ─── PASO 6: Verificación final ───────────────────────────────────────────────
echo "🧪 Paso 6: Verificación final..."

# Health check HTTP
HEALTH=$(curl -s --max-time 5 "http://$NAS_HOST:8080/apps/cronometro/api/api/health" 2>/dev/null || true)
if echo "$HEALTH" | grep -q 'status.*ok'; then
    ok "API health check: OK"
else
    warn "Health check no respondió como esperado: $HEALTH"
fi

# Frontend HTTP
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" --max-time 5 \
    "http://$NAS_HOST:8080/apps/cronometro/www/" 2>/dev/null || echo "000")
if [ "$HTTP_CODE" = "200" ]; then
    ok "Frontend: HTTP 200 OK"
else
    warn "Frontend devolvió HTTP $HTTP_CODE"
fi

echo ""
echo "╔══════════════════════════════════════════╗"
echo "║         Recuperación completada ✓        ║"
echo "╚══════════════════════════════════════════╝"
echo ""
echo "   🌐  http://$NAS_HOST:8080/apps/cronometro/www/"
echo "   📱  (misma URL desde el móvil en la misma WiFi)"
echo ""
