<?php
/**
 * Router principal de la API
 *
 * Rutas disponibles:
 * - GET  /api/actividades
 * - POST /api/actividades
 * - PUT  /api/actividades?id=xxx
 * - DELETE /api/actividades?id=xxx
 *
 * - GET  /api/tipos-tarea
 * - POST /api/tipos-tarea
 *
 * - GET  /api/sesiones?fecha=hoy
 * - GET  /api/sesiones?action=activa
 * - GET  /api/sesiones?action=estadisticas&fecha=hoy
 * - POST /api/sesiones?action=iniciar
 * - POST /api/sesiones?action=detener
 */

require_once __DIR__ . '/config.php';

// Obtener la ruta solicitada
$requestUri = $_SERVER['REQUEST_URI'];
$scriptName = dirname($_SERVER['SCRIPT_NAME']);

// Eliminar el prefijo del script si existe
if ($scriptName !== '/') {
    $requestUri = str_replace($scriptName, '', $requestUri);
}

// Eliminar query string
$path = parse_url($requestUri, PHP_URL_PATH);

// Normalizar path (eliminar / inicial y final)
$path = trim($path, '/');

// Routing
// El path puede llegar como 'api/actividades' (desde HTTP:8080 vía LAN)
// o como 'actividades' (desde HTTPS:443 cuando SCRIPT_NAME incluye /api)
// Normalizamos eliminando el prefijo 'api/' si existe
$path = preg_replace('#^api/#', '', $path);

switch ($path) {
    case 'actividades':
        require __DIR__ . '/api/actividades.php';
        break;

    case 'tipos-tarea':
        require __DIR__ . '/api/tipos-tarea.php';
        break;

    case 'sesiones':
        require __DIR__ . '/api/sesiones.php';
        break;

    case 'health':
        // Health check endpoint
        respondJson([
            'status' => 'ok',
            'timestamp' => time(),
            'version' => '1.0.0'
        ]);
        break;

    case '':
        // Endpoint de bienvenida
        respondJson([
            'app' => 'Mi Cronómetro PSP API',
            'version' => '1.0.0',
            'endpoints' => [
                'GET /api/health' => 'Health check',
                'GET /api/actividades' => 'Listar actividades',
                'POST /api/actividades' => 'Crear actividad',
                'GET /api/tipos-tarea' => 'Listar tipos de tarea',
                'POST /api/tipos-tarea' => 'Crear tipo de tarea',
                'GET /api/sesiones' => 'Listar sesiones',
                'GET /api/sesiones?action=activa' => 'Obtener sesión activa',
                'GET /api/sesiones?action=estadisticas' => 'Estadísticas del día',
                'POST /api/sesiones?action=iniciar' => 'Iniciar sesión',
                'POST /api/sesiones?action=detener' => 'Detener sesión activa'
            ]
        ]);
        break;

    default:
        respondError('Endpoint no encontrado: ' . $path, 404);
}
