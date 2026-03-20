# Mi Cronómetro PSP - Plan de Pruebas
**Versión**: 1.1
**Fecha**: 19 febrero 2026
**Actualizado por**: Revisión post-MVP (arquitectura migrada a PHP+SQLite+API REST)

---

## 1. Introducción

### 1.1 Objetivos del Plan de Pruebas
- Verificar que todos los requisitos funcionales se cumplen
- Detectar regresiones al añadir nuevas funcionalidades
- Servir como documentación ejecutable del comportamiento esperado
- Facilitar validación manual hasta que se implementen tests automatizados

### 1.2 Alcance
Este plan cubre:
- ✅ Tests de API (endpoints backend PHP)
- ✅ Tests de integración frontend-backend (via curl)
- ✅ Tests de interfaz de usuario (manuales en navegador/móvil)
- ⏳ Tests unitarios de lógica JS (pendiente configurar entorno)

No cubre (fuera de v1.x):
- ❌ Tests de carga / estrés
- ❌ Tests de seguridad
- ❌ Tests de accesibilidad WCAG
- ❌ Tests cross-browser automatizados

### 1.3 Herramientas actuales
- **Tests de API**: `curl` desde WSL
- **Tests manuales de UI**: Chrome DevTools + móvil físico
- **Verificación de BD**: PHP endpoint ad-hoc (no hay sqlite3 CLI en el NAS)
- **Tests unitarios JS (futuro)**: Vitest + jsdom

### 1.4 Entorno
- **URL base**: `http://192.168.1.71:8080/apps/cronometro`
- **API base**: `http://192.168.1.71:8080/apps/cronometro/api/api`
- **Ejecutar desde**: WSL en Windows o cualquier máquina en red local

---

## 2. Tests de API (ejecutables con curl)

### Preparación
```bash
BASE="http://192.168.1.71:8080/apps/cronometro/api/api"
```

---

### API-001: Health check
**Requisito**: RNF-002
```bash
curl -s "$BASE/health" | python3 -m json.tool
```
**Resultado esperado**:
```json
{"status": "ok", "timestamp": <unix>, "version": "1.0.0"}
```

---

### API-002: Listar actividades (incluye campo permanente)
**Requisito**: RF-001, RF-008
```bash
curl -s "$BASE/actividades" | python3 -m json.tool
```
**Resultado esperado**: Array de objetos con campos `id`, `nombre`, `color`, `archived` (0), `permanente` (0 o 1)

---

### API-003: Crear actividad permanente
**Requisito**: RF-008, RF-015
```bash
curl -s -X POST "$BASE/actividades" \
  -H "Content-Type: application/json" \
  -d '{"nombre":"Test Permanente","color":"#667eea","permanente":true}' \
  | python3 -m json.tool
```
**Resultado esperado**: `{"success": true, "data": {"id": "test-permanente", ..., "permanente": 1}}`

---

### API-004: Listar tipos de tarea (solo archived=0)
**Requisito**: RF-001, RF-006
```bash
curl -s "$BASE/tipos-tarea" | python3 -m json.tool
```
**Resultado esperado**: Array de tipos de tarea. Si se ejecutó una puesta a cero previa y se archivaron tipos, esos NO deben aparecer.

---

### API-005: Obtener sesión activa (incluye server_time)
**Requisito**: RF-003, RNF-007
```bash
curl -s "$BASE/sesiones?action=activa" | python3 -m json.tool
```
**Resultado esperado si hay sesión**: Objeto con `tarea_id`, `inicio`, `notas`, `server_time`
**Resultado esperado si no hay sesión**: `null`

---

