-- Tablas para el sistema PGCRP (Programa de Gestión de Cátedra Robinsoniana Productiva)

-- 1. Tabla de actividades PGCRP disponibles
CREATE TABLE IF NOT EXISTS "PGCRP" (
    id_pgcrp SERIAL PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL UNIQUE
);

-- 2. Tabla de asignaciones PGCRP por sección
CREATE TABLE IF NOT EXISTS grado_secciones_pgcrp (
    id_grado_secciones INTEGER NOT NULL,
    id_pgcrp INTEGER NOT NULL,
    id_periodo INTEGER NOT NULL,
    fecha_asignacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_grado_secciones, id_periodo),
    FOREIGN KEY (id_grado_secciones) REFERENCES grado_secciones(id_grado_secciones) ON DELETE CASCADE,
    FOREIGN KEY (id_pgcrp) REFERENCES "PGCRP"(id_pgcrp) ON DELETE CASCADE,
    FOREIGN KEY (id_periodo) REFERENCES periodos_escolares(id_periodo) ON DELETE CASCADE
);

-- 3. Tabla de asignaciones PGCRP por estudiante (individual)
CREATE TABLE IF NOT EXISTS estudiantes_pgcrp (
    id_estudiante INTEGER NOT NULL,
    id_pgcrp INTEGER NOT NULL,
    id_periodo INTEGER NOT NULL,
    id_grado_secciones INTEGER NOT NULL,
    observaciones TEXT,
    fecha_asignacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_estudiante, id_periodo),
    FOREIGN KEY (id_estudiante) REFERENCES estudiantes(id) ON DELETE CASCADE,
    FOREIGN KEY (id_pgcrp) REFERENCES "PGCRP"(id_pgcrp) ON DELETE CASCADE,
    FOREIGN KEY (id_periodo) REFERENCES periodos_escolares(id_periodo) ON DELETE CASCADE,
    FOREIGN KEY (id_grado_secciones) REFERENCES grado_secciones(id_grado_secciones) ON DELETE CASCADE
);

-- Insertar actividades PGCRP predefinidas
INSERT INTO "PGCRP" (nombre) VALUES 
    ('Robótica y Tecnología'),
    ('Teatro y Artes Escénicas'),
    ('Voleibol'),
    ('Fútbol'),
    ('Baloncesto'),
    ('Ajedrez'),
    ('Música y Canto'),
    ('Danza Tradicional'),
    ('Agricultura Urbana'),
    ('Artesanías y Manualidades')
ON CONFLICT (nombre) DO NOTHING;

-- Agregar columna id_grado_secciones a tabla existente si no existe
ALTER TABLE estudiantes_pgcrp 
ADD COLUMN IF NOT EXISTS id_grado_secciones INTEGER;

-- Agregar foreign key constraint si no existe
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'estudiantes_pgcrp_id_grado_secciones_fkey'
    ) THEN
        ALTER TABLE estudiantes_pgcrp 
        ADD CONSTRAINT estudiantes_pgcrp_id_grado_secciones_fkey 
        FOREIGN KEY (id_grado_secciones) REFERENCES grado_secciones(id_grado_secciones) ON DELETE CASCADE;
    END IF;
END $$;

-- Índices para mejorar rendimiento
CREATE INDEX IF NOT EXISTS idx_grado_secciones_pgcrp_periodo ON grado_secciones_pgcrp(id_periodo);
CREATE INDEX IF NOT EXISTS idx_estudiantes_pgcrp_periodo ON estudiantes_pgcrp(id_periodo);
CREATE INDEX IF NOT EXISTS idx_estudiantes_pgcrp_actividad ON estudiantes_pgcrp(id_pgcrp);
CREATE INDEX IF NOT EXISTS idx_estudiantes_pgcrp_seccion ON estudiantes_pgcrp(id_grado_secciones);
