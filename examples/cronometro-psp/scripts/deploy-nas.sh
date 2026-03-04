#!/bin/bash

# Script de despliegue en WD My Cloud EX2 Ultra
# Apache + PHP-FPM (socket unix) en /var/www/apps/cronometro/
# Uso: ./deploy-nas.sh  (ejecutar desde la raíz del proyecto en WSL)

NAS_HOST="192.168.1.71"
NAS_USER="sshd"
NAS_PATH="/var/www/apps/cronometro"

# La clave maestra está en Windows (persiste entre sesiones WSL).
# SSH requiere permisos 600, que /mnt/c/ no puede tener → copiamos a /tmp.
WIN_SSH_KEY="/mnt/c/Users/cpcxb/.ssh/id_nas"
HOME_SSH_KEY="$HOME/.ssh/id_nas"
NAS_SSH_KEY="/tmp/id_nas_deploy"
if [ -f "$WIN_SSH_KEY" ]; then
    cp "$WIN_SSH_KEY" "$NAS_SSH_KEY"
    chmod 600 "$NAS_SSH_KEY"
elif [ -f "$HOME_SSH_KEY" ]; then
    cp "$HOME_SSH_KEY" "$NAS_SSH_KEY"
    chmod 600 "$NAS_SSH_KEY"
else
    echo "❌ Clave SSH no encontrada en $WIN_SSH_KEY ni en $HOME_SSH_KEY"
    exit 1
fi

# Wrappers con clave SSH
SSH="ssh -i $NAS_SSH_KEY $NAS_USER@$NAS_HOST"
SCP="scp -i $NAS_SSH_KEY"

echo "==================================="
echo "Deploy Mi Cronómetro PSP"
echo "WD My Cloud EX2 Ultra → $NAS_HOST"
echo "==================================="
echo ""

# Verificar que estamos en el directorio correcto
if [ ! -f "README.md" ]; then
    echo "❌ Ejecuta este script desde la raíz del proyecto"
    exit 1
fi

# Verificar conectividad
echo "🔍 Verificando conexión con el NAS..."
if ! $SSH "echo OK" > /dev/null 2>&1; then
    echo "❌ No se puede conectar a $NAS_USER@$NAS_HOST"
    echo "   Verifica que la clave SSH esté copiada al NAS:"
    echo "   ssh-copy-id -i ~/.ssh/id_nas.pub $NAS_USER@$NAS_HOST"
    exit 1
fi
echo "✅ Conexión SSH OK"
echo ""

# Crear estructura en el NAS
echo "📁 Creando estructura de directorios..."
$SSH "
    mkdir -p $NAS_PATH/www/css
    mkdir -p $NAS_PATH/www/js
    mkdir -p $NAS_PATH/api/db
    mkdir -p $NAS_PATH/api/api
    mkdir -p $NAS_PATH/data
    mkdir -p $NAS_PATH/logs
    echo '✅ Directorios creados'
"

# Subir frontend → www/
echo "📤 Subiendo frontend..."
$SCP frontend/index.html         "$NAS_USER@$NAS_HOST:$NAS_PATH/www/index.html"
$SCP frontend/css/styles.css     "$NAS_USER@$NAS_HOST:$NAS_PATH/www/css/styles.css"
$SCP frontend/js/api-client.js   "$NAS_USER@$NAS_HOST:$NAS_PATH/www/js/api-client.js"
$SCP frontend/js/app.js          "$NAS_USER@$NAS_HOST:$NAS_PATH/www/js/app.js"
echo "✅ Frontend subido"