### API-006: Iniciar sesión con comentario
**Requisito**: RF-002, RF-013
```bash
# Obtener un tarea_id válido primero:
curl -s "$BASE/tipos-tarea" | python3 -m json.tool | grep '"id"' | head -3

# Iniciar sesión (sustituir TAREA_ID por un ID real, ej: "aseo_np")
curl -s -X POST "$BASE/sesiones?action=iniciar" \
  -H "Content-Type: application/json" \
  -d '{"tarea_id":"aseo_np","notas":"Prueba de comentario"}' \
  | python3 -m json.tool
```
**Resultado esperado**: `{"success": true, "data": {"id": "ses_...", "inicio": <unix>, "server_time": <unix>, ...}}`

---

### API-007: Obtener sesiones de hoy
**Requisito**: RF-004, RF-005
```bash
curl -s "$BASE/sesiones?fecha=hoy" | python3 -m json.tool
```
**Resultado esperado**: Array de sesiones con `duracion` (puede ser null si sesión activa), `tipo_tarea_nombre`, `actividad_nombre`

---

### API-008: Acumulado por actividad
**Requisito**: RF-014
```bash
curl -s "$BASE/sesiones?action=acumulado" | python3 -m json.tool
```
**Resultado esperado**: `{"server_time": <unix>, "actividades": [{"actividad_nombre": "...", "tiempo_total": <segundos>}]}`

---

### API-009: Puesta a cero con lista de actividades a conservar
**Requisito**: RF-015
```bash
# PRECAUCIÓN: este test modifica datos reales
# Sustituir los IDs por actividades reales a conservar
curl -s -X POST "$BASE/sesiones?action=reset" \
  -H "Content-Type: application/json" \
  -d '{"conservar":["np","ps","ad"]}' \
  -o /tmp/backup-test.csv \
  -D -
```
**Resultado esperado**:
- Headers: `Content-Type: text/csv`, `Content-Disposition: attachment; filename="cronometro-psp-YYYY-MM-DD.csv"`
- Body: CSV con BOM UTF-8, cabecera y filas de sesiones

**Verificación post-reset**:
```bash
# Las actividades no conservadas deben aparecer como archived=1
# (verificar via GET /actividades — no deben aparecer en la lista)
curl -s "$BASE/actividades" | python3 -m json.tool

# Los tipos de tarea huérfanos no deben aparecer
curl -s "$BASE/tipos-tarea" | python3 -m json.tool
```

---

### API-010: Detener sesión activa
**Requisito**: RF-002
```bash
curl -s -X POST "$BASE/sesiones?action=detener" \
  -H "Content-Type: application/json" \
  | python3 -m json.tool
```
**Resultado esperado**: `{"success": true, "data": {"id": "ses_...", "duracion": <segundos>}}`

---

## 3. Tests de Interfaz de Usuario (manuales)

### Instrucciones generales
- Probar en: Chrome desktop, Chrome móvil (Android), Safari iOS
- URL: `http://192.168.1.71:8080/apps/cronometro/www/`
- Tras cada deploy relevante, incrementar `?v=N` en los scripts y verificar que el móvil descarga la versión nueva

---

### UI-001: Carga inicial
**Requisito**: RF-001, RNF-002

| Paso | Resultado esperado |
|------|-------------------|
| Abrir la URL | Spinner "Cargando..." visible brevemente |
| App cargada | Pestaña "⭐ Frecuentes" activa |
| Si hay usos_7d > 0 | Tarjetas de tareas frecuentes visibles |
| Si todos usos_7d = 0 | Mensaje "Usa el botón + para crear tareas..." |
| Barra inferior | "Hoy: 0:00" (si no hay sesiones) |
| Header | "Ninguna tarea activa" + "--:--" |

---

### UI-002: Iniciar tarea unívoca
**Requisito**: RF-002, RF-013

| Paso | Resultado esperado |
|------|-------------------|
| Pulsar tarjeta de tarea con una sola actividad | Modal de comentario aparece |
| Dejar comentario vacío + "Iniciar" | Sesión inicia sin comentario |
| Header | Muestra icono + nombre de tarea + actividad |
| Timer | Muestra "00:00" (o tiempo si ya pasó 1+ minuto) |
| Barra inferior | "Hoy: 0:00", sin comentario |

---

