#!/bin/sh
# Autorestauración de Mi Cronómetro PSP tras reinicio del NAS WD My Cloud EX2 Ultra
#
# Se ejecuta vía crontab cada 5 minutos. En condiciones normales termina en <0.1s.
# Tras un reinicio, restaura todo automáticamente en menos de 5 minutos.
#
# Fuentes de verdad (en disco persistente, sobreviven reinicios):
#   /mnt/HD/HD_a2/.cronometro-psp/
#     data/      ← base de datos SQLite (nunca se toca)
#     app/       ← copia completa de frontend + backend
#   /usr/local/config/cronometro-psp/  (UBIFS)
#     authorized_keys        ← clave pública SSH
#     apache-extra.conf      ← bloques Directory + Location
#     cron_init.sh           ← script que registra el crontab en el arranque
#     custom_booting_init.sh ← hook de arranque del NAS

LOG=/tmp/cronometro-psp-restore.log
CONFIG_DIR=/usr/local/config/cronometro-psp
DISK_DIR=/mnt/HD/HD_a2/.cronometro-psp
APP_SRC=$DISK_DIR/app
APP_DEST=/var/www/apps/cronometro
APACHE_CONF=/usr/local/apache2/conf/httpd.conf
PORTS_CONF=/usr/local/apache2/conf/extra/ports.conf
CRON_FILE=/var/spool/cron/crontabs/root
CRON_LINE="*/5 * * * * /mnt/HD/HD_a2/.cronometro-psp/autorestaurar.sh"
HOOK=/usr/local/sbin/custom_booting_init.sh
NEEDS_APACHE_RESTART=0

stamp() { echo "$(date '+%Y-%m-%d %H:%M:%S'): $1" >> "$LOG"; }

# --- 1. Restaurar authorized_keys (SSH sin contraseña) ---
if [ ! -f /home/root/.ssh/authorized_keys ] || \
   ! grep -qf "$CONFIG_DIR/authorized_keys" /home/root/.ssh/authorized_keys 2>/dev/null; then
    mkdir -p /home/root/.ssh
    cp "$CONFIG_DIR/authorized_keys" /home/root/.ssh/authorized_keys
    chmod 700 /home/root/.ssh
    chmod 600 /home/root/.ssh/authorized_keys
    stamp "SSH authorized_keys restaurado"
fi

# --- 2. Restaurar Listen 8080 en ports.conf ---
if [ -f "$PORTS_CONF" ] && ! grep -q 'Listen 8080' "$PORTS_CONF"; then
    echo 'Listen 8080' >> "$PORTS_CONF"
    stamp "Listen 8080 añadido a ports.conf"
    NEEDS_APACHE_RESTART=1
fi

# --- 3. Restaurar bloques Directory/Location en httpd.conf ---
if [ -f "$APACHE_CONF" ] && ! grep -q 'cronometro' "$APACHE_CONF"; then
    cat "$CONFIG_DIR/apache-extra.conf" >> "$APACHE_CONF"
    stamp "Bloques Apache restaurados en httpd.conf"
    NEEDS_APACHE_RESTART=1
fi

# --- 4. Recargar Apache si hubo cambios de configuración ---
if [ "$NEEDS_APACHE_RESTART" -eq 1 ]; then
    httpd -f "$APACHE_CONF" -k graceful 2>/dev/null
    stamp "Apache recargado"
fi

# --- 5. Restaurar archivos de la app desde el disco persistente ---
APP_MISSING=0
[ ! -f "$APP_DEST/www/index.html" ]        && APP_MISSING=1
[ ! -f "$APP_DEST/api/index.php" ]         && APP_MISSING=1
[ ! -f "$APP_DEST/api/.htaccess" ]         && APP_MISSING=1
[ ! -f "$APP_DEST/api/db/database.php" ]   && APP_MISSING=1
[ ! -f "$APP_DEST/api/api/sesiones.php" ]  && APP_MISSING=1

if [ "$APP_MISSING" -eq 1 ]; then
    mkdir -p "$APP_DEST/www/css" "$APP_DEST/www/js"
    mkdir -p "$APP_DEST/api/db" "$APP_DEST/api/api"
    mkdir -p "$APP_DEST/data" "$APP_DEST/logs"
    # Usar "." como fuente para incluir ficheros ocultos (ej: .htaccess)
    cp -r "$APP_SRC/www/." "$APP_DEST/www/"
    cp -r "$APP_SRC/api/." "$APP_DEST/api/"
    chmod -R 755 "$APP_DEST/www/" "$APP_DEST/api/"
    chmod -R 755 "$APP_DEST/logs/"
    # Recargar Apache para que reconozca los nuevos archivos
    httpd -f "$APACHE_CONF" -k graceful 2>/dev/null
    stamp "Archivos de la app restaurados desde disco (incluye .htaccess)"
fi

# --- 6. Restaurar custom_booting_init.sh si volvió a ser symlink (post-reinicio) ---
if [ -L "$HOOK" ]; then
    rm "$HOOK"
    cp "$CONFIG_DIR/custom_booting_init.sh" "$HOOK"
    chmod +x "$HOOK"
    stamp "custom_booting_init.sh restaurado (era symlink)"
