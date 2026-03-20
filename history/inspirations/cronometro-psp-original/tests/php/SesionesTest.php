<?php
/**
 * SesionesTest.php — Tests unitarios para backend/api/sesiones.php
 *
 * Cubre:
 *   iniciarSesion()  — modo normal sin sesión previa
 *                    — modo normal con sesión previa (la cierra)
 *                    — retroactivo aplicado íntegramente
 *                    — retroactivo recortado por sesión anterior (mínimo 1 min)
 *                    — retroactivo recortado por medianoche
 *                    — minutos_aplicados devuelto correctamente en todos los casos
 *                    — modo sustituir con sesión activa
 *                    — modo sustituir sin sesión activa → crea sesión nueva
 *                    — modo sustituir garantiza duración mínima 60s
 *                    — tarea inexistente → 404
 *   detenerSesion()  — happy path: cierra sesión + calcula duración + usos_7d
 *                    — sin sesión activa → 404
 *   resetearDatos()  — borra sesiones + resetea usos_7d
 *                    — archiva actividades no conservadas
 *                    — conserva actividades marcadas
 *                    — archiva tipos_tarea huérfanos
 *                    — genera CSV con las sesiones previas
 */

declare(strict_types=1);

use PHPUnit\Framework\TestCase;

class SesionesTest extends TestCase
{
    private PDO $pdo;

    // ─── Fixtures ────────────────────────────────────────────────────────────

    /** IDs de fixtures base (disponibles en todos los tests) */
    private const ACT1   = 'act1';
    private const ACT2   = 'act2';
    private const TT1    = 'tt1';
    private const TT2    = 'tt2';
    private const TAREA1 = 'tt1_act1';
    private const TAREA2 = 'tt2_act2';

