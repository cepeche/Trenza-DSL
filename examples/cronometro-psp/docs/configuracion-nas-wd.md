# Configuración del NAS WD My Cloud EX2 Ultra
## Mi Cronómetro PSP — Despliegue en red local

---

## PARTE 1: CONFIGURACIÓN FINAL (ESTADO ACTUAL)

Esta sección describe cómo ha quedado el sistema tras las sesiones del 17/02/2026.

---

### 1.1 Arquitectura general

```
[PC / Móvil en WiFi]
        │
        │  HTTP :8080
        ▼
[WD My Cloud EX2 Ultra — 192.168.1.71]
        │
        │  Apache 2.4.56 escucha en 0.0.0.0:8080
        │  DocumentRoot: /var/www/
        │
        ├── /var/www/apps/cronometro/www/        ← Frontend estático  ⚠️ RAM
        │       index.html
        │       css/styles.css
        │       js/api-client.js
        │       js/app.js
        │
        ├── /var/www/apps/cronometro/api/        ← Backend PHP        ⚠️ RAM
        │       index.php          (router principal)
        │       config.php
        │       .htaccess          (rewrite a index.php)
        │       api/
        │           actividades.php
        │           tipos-tarea.php
        │           sesiones.php
        │       db/
        │           database.php
        │           schema.sql
        │
        └── /mnt/HD/HD_a2/.cronometro-psp/      ← DISCO PERSISTENTE ✅
                data/
                    cronometro.db      (SQLite3 — datos permanentes)
                app/                   (copia completa frontend + backend)
                autorestaurar.sh       (script de autorecuperación — ejecutado por cron)

/usr/local/config/cronometro-psp/               ← UBIFS PERSISTENTE ✅
        authorized_keys                (clave pública SSH)
        apache-extra.conf              (bloques Directory + Location)
        cron_init.sh                   (registra autorestaurar.sh en crontab al arrancar)
        custom_booting_init.sh         (copia de respaldo del hook de arranque WD)

/usr/local/sbin/custom_booting_init.sh          ← RAM (se restaura desde UBIFS automát.) ⚠️
        → llama a cron_init.sh en background al arrancar
```

> **Nota crítica sobre la volatilidad de `/var/www/`**: ver Apéndice A.

---

### 1.2 Entorno del NAS

| Componente | Versión / Valor |
|---|---|
| Hardware | WD My Cloud EX2 Ultra |
| Sistema operativo | Linux (Debian-based, BusyBox/ash 1.30.1) |
| Apache | 2.4.56 |
| PHP | 7.4.33 |
| PHP-FPM | Socket unix: `/var/run/php-fpm.socket` |
| SQLite | PDO SQLite (vía PHP) — sin sqlite3 CLI |
| ModSecurity | OWASP CRS 3.3.0 (activo) |
| IP del NAS | 192.168.1.71 (fija en LAN) |
| Usuario SSH | `sshd` (mapea a root internamente) |

---

### 1.3 Puertos en uso

| Puerto | Servicio | Accesible desde |
|---|---|---|
| 80 | `nasAdmin` (binario propietario WD) | Red local y WAN |
| 8000 | Apache (interfaz interna WD) | Solo `127.0.0.1` |
| **8080** | **Apache (nuestra app)** | **Red local (192.168.1.x)** |

---

### 1.4 Configuración de Apache

#### Fichero: `/usr/local/apache2/conf/extra/ports.conf`

```apache
Listen 127.0.0.1:8000
Listen 8080
```

> ⚠️ **Este fichero vive en RAM.** Se pierde en cada reinicio del NAS. El script `autorestaurar.sh` lo restaura automáticamente. Ver sección 1.8.

---

#### Fichero: `/usr/local/apache2/conf/httpd.conf` (bloques añadidos al final)

```apache
<Directory "/var/www/apps/cronometro">
    Options +FollowSymLinks -Indexes
    AllowOverride All
    AuthType None
    <RequireAll>
        Require ip 192.168.1
    </RequireAll>
</Directory>

# Mi Cronómetro PSP - acceso red local via Location (mayor prioridad)
<Location "/apps/cronometro">
    AuthType None
    <RequireAll>
        Require ip 192.168.1
    </RequireAll>
</Location>
```

> ⚠️ **Este fichero vive en RAM.** Se pierde en cada reinicio del NAS. La copia persistente está en `/usr/local/config/cronometro-psp/apache-extra.conf`.

