<?php
/**
 * Clase para gestionar la conexión a SQLite
 */
if (!class_exists('Database')) :
class Database {
    private static $instance = null;
    private $db;
    private $dbPath;

    private function __construct() {
        // Ruta persistente en el disco duro del NAS (sobrevive reinicios)
        // Si no existe esa ruta (entorno de desarrollo), usar la ruta local relativa
        $persistentPath = '/mnt/HD/HD_a2/.cronometro-psp/data/cronometro.db';
        $localPath      = __DIR__ . '/../../data/cronometro.db';
        $this->dbPath   = is_dir(dirname($persistentPath)) ? $persistentPath : $localPath;

        // Crear directorio data si no existe
        $dataDir = dirname($this->dbPath);
        if (!is_dir($dataDir)) {
            mkdir($dataDir, 0777, true);
        }

        $dbExists = file_exists($this->dbPath);

        try {
            $this->db = new PDO('sqlite:' . $this->dbPath);
            $this->db->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
            $this->db->setAttribute(PDO::ATTR_DEFAULT_FETCH_MODE, PDO::FETCH_ASSOC);

            // Si la BD no existía, inicializarla con el schema
            if (!$dbExists) {
                $this->initializeSchema();
            }
        } catch (PDOException $e) {
            die('Error de conexión a la base de datos: ' . $e->getMessage());
        }
    }

    public static function getInstance() {
        if (self::$instance === null) {
            self::$instance = new self();
        }
        return self::$instance;
    }

    public function getConnection() {
        return $this->db;
    }

    private function initializeSchema() {
        $schemaPath = __DIR__ . '/schema.sql';

        if (!file_exists($schemaPath)) {
            throw new Exception('No se encuentra el archivo schema.sql');
        }

        $schema = file_get_contents($schemaPath);

        try {
            $this->db->exec($schema);
        } catch (PDOException $e) {
            throw new Exception('Error al inicializar la base de datos: ' . $e->getMessage());
        }
    }

    /**
     * Ejecutar una consulta SELECT y devolver todos los resultados
     */
    public function query($sql, $params = []) {
        try {
            $stmt = $this->db->prepare($sql);
            $stmt->execute($params);
            return $stmt->fetchAll();
        } catch (PDOException $e) {
            throw new Exception('Error en query: ' . $e->getMessage());
        }
    }

    /**
     * Ejecutar una consulta SELECT y devolver un solo resultado
     */
    public function queryOne($sql, $params = []) {
        try {
            $stmt = $this->db->prepare($sql);
            $stmt->execute($params);
            return $stmt->fetch();
        } catch (PDOException $e) {
            throw new Exception('Error en queryOne: ' . $e->getMessage());
        }
    }

    /**
     * Ejecutar una consulta INSERT, UPDATE o DELETE
     */
    public function execute($sql, $params = []) {
        try {
            $stmt = $this->db->prepare($sql);
            $stmt->execute($params);
            return $stmt->rowCount();
        } catch (PDOException $e) {
            throw new Exception('Error en execute: ' . $e->getMessage());
        }
    }

    /**
     * Obtener el ID del último registro insertado
     */
    public function lastInsertId() {
        return $this->db->lastInsertId();
    }

    /**
     * Iniciar una transacción
     */
    public function beginTransaction() {
        return $this->db->beginTransaction();
    }

    /**
     * Confirmar una transacción
     */
    public function commit() {
        return $this->db->commit();
    }

    /**
     * Revertir una transacción
     */
    public function rollback() {
        return $this->db->rollBack();
    }
}
endif; // !class_exists('Database')
