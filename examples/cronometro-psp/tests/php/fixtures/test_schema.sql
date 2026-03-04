-- Schema mínimo para tests unitarios PHP
-- Replica la estructura real de producción SIN datos iniciales
-- Los fixtures se insertan por test en SesionesTest.php

CREATE TABLE actividades (
    id         TEXT    PRIMARY KEY,
    nombre     TEXT    NOT NULL,
    color      TEXT    NOT NULL,
    created_at INTEGER NOT NULL,
    archived   INTEGER DEFAULT 0,
    permanente INTEGER DEFAULT 0
);

CREATE TABLE tipos_tarea (
    id                     TEXT    PRIMARY KEY,
    nombre                 TEXT    NOT NULL,
    icono                  TEXT    NOT NULL,
    actividades_permitidas TEXT    NOT NULL, -- JSON array de IDs de actividades
    usos_7d                INTEGER DEFAULT 0,
    created_at             INTEGER NOT NULL,
    archived               INTEGER DEFAULT 0
);

CREATE TABLE tareas (
    id            TEXT PRIMARY KEY,   -- Formato: "tipo_tarea_id_actividad_id"
    tipo_tarea_id TEXT NOT NULL,
    actividad_id  TEXT NOT NULL,
    UNIQUE(tipo_tarea_id, actividad_id)
);

CREATE TABLE sesiones (
    id       TEXT    PRIMARY KEY,
    tarea_id TEXT    NOT NULL,
    inicio   INTEGER NOT NULL,
    fin      INTEGER,
    duracion INTEGER,
    notas    TEXT
);

CREATE INDEX idx_sesiones_activa ON sesiones(fin) WHERE fin IS NULL;