- `AllowOverride All` es necesario para que el `.htaccess` del backend funcione.
- `Require ip 192.168.1` restringe el acceso a la subred local.
- El bloque `<Location>` tiene mayor prioridad que `<Directory>` y permite sobreescribir las restricciones del directorio padre.

---

#### Fichero: `/var/www/apps/cronometro/api/.htaccess`

```apache
RewriteEngine On

# Redirigir todas las peticiones (excepto archivos existentes) a index.php
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_FILENAME} !-d
RewriteRule ^ index.php [QSA,L]
```

> ⚠️ **Este fichero vive en RAM.** Se restaura automáticamente por `autorestaurar.sh`.

---

### 1.5 PHP-FPM

PHP-FPM estaba preconfigurado en Apache. El módulo `proxy_fcgi` enruta automáticamente todos los archivos `.php` al socket unix:

**Fichero:** `/usr/local/apache2/conf/mods-enabled/proxy_php-fpm.conf`

```apache
<IfModule proxy_fcgi_module>
    <FilesMatch ".+\.php$">
        SetHandler "proxy:unix:/var/run/php-fpm.socket|fcgi://localhost"
    </FilesMatch>
</IfModule>
```

Esta configuración es global y aplica a cualquier `.php` bajo el `DocumentRoot`.

---

### 1.6 Base de datos SQLite

- **Ruta:** `/mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db`
- **Persistencia:** ✅ Disco duro físico — sobrevive reinicios
- **Permisos:** `666` (lectura y escritura para el proceso PHP-FPM)
- **Tablas:** `actividades`, `tipos_tarea`, `tareas`, `sesiones`

`database.php` detecta automáticamente si existe el directorio persistente del NAS y usa la ruta del disco; en caso contrario (entorno de desarrollo local) usa la ruta relativa `data/cronometro.db`:

```php
$persistentPath = '/mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db';
$localPath      = __DIR__ . '/../../data/cronometro.db';
$this->dbPath   = is_dir(dirname($persistentPath)) ? $persistentPath : $localPath;
```

---

### 1.7 Frontend — URL del backend

**Fichero:** `frontend/js/api-client.js`

```javascript
const API_CONFIG = {
    baseURL: window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1'
        ? 'http://localhost:8080/apps/cronometro/api'
        : `http://${window.location.hostname}:8080/apps/cronometro/api`,
    timeout: 10000
};
```

El puerto `:8080` se incluye explícitamente porque Apache no está en el puerto estándar (80).

---

### 1.8 Resiliencia ante reinicios

El NAS pierde toda la configuración de `/var/www/` y de Apache en cada reinicio (ver Apéndice A). El sistema de recuperación tiene **tres capas**, diseñadas para que el NAS se recupere completamente de forma autónoma tras cualquier corte de alimentación, sin intervención manual.

---

#### Capa 0 — Hook de arranque WD (`custom_booting_init.sh`)

`system_init`, el script de arranque del NAS, invoca el fichero `/usr/local/sbin/custom_booting_init.sh` como gancho oficial para personalizaciones de usuario (detectado en la línea 1162 del script de arranque). En fábrica es un symlink a una shell vacía en SquashFS.

Hemos sustituido ese symlink por un fichero real que arranca `cron_init.sh` en background:

```sh
#!/bin/sh
# Hook oficial WD — Mi Cronómetro PSP
if [ -f /usr/local/config/cronometro-psp/cron_init.sh ]; then
    /bin/sh /usr/local/config/cronometro-psp/cron_init.sh &
fi
```

Una copia de respaldo se guarda en UBIFS (`/usr/local/config/cronometro-psp/custom_booting_init.sh`). Si algún arranque volviera a reemplazarlo por symlink, `autorestaurar.sh` lo detecta y restaura la versión real.

> ⚠️ **Restricción de timing**: En el momento en que se ejecuta el hook, el disco duro (`/mnt/HD/HD_a2/`) aún **no está montado**. Por eso el hook no llama directamente a `autorestaurar.sh`, sino a `cron_init.sh` (que sí está en UBIFS, disponible en ese momento).

---

#### `cron_init.sh` (UBIFS — disponible desde el primer instante)

```sh
CRON_FILE=/var/spool/cron/crontabs/root
CRON_LINE="*/5 * * * * /mnt/HD/HD_a2/.cronometro-psp/autorestaurar.sh"