# Subir backend → api/
echo "📤 Subiendo backend..."
$SCP backend/index.php           "$NAS_USER@$NAS_HOST:$NAS_PATH/api/index.php"
$SCP backend/config.php          "$NAS_USER@$NAS_HOST:$NAS_PATH/api/config.php"
$SCP backend/db/database.php     "$NAS_USER@$NAS_HOST:$NAS_PATH/api/db/database.php"
$SCP backend/db/schema.sql       "$NAS_USER@$NAS_HOST:$NAS_PATH/api/db/schema.sql"
$SCP backend/api/actividades.php "$NAS_USER@$NAS_HOST:$NAS_PATH/api/api/actividades.php"
$SCP backend/api/tipos-tarea.php "$NAS_USER@$NAS_HOST:$NAS_PATH/api/api/tipos-tarea.php"
$SCP backend/api/sesiones.php    "$NAS_USER@$NAS_HOST:$NAS_PATH/api/api/sesiones.php"
echo "✅ Backend subido"

# Configurar permisos
echo "🔧 Configurando permisos..."
$SSH "
    chmod -R 755 $NAS_PATH/www/
    chmod -R 755 $NAS_PATH/api/
    chmod -R 777 $NAS_PATH/data/
    chmod -R 755 $NAS_PATH/logs/
    echo '✅ Permisos configurados'
"

# Inicializar base de datos si no existe
echo "🗄️  Preparando base de datos..."
$SSH "
    if [ ! -f $NAS_PATH/data/cronometro.db ]; then
        touch $NAS_PATH/data/cronometro.db
        chmod 666 $NAS_PATH/data/cronometro.db
        echo '✅ Archivo BD creado (schema se cargará en el primer request)'
    else
        echo '✅ BD ya existe, no se sobreescribe'
    fi
"

# Subir scripts de mantenimiento al disco persistente del NAS
DISK_DIR="/mnt/HD/HD_a2/.cronometro-psp"
echo "📤 Subiendo scripts de mantenimiento..."
$SCP scripts/autorestaurar.sh "$NAS_USER@$NAS_HOST:$DISK_DIR/autorestaurar.sh"
$SCP scripts/backup.sh        "$NAS_USER@$NAS_HOST:$DISK_DIR/backup.sh"
$SSH "chmod +x $DISK_DIR/autorestaurar.sh $DISK_DIR/backup.sh"
echo "✅ Scripts de mantenimiento subidos"

# Sincronizar al disco persistente (fuente de verdad para reinicios vía autorestaurar.sh)
echo "💾 Actualizando copia en disco persistente..."
$SSH "
    mkdir -p $DISK_DIR/app/www/css $DISK_DIR/app/www/js
    mkdir -p $DISK_DIR/app/api/db  $DISK_DIR/app/api/api
    cp -r $NAS_PATH/www/. $DISK_DIR/app/www/
    cp -r $NAS_PATH/api/. $DISK_DIR/app/api/
    echo 'Disco persistente actualizado: $DISK_DIR/app'
"
echo "✅ Fuente de verdad actualizada ($DISK_DIR/app)"

# Verificar estructura final
echo ""
echo "🔍 Verificando instalación..."
$SSH "
    echo '--- Estructura ---'
    ls -la $NAS_PATH/
    echo ''
    echo '--- Frontend (www/) ---'
    ls $NAS_PATH/www/
    echo ''
    echo '--- Backend (api/) ---'
    ls $NAS_PATH/api/
"

# Test PHP desde CLI
echo ""
echo "🧪 Probando PHP + SQLite..."
$SSH "
    cd $NAS_PATH/api && php -r \"
        require_once 'config.php';
        require_once 'db/database.php';
        \\\$db = Database::getInstance();
        echo '✅ PHP + SQLite OK' . PHP_EOL;
    \" 2>&1
"

echo ""
echo "==================================="
echo "✅ Despliegue completado"
echo "==================================="
echo ""
echo "🌐 Abre en el navegador:"
echo "   http://$NAS_HOST/apps/cronometro/www/"
echo ""
echo "🧪 Health check de la API:"
echo "   http://$NAS_HOST/apps/cronometro/api/api/health"
echo ""
echo "📱 Desde móvil (misma WiFi):"
echo "   http://$NAS_HOST/apps/cronometro/www/"
echo ""