### UI-003: Iniciar tarea con comentario
**Requisito**: RF-002, RF-013

| Paso | Resultado esperado |
|------|-------------------|
| Pulsar tarjeta | Modal de comentario |
| Escribir texto en el campo | Campo acepta hasta 120 caracteres |
| Pulsar Enter (teclado) | Inicia sesión |
| Barra inferior izquierda | Muestra el comentario escrito (truncado si largo) |
| Al iniciar otra tarea | El comentario de la tarea anterior desaparece |

---

### UI-004: Iniciar tarea multi-actividad
**Requisito**: RF-011, RF-002

| Paso | Resultado esperado |
|------|-------------------|
| Pulsar tarjeta multi-actividad | Modal con "¿En qué actividad?" |
| Ver botones | Un botón por actividad permitida |
| Seleccionar actividad | Modal de actividad se cierra, abre modal de comentario |
| Confirmar | Sesión inicia en la actividad seleccionada |
| Pulsar Cancelar en modal actividad | Modal cierra, sin sesión iniciada |

---

### UI-005: Navegación por pestañas
**Requisito**: RF-010

| Paso | Resultado esperado |
|------|-------------------|
| Pestañas visibles | Frecuentes + una por actividad activa |
| Pulsar pestaña de actividad | Grid muestra tareas de esa actividad |
| Hacer scroll horizontal en pestañas (móvil) | Pestañas desplazan suavemente |
| Pestaña activa | Destacada en azul con borde inferior |
| Header del timer | Sigue visible (sticky) |

---

### UI-006: Acumulado de tiempo
**Requisito**: RF-004, RF-005

| Paso | Resultado esperado |
|------|-------------------|
| Trabajar 5+ minutos en tarea | Tarjeta muestra tiempo acumulado (ej: "0:05") |
| Cambiar a otra tarea y volver a la pestaña | Tiempo de la primera tarea se ha actualizado |
| Barra inferior | "Hoy: H:MM" suma de todas las sesiones del día |
| Al pasar de medianoche | Contadores se resetean a 0:00 |

---

### UI-007: Crear nuevo tipo de tarea
**Requisito**: RF-007

| Paso | Resultado esperado |
|------|-------------------|
| Pulsar botón "+" | Modal "Nueva tarea" |
| Dejar nombre vacío + Crear | Alert "Por favor ingresa un nombre..." |
| No marcar actividades + Crear | Alert "Selecciona al menos una actividad" |
| Rellenar correctamente + Crear | Alert "✅ Tarea X creada" |
| Ir a pestaña de actividad marcada | Nueva tarea aparece en el grid |
| Abrir desde pestaña de actividad | La actividad de esa pestaña viene pre-marcada |

---

### UI-008: Crear nueva actividad
**Requisito**: RF-008

| Paso | Resultado esperado |
|------|-------------------|
| ⚙️ → "Nueva actividad" | Modal con nombre, color, toggle permanente |
| Toggle "🔒 Conservar" desmarcado | Por defecto |
| Crear actividad | Nueva pestaña aparece inmediatamente |
| La nueva actividad aparece en la lista de checkboxes al crear tareas | ✅ |

---

### UI-009: Menú de configuración
**Requisito**: RF-009

| Paso | Resultado esperado |
|------|-------------------|
| Pulsar ⚙️ | Menú dropdown aparece |
| Menú muestra | "📁 Nueva actividad", "ℹ️ Acerca de", "🗑️ Puesta a cero" (en rojo) |
| Pulsar fuera del menú | Menú se cierra |
| Pulsar una opción | Menú se cierra y se abre el modal correspondiente |

---

### UI-010: Acerca de
**Requisito**: RF-014

| Paso | Resultado esperado |
|------|-------------------|
| ⚙️ → "Acerca de" | Modal con indicador de conexión |
| Con NAS accesible | Punto verde + "Conectado · Xms · v1.0.0" |
| Tiempo acumulado | Lista de actividades con tiempo total |
| Sin sesiones | "Sin sesiones registradas aún" |

