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

        case 'PATCH':
            handlePatch($db);
            break;

        default:
            respondError('Método no permitido', 405);
    }
} catch (Exception $e) {
    respondError($e->getMessage(), 500);
}

/**
 * GET /api/tipos-tarea
 * Obtener todos los tipos de tarea con sus actividades permitidas
 */
function handleGet($db) {
    // Asegurar que la columna archived existe en tipos_tarea (migración al vuelo)
    try {
        $db->execute('ALTER TABLE tipos_tarea ADD COLUMN archived INTEGER NOT NULL DEFAULT 0');
    } catch (Exception $e) {
        // La columna ya existe, ignorar
    }

    // Calcular usos reales en los últimos 7 días desde las sesiones
    $hace7dias = strtotime('-7 days');
    $usos7d = $db->query('
        SELECT t.tipo_tarea_id, COUNT(*) as usos
        FROM sesiones s
        JOIN tareas t ON s.tarea_id = t.id
        WHERE s.inicio >= ? AND s.fin IS NOT NULL
        GROUP BY t.tipo_tarea_id
    ', [$hace7dias]);
    $usosPorTipo = [];
    foreach ($usos7d as $u) {
        $usosPorTipo[$u['tipo_tarea_id']] = (int)$u['usos'];
    }

    $tipos = $db->query('
        SELECT id, nombre, icono, actividades_permitidas, usos_7d, created_at
        FROM tipos_tarea
        WHERE archived = 0
        ORDER BY nombre
    ');

    // Decodificar JSON y sobreescribir usos_7d con el valor real calculado
    foreach ($tipos as &$tipo) {
        $tipo['actividades_permitidas'] = json_decode($tipo['actividades_permitidas'], true);
        $tipo['usos_7d'] = $usosPorTipo[$tipo['id']] ?? 0;
    }

    respondJson($tipos);
}

/**
 * POST /api/tipos-tarea
 * Crear un nuevo tipo de tarea
 * Body: { nombre, icono, actividades_permitidas: [] }
 */
function handlePost($db) {
    $body = getRequestBody();
    requireParams($body, ['nombre', 'icono', 'actividades_permitidas']);

    if (!is_array($body['actividades_permitidas']) || empty($body['actividades_permitidas'])) {
        respondError('Debe seleccionar al menos una actividad', 400);
    }

    $id = slugify($body['nombre']);
    $nombre = trim($body['nombre']);
    $icono = trim($body['icono']);
    $actividades_permitidas = json_encode($body['actividades_permitidas']);
    $created_at = time();

    // Verificar si ya existe
    $exists = $db->queryOne('SELECT id FROM tipos_tarea WHERE id = ?', [$id]);
    if ($exists) {
        respondError('Ya existe un tipo de tarea con ese nombre', 409);
    }

    // Iniciar transacción
    $db->beginTransaction();

    try {
        // Insertar tipo de tarea
        $db->execute('
            INSERT INTO tipos_tarea (id, nombre, icono, actividades_permitidas, usos_7d, created_at)
            VALUES (?, ?, ?, ?, 0, ?)
        ', [$id, $nombre, $icono, $actividades_permitidas, $created_at]);

        // Generar tareas expandidas (tipo_tarea × actividad)
        foreach ($body['actividades_permitidas'] as $actividadId) {
            $tareaId = $id . '_' . $actividadId;
            $db->execute('
                INSERT INTO tareas (id, tipo_tarea_id, actividad_id)
                VALUES (?, ?, ?)
            ', [$tareaId, $id, $actividadId]);
        }

        $db->commit();

        respondSuccess('Tipo de tarea creado', [
            'id' => $id,
            'nombre' => $nombre,
            'icono' => $icono,
            'actividades_permitidas' => $body['actividades_permitidas'],
            'usos_7d' => 0,
            'created_at' => $created_at
        ]);
    } catch (Exception $e) {
        $db->rollback();
        throw $e;
    }
}

/**
 * PATCH /api/tipos-tarea
 * Editar nombre e/o icono de un tipo de tarea existente
 * Body: { id, nombre?, icono? }
 */
function handlePatch($db) {
    $body = getRequestBody();
    requireParams($body, ['id']);

    $id = $body['id'];
    $tipo = $db->queryOne('SELECT id, nombre, icono FROM tipos_tarea WHERE id = ? AND archived = 0', [$id]);
    if (!$tipo) {
        respondError('Tipo de tarea no encontrado', 404);
    }

    $nombre = isset($body['nombre']) ? trim($body['nombre']) : $tipo['nombre'];
    $icono  = isset($body['icono'])  ? trim($body['icono'])  : $tipo['icono'];

    if ($nombre === '') {
        respondError('El nombre no puede estar vacío', 400);
    }
    if ($icono === '') {
        respondError('El icono no puede estar vacío', 400);
    }

    $db->execute(
        'UPDATE tipos_tarea SET nombre = ?, icono = ? WHERE id = ?',
        [$nombre, $icono, $id]
    );

    respondSuccess('Tipo de tarea actualizado', [
        'id'     => $id,
        'nombre' => $nombre,
        'icono'  => $icono
    ]);
}
