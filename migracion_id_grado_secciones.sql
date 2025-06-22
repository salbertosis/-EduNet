-- Script para migrar id_grado_secciones en registros existentes de estudiantes_pgcrp
-- Este script actualiza los registros que tienen id_grado_secciones NULL o 0

-- Paso 1: Verificar cuántos registros necesitan migración
SELECT 
    COUNT(*) as total_registros,
    COUNT(CASE WHEN id_grado_secciones IS NULL OR id_grado_secciones = 0 THEN 1 END) as sin_grado_seccion,
    COUNT(CASE WHEN id_grado_secciones IS NOT NULL AND id_grado_secciones > 0 THEN 1 END) as con_grado_seccion
FROM estudiantes_pgcrp;

-- Paso 2: Ver registros problemáticos antes de la migración
SELECT 
    ep.id_estudiante,
    ep.id_periodo,
    ep.id_grado_secciones,
    e.nombres,
    e.apellidos,
    hge.id_grado_secciones as grado_seccion_historial
FROM estudiantes_pgcrp ep
JOIN estudiantes e ON e.id = ep.id_estudiante
LEFT JOIN historial_grado_estudiantes hge ON hge.id_estudiante = ep.id_estudiante 
    AND hge.id_periodo = ep.id_periodo 
    AND hge.es_actual = true
WHERE ep.id_grado_secciones IS NULL OR ep.id_grado_secciones = 0
ORDER BY ep.id_estudiante, ep.id_periodo;

-- Paso 3: Actualizar registros usando el historial de grado-secciones
UPDATE estudiantes_pgcrp 
SET id_grado_secciones = hge.id_grado_secciones
FROM historial_grado_estudiantes hge
WHERE estudiantes_pgcrp.id_estudiante = hge.id_estudiante
    AND estudiantes_pgcrp.id_periodo = hge.id_periodo
    AND hge.es_actual = true
    AND (estudiantes_pgcrp.id_grado_secciones IS NULL OR estudiantes_pgcrp.id_grado_secciones = 0);

-- Paso 4: Verificar si hay registros que no se pudieron migrar (sin historial)
SELECT 
    ep.id_estudiante,
    ep.id_periodo,
    ep.id_grado_secciones,
    e.nombres,
    e.apellidos,
    'Sin historial de grado-sección para este período' as problema
FROM estudiantes_pgcrp ep
JOIN estudiantes e ON e.id = ep.id_estudiante
LEFT JOIN historial_grado_estudiantes hge ON hge.id_estudiante = ep.id_estudiante 
    AND hge.id_periodo = ep.id_periodo 
    AND hge.es_actual = true
WHERE (ep.id_grado_secciones IS NULL OR ep.id_grado_secciones = 0)
    AND hge.id_grado_secciones IS NULL
ORDER BY ep.id_estudiante, ep.id_periodo;

-- Paso 5: Para registros sin historial, usar la sección actual del estudiante
UPDATE estudiantes_pgcrp 
SET id_grado_secciones = e.id_grado_secciones
FROM estudiantes e
WHERE estudiantes_pgcrp.id_estudiante = e.id
    AND (estudiantes_pgcrp.id_grado_secciones IS NULL OR estudiantes_pgcrp.id_grado_secciones = 0)
    AND e.id_grado_secciones IS NOT NULL;

-- Paso 6: Verificación final - mostrar estadísticas después de la migración
SELECT 
    COUNT(*) as total_registros,
    COUNT(CASE WHEN id_grado_secciones IS NULL OR id_grado_secciones = 0 THEN 1 END) as aun_sin_grado_seccion,
    COUNT(CASE WHEN id_grado_secciones IS NOT NULL AND id_grado_secciones > 0 THEN 1 END) as migrados_exitosamente
FROM estudiantes_pgcrp;

-- Paso 7: Mostrar registros que aún no tienen grado_secciones (si los hay)
SELECT 
    ep.id_estudiante,
    ep.id_periodo,
    ep.id_grado_secciones,
    e.nombres,
    e.apellidos,
    'Requiere intervención manual' as estado
FROM estudiantes_pgcrp ep
JOIN estudiantes e ON e.id = ep.id_estudiante
WHERE ep.id_grado_secciones IS NULL OR ep.id_grado_secciones = 0
ORDER BY ep.id_estudiante, ep.id_periodo;

-- Comentarios para ejecutar paso a paso:
-- 1. Ejecuta el Paso 1 para ver cuántos registros necesitan migración
-- 2. Ejecuta el Paso 2 para ver los registros problemáticos
-- 3. Ejecuta el Paso 3 para hacer la migración principal
-- 4. Ejecuta el Paso 4 para ver si hay registros sin historial
-- 5. Ejecuta el Paso 5 para migrar registros usando sección actual
-- 6. Ejecuta el Paso 6 para verificar el resultado
-- 7. Ejecuta el Paso 7 para ver registros que aún necesitan atención manual 