---

### UI-011: Puesta a cero — flujo completo
**Requisito**: RF-015

| Paso | Resultado esperado |
|------|-------------------|
| ⚙️ → "Puesta a cero" | Modal fase 1: texto descriptivo + "Continuar" |
| Pulsar "Cancelar" | Modal se cierra |
| Pulsar "Continuar" | Fase 2: lista de actividades con checkboxes |
| Actividades permanentes | Aparecen pre-marcadas con 🔒 |
| Desmarcar una actividad permanente | Se puede desmarcar |
| Pulsar "← Atrás" | Vuelve a fase 1 |
| Continuar a fase 3 | Campo de texto "BORRAR" |
| Escribir "borrar" (minúsculas) | Campo se pone rojo momentáneamente |
| Escribir "BORRAR" + Enter | Reset se ejecuta |
| CSV descargado | Archivo `cronometro-psp-YYYY-MM-DD.csv` |
| Tras reset | Modal se cierra |
| Pestaña Frecuentes | Vacía (mensaje de estado vacío) |
| Pestañas de actividades archivadas | Desaparecen |
| Pestañas de actividades conservadas | Siguen visibles |
| Header | "Ninguna tarea activa" + "--:--" |
| Barra inferior | "Hoy: 0:00" |

---

### UI-012: Responsive en móvil
**Requisito**: RNF-003

| Paso | Resultado esperado |
|------|-------------------|
| Abrir en móvil | Sin zoom automático |
| Grid de tareas | 3 columnas exactas |
| Tarjetas | Pulsables cómodamente con dedo |
| Pestañas | Scroll horizontal con swipe |
| Modales | Se abren centrados, sin desbordamiento |

---

### UI-013: Persistencia al recargar
**Requisito**: RF-012

| Paso | Resultado esperado |
|------|-------------------|
| Iniciar sesión | Timer en marcha |
| Cerrar pestaña del navegador | - |
| Reabrir la URL | Sesión activa recuperada (si no pasó medianoche) |
| Header | Muestra la tarea que había activa |
| Timer | Muestra tiempo correcto (corregido con server_time) |

---

## 4. Checklist de Regresión (post-deploy)

Ejecutar tras cada deploy relevante:

```
□ API-001: Health check responde OK
□ API-002: GET /actividades devuelve campo "permanente"
□ API-004: GET /tipos-tarea solo devuelve archived=0
□ API-005: GET sesión activa incluye server_time
□ UI-001: Carga inicial < 3 segundos en LAN
□ UI-002: Iniciar tarea unívoca — modal de comentario aparece
□ UI-003: Comentario aparece en barra inferior
□ UI-005: Navegación por pestañas funciona
□ UI-006: Tiempo acumulado se muestra en tarjetas
□ UI-009: Menú ⚙️ abre correctamente
□ UI-011: Puesta a cero — las 3 fases funcionan
□ UI-012: Móvil — 3 columnas, sin zoom, sin desbordamiento
□ UI-013: Recarga — sesión activa se recupera
```

---

## 5. Tests Unitarios JS (pendiente implementar)

La siguiente lógica de `app.js` es candidata a tests unitarios cuando se configure Vitest:

### UNIT-001: formatearDuracion
**Requisito**: RF-003, RF-004
```javascript
describe('formatearDuracion', () => {
    test('0 segundos → "0:00"', () => {
        expect(formatearDuracion(0)).toBe('0:00');
    });
    test('3600 segundos → "1:00"', () => {
        expect(formatearDuracion(3600)).toBe('1:00');
    });
    test('3661 segundos → "1:01"', () => {
        expect(formatearDuracion(3661)).toBe('1:01');
    });
    test('90 segundos → "0:01"', () => {
        expect(formatearDuracion(90)).toBe('0:01');
    });
});
```

---

