<?php
/**
 * Router para el servidor de desarrollo PHP local.
 * Uso: php -S 0.0.0.0:8080 dev-server.php
 *
 * Mapea:
 *   /apps/cronometro/api/... → backend/index.php
 *   /apps/cronometro/www/... → frontend/
 *   /                        → redirige a /apps/cronometro/www/
 */

$uri  = $_SERVER['REQUEST_URI'];
$path = parse_url($uri, PHP_URL_PATH);

// Redirigir raíz a la app
if ($path === '/' || $path === '') {
    header('Location: /apps/cronometro/www/');
    exit;
}

// Rutas API → backend
if (preg_match('#^/apps/cronometro/api/?(.*)$#', $path, $m)) {
    // Reescribir REQUEST_URI para que backend/index.php lo enrute correctamente
    $qs = isset($_SERVER['QUERY_STRING']) && $_SERVER['QUERY_STRING'] !== ''
        ? '?' . $_SERVER['QUERY_STRING']
        : '';
    $_SERVER['REQUEST_URI']  = '/api/' . $m[1] . $qs;
    $_SERVER['SCRIPT_NAME']  = '/index.php';
    require __DIR__ . '/backend/index.php';
    exit;
}

// Archivos estáticos del frontend
if (preg_match('#^/apps/cronometro/www/?(.*)$#', $path, $m)) {
    $rel  = $m[1] !== '' ? $m[1] : 'index.html';
    $file = __DIR__ . '/frontend/' . ltrim($rel, '/');

    if (!is_file($file)) {
        http_response_code(404);
        echo "404 Not found: $file";
        exit;
    }

    $ext   = strtolower(pathinfo($file, PATHINFO_EXTENSION));
    $mimes = [
        'html' => 'text/html; charset=UTF-8',
        'css'  => 'text/css',
        'js'   => 'application/javascript',
        'json' => 'application/json',
        'ttf'  => 'font/ttf',
        'woff' => 'font/woff',
        'woff2'=> 'font/woff2',
        'png'  => 'image/png',
        'jpg'  => 'image/jpeg',
        'svg'  => 'image/svg+xml',
        'ico'  => 'image/x-icon',
    ];
    header('Content-Type: ' . ($mimes[$ext] ?? 'application/octet-stream'));
    header('Cache-Control: no-cache');
    readfile($file);
    exit;
}

http_response_code(404);
echo "404: $path";
