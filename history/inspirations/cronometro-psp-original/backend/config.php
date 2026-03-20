<?php
/**
 * Configuración de la aplicación
 */

// Configuración de errores (cambiar a false en producción)
define('DEBUG_MODE', true);

if (DEBUG_MODE) {
    error_reporting(E_ALL);
    ini_set('display_errors', 1);
} else {
    error_reporting(0);
    ini_set('display_errors', 0);
}

// Configuración de CORS para desarrollo local
header('Access-Control-Allow-Origin: *');
header('Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS');
header('Access-Control-Allow-Headers: Content-Type, Authorization');
header('Content-Type: application/json; charset=utf-8');

// Si es una petición OPTIONS (preflight), responder inmediatamente
if ($_SERVER['REQUEST_METHOD'] === 'OPTIONS') {
    http_response_code(200);
    exit;
}

// Configuración de zona horaria
date_default_timezone_set('Europe/Madrid');

// Funciones auxiliares
// Envueltas en function_exists para permitir que el bootstrap de tests las sobreescriba
if (!function_exists('respondJson')) {
    function respondJson($data, $statusCode = 200) {
        http_response_code($statusCode);
        echo json_encode($data, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT);
        exit;
    }
}

if (!function_exists('respondError')) {
    function respondError($message, $statusCode = 400) {
        respondJson(['error' => $message], $statusCode);
    }
}

if (!function_exists('respondSuccess')) {
    function respondSuccess($message = 'Operación exitosa', $data = null) {
        $response = ['success' => true, 'message' => $message];
        if ($data !== null) {
            $response['data'] = $data;
        }
        respondJson($response);
    }
}

if (!function_exists('getRequestBody')) {
    function getRequestBody() {
        $body = file_get_contents('php://input');
        return json_decode($body, true) ?? [];
    }
}

if (!function_exists('getQueryParam')) {
    function getQueryParam($name, $default = null) {
        return $_GET[$name] ?? $default;
    }
}

if (!function_exists('requireParams')) {
    function requireParams($params, $requiredFields) {
        foreach ($requiredFields as $field) {
            if (!isset($params[$field]) || $params[$field] === '') {
                respondError("Campo requerido: $field", 400);
            }
        }
    }
}

if (!function_exists('generateId')) {
    function generateId($prefix = '') {
        return $prefix . bin2hex(random_bytes(8));
    }
}

if (!function_exists('slugify')) {
    function slugify($text) {
        // Convertir a minúsculas
        $text = mb_strtolower($text, 'UTF-8');

        // Reemplazar caracteres especiales
        $text = preg_replace('/[^a-z0-9\s-]/', '', $text);

        // Reemplazar espacios y múltiples guiones por un solo guion
        $text = preg_replace('/[\s-]+/', '-', $text);

        // Eliminar guiones al inicio y final
        return trim($text, '-');
    }
}

if (!function_exists('enviarCsv')) {
    function enviarCsv($csv) {
        header('Content-Type: text/csv; charset=utf-8');
        header('Content-Disposition: attachment; filename="sesiones.csv"');
        echo $csv;
        exit;
    }
}
