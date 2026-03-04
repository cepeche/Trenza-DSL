-- Schema para Mi Cronómetro PSP
-- Base de datos SQLite

-- Tabla de actividades/proyectos
CREATE TABLE IF NOT EXISTS actividades (
    id TEXT PRIMARY KEY,
    nombre TEXT NOT NULL,
    color TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    archived INTEGER DEFAULT 0
);

-- Tabla de tipos de tarea (pueden aplicarse a múltiples actividades)
CREATE TABLE IF NOT EXISTS tipos_tarea (
    id TEXT PRIMARY KEY,
    nombre TEXT NOT NULL,
    icono TEXT NOT NULL,
    actividades_permitidas TEXT NOT NULL, -- JSON array de IDs de actividades
    usos_7d INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL
);

-- Tabla de tareas expandidas (tipo_tarea × actividad)
-- Estas se generan automáticamente basándose en tipos_tarea.actividades_permitidas
CREATE TABLE IF NOT EXISTS tareas (
    id TEXT PRIMARY KEY,              -- Formato: "tipo_tarea_id_actividad_id"
    tipo_tarea_id TEXT NOT NULL,
    actividad_id TEXT NOT NULL,
    FOREIGN KEY (tipo_tarea_id) REFERENCES tipos_tarea(id) ON DELETE CASCADE,
    FOREIGN KEY (actividad_id) REFERENCES actividades(id) ON DELETE CASCADE,
    UNIQUE(tipo_tarea_id, actividad_id)
);

-- Tabla de sesiones de trabajo
CREATE TABLE IF NOT EXISTS sesiones (
    id TEXT PRIMARY KEY,
    tarea_id TEXT NOT NULL,
    inicio INTEGER NOT NULL,         -- timestamp Unix (segundos)
    fin INTEGER,                     -- NULL si sesión activa
    duracion INTEGER,                -- segundos (calculado al cerrar)
    notas TEXT,
    FOREIGN KEY (tarea_id) REFERENCES tareas(id) ON DELETE CASCADE
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_sesiones_inicio ON sesiones(inicio);
CREATE INDEX IF NOT EXISTS idx_sesiones_tarea ON sesiones(tarea_id);
CREATE INDEX IF NOT EXISTS idx_sesiones_activa ON sesiones(fin) WHERE fin IS NULL;
CREATE INDEX IF NOT EXISTS idx_tareas_actividad ON tareas(actividad_id);
CREATE INDEX IF NOT EXISTS idx_tareas_tipo ON tareas(tipo_tarea_id);

-- Datos iniciales: Actividades
INSERT INTO actividades (id, nombre, color, created_at) VALUES
    ('px', 'Proyecto X', '#667eea', strftime('%s', 'now')),
    ('py', 'Proyecto Y', '#f093fb', strftime('%s', 'now')),
    ('ad', 'Admin', '#fa709a', strftime('%s', 'now')),
    ('np', 'No productivo', '#4facfe', strftime('%s', 'now')),
    ('ps', 'Personal', '#43e97b', strftime('%s', 'now'));

-- Datos iniciales: Tipos de tarea
INSERT INTO tipos_tarea (id, nombre, icono, actividades_permitidas, usos_7d, created_at) VALUES
    ('codificar', 'Codificar', '💻', '["px","py","ps"]', 15, strftime('%s', 'now')),
    ('documentar', 'Documentar', '📝', '["px","py","ad"]', 8, strftime('%s', 'now')),
    ('bugfix', 'Bug fixes', '🐛', '["px","py"]', 6, strftime('%s', 'now')),
    ('reuniones', 'Reuniones', '👥', '["px","py","ad"]', 12, strftime('%s', 'now')),
    ('review', 'Code review', '👀', '["px","py"]', 4, strftime('%s', 'now')),
    ('testing', 'Testing', '🧪', '["px","py"]', 3, strftime('%s', 'now')),
    ('email', 'Email', '📧', '["ad"]', 12, strftime('%s', 'now')),
    ('planificacion', 'Planificación', '📅', '["ad","px","py"]', 4, strftime('%s', 'now')),
    ('gestion', 'Gestión', '📊', '["ad"]', 3, strftime('%s', 'now')),
    ('comida', 'Comida', '🍽️', '["np"]', 14, strftime('%s', 'now')),
    ('descanso', 'Descanso', '☕', '["np"]', 10, strftime('%s', 'now')),
    ('aseo', 'Aseo', '🚿', '["np"]', 7, strftime('%s', 'now')),
    ('desplazamiento', 'Desplazamiento', '🚗', '["np"]', 5, strftime('%s', 'now')),
    ('lectura', 'Lectura', '📚', '["ps"]', 4, strftime('%s', 'now')),
    ('ejercicio', 'Ejercicio', '🏃', '["ps"]', 3, strftime('%s', 'now')),
    ('hobby', 'Hobby', '🎨', '["ps"]', 2, strftime('%s', 'now'));

-- Generar tareas expandidas (tipo_tarea × actividad)
-- Codificar en Proyecto X, Proyecto Y, Personal
INSERT INTO tareas (id, tipo_tarea_id, actividad_id) VALUES
    ('codificar_px', 'codificar', 'px'),
    ('codificar_py', 'codificar', 'py'),
    ('codificar_ps', 'codificar', 'ps'),
    ('documentar_px', 'documentar', 'px'),
    ('documentar_py', 'documentar', 'py'),
    ('documentar_ad', 'documentar', 'ad'),
    ('bugfix_px', 'bugfix', 'px'),
    ('bugfix_py', 'bugfix', 'py'),
    ('reuniones_px', 'reuniones', 'px'),
    ('reuniones_py', 'reuniones', 'py'),
    ('reuniones_ad', 'reuniones', 'ad'),
    ('review_px', 'review', 'px'),
    ('review_py', 'review', 'py'),
    ('testing_px', 'testing', 'px'),
    ('testing_py', 'testing', 'py'),
    ('email_ad', 'email', 'ad'),
    ('planificacion_ad', 'planificacion', 'ad'),
    ('planificacion_px', 'planificacion', 'px'),
    ('planificacion_py', 'planificacion', 'py'),
    ('gestion_ad', 'gestion', 'ad'),
    ('comida_np', 'comida', 'np'),
    ('descanso_np', 'descanso', 'np'),
    ('aseo_np', 'aseo', 'np'),
    ('desplazamiento_np', 'desplazamiento', 'np'),
    ('lectura_ps', 'lectura', 'ps'),
    ('ejercicio_ps', 'ejercicio', 'ps'),
    ('hobby_ps', 'hobby', 'ps');