# Esperar a que wd_compinit -p cree el fichero de crontab (máx. 120 s)
WAIT=0
while [ ! -f "$CRON_FILE" ] && [ $WAIT -lt 120 ]; do
    sleep 5
    WAIT=$((WAIT + 5))
done

if [ -f "$CRON_FILE" ] && ! grep -q 'autorestaurar' "$CRON_FILE"; then
    echo "$CRON_LINE" >> "$CRON_FILE"
fi
```

Una vez que `wd_compinit -p` crea el crontab de root (con las entradas propias de WD), `cron_init.sh` añade la entrada de `autorestaurar.sh`. A partir de ahí, el cron ejecuta el script cada 5 minutos y completa el resto de la recuperación.

---

#### Capa 1 — Autorestauración automática (`autorestaurar.sh`)

Un script en el disco duro se ejecuta vía crontab cada 5 minutos:

```
/mnt/HD/HD_a2/.cronometro-psp/autorestaurar.sh
```

El script detecta qué falta y restaura solo lo necesario. En condiciones normales termina en menos de 0.1 segundos sin hacer nada. Lo que restaura:

1. **`authorized_keys`** desde `/usr/local/config/cronometro-psp/` (UBIFS)
2. **`Listen 8080`** en `ports.conf`
3. **Bloques `<Directory>` y `<Location>`** en `httpd.conf` (desde UBIFS)
4. **Archivos de la app** (frontend + backend) desde `/mnt/HD/HD_a2/.cronometro-psp/app/`
   *(usa `cp -r "src/."` en lugar de `src/*` para incluir `.htaccess` y otros ficheros ocultos)*
5. **Recarga Apache** si hubo cambios de configuración
6. **`custom_booting_init.sh`** si volvió a ser un symlink (restaura la versión real desde UBIFS)
7. **Entrada en crontab** si desapareciera (idempotente, se re-añade automáticamente)

La BD no necesita restaurarse: vive directamente en el disco persistente.

Log en: `/tmp/cronometro-psp-restore.log`

---

#### Capa 2 — Recuperación manual desde PC (fallback)

Si las capas anteriores no pudieran completarse por algún motivo excepcional:

```bash
# Desde la raíz del proyecto en WSL:
bash scripts/recuperar-nas.sh

# Solo reconfigurar Apache (sin re-deploy):
bash scripts/recuperar-nas.sh --solo-apache
```

El script verifica cada componente, actúa solo en lo que falta, y finaliza con un health check HTTP.

---

#### Secuencia completa tras un corte de alimentación

```
Arranque NAS
    │
    ├─ system_init ejecuta custom_booting_init.sh (UBIFS disponible, disco NO)
    │       └─ cron_init.sh &  (en background)
    │
    ├─ wd_compinit -p crea /var/spool/cron/crontabs/root
    │       └─ cron_init.sh detecta el fichero y añade entrada de autorestaurar.sh
    │
    ├─ El disco duro se monta en /mnt/HD/HD_a2/
    │
    └─ ~5 min: crond ejecuta autorestaurar.sh por primera vez
            ├─ Restaura authorized_keys  → SSH sin contraseña
            ├─ Restaura ports.conf       → Listen 8080
            ├─ Restaura httpd.conf       → bloques Directory + Location
            ├─ Restaura archivos de app  → frontend y backend en /var/www/
            ├─ Recarga Apache
            └─ App disponible en http://192.168.1.71:8080/apps/cronometro/www/
```

> **Resultado:** Tras un corte de alimentación, la app se recupera automáticamente en un máximo de ~5-10 minutos. **No es necesaria ninguna intervención manual.**

---

### 1.9 Acceso SSH

- **Usuario:** `sshd` (mapea a root en el NAS)
- **Clave privada:** `C:\Users\cpcxb\.ssh\id_nas` (ed25519, fuera del repositorio — nunca commitear)
- **Clave pública guardada en:** `/usr/local/config/cronometro-psp/authorized_keys` (UBIFS persistente)
- **Clave pública activa en:** `~/.ssh/authorized_keys` (`/home/root/.ssh/` — RAM, se restaura automáticamente)

Comando de conexión:
```bash
ssh -i ~/.ssh/id_nas sshd@192.168.1.71
```

Para regenerar la clave si se pierde:
```powershell
# En PowerShell — generar nueva clave
ssh-keygen -t ed25519 -f C:\Users\cpcxb\.ssh\id_nas

# Copiar al NAS (pide contraseña una sola vez)
type C:\Users\cpcxb\.ssh\id_nas.pub | ssh sshd@192.168.1.71 "mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys"

# Guardar también en UBIFS para persistencia automática
ssh sshd@192.168.1.71 "cp ~/.ssh/authorized_keys /usr/local/config/cronometro-psp/authorized_keys"
```

---

### 1.10 URLs de acceso

| Recurso | URL |
|---|---|
| **Aplicación (PC/móvil)** | `http://192.168.1.71:8080/apps/cronometro/www/` |
| API health check | `http://192.168.1.71:8080/apps/cronometro/api/api/health` |
| API actividades | `http://192.168.1.71:8080/apps/cronometro/api/api/actividades` |
| API sesiones | `http://192.168.1.71:8080/apps/cronometro/api/api/sesiones` |

> **Nota:** El puerto 8080 debe especificarse siempre.

---

### 1.11 Script de backup

`scripts/backup.sh` está diseñado para ejecutarse en el propio NAS (vía cron). Realiza:

1. Copia de la BD con `cp` (sqlite3 CLI no disponible en el NAS)
2. Compresión con gzip
3. Limpieza de backups con más de 30 días de antigüedad
4. Log de operaciones

Rutas:
- BD fuente: `/mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db`
- Backups: `/mnt/HD/HD_a2/.cronometro-psp/data/backups/`
- Log: `/mnt/HD/HD_a2/.cronometro-psp/logs/backup.log`

Para activar en cron del NAS:
```bash
# Añadir al crontab de root en el NAS:
0 3 * * * /mnt/HD/HD_a2/.cronometro-psp/autorestaurar.sh  # ya existe
0 2 * * * /bin/sh /mnt/HD/HD_a2/.cronometro-psp/backup.sh
```

> ⚠️ El crontab vive en RAM; la entrada se pierde en reinicios. Pendiente: añadir el backup al script `autorestaurar.sh` para que registre también esta entrada.

---

### 1.12 Estructura completa en disco persistente

```
/mnt/HD/HD_a2/.cronometro-psp/        ← Disco físico (ext4, siempre persistente)
├── autorestaurar.sh                   ← Script de autorecuperación (ejecutado por cron)
├── data/
│   ├── cronometro.db                  ← Base de datos SQLite (NUNCA se pierde)
│   └── backups/                       ← Backups automáticos (pendiente activar)
├── logs/
│   └── backup.log
└── app/                               ← Copia completa de la app (fuente para restaurar)
    ├── www/
    │   ├── index.html
    │   ├── css/styles.css
    │   └── js/
    │       ├── api-client.js
    │       └── app.js
    └── api/
        ├── index.php
        ├── config.php
        ├── .htaccess
        ├── db/
        │   ├── database.php
        │   └── schema.sql
        └── api/
            ├── actividades.php
            ├── tipos-tarea.php
            └── sesiones.php

/usr/local/config/cronometro-psp/      ← UBIFS (flash NOR, persistente)
├── authorized_keys                    ← Clave pública SSH del PC de desarrollo
├── apache-extra.conf                  ← Bloques Directory + Location para Apache
├── cron_init.sh                       ← Script que añade autorestaurar.sh al crontab en arranque
└── custom_booting_init.sh             ← Copia de respaldo del hook de arranque WD
```

---

## PARTE 2: PROBLEMAS ENCONTRADOS Y DECISIONES DE CONFIGURACIÓN

Esta sección documenta los obstáculos encontrados durante el despliegue y las decisiones tomadas para resolverlos.

---

### Problema 1: El NAS no es Synology

**Situación:** La documentación y los scripts se redactaron inicialmente asumiendo un NAS Synology (DSM 7.x, Web Station, rutas `/volume1/web/`, usuario `admin`).

**Realidad:** El NAS es un WD My Cloud EX2 Ultra con un sistema Linux Debian-based propio, sin DSM, con rutas completamente diferentes.

**Decisión:** Reescribir todos los scripts y documentación:
- Rutas: `/shares/Public/` → `/var/www/apps/` (y luego BD a `/mnt/HD/HD_a2/`)
- Usuario: `admin` → `sshd`
- Web Station → Apache nativo del NAS
- Synology PHP → PHP-FPM nativo

---

### Problema 2: Apache solo escuchaba en localhost

**Situación:** Apache estaba configurado para escuchar únicamente en `127.0.0.1:8000`.

**Causa:** El WD My Cloud EX2 Ultra usa `nasAdmin` como proxy frontal en el puerto 80. Apache estaba aislado, solo accesible desde el propio NAS.

**Arquitectura original:**
```
[Red local] → puerto 80 → nasAdmin (propietario WD) → proxy → Apache (127.0.0.1:8000)
```

**Decisión:** Añadir `Listen 8080` en `ports.conf` para abrir un puerto adicional accesible desde la red, sin tocar `nasAdmin`.

---

### Problema 3: Error 403 — `Require local` en el directorio padre

**Situación:** El bloque `<Directory "/var/www/apps/cronometro">` con `Require ip 192.168.1` no era suficiente: seguían llegando 403 desde la red.

**Causa:** Apache evalúa los `<RequireAll>` de forma acumulativa. El bloque padre `<Directory "/var/www">` tenía `Require local`, que bloqueaba antes de que el bloque hijo pudiera actuar.

**Decisión:** Añadir un bloque `<Location "/apps/cronometro">` que tiene mayor prioridad que `<Directory>` y puede sobreescribir las restricciones del padre.

---

### Problema 4: Error 404 en la API

**Situación:** El frontend cargaba (200 OK) pero la API devolvía 404 para cualquier endpoint.

**Causa:** Sin rewrite rules, Apache busca archivos físicos. La ruta `/apps/cronometro/api/api/health` no existe como fichero.

**Decisión:** Crear `.htaccess` con `RewriteRule ^ index.php [QSA,L]` en la carpeta `api/`.

---

### Problema 5: `AllowOverride None` bloqueaba el `.htaccess`

**Situación:** El `.htaccess` no tenía efecto aunque estaba correctamente escrito.

**Causa:** `AllowOverride None` en el bloque `<Directory>` impide que Apache procese ficheros `.htaccess`.

**Decisión:** Cambiar a `AllowOverride All`.

---

### Problema 6: BD vacía sin schema (primera instalación)

**Situación:** API respondía al health check pero los endpoints devolvían `no such table: actividades`.

**Causa:** `database.php` solo ejecuta `initializeSchema()` si el fichero `.db` **no existía** previamente. El script de deploy creaba el fichero vacío con `touch` antes del primer request, así que PHP nunca aplicaba el schema.

**Decisión:** Inicializar el schema manualmente vía PHP CLI. Como mejora, `database.php` debería verificar si las tablas existen, no solo si el fichero existe:

```php
$tables = $db->query("SELECT count(*) FROM sqlite_master WHERE type='table'")->fetchColumn();
if ($tables == 0) {
    $this->initializeSchema();
}
```

---

### Problema 7: `sqlite3` CLI no disponible en el NAS

**Situación:** `sqlite3 cronometro.db < schema.sql` → `sh: sqlite3: not found`

**Causa:** El NAS usa BusyBox y no tiene el cliente CLI de SQLite, aunque PHP sí tiene soporte PDO SQLite.

**Decisión:** Toda manipulación de la BD debe hacerse via PHP (scripts temporales o vía la propia API).

---

### Problema 8: Comando incorrecto para reiniciar Apache

```bash
# Incorrecto:
/usr/local/apache2/bin/apachectl graceful   # no existe
httpd -k graceful                            # busca config en ruta incorrecta

# Correcto:
httpd -f /usr/local/apache2/conf/httpd.conf -k graceful
```

---

### Problema 9: Clave SSH perdida tras reinicio (descubierto el 17/02/2026)

**Situación:** Tras un corte de alimentación, la autenticación por clave SSH dejó de funcionar. Había que introducir contraseña en cada conexión.

**Causa:** `~/.ssh/authorized_keys` (que en este NAS está en `/home/root/.ssh/`) vive en el sistema de ficheros raíz, que es un **ramdisk** (`/dev/ram0`). Se pierde en cada reinicio. Ver Apéndice A.

**Decisión:**
1. Guardar copia de `authorized_keys` en `/usr/local/config/cronometro-psp/` (partición UBIFS persistente).
2. El script `autorestaurar.sh` detecta si falta y la restaura automáticamente.

---

### Problema 10: Configuración Apache perdida tras reinicio (descubierto el 17/02/2026)

**Situación:** Tras el mismo corte, Apache volvió a escuchar solo en `127.0.0.1:8000` y `/var/www/apps/cronometro/` había desaparecido.

**Causa:** La misma que el problema 9: `/var/www/` y `/usr/local/apache2/conf/` viven en el ramdisk. Además, en cada arranque el sistema sobreescribe `/usr/local/apache2/` completo copiando desde `/usr/local/modules/apache2/` (imagen squashfs de solo lectura). Ver Apéndice A.

**Decisión:**
1. Guardar los bloques extra de Apache en `/usr/local/config/cronometro-psp/apache-extra.conf` (UBIFS).
2. Mover la BD al disco físico (`/mnt/HD/HD_a2/`).
3. Guardar copia completa de los archivos de la app en el disco físico.
4. Implementar sistema de autorestauración de dos capas (secciones 1.8 y scripts/).

---

### Resumen de ficheros modificados o creados en el NAS

| Fichero / Ruta | Tipo | Persistencia | Contenido |
|---|---|---|---|
| `/usr/local/apache2/conf/extra/ports.conf` | Modificado | ⚠️ RAM | Añadido `Listen 8080` |
| `/usr/local/apache2/conf/httpd.conf` | Modificado | ⚠️ RAM | Bloques `<Directory>` y `<Location>` |
| `/var/www/apps/cronometro/api/.htaccess` | Creado | ⚠️ RAM | Rewrite rules para router PHP |
| `/usr/local/sbin/custom_booting_init.sh` | Modificado | ⚠️ RAM | Reemplazado symlink→SquashFS por fichero real que llama a `cron_init.sh` |
| `/var/spool/cron/crontabs/root` | Modificado | ⚠️ RAM | Entrada `*/5 * * * *` para autorestaurar.sh |
| `/usr/local/config/cronometro-psp/authorized_keys` | Creado | ✅ UBIFS | Clave pública SSH |
| `/usr/local/config/cronometro-psp/apache-extra.conf` | Creado | ✅ UBIFS | Bloques Apache extra |
| `/usr/local/config/cronometro-psp/cron_init.sh` | Creado | ✅ UBIFS | Registra autorestaurar.sh en crontab al arrancar |
| `/usr/local/config/cronometro-psp/custom_booting_init.sh` | Creado | ✅ UBIFS | Copia de respaldo del hook de arranque |
| `/mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db` | Creado | ✅ Disco | Base de datos SQLite |
| `/mnt/HD/HD_a2/.cronometro-psp/app/` | Creado | ✅ Disco | Copia completa de la app (incluye `.htaccess`) |
| `/mnt/HD/HD_a2/.cronometro-psp/autorestaurar.sh` | Creado | ✅ Disco | Script de autorestauración (7 elementos) |

---

## APÉNDICE A: Mapa de volatilidad del sistema de ficheros del NAS

Este apéndice documenta el hallazgo más importante de la sesión del 17/02/2026: **el WD My Cloud EX2 Ultra monta su sistema de ficheros raíz en RAM**, lo que hace que toda modificación sobre rutas estándar como `/etc/`, `/var/`, `/home/` o `/usr/local/apache2/` desaparezca en cada reinicio.

---

#### A.1 Particiones del sistema

```
/dev/ram0      →  /                        ← RAMDISK (volátil)
ubi0:config    →  /usr/local/config        ← UBIFS en flash NOR (persistente)
squashfs img   →  /usr/local/modules       ← SquashFS de solo lectura (ROM)
/dev/md0p1     →  /usr/local/upload        ← Partición ext4 (persistente)
/dev/md1       →  /mnt/HD/HD_a2            ← DISCO DURO (siempre persistente)
```

---

#### A.2 Comportamiento en el arranque

El script `/usr/local/modules/script/system_init` (que se ejecuta como `sysinit` según `/etc/inittab`) hace lo siguiente en cada arranque:

1. **Monta el ramdisk** como raíz del sistema con `ext4`.
2. **Crea los directorios estándar** en RAM: `/home/root/`, `/var/www/`, `/var/spool/cron/`, etc. — todos vacíos.
3. **Copia `/usr/local/apache2/` completo** desde la imagen SquashFS (`/usr/local/modules/apache2/*`), sobreescribiendo cualquier modificación previa. Esto incluye `httpd.conf` y `ports.conf`.
4. **(línea ~1162)** **Invoca `/usr/local/sbin/custom_booting_init.sh`** — hook oficial WD para personalizaciones de usuario. En este momento, UBIFS (`/usr/local/config/`) ya está disponible pero el disco duro (`/mnt/HD/HD_a2/`) aún NO está montado. Nuestro hook llama a `cron_init.sh &` (desde UBIFS).
5. **Genera el crontab de root** dinámicamente (vía `wd_compinit -p`) con las entradas del sistema WD. Las entradas de usuario añadidas en sesiones anteriores desaparecen. `cron_init.sh` espera en loop hasta que este fichero esté disponible y entonces añade la entrada de `autorestaurar.sh`.
6. **Arranca Apache** con la configuración limpia de fábrica: `Listen 127.0.0.1:8000`, sin acceso desde la red.
7. **Monta el disco duro** en `/mnt/HD/HD_a2/`. A partir de este punto `autorestaurar.sh` puede ejecutarse.
8. **~5 minutos después**: crond ejecuta `autorestaurar.sh` por primera vez → restaura Apache, SSH, archivos de app → app disponible.

---

#### A.3 Tabla de volatilidad por ruta

| Ruta | Persistente | Razón |
|---|---|---|
| `/var/www/` | ❌ No | Ramdisk, creada vacía en arranque |
| `/home/root/.ssh/` | ❌ No | Ramdisk, creada vacía en arranque |
| `/etc/` (la mayoría) | ❌ No | Ramdisk |
| `/var/spool/cron/` | ❌ No | Ramdisk |
| `/usr/local/apache2/` | ❌ No | Ramdisk + sobreescrita desde SquashFS |
| `/tmp/` | ❌ No | Ramdisk (esperado) |
| `/usr/local/config/` | ✅ Sí | Partición UBIFS en flash NOR |
| `/usr/local/upload/` | ✅ Sí | Partición ext4 en `/dev/md0p1` |
| `/mnt/HD/HD_a2/` | ✅ Sí | Disco duro (RAID o single disk) |
| `/shares/` | ✅ Sí | Symlink a `/mnt/HD/HD_a2/` (comparticiones) |

---

#### A.4 Reglas de diseño derivadas

De este mapa se derivan las siguientes reglas para cualquier personalización del NAS:

1. **Nunca almacenar datos de usuario en `/var/www/`**. Es RAM. La BD está en `/mnt/HD/HD_a2/`.
2. **Para configuración que debe persistir**, usar `/usr/local/config/` (UBIFS). Tiene espacio limitado, solo para ficheros de configuración pequeños.
3. **Para datos y ficheros grandes**, usar `/mnt/HD/HD_a2/` (disco físico).
4. **No modificar `/etc/inittab` ni `/usr/local/modules/script/system_init`**. Son ROM o se regeneran. El comentario `#### please don't add anything end ####` en `system_init` es explícito.
5. **El crontab de root** se regenera en cada arranque vía `wd_compinit -p`. Las entradas de usuario se pierden. El mecanismo para registrar entradas persistentes es: `custom_booting_init.sh` (hook oficial WD en UBIFS) → `cron_init.sh` (UBIFS) → añade la entrada a `/var/spool/cron/crontabs/root` una vez que `wd_compinit -p` lo crea. **BusyBox crond v1.30.1 no lee `/etc/cron.d/`** (confirmado por pruebas); solo procesa `/var/spool/cron/crontabs/`.
6. **Apache se sobreescribe en cada arranque**. Cualquier modificación a `httpd.conf` o `ports.conf` debe reproducirse tras cada reinicio (lo hace `autorestaurar.sh`).

---

#### A.5 Por qué `/usr/local/config/` sí persiste

`/usr/local/config/` está montado desde una partición **UBIFS** (Unsorted Block Image File System) sobre flash NOR integrada en la placa del NAS. Esta tecnología está diseñada para soportar muchos ciclos de escritura en memoria flash y sobrevive a cortes de alimentación. Es donde WD guarda su propia configuración persistente: `passwd`, `shadow`, `smbpasswd`, `config.xml`, certificados SSL, etc.

Nosotros la usamos para guardar `authorized_keys` y `apache-extra.conf`, que son ficheros pequeños y se escriben pocas veces.

---

*Documento actualizado el 17/02/2026 (sesión tarde). Estado: app funcional con recuperación autónoma de tres capas. URL: http://192.168.1.71:8080/apps/cronometro/www/*
