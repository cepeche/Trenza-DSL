# Historial de sesiones anteriores — Mi Cronómetro PSP

Registro compacto de estados alcanzados en sesiones previas.
Para el detalle completo de una sesión, ver el `.md` individual en esta misma carpeta.

---

## v1.3.0-dev — 26/02/2026 (noche) — Fix historial: recorte en medianoche

- `getHistorial()` reescrita: `MIN(fin,$finDia) - MAX(inicio,$inicioDia)` recorta cada sesión al día
- Días que antes mostraban 32h ahora muestran ~24h (24/02 y 22/02: exactamente 86 400 s)
- Sesión activa también distribuida correctamente entre todos los días que solapa
- **PDO + SQLite**: `MIN(col, ?)` no funciona como placeholder en SELECT → embeber enteros directamente
- **Estructura NAS**: Apache sirve desde `api/api/` (doble); hay 3 copias del PHP; la de HD es fuente de verdad para reinicios
- Commit: `256b7eb`; sesión detallada: `2026-02-26-Fix-Historial-Medianoche.md`

## v1.3.0-dev — 26/02/2026 (mañana) — Backup NAS secundario

- `scripts/backup.sh` reescrito: BusyBox sh, `cp` para snapshot, rsync opcional vía SSH al NAS secundario
- Paso 14 en `autorestaurar.sh`: llama a `backup.sh` una vez al día (idempotente vía `/tmp/cronometro-backup-lastrun.txt`)
- `scripts/deploy-nas.sh` actualizado: también sube `autorestaurar.sh` y `backup.sh` al NAS
- Verificado en NAS: snapshot en `/mnt/HD/HD_a2/.cronometro-psp/data/backups/`; rsync omitido limpiamente (sin `id_backup`)
- **Para activar rsync**: generar `id_backup` en el NAS e instalar pubkey en NAS secundario (ver cabecera de `backup.sh`)
- Reorganización MEMORY.md: historial de estados movido a este archivo; instrucción de inicio de sesión añadida
- Commits: `102d92b` (docs), `3abb565` (feat backup)
- Sesión detallada: `2026-02-26-Backup-NAS-Secundario.md`

## v1.2.2 — 25/02/2026

- Documentación SSL/mTLS creada en `docs/configuracion-ssl-mtls.md` (commit 06f4294)
- `autorestaurar.sh` añadido al repo (`scripts/`) y bug corregido: `SSL_CONF_SRC` y `HTTP_CONF_SRC` apuntaban a `/` en vez de `$DISK_DIR` → sin el fix los VirtualHost SSL/HTTP no se restauraban tras reinicio
- `autorestaurar.sh` corregido también en el NAS (subido directamente por SCP)
- `futuras-versiones.md` desactualizado: acceso remoto HTTPS+DDNS ya operativo; autenticación ya implementada (mTLS con CA privada + certificado de cliente `cpcxb-movil`)
- CA privada: C=ES, O=Mi Cronometro PSP, CN=Cronometro-CA, válida hasta 2036-02-18
- Cert servidor Let's Encrypt: caduca 2026-05-20, renovación automática vía acme.sh
- Cert de cliente instalado en móvil principal (`client.p12`, CN=cpcxb-movil)
- GoDaddy API key/secret: en `ddns-update.sh` y `.acme.sh/account.conf` en el NAS (NO en el repo)
- `LICENSES.md` creado (commit ef8fb0e)
- `deploy-nas.sh` ejecutado con éxito: backend fixes en producción

## v1.2.1 — 23/02/2026

- Suite de tests automatizados creada y funcionando (commit 9f2f512)
- PHP (PHPUnit 11): 25 tests, 73 assertions — todos en verde
- JS (Node.js runner propio): 29 tests — todos en verde
- Smoke bash: 48/49 — SMOKE-015 falla por intermitencia de red WSL, no es un bug real
- Backend fixes para compatibilidad PHP 8.3: funciones en `config.php` envueltas en `function_exists()`, clase `Database` en `class_exists()`, `enviarCsv()` en `sesiones.php` ídem; `clamp max(0,…)` en `minutos_aplicados`
- Licencias documentadas en `LICENSES.md`
- PHPUnit phar en `tests/php/phpunit.phar`; ejecutar desde WSL:
  `wsl -u root bash -c "php /tmp/phpunit.phar --configuration /mnt/c/ProyectosClaude/mi-cronometro-psp/tests/php/phpunit.xml"`
- Tests JS: `wsl -u root bash -c "node /mnt/c/ProyectosClaude/mi-cronometro-psp/tests/js/runner.js"`
- Smoke tests: `wsl bash /mnt/c/ProyectosClaude/mi-cronometro-psp/tests/smoke/smoke-test.sh`
- Versiones en producción (NAS): `styles.css v7`, `api-client.js v6`, `app.js v14`
- Último commit antes de tests: 2c7d6b3

## v1.1.0 — 21/02/2026

- Git local inicializado; tag v1.0 en estado MVP; rama master con commits incrementales
- Modo edición: botón ✏️ junto al + (zona inferior dcha), fondo rosado, clic directo sin delay
- Grid: 4 columnas fijas, tarjetas compactas (texto 0.72rem, min-height 80px, icono intacto)
- Pestañas con `flex-wrap` (varias filas si hay muchas actividades)
- Táctil: umbral 10px de movimiento para cancelar tap (evita activación al hacer scroll)
- `app.js` en v=11, `styles.css` en v=5, `api-client.js` en v=4
- Bug corregido: `detenerSesion()` incluye `tarea_id` en SELECT → `usos_7d` se incrementa al detener

## v1.0.0 — 19/02/2026

- Menú Configuración operativo: "Acerca de" y "Puesta a cero" implementados
- Comentario opcional al iniciar tarea (se muestra en barra inferior)
- Barra inferior: comentario activo a la izquierda + "Hoy: H:MM" a la derecha
- Pestañas de actividades generadas dinámicamente desde la BD (no hardcodeadas)
- Frecuentes muestra solo tareas con `usos_7d > 0`; tras Puesta a cero queda vacía
- Puesta a cero con 3 fases: aviso → selección de actividades a conservar → confirmación BORRAR
- Actividades con campo `permanente` (BD + toggle al crear); las permanentes aparecen pre-marcadas en reset
- Reset archiva actividades no marcadas y `tipos_tarea` que quedan sin actividades activas

## 18/02/2026

- App funcional desplegada en `http://192.168.1.71:8080/apps/cronometro/www/`
- Reloj del NAS sincronizado con NTP (cron cada 5 min en `autorestaurar.sh`)
- Scripts de deploy y recuperación usan clave SSH desde Windows (no más petición de contraseña)
