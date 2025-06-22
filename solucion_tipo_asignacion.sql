-- Solución: Agregar campo tipo_asignacion para distinguir asignaciones por sección vs individuales

-- Paso 1: Agregar columna tipo_asignacion
ALTER TABLE estudiantes_pgcrp 
ADD COLUMN IF NOT EXISTS tipo_asignacion VARCHAR(20) DEFAULT 'seccion';

-- Crear tipo ENUM para mejor control
CREATE TYPE tipo_asignacion_pgcrp AS ENUM ('seccion', 'individual');

-- Cambiar la columna para usar el ENUM (opcional, más estricto)
-- ALTER TABLE estudiantes_pgcrp 
-- ALTER COLUMN tipo_asignacion TYPE tipo_asignacion_pgcrp USING tipo_asignacion::tipo_asignacion_pgcrp;

-- Paso 2: Marcar como 'individual' los registros que tienen observaciones específicas
-- (asumiendo que las asignaciones individuales tienen observaciones personalizadas)
UPDATE estudiantes_pgcrp 
SET tipo_asignacion = 'individual'
WHERE observaciones IS NOT NULL 
    AND observaciones != '' 
    AND observaciones != 'Asignado por sección';

-- Paso 3: Verificar el resultado
SELECT 
    tipo_asignacion,
    COUNT(*) as cantidad,
    COUNT(CASE WHEN observaciones IS NOT NULL AND observaciones != '' THEN 1 END) as con_observaciones
FROM estudiantes_pgcrp 
GROUP BY tipo_asignacion;

-- Paso 4: Ver ejemplos de cada tipo
SELECT 
    e.nombres,
    e.apellidos,
    ep.tipo_asignacion,
    ep.observaciones,
    p.nombre as actividad_pgcrp
FROM estudiantes_pgcrp ep
JOIN estudiantes e ON e.id = ep.id_estudiante
JOIN "PGCRP" p ON p.id_pgcrp = ep.id_pgcrp
ORDER BY ep.tipo_asignacion, e.apellidos
LIMIT 10; 