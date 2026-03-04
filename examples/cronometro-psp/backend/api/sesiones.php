<?php
require_once __DIR__ . '/../config.php';
require_once __DIR__ . '/../db/database.php';

$db = Database::getInstance();
$method = $_SERVER['REQUEST_METHOD'];
$action = getQueryParam('action');

try {
    switch ($method) {
        case 'GET':
            if ($action === 'activa') {
                getSesionActiva($db);
            } else if ($action === 'estadisticas') {
                getEstadisticas($db);
            } else if ($action === 'acumulado') {
                getAcumulado($db);
            } else if ($action === 'historial') {
                getHistorial($db);
            } else if ($action === 'exportar') {
                exportarCsv($db);
            } else {
                getSesiones($db);
            }
            break;

        case 'POST':
            if ($action === 'iniciar') {
                iniciarSesion($db);
            } else if ($action === 'detener') {
                detenerSesion($db);
            } else if ($action === 'reset') {
                resetearDatos($db);
            } else {
                respondError('Acción no especificada', 400);
            }
            break;

        default:
            respondError('Método no permitido', 405);
    }
} catch (Exception $e) {
    respondError($e->getMessage(), 500);
}

/**
 * GET /api/sesiones?fecha=hoy|ayer|YYYY-MM-DD
 * Obtener sesiones de una fecha
 */
