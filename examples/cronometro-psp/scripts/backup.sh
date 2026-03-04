#!/bin/sh
# backup.sh — Backup diario de cronometro.db al NAS secundario
# Compatible con BusyBox sh (WD My Cloud EX2 Ultra)
#
# Funcionamiento:
#   1. Crea un snapshot timestamped de cronometro.db en el disco local
#   2. rsync del directorio de backups al NAS secundario (si SSH key disponible)
#   3. Limpia snapshots locales con más de KEEP_DAYS días
#   4. Actualiza el marcador de fecha para que autorestaurar.sh no lo llame dos veces al día
#
# Se invoca automáticamente desde autorestaurar.sh (paso 14) una vez al día.
# También puede ejecutarse manualmente:
#   sh /mnt/HD/HD_a2/.cronometro-psp/backup.sh
#
# Despliegue inicial (desde WSL, en la raíz del proyecto):
#   WIN_KEY=/mnt/c/Users/cpcxb/.ssh/id_nas && cp $WIN_KEY /tmp/id_nas_deploy && chmod 600 /tmp/id_nas_deploy
#   scp -i /tmp/id_nas_deploy scripts/backup.sh sshd@192.168.1.71:/mnt/HD/HD_a2/.cronometro-psp/backup.sh
#   ssh -i /tmp/id_nas_deploy sshd@192.168.1.71 chmod +x /mnt/HD/HD_a2/.cronometro-psp/backup.sh
#
# Para habilitar rsync al NAS secundario (ejecutar en el NAS principal via SSH):
#   1. Generar clave SSH sin contraseña:
#      ssh-keygen -t ed25519 -f /mnt/HD/HD_a2/.cronometro-psp/ssl/id_backup -N ""
#   2. Instalar la clave pública en el NAS secundario:
#      ssh-copy-id -i /mnt/HD/HD_a2/.cronometro-psp/ssl/id_backup.pub admin@192.168.1.68
#   3. Crear el directorio de destino en el NAS secundario:
#      ssh admin@192.168.1.68 "mkdir -p /mnt/HD/HD_a2/backups/cronometro-psp"
#   4. Verificar que funciona:
#      rsync -az -e "ssh -i /mnt/HD/HD_a2/.cronometro-psp/ssl/id_backup -o BatchMode=yes" \
#        /mnt/HD/HD_a2/.cronometro-psp/data/backups/ \
#        admin@192.168.1.68:/mnt/HD/HD_a2/backups/cronometro-psp/

DISK_DIR=/mnt/HD/HD_a2/.cronometro-psp
DB_SRC=$DISK_DIR/data/cronometro.db
BACKUP_DIR=$DISK_DIR/data/backups
LOG=$DISK_DIR/logs/backup.log
KEEP_DAYS=7
LAST_RUN_FILE=/tmp/cronometro-backup-lastrun.txt

# NAS secundario: WD My Cloud antiguo (solo acepta ssh-rsa, usuario root)
REMOTE_IPS="192.168.1.68 192.168.1.75"
REMOTE_USER=root
REMOTE_DIR=/mnt/HD/HD_a2/backups/cronometro-psp
SSH_KEY=$DISK_DIR/ssl/id_backup
# Opciones SSH para compatibilidad con el My Cloud antiguo (Dropbear/OpenSSH legacy)
SSH_OPTS="-o StrictHostKeyChecking=no -o HostKeyAlgorithms=+ssh-rsa -o ConnectTimeout=10 -o BatchMode=yes"

# ─── Utilidades ────────────────────────────────────────────────────────────────
stamp() { echo "$(date '+%Y-%m-%d %H:%M:%S'): [BACKUP] $1" >> "$LOG"; }

mkdir -p "$BACKUP_DIR" "$(dirname "$LOG")"

# ─── 1. Verificar que la BD existe ────────────────────────────────────────────
if [ ! -f "$DB_SRC" ]; then
    stamp "ERROR: BD no encontrada en $DB_SRC"
    exit 1
fi

# ─── 2. Snapshot local ────────────────────────────────────────────────────────
FECHA=$(date +%Y%m%d-%H%M%S)
SNAPSHOT=$BACKUP_DIR/cronometro-$FECHA.db
cp "$DB_SRC" "$SNAPSHOT" || { stamp "ERROR: cp falló ($DB_SRC → $SNAPSHOT)"; exit 1; }
stamp "Snapshot creado: cronometro-$FECHA.db"

# ─── 3. rsync al NAS secundario ───────────────────────────────────────────────
if [ -f "$SSH_KEY" ]; then
    RSYNC_OK=0
    for REMOTE_IP in $REMOTE_IPS; do
        if ping -c 1 "$REMOTE_IP" >/dev/null 2>&1; then
            rsync -az --timeout=30 \
                -e "ssh -i $SSH_KEY $SSH_OPTS" \
                "$BACKUP_DIR/" \
                "${REMOTE_USER}@${REMOTE_IP}:${REMOTE_DIR}/"
            if [ $? -eq 0 ]; then
                stamp "rsync OK: $REMOTE_IP:$REMOTE_DIR"
                RSYNC_OK=1
                break
            else
                stamp "rsync FALLIDO: $REMOTE_IP (continuando con siguiente IP)"
            fi
        else
            stamp "NAS secundario no alcanzable: $REMOTE_IP"
        fi
    done
    [ "$RSYNC_OK" -eq 0 ] && stamp "AVISO: rsync no completado en ningun NAS secundario"
else
    stamp "Sin clave SSH en $SSH_KEY — rsync omitido (ver cabecera del script)"
fi

# ─── 4. Limpiar snapshots locales antiguos ────────────────────────────────────
find "$BACKUP_DIR" -name "cronometro-*.db" -mtime "+$KEEP_DAYS" 2>/dev/null | while read -r f; do
    rm -f "$f" && stamp "Snapshot antiguo eliminado: $(basename "$f")"
done

# ─── 5. Marcar fecha de ejecución (idempotencia en autorestaurar.sh) ──────────
date +%Y%m%d > "$LAST_RUN_FILE"
stamp "Backup completado"
