# Tests — Mi Cronómetro PSP

Tres niveles de tests, sin dependencias previas salvo PHP CLI y Node.js.

---

## Nivel 1: Tests unitarios PHP (PHPUnit)

Cubren la lógica de negocio crítica de `backend/api/sesiones.php`:
`iniciarSesion`, `detenerSesion` y `resetearDatos` en todos sus casos de borde.

### Instalación (una sola vez, desde WSL)

```bash
# PHP con extensión SQLite
sudo apt-get install -y php-cli php-sqlite3

# PHPUnit como PHAR (no requiere composer)
wget https://phar.phpunit.de/phpunit-11.phar -O tests/php/phpunit.phar
chmod +x tests/php/phpunit.phar
```

### Ejecución

```bash
# Desde la raíz del proyecto (WSL)
php tests/php/phpunit.phar --configuration tests/php/phpunit.xml
```

### Cómo funciona

`bootstrap.php` prepara el entorno antes de cada test:
- Define una clase `Database` alternativa que usa SQLite `:memory:` en lugar del NAS.
- Redefine `respondJson/respondError/respondSuccess` para lanzar excepciones
  (`TestResponseException`) en lugar de `exit`, permitiendo capturar respuestas.
- Redefine `enviarCsv` para lanzar `TestCsvResponseException` (permite testear `resetearDatos`).
- Suprime `header()` calls de `config.php` (inválidos en CLI).

Cada test en `SesionesTest.php` crea una BD fresca en memoria, inserta sus propios
fixtures y llama directamente a las funciones PHP bajo test.

---

## Nivel 2: Tests unitarios JavaScript (Node puro)

Cubren las funciones puras de `frontend/js/app.js`:
`formatearDuracion`, `getTareasFrecuentes`, `getTiempoHoyTipoTarea`, `getTiempoHoyTarea`.

### Requisitos

Node.js ≥ 18 (ya instalado). **Sin `npm install`.**

### Ejecución

```bash
# Desde Windows (CMD o PowerShell)
node tests\js\runner.js

# Desde WSL
node tests/js/runner.js
```

### Cómo funciona

`runner.js`:
- Define stubs globales (`document=null`, `fetch=null`, etc.) que `app.js` referencia.
- Carga `app.js` con `vm.runInThisContext` para que las funciones queden en el scope
  global de Node (las declaraciones `const` del top-level no son globales en módulos
  normales, pero sí en `vm.runInThisContext`).
- Parchea en memoria la última línea de `app.js` para desactivar el listener
  `DOMContentLoaded` (evita TypeError con `document=null`).
- Incluye un mini-framework de aserciones en ~50 líneas sin dependencias externas.

---

## Nivel 3: Smoke tests (bash + curl)

Verifican los flujos críticos contra el **NAS real** (192.168.1.71:8080).
Útiles para ejecutar tras cada deploy.

### Requisitos

- Estar en la red local (192.168.1.71 accesible).
- `curl` y `python3` disponibles (estándar en WSL Ubuntu).

### Ejecución

```bash
# Desde WSL, raíz del proyecto
bash tests/smoke/smoke-test.sh

# URL alternativa (ej. HTTPS externo)
bash tests/smoke/smoke-test.sh https://cronometro.hash-pointer.com/api/api
```

### Qué cubre (15 grupos de asserts)

| Smoke | Endpoint | Verificación |
|-------|----------|-------------|
| 000 | `/health` | Conectividad básica, versión, timestamp |
| 001 | `GET /actividades` | Campos obligatorios, al menos una actividad |
| 002 | `GET /tipos-tarea` | Campos obligatorios, al menos un tipo |
| 003 | `GET /sesiones?action=activa` | Respuesta válida; limpia sesión activa previa |
| 004 | `POST /sesiones?action=iniciar` | Modo normal: id, inicio, server_time, minutos_aplicados=0 |
| 005 | `GET /sesiones?action=activa` | Sesión visible tras iniciar |
| 006 | `POST /sesiones?action=iniciar` | Segunda sesión cierra la primera, id diferente |
| 007 | `POST /sesiones?action=iniciar` | Retroactivo 5 min, minutos_aplicados=5, inicio correcto |
| 008 | `POST /sesiones?action=iniciar` | Retroactivo recortado (sesión previa corta) |
| 009 | `POST /sesiones?action=iniciar` | Modo sustituir: mismo id, notas actualizadas |
| 010 | `POST /sesiones?action=detener` | Duración > 0, sesión activa = null después |
| 011 | `POST /sesiones?action=detener` | Sin sesión activa → HTTP 404 |
| 012 | `GET /sesiones?fecha=hoy` | Array con sesiones registradas |
| 013 | `GET /sesiones?action=estadisticas` | tiempo_total, por_tipo_tarea, por_tarea |
| 014 | `GET /sesiones?action=acumulado` | server_time, actividades |
| 015 | `POST /sesiones?action=iniciar` | Tarea inexistente → HTTP 404 |

---

## Ejecutar todo de una vez

```bash
# Desde WSL, raíz del proyecto
php tests/php/phpunit.phar --configuration tests/php/phpunit.xml \
  && node tests/js/runner.js \
  && bash tests/smoke/smoke-test.sh
```