fi

# --- 7. Mantener entrada en crontab (idempotente) ---
if [ -f "$CRON_FILE" ] && ! grep -q 'autorestaurar' "$CRON_FILE"; then
    echo "$CRON_LINE" >> "$CRON_FILE"
    stamp "Entrada crontab re-añadida"
fi

# --- 8. Sincronización horaria (NTP) ---
# Sincronizar si el desfase supera 5 segundos (se ejecuta cada 5 min, rápido si ya está OK)
SNTP_OUT=$(/usr/sbin/sntp -q pool.ntp.org 2>/dev/null | grep -Eo "[+-][0-9]+" | head -1)
if [ -n "$SNTP_OUT" ] && [ "${SNTP_OUT#-}" -gt 5 ] 2>/dev/null; then
    /usr/sbin/sntp -s pool.ntp.org 2>/dev/null
    stamp "Reloj sincronizado con NTP (desfase: ${SNTP_OUT}s)"
fi

# --- 9. La BD vive directamente en el disco: no hay nada que restaurar ---
# Ruta: /mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db

# --- 10. Restaurar VirtualHost SSL (HTTPS :443) ---
SSL_CONF=/usr/local/apache2/conf/extra/ssl-cronometro.conf
HTTP_CONF=/usr/local/apache2/conf/extra/http-cronometro.conf
SSL_CONF_SRC=$DISK_DIR/ssl-cronometro.conf
HTTP_CONF_SRC=$DISK_DIR/http-cronometro.conf

if [ ! -f "$SSL_CONF" ] && [ -f "$SSL_CONF_SRC" ]; then
    cp "$SSL_CONF_SRC" "$SSL_CONF"
    stamp "ssl-cronometro.conf restaurado"
    NEEDS_APACHE_RESTART=1
fi

if [ ! -f "$HTTP_CONF" ] && [ -f "$HTTP_CONF_SRC" ]; then
    cp "$HTTP_CONF_SRC" "$HTTP_CONF"
    stamp "http-cronometro.conf restaurado"
    NEEDS_APACHE_RESTART=1
fi

# Restaurar mod_ssl si no está cargado
SSL_LOAD=/usr/local/apache2/conf/mods-enabled/ssl.load
if [ ! -f "$SSL_LOAD" ]; then
    echo 'LoadModule ssl_module modules/mod_ssl.so' > "$SSL_LOAD"
    stamp "mod_ssl restaurado"
    NEEDS_APACHE_RESTART=1
fi

# Restaurar Listen 443 en ports.conf
if [ -f "$PORTS_CONF" ] && ! grep -q 'Listen 443' "$PORTS_CONF"; then
    echo 'Listen 443' >> "$PORTS_CONF"
    stamp "Listen 443 añadido a ports.conf"
    NEEDS_APACHE_RESTART=1
fi

if [ "$NEEDS_APACHE_RESTART" -eq 1 ]; then
    httpd -f "$APACHE_CONF" -k graceful 2>/dev/null
    stamp "Apache recargado (SSL)"
fi

# --- 11. Actualización DDNS (GoDaddy) ---
DDNS_SCRIPT=$DISK_DIR/ddns-update.sh
if [ -f "$DDNS_SCRIPT" ]; then
    sh "$DDNS_SCRIPT" >> "$LOG" 2>&1
fi

# --- 12. Renovación automática del certificado Let's Encrypt (acme.sh) ---
ACME_HOME=$DISK_DIR/.acme.sh
if [ -f "$ACME_HOME/acme.sh" ]; then
    "$ACME_HOME/acme.sh" --cron --home "$ACME_HOME" >> "$LOG" 2>&1
fi

# --- 13. Restaurar Adminer (solo LAN, acceso por IP) ---
ADMINER_DIR=/var/www/apps/adminer
ADMINER_SRC=$DISK_DIR/adminer/index.php
if [ ! -f "$ADMINER_DIR/index.php" ] && [ -f "$ADMINER_SRC" ]; then
    mkdir -p "$ADMINER_DIR"
    cp "$ADMINER_SRC" "$ADMINER_DIR/index.php"
    stamp "Adminer restaurado"
fi

# --- 14. Backup diario al NAS secundario ---
# Llama a backup.sh una sola vez al día (controla con fecha en /tmp).
# Si backup.sh no existe aún, este paso se omite silenciosamente.
BACKUP_SCRIPT=$DISK_DIR/backup.sh
BACKUP_LASTRUN=/tmp/cronometro-backup-lastrun.txt
TODAY=$(date +%Y%m%d)
if [ -f "$BACKUP_SCRIPT" ]; then
    LAST=$(cat "$BACKUP_LASTRUN" 2>/dev/null || echo "0")
    if [ "$LAST" != "$TODAY" ]; then
        sh "$BACKUP_SCRIPT" >> "$LOG" 2>&1
    fi
fi