    protected function setUp(): void
    {
        // BD SQLite en memoria: estado fresco para cada test
        $this->pdo = new PDO('sqlite::memory:');
        $this->pdo->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
        $this->pdo->setAttribute(PDO::ATTR_DEFAULT_FETCH_MODE, PDO::FETCH_ASSOC);

        // Cargar schema de test
        $schema = file_get_contents(__DIR__ . '/fixtures/test_schema.sql');
        $this->pdo->exec($schema);

        // Registrar instancia de test en el singleton
        Database::setTestInstance($this->pdo);

        // Insertar fixtures base
        $now = time();
        $this->pdo->exec("
            INSERT INTO actividades VALUES ('" . self::ACT1 . "','Proyecto X','#667eea',$now,0,0);
            INSERT INTO actividades VALUES ('" . self::ACT2 . "','Admin','#fa709a',$now,0,0);
            INSERT INTO tipos_tarea VALUES ('" . self::TT1 . "','Codificar','code','[\"" . self::ACT1 . "\"]',5,$now,0);
            INSERT INTO tipos_tarea VALUES ('" . self::TT2 . "','Email','email','[\"" . self::ACT2 . "\"]',3,$now,0);
            INSERT INTO tareas VALUES ('" . self::TAREA1 . "','" . self::TT1 . "','" . self::ACT1 . "');
            INSERT INTO tareas VALUES ('" . self::TAREA2 . "','" . self::TT2 . "','" . self::ACT2 . "');
        ");

        // Limpiar estado global entre tests
        $_GET = [];
        $GLOBALS['_TEST_BODY'] = [];
    }

    protected function tearDown(): void
    {
        Database::resetInstance();
    }

    // ─── Helper: invocar función API y capturar respuesta ────────────────────

    /**
     * Llama a una función de sesiones.php y devuelve la respuesta capturada.
     * @return array{data: array, status: int}
     */
    private function callApi(callable $fn): array
    {
        try {
            $fn(Database::getInstance());
            $this->fail('La función debería haber lanzado TestResponseException');
        } catch (TestResponseException $e) {
            return ['data' => $e->data, 'status' => $e->statusCode];
        } catch (TestCsvResponseException $e) {
            return ['csv' => $e->csv, 'status' => 200];
        }
        // @phpstan-ignore-next-line
        return [];
    }

    private function iniciar(array $body): array
    {
        $GLOBALS['_TEST_BODY'] = $body;
        return $this->callApi('iniciarSesion');
    }

    private function detener(): array
    {
        $GLOBALS['_TEST_BODY'] = [];
        return $this->callApi('detenerSesion');
    }

    private function sesionActivaEnBd(): ?array
    {
        $row = $this->pdo->query("SELECT * FROM sesiones WHERE fin IS NULL")->fetch();
        return $row === false ? null : $row;
    }

    private function sesionEnBd(string $id): ?array
    {
        $stmt = $this->pdo->prepare("SELECT * FROM sesiones WHERE id = ?");
        $stmt->execute([$id]);
        $row = $stmt->fetch();
        return $row === false ? null : $row;
    }

    private function usos7d(string $tipoTareaId): int
    {
        $stmt = $this->pdo->prepare("SELECT usos_7d FROM tipos_tarea WHERE id = ?");
        $stmt->execute([$tipoTareaId]);
        return (int)$stmt->fetchColumn();
    }

    private function insertarSesionActiva(string $id, string $tareaId, int $inicioOffset = -300): void
    {
        $inicio = time() + $inicioOffset;
        $this->pdo->prepare("INSERT INTO sesiones VALUES (?,?,?,NULL,NULL,NULL)")
                  ->execute([$id, $tareaId, $inicio]);
    }

    // =========================================================================
    // iniciarSesion — MODO NORMAL SIN SESIÓN PREVIA
    // =========================================================================

    public function test_iniciar_sinSesionPrevia_creaUna(): void
    {
        $resp = $this->iniciar(['tarea_id' => self::TAREA1]);

        $this->assertSame(200, $resp['status']);
        $this->assertTrue($resp['data']['success']);

        $sesion = $resp['data']['data'];
        $this->assertStringStartsWith('ses_', $sesion['id']);
        $this->assertSame(self::TAREA1, $sesion['tarea_id']);
        $this->assertArrayHasKey('server_time', $sesion);
        $this->assertArrayHasKey('minutos_aplicados', $sesion);

        // En BD: exactamente una sesión activa
        $this->assertNotNull($this->sesionActivaEnBd());
    }

    public function test_iniciar_sinSesionPrevia_sinRetroactivo_minutosAplicados0(): void
    {
        $resp = $this->iniciar(['tarea_id' => self::TAREA1]);
        $this->assertSame(0, $resp['data']['data']['minutos_aplicados']);
    }

    // =========================================================================
    // iniciarSesion — MODO NORMAL CON SESIÓN PREVIA
    // =========================================================================

    public function test_iniciar_conSesionPrevia_cierraLaAnterior(): void
    {
        $this->insertarSesionActiva('ses_prev', self::TAREA1, -300);

        $resp = $this->iniciar(['tarea_id' => self::TAREA2]);

        $this->assertSame(200, $resp['status']);

        $anterior = $this->sesionEnBd('ses_prev');
        $this->assertNotNull($anterior['fin'],      'La sesión anterior debe tener fin');
        $this->assertNotNull($anterior['duracion'], 'La sesión anterior debe tener duración');
        $this->assertGreaterThan(0, (int)$anterior['duracion']);
    }

    public function test_iniciar_conSesionPrevia_incrementaUsos7d(): void
    {
        $this->insertarSesionActiva('ses_prev', self::TAREA1, -300);
        $usosAntes = $this->usos7d(self::TT1); // 5 del fixture

        $this->iniciar(['tarea_id' => self::TAREA2]);

        $this->assertSame($usosAntes + 1, $this->usos7d(self::TT1));
    }

    public function test_iniciar_conSesionPrevia_hayNuevaSesionActiva(): void
    {
        $this->insertarSesionActiva('ses_prev', self::TAREA1, -300);

        $resp = $this->iniciar(['tarea_id' => self::TAREA2]);

        $activa = $this->sesionActivaEnBd();
        $this->assertNotNull($activa);
        $this->assertSame($resp['data']['data']['id'], $activa['id']);
        $this->assertSame(self::TAREA2, $activa['tarea_id']);
    }

    // =========================================================================
    // iniciarSesion — RETROACTIVO APLICADO ÍNTEGRAMENTE
    // =========================================================================

    public function test_iniciar_retroactivoAplicado_sinSesionPrevia(): void
    {
        $resp = $this->iniciar(['tarea_id' => self::TAREA1, 'minutos_retroactivos' => 5]);

        $sesion = $resp['data']['data'];
        $this->assertSame(5, $sesion['minutos_aplicados']);

        // El inicio debe estar ~300s antes del server_time
        $retroSegundos = $sesion['server_time'] - (int)$sesion['inicio'];
        $this->assertGreaterThanOrEqual(290, $retroSegundos);
        $this->assertLessThanOrEqual(310, $retroSegundos);
    }

    public function test_iniciar_retroactivoAplicado_conSesionPreviaLarga(): void
    {
        // Sesión previa de 10 minutos → caben 5 min retroactivos (quedan 5 min para la anterior)
        $this->insertarSesionActiva('ses_prev', self::TAREA1, -600);

        $resp = $this->iniciar(['tarea_id' => self::TAREA2, 'minutos_retroactivos' => 5]);

        $this->assertSame(5, $resp['data']['data']['minutos_aplicados']);

        // La sesión anterior debe durar exactamente 5 min (600 - 300 = 300s)
        $anterior = $this->sesionEnBd('ses_prev');
        $this->assertGreaterThanOrEqual(295, (int)$anterior['duracion']);
        $this->assertLessThanOrEqual(310, (int)$anterior['duracion']);
    }

    // =========================================================================
    // iniciarSesion — RETROACTIVO RECORTADO POR SESIÓN ANTERIOR (mínimo 1 min)
    // =========================================================================

    public function test_iniciar_retroactivoRecortado_sesionPreviaCorta(): void
    {
        // Sesión previa de solo 90 segundos → solo puede retrotraer 30s (90 - 60 = 30s)
        $this->insertarSesionActiva('ses_prev', self::TAREA1, -90);

        $resp = $this->iniciar(['tarea_id' => self::TAREA2, 'minutos_retroactivos' => 10]);

        $minutosAplicados = $resp['data']['data']['minutos_aplicados'];
        // Con 90s de sesión previa, el máximo aplicable es 0 min (90-60=30s < 1min)
        $this->assertLessThan(10, $minutosAplicados);
        // La sesión anterior no puede tener menos de 60s de duración
        $anterior = $this->sesionEnBd('ses_prev');
        $this->assertGreaterThanOrEqual(60, (int)$anterior['duracion']);
    }

    public function test_iniciar_retroactivoRecortado_sesionActivaMuyReciente(): void
    {
        // Sesión previa de solo 30 segundos → no puede retrotraer nada
        $this->insertarSesionActiva('ses_prev', self::TAREA1, -30);

        $resp = $this->iniciar(['tarea_id' => self::TAREA2, 'minutos_retroactivos' => 5]);

        $this->assertSame(0, $resp['data']['data']['minutos_aplicados']);

        $anterior = $this->sesionEnBd('ses_prev');
        $this->assertGreaterThanOrEqual(60, (int)$anterior['duracion'],
            'La sesión anterior debe tener mínimo 60s de duración');
    }

    // =========================================================================
    // iniciarSesion — RETROACTIVO RECORTADO POR MEDIANOCHE
    // =========================================================================

    public function test_iniciar_retroactivoRecortado_porMedianoche(): void
    {
        // Sin sesión previa, pedimos más minutos de los que han pasado desde medianoche
        $medianoche       = strtotime('today');
        $minutosDesdeMdn  = (int)floor((time() - $medianoche) / 60);
        $minutosPedidos   = $minutosDesdeMdn + 120; // siempre mayor al límite

        $resp = $this->iniciar([
            'tarea_id'             => self::TAREA1,
            'minutos_retroactivos' => $minutosPedidos,
        ]);

        $minutosAplicados = $resp['data']['data']['minutos_aplicados'];
        $this->assertLessThanOrEqual($minutosDesdeMdn + 1, $minutosAplicados,
            'No puede retrotraer más allá de medianoche');
        $this->assertLessThan($minutosPedidos, $minutosAplicados);
    }

    // =========================================================================
    // iniciarSesion — TAREA INEXISTENTE → 404
    // =========================================================================

    public function test_iniciar_tareaInexistente_devuelve404(): void
    {
        $resp = $this->iniciar(['tarea_id' => 'no_existe_jamas']);

        $this->assertSame(404, $resp['status']);
        $this->assertArrayHasKey('error', $resp['data']);
    }

    // =========================================================================
    // iniciarSesion — MODO SUSTITUIR CON SESIÓN ACTIVA
    // =========================================================================

    public function test_iniciar_sustituir_conSesionActiva_actualizaTareaYNotas(): void
    {
        $this->insertarSesionActiva('ses_orig', self::TAREA1, -300);

        $resp = $this->iniciar([
            'tarea_id' => self::TAREA2,
            'notas'    => 'nueva nota',
            'sustituir' => true,
        ]);

        $this->assertSame(200, $resp['status']);
        $this->assertStringContainsString('sustituida', $resp['data']['message']);

        // La sesión en BD debe seguir siendo la misma (mismo id)
        $sesion = $this->sesionActivaEnBd();
        $this->assertNotNull($sesion);
        $this->assertSame('ses_orig', $sesion['id']);
        $this->assertSame(self::TAREA2, $sesion['tarea_id']);
        $this->assertSame('nueva nota', $sesion['notas']);
    }

    public function test_iniciar_sustituir_conSesionActiva_noCreaNuevaSesion(): void
    {
        $this->insertarSesionActiva('ses_orig', self::TAREA1, -300);

        $this->iniciar(['tarea_id' => self::TAREA2, 'sustituir' => true]);

        $count = (int)$this->pdo->query("SELECT COUNT(*) FROM sesiones")->fetchColumn();
        $this->assertSame(1, $count, 'Solo debe existir una sesión (la original sustituida)');
    }

    public function test_iniciar_sustituir_conSesionActiva_minutosAplicadosNull(): void
    {
        $this->insertarSesionActiva('ses_orig', self::TAREA1, -300);

        $resp = $this->iniciar(['tarea_id' => self::TAREA2, 'sustituir' => true]);

        $this->assertNull($resp['data']['data']['minutos_aplicados']);
    }

    public function test_iniciar_sustituir_conservaInicioSiDuracionSuficiente(): void
    {
        $inicioOriginal = time() - 300; // 5 minutos
        $this->pdo->prepare("INSERT INTO sesiones VALUES ('ses_orig',?,?,NULL,NULL,NULL)")
                  ->execute([self::TAREA1, $inicioOriginal]);

        $this->iniciar(['tarea_id' => self::TAREA2, 'sustituir' => true]);

        $sesion = $this->sesionEnBd('ses_orig');
        // El inicio no debe haber cambiado (ya tenía >= 60s)
        $this->assertSame($inicioOriginal, (int)$sesion['inicio']);
    }

    // =========================================================================
    // iniciarSesion — MODO SUSTITUIR GARANTIZA DURACIÓN MÍNIMA 60s
    // =========================================================================

    public function test_iniciar_sustituir_ajustaInicioSiMenosDe60s(): void
    {
        $inicioMuyReciente = time() - 10; // solo 10 segundos
        $this->pdo->prepare("INSERT INTO sesiones VALUES ('ses_corta',?,?,NULL,NULL,NULL)")
                  ->execute([self::TAREA1, $inicioMuyReciente]);

        $this->iniciar(['tarea_id' => self::TAREA2, 'sustituir' => true]);

        $sesion = $this->sesionEnBd('ses_corta');
        $duracionActual = time() - (int)$sesion['inicio'];
        $this->assertGreaterThanOrEqual(60, $duracionActual,
            'El inicio debe haberse retrocedido para garantizar ≥ 60s');
    }

    // =========================================================================
    // iniciarSesion — MODO SUSTITUIR SIN SESIÓN ACTIVA → comportamiento normal
    // =========================================================================

    public function test_iniciar_sustituir_sinSesionActiva_creaNuevaSesion(): void
    {
        $resp = $this->iniciar(['tarea_id' => self::TAREA1, 'sustituir' => true]);

        // Debe comportarse como inicio normal (crea sesión nueva)
        $this->assertSame(200, $resp['status']);
        $this->assertStringStartsWith('ses_', $resp['data']['data']['id']);

        $count = (int)$this->pdo->query("SELECT COUNT(*) FROM sesiones")->fetchColumn();
        $this->assertSame(1, $count);
    }

    // =========================================================================
    // detenerSesion — HAPPY PATH
    // =========================================================================

    public function test_detener_cierraSesionYCalculaDuracion(): void
    {
        $this->insertarSesionActiva('ses_activa', self::TAREA1, -180);

        $resp = $this->detener();

        $this->assertSame(200, $resp['status']);
        $this->assertTrue($resp['data']['success']);

        $data = $resp['data']['data'];
        $this->assertSame('ses_activa', $data['id']);
        $this->assertGreaterThanOrEqual(180, $data['duracion']);

        // En BD: sesión cerrada
        $sesion = $this->sesionEnBd('ses_activa');
        $this->assertNotNull($sesion['fin'],      'La sesión debe tener fin');
        $this->assertNotNull($sesion['duracion'], 'La sesión debe tener duración');
    }

    public function test_detener_incrementaUsos7d(): void
    {
        $this->insertarSesionActiva('ses_activa', self::TAREA1, -120);
        $usosAntes = $this->usos7d(self::TT1);

        $this->detener();

        $this->assertSame($usosAntes + 1, $this->usos7d(self::TT1));
    }

    public function test_detener_sinSesionActiva_devuelve404(): void
    {
        $resp = $this->detener();

        $this->assertSame(404, $resp['status']);
        $this->assertArrayHasKey('error', $resp['data']);
    }

    // =========================================================================
    // resetearDatos
    // =========================================================================

    public function test_reset_borraSesionesYResetea7d(): void
    {
        $ahora = time();
        // Insertar sesiones cerradas
        $this->pdo->exec("
            INSERT INTO sesiones VALUES ('s1','" . self::TAREA1 . "',".($ahora-3600).",$ahora,3600,NULL);
            INSERT INTO sesiones VALUES ('s2','" . self::TAREA1 . "',".($ahora-1800).",$ahora,1800,NULL);
        ");

        $GLOBALS['_TEST_BODY'] = ['conservar' => [self::ACT1, self::ACT2]];
        $resp = $this->callApi('resetearDatos');

        $this->assertArrayHasKey('csv', $resp); // resetearDatos devuelve CSV

        // Sesiones borradas
        $count = (int)$this->pdo->query("SELECT COUNT(*) FROM sesiones")->fetchColumn();
        $this->assertSame(0, $count);

        // usos_7d reseteados
        $this->assertSame(0, $this->usos7d(self::TT1));
        $this->assertSame(0, $this->usos7d(self::TT2));
    }

    public function test_reset_archivaNoCons_conservaLasMarcadas(): void
    {
        $ahora = time();
        $this->pdo->exec("INSERT INTO sesiones VALUES ('s1','" . self::TAREA1 . "',".($ahora-3600).",$ahora,3600,NULL)");

        // Conservar solo act1, act2 queda archivada
        $GLOBALS['_TEST_BODY'] = ['conservar' => [self::ACT1]];
        $this->callApi('resetearDatos');

        $act1 = $this->pdo->query("SELECT archived FROM actividades WHERE id = '" . self::ACT1 . "'")->fetch();
        $act2 = $this->pdo->query("SELECT archived FROM actividades WHERE id = '" . self::ACT2 . "'")->fetch();

        // PHP 8+ PDO devuelve int nativo para columnas INTEGER en SQLite
        $this->assertSame(0, (int)$act1['archived'], 'act1 debe conservarse');
        $this->assertSame(1, (int)$act2['archived'], 'act2 debe archivarse');
    }

    public function test_reset_archivaTiposTareaHuerfanos(): void
    {
        $ahora = time();
        $this->pdo->exec("INSERT INTO sesiones VALUES ('s1','" . self::TAREA1 . "',".($ahora-3600).",$ahora,3600,NULL)");

        // Conservar act1 pero NO act2 → tt2 queda huérfano
        $GLOBALS['_TEST_BODY'] = ['conservar' => [self::ACT1]];
        $this->callApi('resetearDatos');

        $tt2 = $this->pdo->query("SELECT archived FROM tipos_tarea WHERE id = '" . self::TT2 . "'")->fetch();
        $this->assertSame(1, (int)$tt2['archived'], 'tt2 debe archivarse por quedarse sin actividades');

        $tt1 = $this->pdo->query("SELECT archived FROM tipos_tarea WHERE id = '" . self::TT1 . "'")->fetch();
        $this->assertSame(0, (int)$tt1['archived'], 'tt1 debe conservarse');
    }

    public function test_reset_csvContieneTodasLasSesiones(): void
    {
        $ahora = time();
        $this->pdo->exec("
            INSERT INTO sesiones VALUES ('s1','" . self::TAREA1 . "',".($ahora-3600).",$ahora,3600,'nota test');
        ");

        $GLOBALS['_TEST_BODY'] = ['conservar' => [self::ACT1, self::ACT2]];
        $resp = $this->callApi('resetearDatos');

        $this->assertArrayHasKey('csv', $resp);
        $this->assertStringContainsString('tipo_tarea', $resp['csv']); // cabecera CSV
        $this->assertStringContainsString('nota test', $resp['csv']);  // dato de la sesión
    }

    public function test_reset_cierraSessionActivaSiExiste(): void
    {
        $this->insertarSesionActiva('ses_activa', self::TAREA1, -120);

        $GLOBALS['_TEST_BODY'] = ['conservar' => [self::ACT1, self::ACT2]];
        $this->callApi('resetearDatos');

        // Todas las sesiones deben estar borradas (incluida la que estaba activa)
        $count = (int)$this->pdo->query("SELECT COUNT(*) FROM sesiones")->fetchColumn();
        $this->assertSame(0, $count);
    }
}
