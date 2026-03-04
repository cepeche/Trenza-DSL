<?php
require_once __DIR__ . '/../config.php';
require_once __DIR__ . '/../db/database.php';

$db = Database::getInstance();
$method = $_SERVER['REQUEST_METHOD'];

try {
    switch ($method) {
        case 'GET':
            handleGet($db);
            break;

        case 'POST':
            handlePost($db);
            break;

        case 'PUT':
            handlePut($db);
            break;

        case 'DELETE':
            handleDelete($db);
            break;

        default:
            respondError('Método no permitido', 405);
    }
} catch (Exception $e) {
    respondError($e->getMessage(), 500);
}

/**
 * GET /api/actividades
 * Obtener todas las actividades
 */
function handleGet($db) {
    // Asegurar que la columna permanente existe (migración al vuelo)
    try {
        $db->execute('ALTER TABLE actividades ADD COLUMN permanente INTEGER NOT NULL DEFAULT 0');
    } catch (Exception $e) {
        // La columna ya existe, ignorar
    }

    $actividades = $db->query('
        SELECT id, nombre, color, created_at, archived, permanente
        FROM actividades
        WHERE archived = 0
        ORDER BY nombre
    ');

    respondJson($actividades);
}

/**
 * POST /api/actividades
 * Crear una nueva actividad
 * Body: { nombre, color, permanente? }
 */
function handlePost($db) {
    $body = getRequestBody();
    requireParams($body, ['nombre', 'color']);

    $id = slugify($body['nombre']);
    $nombre = trim($body['nombre']);
    $color = trim($body['color']);
    $permanente = isset($body['permanente']) && $body['permanente'] ? 1 : 0;
    $created_at = time();

    // Verificar si ya existe
    $exists = $db->queryOne('SELECT id FROM actividades WHERE id = ?', [$id]);
    if ($exists) {
        respondError('Ya existe una actividad con ese nombre', 409);
    }

    $db->execute('
        INSERT INTO actividades (id, nombre, color, created_at, permanente)
        VALUES (?, ?, ?, ?, ?)
    ', [$id, $nombre, $color, $created_at, $permanente]);

    respondSuccess('Actividad creada', [
        'id' => $id,
        'nombre' => $nombre,
        'color' => $color,
        'created_at' => $created_at,
        'archived' => 0,
        'permanente' => $permanente
    ]);
}

/**
 * PUT /api/actividades/:id
 * Actualizar una actividad
 * Body: { nombre?, color? }
 */
function handlePut($db) {
    $id = getQueryParam('id');
    if (!$id) {
        respondError('ID de actividad requerido', 400);
    }

    $body = getRequestBody();
    $fields = [];
    $params = [];

    if (isset($body['nombre'])) {
        $fields[] = 'nombre = ?';
        $params[] = trim($body['nombre']);
    }

    if (isset($body['color'])) {
        $fields[] = 'color = ?';
        $params[] = trim($body['color']);
    }

    if (isset($body['permanente'])) {
        $fields[] = 'permanente = ?';
        $params[] = $body['permanente'] ? 1 : 0;
    }

    if (empty($fields)) {
        respondError('No hay campos para actualizar', 400);
    }

    $params[] = $id;
    $sql = 'UPDATE actividades SET ' . implode(', ', $fields) . ' WHERE id = ?';

    $affected = $db->execute($sql, $params);

    if ($affected === 0) {
        respondError('Actividad no encontrada', 404);
    }

    respondSuccess('Actividad actualizada');
}

/**
 * DELETE /api/actividades/:id
 * Archivar una actividad (soft delete)
 */
function handleDelete($db) {
    $id = getQueryParam('id');
    if (!$id) {
        respondError('ID de actividad requerido', 400);
    }

    $affected = $db->execute('
        UPDATE actividades SET archived = 1 WHERE id = ?
    ', [$id]);

    if ($affected === 0) {
        respondError('Actividad no encontrada', 404);
    }

    respondSuccess('Actividad archivada');
}
