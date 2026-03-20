<?php
/**
 * bootstrap.php — Entorno de test para PHPUnit
 *
 * Estrategia:
 * 1. Define todas las funciones que sesiones.php importa de config.php
 *    (respondJson, respondError, respondSuccess, getRequestBody, …)
 *    para que NOT sea necesario hacer require de config.php (que emite
 *    header() al cargarse, inválido en CLI).
 * 2. Reemplaza la clase Database real (singleton con ruta al NAS) por
 *    una implementación de test que delega en un PDO in-memory.
 * 3. Las funciones respondJson/respondSuccess/respondError lanzan
 *    TestResponseException en lugar de exit, permitiendo que el test
 *    capture la "respuesta" con try/catch.
 * 4. enviarCsv() lanza TestCsvResponseException para que resetearDatos
 *    también sea testeable.
 */

declare(strict_types=1);

date_default_timezone_set('Europe/Madrid');

// ─── Excepciones de captura de respuesta ─────────────────────────────────────

// Extendemos Error (no Exception) para que los catch (Exception $e) del código
// de producción no capturen estas excepciones de test y no interfieran con el
// control de flujo (p.ej. rollback tras un commit ya realizado).
class TestResponseException extends Error
{
    public array $data;
    public int   $statusCode;

    public function __construct(array $data, int $statusCode)
    {
        $this->data       = $data;
        $this->statusCode = $statusCode;
        parent::__construct(json_encode($data));
    }
}

class TestCsvResponseException extends Error
{
    public string $csv;

    public function __construct(string $csv)
    {
        $this->csv = $csv;
        parent::__construct('CSV response');
    }
}

// ─── Stubs de las funciones HTTP de config.php ───────────────────────────────

function respondJson(array $data, int $statusCode = 200): void
{
    throw new TestResponseException($data, $statusCode);
}

function respondError(string $message, int $statusCode = 400): void
{
    respondJson(['error' => $message], $statusCode);
}

function respondSuccess(string $message = 'Operación exitosa', $data = null): void
{
    $response = ['success' => true, 'message' => $message];
    if ($data !== null) {
        $response['data'] = $data;
    }
    respondJson($response);
}

function getRequestBody(): array
{
    // En tests, el cuerpo se inyecta via $GLOBALS['_TEST_BODY']
    return $GLOBALS['_TEST_BODY'] ?? [];
}

function getQueryParam(string $name, $default = null)
{
    return $_GET[$name] ?? $default;
}

function requireParams(array $params, array $requiredFields): void
{
    foreach ($requiredFields as $field) {
        if (!isset($params[$field]) || $params[$field] === '') {
            respondError("Campo requerido: $field", 400);
        }
    }
}

function generateId(string $prefix = ''): string
{
    return $prefix . bin2hex(random_bytes(8));
}

function slugify(string $text): string
{
    $text = mb_strtolower($text, 'UTF-8');
    $text = preg_replace('/[^a-z0-9\s-]/', '', $text);
    $text = preg_replace('/[\s-]+/', '-', $text);
    return trim($text, '-');
}

function enviarCsv(string $csv): void
{
    throw new TestCsvResponseException($csv);
}

// ─── Clase Database de test (sustituye al singleton real) ────────────────────
// Se define ANTES de que sesiones.php sea incluido, por lo que PHP usa esta
// versión y nunca ve la clase real (no se pueden redefinir clases).

class Database
{
    private static ?Database $instance = null;
    private PDO $db;

    private function __construct(PDO $db)
    {
        $this->db = $db;
    }

    /** Registra el PDO in-memory creado por cada test */
    public static function setTestInstance(PDO $pdo): void
    {
        self::$instance = new self($pdo);
    }

    /** Elimina la instancia al finalizar cada test */
    public static function resetInstance(): void
    {
        self::$instance = null;
    }

    public static function getInstance(): self
    {
        if (self::$instance === null) {
            throw new RuntimeException(
                'Database no inicializada. Llama a Database::setTestInstance() en setUp().'
            );
        }
        return self::$instance;
    }

    public function query(string $sql, array $params = []): array
    {
        $stmt = $this->db->prepare($sql);
        $stmt->execute($params);
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    }

    public function queryOne(string $sql, array $params = [])
    {
        $stmt = $this->db->prepare($sql);
        $stmt->execute($params);
        $row = $stmt->fetch(PDO::FETCH_ASSOC);
        return $row === false ? null : $row;
    }

    public function execute(string $sql, array $params = []): int
    {
        $stmt = $this->db->prepare($sql);
        $stmt->execute($params);
        return $stmt->rowCount();
    }

    public function lastInsertId(): string
    {
        return $this->db->lastInsertId();
    }

    public function beginTransaction(): bool
    {
        return $this->db->beginTransaction();
    }

    public function commit(): bool
    {
        return $this->db->commit();
    }

    public function rollback(): bool
    {
        return $this->db->rollBack();
    }
}

// ─── Incluir el código bajo test ─────────────────────────────────────────────
// sesiones.php empieza con require_once config.php y $db = Database::getInstance()
// pero en CLI:
//   • require_once config.php fallaría en los header() → lo omitimos (ya definimos
//     todas sus funciones arriba).
//   • $db = Database::getInstance() en el scope global del archivo; como usamos
//     require dentro de funciones de test y el singleton ya está inicializado,
//     esto es seguro. Sin embargo, sesiones.php tiene ese require al principio
//     junto con el switch de enrutamiento. Para evitar ejecutar el enrutamiento
//     en el require, lo envolvemos con un buffer de salida y gestionamos la
//     excepción que podría lanzar el switch (action no especificada).
//
// Solución: incluimos sesiones.php con output buffering activo. El switch intenta
// llamar a respondError('Acción no especificada') → lanza TestResponseException →
// la capturamos silenciosamente aquí. Las funciones quedan definidas en el scope
// global para que los tests las puedan llamar directamente.

$_SERVER['REQUEST_METHOD'] = 'GET'; // valor inocuo para el switch
$_GET = [];
$GLOBALS['_TEST_BODY'] = [];

// Forzar que require_once config.php sea un no-op si sesiones.php lo llama:
// No podemos evitar el require_once dentro de sesiones.php, pero como config.php
// llama a header() que en CLI solo emite un warning suprimible, lo gestionamos
// suprimiendo errores durante el include:
$prevErrorLevel = error_reporting(0);

try {
    ob_start();
    require_once __DIR__ . '/../../backend/api/sesiones.php';
    ob_end_clean();
} catch (Throwable $e) {
    // Silenciamos cualquier excepción durante la carga de sesiones.php:
    // - TestResponseException: respuesta al switch de enrutamiento (acción no definida)
    // - RuntimeException: Database::getInstance() falla porque aún no hay instancia de test
    // En ambos casos el objetivo del bootstrap (definir las funciones en el scope global)
    // ya se ha cumplido antes de que se lance la excepción.
    if (ob_get_level() > 0) {
        ob_end_clean();
    }
}

error_reporting($prevErrorLevel);
