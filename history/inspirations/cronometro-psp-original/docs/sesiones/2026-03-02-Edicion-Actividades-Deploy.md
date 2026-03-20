# Sesión 2026-03-02 — Edición de actividades, deploy-nas.sh y rsync

## Qué se hizo

### 1. Editar actividades (nuevo)

El backend (`PUT /api/actividades?id=`) y el api-client (`actualizarActividad`) ya existían. Faltaba la UI completa.

**Implementado:**
- Modal `#editActivityModal` en `index.html` (nombre, color picker, checkbox permanente)
- En modo ✏️, clic en pestaña de actividad → abre el modal pre-rellenado con los datos actuales
- CSS: en edit-mode, las pestañas de actividad muestran color rosa + ✏️ (indicador visual)
- Nuevas funciones en `app.js`:
  - `mostrarModalEditarActividad(id)`: pre-rellena el modal
  - `cerrarModalEditarActividad()`
  - `seleccionarColorEdit(color)`: actualiza `AppState.colorEditSeleccionado`
  - `guardarEdicionActividad()`: llama a `api.actualizarActividad`, recarga datos, cierra modal

**Verificado en dev local:**
- `PUT /api/actividades?id=ad → 200 OK`
- Modal se abre con datos correctos (nombre, color seleccionado, permanente)
- Tras guardar, modal se cierra y UI se actualiza

### 2. Editar tipos de tarea

Ya existía completamente (modal `#editTaskModal`, funciones `mostrarModalEditarTarea`/`guardarEdicionTarea`, backend `PATCH /api/tipos-tarea`). Verificado: funciona correctamente en modo edición.

### 3. deploy-nas.sh: sincronizar disco persistente

**Problema**: después de cada deploy, `/mnt/HD/HD_a2/.cronometro-psp/app/` quedaba desactualizado. Si el NAS reiniciaba, `autorestaurar.sh` restauraba la versión antigua.

**Solución**: nuevo paso en `deploy-nas.sh` tras subir los scripts de mantenimiento:
```bash
$SSH "
    mkdir -p $DISK_DIR/app/www/css $DISK_DIR/app/www/js
    mkdir -p $DISK_DIR/app/api/db  $DISK_DIR/app/api/api
    cp -r $NAS_PATH/www/. $DISK_DIR/app/www/
    cp -r $NAS_PATH/api/. $DISK_DIR/app/api/
"
```

**Bonus**: añadido fallback en la búsqueda de clave SSH:
- Antes: solo `/mnt/c/Users/cpcxb/.ssh/id_nas` (ruta WSL con `/mnt/c/`)
- Ahora: también busca en `$HOME/.ssh/id_nas` (funciona desde bash de Windows)

### 4. rsync al NAS secundario

**Estado**: parcialmente completado.

- ✅ Clave SSH generada en el NAS primario: `/mnt/HD/HD_a2/.cronometro-psp/ssl/id_backup`
  - Fingerprint: `SHA256:kOJulvG9+ObvqKeWtHmv8zXs9rXC5yFus6NHz6C6mO8`
  - Pubkey: `ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIObEocDmdiDqF7WmJ/8qOQq+OWhBI8C0uhOKh0GeCsqu cronometro-psp-backup`
- ✅ NAS secundario (192.168.1.68 y .75) son alcanzables por ping desde el NAS primario
- ❌ SSH cerrado en el NAS secundario (puerto 22, 222, 2222 rechazados)
- ✅ **Completado en la misma sesión** — ver sección siguiente.

### 4b. rsync activado — detalles del NAS secundario (descubierto en sesión)

El NAS secundario resultó ser un **WD My Cloud antiguo** con características sorprendentes:

| Característica | Valor |
|---|---|
| IP | 192.168.1.68 |
| Hardware | ARM Cortex-A9 Comcerto 2000 |
| RAM | 226 MB total / ~44 MB libres |
| Kernel | Linux 3.2.26, julio 2015 |
| SSH | Solo `ssh-rsa`/`ssh-dss` (Dropbear legacy) |
| Usuario SSH | `root` (no `admin`) |
| Apache | 2.4.10 en :80/:443 (sirve UI de WD) |
| PHP | 5.6.7 con PDO + pdo_sqlite + json ✅ |
| sqlite3 CLI | `/usr/bin/sqlite3` ✅ |
| Disco | 2.7 TB en `/DataVolume`, ~2.3 TB libres |

**Incidencias durante la configuración:**
1. `nc -w 3` en BusyBox del NAS primario daba falsos negativos → puerto 22 sí estaba abierto, verificado por SSH directo
2. Clave ed25519 rechazada por el My Cloud (solo acepta RSA) → regenerada como RSA 2048
3. `/tmp` en NAS primario tiene `noexec` → script `askpass.sh` colocado en `/mnt/HD/HD_a2/.cronometro-psp/ssl/`
4. `SSH_ASKPASS_REQUIRE` no disponible en OpenSSH del NAS primario → usar solo `DISPLAY=fake SSH_ASKPASS=`
5. `PubkeyAcceptedAlgorithms` no soportado en el cliente SSH del NAS primario → omitir esa opción

**Comando SSH correcto para conectar al secundario:**
```bash
ssh -o HostKeyAlgorithms=+ssh-rsa -o PubkeyAcceptedAlgorithms=+ssh-rsa root@192.168.1.68
# Desde el NAS primario (sin PubkeyAcceptedAlgorithms, no lo soporta):
ssh -i $KEY -o HostKeyAlgorithms=+ssh-rsa root@192.168.1.68
```

**¿Backend alternativo?** Técnicamente posible (Apache + PHP 5.6 + PDO + SQLite), pero no recomendado como servicio activo (PHP EOL, RAM justa, Apache ocupado). Usar solo como backup; despliegue manual de emergencia si falla el primario.

## Archivos modificados

| Archivo | Cambio |
|---|---|
| `frontend/js/app.js` | `colorEditSeleccionado` en AppState; click de pestañas detecta modo edición; 4 funciones de editar actividad (v16) |
| `frontend/index.html` | Modal `#editActivityModal`; CSS→v8, JS→v16 |
| `frontend/css/styles.css` | Pestañas en edit-mode: color rosa + `::after` con ✏️ (v8) |
| `scripts/deploy-nas.sh` | Paso de sincronización a `$DISK_DIR/app/`; fallback clave SSH `$HOME/.ssh/id_nas` |

## Notas técnicas

- El dev server (`preview_start` → WSL PHP) en esta sesión tenía problemas con `Failed to fetch` en los primeros recargas, pero la API funcionaba correctamente verificado con `preview_eval`
- La `preview_start` no admite rutas `/mnt/c/`; la clave SSH está en `~/.ssh/id_nas` en el contexto del bash de Windows
- Python3 no disponible como `python3` en el bash de Windows (alias redirige a Microsoft Store)

## Pendiente para la próxima sesión

- [ ] **Activar rsync**: habilitar SSH en NAS secundario e instalar pubkey (ver sección 4)
- [ ] Ver `docs/futuras-versiones.md` para próximas mejoras v1.4