### UNIT-002: getTareasFrecuentes
**Requisito**: RF-006
```javascript
describe('getTareasFrecuentes', () => {
    test('devuelve vacío si todos usos_7d = 0', () => {
        // Simular AppState con tiposTarea todos usos_7d=0
        AppState.tiposTarea = [
            { id: 't1', usos_7d: 0 },
            { id: 't2', usos_7d: 0 }
        ];
        AppState.sesionesHoy = [];
        expect(getTareasFrecuentes()).toHaveLength(0);
    });

    test('devuelve solo los de usos_7d > 0, máximo 8', () => {
        AppState.tiposTarea = Array.from({length: 10}, (_, i) => ({
            id: `t${i}`, usos_7d: i + 1
        }));
        AppState.sesionesHoy = [];
        const result = getTareasFrecuentes();
        expect(result).toHaveLength(8);
        expect(result[0].usos_7d).toBeGreaterThanOrEqual(result[7].usos_7d);
    });
});
```

---

### UNIT-003: getTiempoHoyTipoTarea
**Requisito**: RF-004
```javascript
describe('getTiempoHoyTipoTarea', () => {
    test('suma sesiones cerradas del mismo tipo', () => {
        AppState.sesionesHoy = [
            { tipo_tarea_id: 'codificar', duracion: '3600' }, // string (PDO)
            { tipo_tarea_id: 'codificar', duracion: '1800' },
            { tipo_tarea_id: 'email', duracion: '900' }
        ];
        AppState.sesionActiva = null;
        expect(getTiempoHoyTipoTarea('codificar')).toBe(5400);
    });

    test('no suma sesiones de otro tipo', () => {
        AppState.sesionesHoy = [
            { tipo_tarea_id: 'email', duracion: '900' }
        ];
        AppState.sesionActiva = null;
        expect(getTiempoHoyTipoTarea('codificar')).toBe(0);
    });
});
```

---

### UNIT-004: clockOffset en iniciarSesion
**Requisito**: RNF-007
```javascript
describe('Compensación de reloj', () => {
    test('inicio se ajusta con clockOffset y se trunca al minuto', () => {
        // server_time adelantado 61 segundos respecto al cliente
        const serverTime = Math.floor(Date.now() / 1000) + 61;
        const inicioServer = serverTime - 5; // hace 5 segundos según el servidor

        const clockOffset = serverTime * 1000 - Date.now();
        const inicioMs = inicioServer * 1000 - clockOffset;
        const inicioTruncado = Math.floor(inicioMs / 60000) * 60000;

        expect(inicioTruncado % 60000).toBe(0); // truncado al minuto
    });
});
```

---

## 6. Configuración futura de Vitest

Cuando se quiera automatizar los tests unitarios de JS:

```javascript
// vitest.config.js
import { defineConfig } from 'vitest/config';

export default defineConfig({
    test: {
        globals: true,
        environment: 'jsdom',
        setupFiles: './tests/setup.js',
    }
});
```

```javascript
// tests/setup.js
// Mock de la API — el código real hace fetch() al NAS
global.fetch = vi.fn();

// Mock de api client para tests unitarios
vi.mock('../frontend/js/api-client.js', () => ({
    api: {
        getActividades: vi.fn(),
        getTiposTarea: vi.fn(),
        getSesiones: vi.fn(),
        getSesionActiva: vi.fn(),
    }
}));
```

---

## 7. Criterios de Aceptación de Tests

### Para el MVP (pruebas manuales):
✅ Todos los checks de la **sección 4 (Checklist de Regresión)** pasan en Chrome desktop
✅ UI-011 (Puesta a cero completa) funciona en móvil
✅ UI-013 (Persistencia al recargar) funciona correctamente
✅ API-009 (Reset via curl) devuelve CSV válido

### Para versiones futuras (tests automatizados):
⏳ Suite de tests unitarios UNIT-001 a UNIT-004 pasan
⏳ Cobertura de lógica JS > 70%
⏳ Tests de API ejecutables sin intervención manual

---

**Fin del Plan de Pruebas v1.1**