function getSesiones($db) {
    $fecha = getQueryParam('fecha', 'hoy');

    // Calcular timestamps de inicio y fin del día
    if ($fecha === 'hoy') {
        $inicio = strtotime('today');
        $fin = strtotime('tomorrow') - 1;
    } else if ($fecha === 'ayer') {
        $inicio = strtotime('yesterday');
        $fin = strtotime('today') - 1;
    } else {
        // Formato YYYY-MM-DD
        $inicio = strtotime($fecha);
        $fin = strtotime($fecha . ' +1 day') - 1;
    }

    $sesiones = $db->query('
        SELECT
            s.id,
            s.tarea_id,
            s.inicio,
            s.fin,
            s.duracion,
            s.notas,
            t.tipo_tarea_id,
            t.actividad_id,
            tt.nombre as tipo_tarea_nombre,
            tt.icono as tipo_tarea_icono,
            a.nombre as actividad_nombre,
            a.color as actividad_color
        FROM sesiones s
        JOIN tareas t ON s.tarea_id = t.id
        JOIN tipos_tarea tt ON t.tipo_tarea_id = tt.id
        JOIN actividades a ON t.actividad_id = a.id
        WHERE s.inicio >= ? AND s.inicio <= ?
        ORDER BY s.inicio DESC
    ', [$inicio, $fin]);

    respondJson($sesiones);
}

/**
 * GET /api/sesiones?action=activa
 * Obtener la sesión activa actual (si existe)
 */
function getSesionActiva($db) {
    $sesion = $db->queryOne('
        SELECT
            s.id,
            s.tarea_id,
            s.inicio,
            s.notas,
            t.tipo_tarea_id,
            t.actividad_id,
            tt.nombre as tipo_tarea_nombre,
            tt.icono as tipo_tarea_icono,
            a.nombre as actividad_nombre,
            a.color as actividad_color
        FROM sesiones s
        JOIN tareas t ON s.tarea_id = t.id
        JOIN tipos_tarea tt ON t.tipo_tarea_id = tt.id
        JOIN actividades a ON t.actividad_id = a.id
        WHERE s.fin IS NULL
        LIMIT 1
    ');

    if ($sesion) {
        $sesion['server_time'] = time();
        respondJson($sesion);
    } else {
        respondJson(null);
    }
}

/**
 * GET /api/sesiones?action=estadisticas&fecha=hoy
 * Obtener estadísticas de tiempo por tipo de tarea y actividad
 */
function getEstadisticas($db) {
    $fecha = getQueryParam('fecha', 'hoy');

    if ($fecha === 'hoy') {
        $inicio = strtotime('today');
        $fin = strtotime('tomorrow') - 1;
    } else if ($fecha === 'ayer') {
        $inicio = strtotime('yesterday');
        $fin = strtotime('today') - 1;
    } else {
        $inicio = strtotime($fecha);
        $fin = strtotime($fecha . ' +1 day') - 1;
    }

    // Tiempo por tipo de tarea
    $porTipoTarea = $db->query('
        SELECT
            tt.id,
            tt.nombre,
            tt.icono,
            SUM(s.duracion) as tiempo_total
        FROM sesiones s
        JOIN tareas t ON s.tarea_id = t.id
        JOIN tipos_tarea tt ON t.tipo_tarea_id = tt.id
        WHERE s.inicio >= ? AND s.inicio <= ? AND s.fin IS NOT NULL
        GROUP BY tt.id, tt.nombre, tt.icono
        ORDER BY tiempo_total DESC
    ', [$inicio, $fin]);

    // Tiempo por tarea específica (tipo × actividad)
    $porTarea = $db->query('
        SELECT
            t.id as tarea_id,
            tt.nombre as tipo_tarea_nombre,
            tt.icono as tipo_tarea_icono,
            a.nombre as actividad_nombre,
            a.color as actividad_color,
            SUM(s.duracion) as tiempo_total
        FROM sesiones s
        JOIN tareas t ON s.tarea_id = t.id
        JOIN tipos_tarea tt ON t.tipo_tarea_id = tt.id
        JOIN actividades a ON t.actividad_id = a.id
        WHERE s.inicio >= ? AND s.inicio <= ? AND s.fin IS NOT NULL
        GROUP BY t.id, tt.nombre, tt.icono, a.nombre, a.color
        ORDER BY tiempo_total DESC
    ', [$inicio, $fin]);

    // Tiempo total del día
    $totalResult = $db->queryOne('
        SELECT SUM(duracion) as total
        FROM sesiones
        WHERE inicio >= ? AND inicio <= ? AND fin IS NOT NULL
    ', [$inicio, $fin]);

    $tiempoTotal = $totalResult['total'] ?? 0;

    // Si hay sesión activa hoy, sumar el tiempo transcurrido
    $sesionActiva = $db->queryOne('
        SELECT inicio
        FROM sesiones
        WHERE fin IS NULL AND inicio >= ?
        LIMIT 1
    ', [$inicio]);

    $tiempoSesionActiva = 0;
    if ($sesionActiva) {
        $tiempoSesionActiva = time() - $sesionActiva['inicio'];
        $tiempoTotal += $tiempoSesionActiva;
    }

    respondJson([
        'tiempo_total' => $tiempoTotal,
        'sesion_activa_tiempo' => $tiempoSesionActiva,
        'por_tipo_tarea' => $porTipoTarea,
        'por_tarea' => $porTarea
    ]);
}

/**
 * GET /api/sesiones?action=historial&dias=7
 * Resumen diario de los últimos N días, agrupado por actividad.
 * Devuelve array de días (más reciente primero) + totales del período.
 *
 * Las sesiones que cruzan la medianoche se recortan a los límites de cada día,
 * de modo que ningún día puede superar 86 400 s.
 */
function getHistorial($db) {
    $dias = min(90, max(1, (int)getQueryParam('dias', 7)));
    $ahora = time();

    // Sesión activa (consultada una sola vez fuera del bucle)
    $activa = $db->queryOne('
        SELECT s.inicio, t.actividad_id,
               a.nombre AS actividad_nombre, a.color AS actividad_color
        FROM sesiones s
        JOIN tareas t      ON s.tarea_id     = t.id
        JOIN actividades a ON t.actividad_id = a.id
        WHERE s.fin IS NULL
        LIMIT 1
    ');

    $resultado = [];
    for ($i = 0; $i < $dias; $i++) {
        $inicioDia = strtotime("today - {$i} days");
        $finDia    = $inicioDia + 86400; // exclusivo

        // Sesiones cerradas que se solapan con este día.
        // MIN/MAX recortan cada sesión a los límites del día.
        // Los valores $finDia y $inicioDia son enteros de strtotime(), no input del usuario.
        $filas = $db->query("
            SELECT
                a.id   AS actividad_id,
                a.nombre AS actividad_nombre,
                a.color  AS actividad_color,
                SUM(MIN(s.fin, $finDia) - MAX(s.inicio, $inicioDia)) AS tiempo
            FROM sesiones s
            JOIN tareas t      ON s.tarea_id      = t.id
            JOIN actividades a ON t.actividad_id   = a.id
            WHERE s.fin IS NOT NULL
              AND s.inicio < $finDia
              AND s.fin    > $inicioDia
            GROUP BY a.id, a.nombre, a.color
            ORDER BY tiempo DESC
        ");

        // Sumar sesión activa si se solapa con este día
        if ($activa && (int)$activa['inicio'] < $finDia && $ahora > $inicioDia) {
            $transcurrido = min($ahora, $finDia) - max((int)$activa['inicio'], $inicioDia);
            if ($transcurrido > 0) {
                $encontrado = false;
                foreach ($filas as &$fila) {
                    if ($fila['actividad_id'] === $activa['actividad_id']) {
                        $fila['tiempo'] = (int)$fila['tiempo'] + $transcurrido;
                        $encontrado = true;
                        break;
                    }
                }
                unset($fila);
                if (!$encontrado) {
                    $filas[] = [
                        'actividad_id'     => $activa['actividad_id'],
                        'actividad_nombre' => $activa['actividad_nombre'],
                        'actividad_color'  => $activa['actividad_color'],
                        'tiempo'           => $transcurrido,
                    ];
                }
                // Reordenar por tiempo desc tras inserción
                usort($filas, function($a, $b) { return $b['tiempo'] - $a['tiempo']; });
            }
        }

        $total = array_sum(array_column($filas, 'tiempo'));

        $resultado[] = [
            'fecha'         => date('Y-m-d', $inicioDia),
            'timestamp'     => $inicioDia,
            'total'         => $total,
            'por_actividad' => array_values($filas),
        ];
    }

    // Totales del período por actividad (también con recorte al período)
    $inicioPeríodo = strtotime("today - {$dias} days");
    $totales = $db->query("
        SELECT
            a.id   AS actividad_id,
            a.nombre AS actividad_nombre,
            a.color  AS actividad_color,
            SUM(MIN(s.fin, $ahora) - MAX(s.inicio, $inicioPeríodo)) AS tiempo
        FROM sesiones s
        JOIN tareas t      ON s.tarea_id      = t.id
        JOIN actividades a ON t.actividad_id   = a.id
        WHERE s.fin IS NOT NULL
          AND s.inicio < $ahora
          AND s.fin    > $inicioPeríodo
        GROUP BY a.id, a.nombre, a.color
        ORDER BY tiempo DESC
    ");

    // Sumar sesión activa a los totales del período
    if ($activa && $ahora > $inicioPeríodo) {
        $transcurrido = $ahora - max((int)$activa['inicio'], $inicioPeríodo);
        if ($transcurrido > 0) {
            $encontrado = false;
            foreach ($totales as &$totRow) {
                if ($totRow['actividad_id'] === $activa['actividad_id']) {
                    $totRow['tiempo'] = (int)$totRow['tiempo'] + $transcurrido;
                    $encontrado = true;
                    break;
                }
            }
            unset($totRow);
            if (!$encontrado) {
                $totales[] = [
                    'actividad_id'     => $activa['actividad_id'],
                    'actividad_nombre' => $activa['actividad_nombre'],
                    'actividad_color'  => $activa['actividad_color'],
                    'tiempo'           => $transcurrido,
                ];
            }
        }
    }

    respondJson([
        'dias'        => $resultado,
        'totales'     => array_values($totales),
        'server_time' => $ahora,
    ]);
}

/**
 * POST /api/sesiones?action=iniciar
 * Iniciar una nueva sesión (o sustituir la activa)
 * Body: { tarea_id, notas?, minutos_retroactivos?, sustituir? }
 *
 * Si sustituir=true y hay sesión activa:
 *   - Actualiza tarea_id y notas de la sesión activa (conserva inicio)
 *   - Garantiza duración mínima de 60s ajustando inicio si es necesario
 * Si sustituir=false o no hay sesión activa:
 *   - Comportamiento normal: cierra sesión activa y abre nueva
 */
function iniciarSesion($db) {
    $body = getRequestBody();
    requireParams($body, ['tarea_id']);

    $tareaId = $body['tarea_id'];
    $notas = isset($body['notas']) && $body['notas'] !== '' ? trim($body['notas']) : null;
    $minutosRetroactivos = isset($body['minutos_retroactivos']) ? max(0, (int)$body['minutos_retroactivos']) : 0;
    $sustituir = !empty($body['sustituir']);

    // Verificar que la tarea existe
    $tarea = $db->queryOne('SELECT id FROM tareas WHERE id = ?', [$tareaId]);
    if (!$tarea) {
        respondError('Tarea no encontrada', 404);
    }

    $db->beginTransaction();

    try {
        // Buscar sesión activa
        $sesionActiva = $db->queryOne('SELECT id, inicio, tarea_id FROM sesiones WHERE fin IS NULL LIMIT 1');

        // ── MODO SUSTITUIR ──────────────────────────────────────────────────
        if ($sustituir && $sesionActiva) {
            $ahora = time();
            $inicioActual = (int)$sesionActiva['inicio'];

            // Garantizar duración mínima de 60s
            $inicioMinimo = $ahora - 60;
            $inicioFinal  = min($inicioActual, $inicioMinimo);
            // (si ya lleva ≥ 60s, $inicioActual ≤ $inicioMinimo y no cambia)

            $db->execute('
                UPDATE sesiones
                SET tarea_id = ?, notas = ?, inicio = ?
                WHERE id = ?
            ', [$tareaId, $notas, $inicioFinal, $sesionActiva['id']]);

            $db->commit();

            // Obtener datos completos de la sesión actualizada
            $sesionData = $db->queryOne('
                SELECT
                    s.id,
                    s.tarea_id,
                    s.inicio,
                    t.tipo_tarea_id,
                    t.actividad_id,
                    tt.nombre as tipo_tarea_nombre,
                    tt.icono  as tipo_tarea_icono,
                    a.nombre  as actividad_nombre,
                    a.color   as actividad_color
                FROM sesiones s
                JOIN tareas t      ON s.tarea_id      = t.id
                JOIN tipos_tarea tt ON t.tipo_tarea_id = tt.id
                JOIN actividades a  ON t.actividad_id  = a.id
                WHERE s.id = ?
            ', [$sesionActiva['id']]);

            $sesionData['server_time']       = time();
            $sesionData['minutos_aplicados'] = null; // no aplica en modo sustituir
            respondSuccess('Sesión sustituida', $sesionData);
            return;
        }

        // ── MODO NORMAL ─────────────────────────────────────────────────────

        $sesionId = generateId('ses_');
        $ahora    = time();
        $medianoche = strtotime('today');

        // Calcular el inicio real de la nueva sesión
        $inicioSolicitado = $ahora - ($minutosRetroactivos * 60);

        if ($sesionActiva) {
            // Límite inferior: la sesión anterior debe terminar con al menos 1 min de duración.
            // Eso significa que el inicio de la nueva no puede ser anterior a
            // inicio_sesion_anterior + 60 s.
            $limiteInferior = max($medianoche, (int)$sesionActiva['inicio'] + 60);
            $inicio = max($inicioSolicitado, $limiteInferior);

            // La sesión anterior termina justo cuando empieza la nueva
            $finAnterior  = $inicio;
            $durAnterior  = $finAnterior - (int)$sesionActiva['inicio'];
            $db->execute('
                UPDATE sesiones SET fin = ?, duracion = ? WHERE id = ?
            ', [$finAnterior, $durAnterior, $sesionActiva['id']]);
            // Incrementar usos_7d del tipo de tarea correspondiente
            $db->execute('
                UPDATE tipos_tarea SET usos_7d = usos_7d + 1
                WHERE id = (SELECT tipo_tarea_id FROM tareas WHERE id = ?)
            ', [$sesionActiva['tarea_id']]);
        } else {
            // Sin sesión activa: solo limitar a medianoche
            $inicio = max($inicioSolicitado, $medianoche);
        }

        // Calcular minutos realmente aplicados (para avisar al frontend si hubo recorte).
        // Se clampea a 0 para evitar valores negativos cuando el inicio calculado
        // queda en el futuro por restricciones de duración mínima de la sesión anterior.
        $minutosAplicados = max(0, (int)round(($ahora - $inicio) / 60));

        $db->execute('
            INSERT INTO sesiones (id, tarea_id, inicio, notas)
            VALUES (?, ?, ?, ?)
        ', [$sesionId, $tareaId, $inicio, $notas]);

        $db->commit();

        // Obtener datos completos de la sesión creada
        $nuevaSesion = $db->queryOne('
            SELECT
                s.id,
                s.tarea_id,
                s.inicio,
                t.tipo_tarea_id,
                t.actividad_id,
                tt.nombre as tipo_tarea_nombre,
                tt.icono  as tipo_tarea_icono,
                a.nombre  as actividad_nombre,
                a.color   as actividad_color
            FROM sesiones s
            JOIN tareas t      ON s.tarea_id      = t.id
            JOIN tipos_tarea tt ON t.tipo_tarea_id = tt.id
            JOIN actividades a  ON t.actividad_id  = a.id
            WHERE s.id = ?
        ', [$sesionId]);

        $nuevaSesion['server_time']       = time();
        $nuevaSesion['minutos_aplicados'] = $minutosAplicados;
        respondSuccess('Sesión iniciada', $nuevaSesion);
    } catch (Exception $e) {
        $db->rollback();
        throw $e;
    }
}

/**
 * GET /api/sesiones?action=acumulado
 * Tiempo total acumulado por actividad (todas las sesiones cerradas)
 */
function getAcumulado($db) {
    $filas = $db->query('
        SELECT
            a.id as actividad_id,
            a.nombre as actividad_nombre,
            a.color as actividad_color,
            SUM(s.duracion) as tiempo_total
        FROM sesiones s
        JOIN tareas t ON s.tarea_id = t.id
        JOIN actividades a ON t.actividad_id = a.id
        WHERE s.fin IS NOT NULL
        GROUP BY a.id, a.nombre, a.color
        ORDER BY tiempo_total DESC
    ');

    // Sumar sesión activa si existe
    $activa = $db->queryOne('
        SELECT s.inicio, t.actividad_id
        FROM sesiones s
        JOIN tareas t ON s.tarea_id = t.id
        WHERE s.fin IS NULL
        LIMIT 1
    ');
    if ($activa) {
        $transcurrido = time() - $activa['inicio'];
        foreach ($filas as &$fila) {
            if ($fila['actividad_id'] === $activa['actividad_id']) {
                $fila['tiempo_total'] = (int)$fila['tiempo_total'] + $transcurrido;
                break;
            }
        }
        unset($fila);
        // Si la actividad activa no tenía sesiones cerradas aún, añadirla
        $ids = array_column($filas, 'actividad_id');
        if (!in_array($activa['actividad_id'], $ids)) {
            $act = $db->queryOne('SELECT id, nombre, color FROM actividades WHERE id = ?', [$activa['actividad_id']]);
            if ($act) {
                $filas[] = [
                    'actividad_id'     => $act['id'],
                    'actividad_nombre' => $act['nombre'],
                    'actividad_color'  => $act['color'],
                    'tiempo_total'     => $transcurrido
                ];
            }
        }
    }

    respondJson([
        'server_time' => time(),
        'actividades' => $filas
    ]);
}

/**
 * POST /api/sesiones?action=detener
 * Detener la sesión activa
 */
function detenerSesion($db) {
    $sesionActiva = $db->queryOne('SELECT id, inicio, tarea_id FROM sesiones WHERE fin IS NULL LIMIT 1');

    if (!$sesionActiva) {
        respondError('No hay sesión activa', 404);
    }

    $fin = time();
    $duracion = $fin - $sesionActiva['inicio'];

    $db->execute('
        UPDATE sesiones SET fin = ?, duracion = ? WHERE id = ?
    ', [$fin, $duracion, $sesionActiva['id']]);

    // Incrementar usos_7d del tipo de tarea correspondiente
    $db->execute('
        UPDATE tipos_tarea SET usos_7d = usos_7d + 1
        WHERE id = (SELECT tipo_tarea_id FROM tareas WHERE id = ?)
    ', [$sesionActiva['tarea_id']]);

    respondSuccess('Sesión detenida', [
        'id' => $sesionActiva['id'],
        'duracion' => $duracion
    ]);
}

/**
 * GET /api/sesiones?action=exportar
 * Descarga el historial completo como CSV sin modificar nada
 */
function exportarCsv($db) {
    enviarCsv(generarCsvString($db));
}

/**
 * Genera el string CSV de todas las sesiones cerradas.
 * Función compartida entre exportarCsv() y resetearDatos().
 */
function generarCsvString($db) {
    $sesiones = $db->query('
        SELECT
            s.id,
            s.inicio,
            s.fin,
            s.duracion,
            s.notas,
            tt.nombre  as tipo_tarea,
            tt.icono   as icono,
            a.nombre   as actividad,
            a.color    as color
        FROM sesiones s
        JOIN tareas t       ON s.tarea_id       = t.id
        JOIN tipos_tarea tt ON t.tipo_tarea_id  = tt.id
        JOIN actividades a  ON t.actividad_id   = a.id
        WHERE s.fin IS NOT NULL
        ORDER BY s.inicio ASC
    ');

    $csv  = "\xEF\xBB\xBF"; // BOM UTF-8 para Excel
    $csv .= "id,inicio,fin,duracion_seg,duracion_hhmm,tipo_tarea,icono,actividad,notas\n";
    foreach ($sesiones as $s) {
        $inicioFmt = $s['inicio'] ? date('Y-m-d H:i', (int)$s['inicio']) : '';
        $finFmt    = $s['fin']    ? date('Y-m-d H:i', (int)$s['fin'])    : '';
        $dur       = (int)$s['duracion'];
        $durFmt    = sprintf('%d:%02d', intdiv($dur, 3600), intdiv($dur % 3600, 60));
        $notas     = str_replace(["\r", "\n", '"'], [' ', ' ', '""'], (string)$s['notas']);
        $csv .= implode(',', [
            $s['id'],
            $inicioFmt,
            $finFmt,
            $dur,
            $durFmt,
            '"' . str_replace('"', '""', $s['tipo_tarea']) . '"',
            $s['icono'],
            '"' . str_replace('"', '""', $s['actividad']) . '"',
            '"' . $notas . '"'
        ]) . "\n";
    }
    return $csv;
}

/**
 * Envía un string CSV como descarga al navegador.
 * Envuelta en function_exists para permitir que el bootstrap de tests la sobreescriba.
 */
if (!function_exists('enviarCsv')) {
    function enviarCsv($csv) {
        $filename = 'cronometro-psp-' . date('Y-m-d') . '.csv';
        header('Content-Type: text/csv; charset=UTF-8');
        header('Content-Disposition: attachment; filename="' . $filename . '"');
        header('Cache-Control: no-cache');
        echo $csv;
        exit;
    }
}

/**
 * POST /api/sesiones?action=reset
 * Body: { conservar: ["act-id-1", "act-id-2", ...] }
 *
 * 1. Detiene la sesión activa si existe
 * 2. Genera CSV con todas las sesiones
 * 3. Borra todas las sesiones y resetea usos_7d
 * 4. Archiva las actividades NO incluidas en "conservar"
 * 5. Archiva los tipos_tarea que quedan sin ninguna actividad activa
 * Devuelve el CSV como descarga
 */
function resetearDatos($db) {
    $body = getRequestBody();
    // IDs de actividades a conservar (array de strings)
    $conservar = isset($body['conservar']) && is_array($body['conservar'])
        ? array_values(array_filter($body['conservar'], 'is_string'))
        : [];

    // 1. Cerrar sesión activa si existe
    $activa = $db->queryOne('SELECT id, inicio FROM sesiones WHERE fin IS NULL LIMIT 1');
    if ($activa) {
        $fin = time();
        $duracion = $fin - $activa['inicio'];
        $db->execute('UPDATE sesiones SET fin = ?, duracion = ? WHERE id = ?',
            [$fin, $duracion, $activa['id']]);
    }

    // 2. Generar CSV antes de borrar (aún en memoria, no se envía todavía)
    $csv = generarCsvString($db);

    // 3. Borrar sesiones y resetear usos_7d
    $db->execute('DELETE FROM sesiones');
    $db->execute('UPDATE tipos_tarea SET usos_7d = 0');

    // 4. Archivar actividades no conservadas (si se envió lista)
    if (!empty($conservar)) {
        // Construir placeholders para el IN
        $placeholders = implode(',', array_fill(0, count($conservar), '?'));
        $db->execute(
            "UPDATE actividades SET archived = 1 WHERE archived = 0 AND id NOT IN ($placeholders)",
            $conservar
        );
    } else {
        // Sin lista explícita: archivar todas las no-permanentes
        $db->execute('UPDATE actividades SET archived = 1 WHERE archived = 0 AND permanente = 0');
    }

    // 5. Archivar tipos_tarea que ya no tienen ninguna actividad activa
    //    Un tipo_tarea tiene actividades_permitidas como JSON array de IDs.
    //    Obtenemos las actividades activas y limpiamos los tipos_tarea.
    $activasRestantes = $db->query('SELECT id FROM actividades WHERE archived = 0');
    $idsActivas = array_column($activasRestantes, 'id');

    $todosTipos = $db->query('SELECT id, actividades_permitidas FROM tipos_tarea WHERE archived = 0');
    foreach ($todosTipos as $tipo) {
        $permitidas = json_decode($tipo['actividades_permitidas'], true) ?: [];
        // Filtrar: quedarse solo con actividades que siguen activas
        $permitidasFiltradas = array_values(array_intersect($permitidas, $idsActivas));

        if (empty($permitidasFiltradas)) {
            // Sin actividades activas → archivar el tipo de tarea
            $db->execute('UPDATE tipos_tarea SET archived = 1 WHERE id = ?', [$tipo['id']]);
        } else if (count($permitidasFiltradas) !== count($permitidas)) {
            // Algunas actividades se archivaron → actualizar la lista
            $db->execute(
                'UPDATE tipos_tarea SET actividades_permitidas = ? WHERE id = ?',
                [json_encode($permitidasFiltradas), $tipo['id']]
            );
        }
    }

    // 6. Devolver CSV
    enviarCsv($csv);
}
