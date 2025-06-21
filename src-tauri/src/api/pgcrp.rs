use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;
use crate::models::pgcrp::*;

// ===== GESTIÓN DE ACTIVIDADES PGCRP =====

#[tauri::command]
pub async fn obtener_actividades_pgcrp(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
) -> Result<Vec<Pgcrp>, String> {
    let db = db_pool.lock().await;
    
    let stmt = db.prepare(
        "SELECT id_pgcrp, nombre, descripcion, activo, fecha_creacion 
         FROM pgcrp 
         WHERE activo = true 
         ORDER BY nombre"
    ).await.map_err(|e| format!("Error preparando consulta: {}", e))?;
    
    let rows = db.query(&stmt, &[]).await.map_err(|e| format!("Error ejecutando consulta: {}", e))?;
    
    let actividades = rows.into_iter().map(|row| Pgcrp {
        id_pgcrp: row.get("id_pgcrp"),
        nombre: row.get("nombre"),
        descripcion: row.get("descripcion"),
        activo: row.get("activo"),
        fecha_creacion: row.get("fecha_creacion"),
    }).collect();
    
    Ok(actividades)
}

#[tauri::command]
pub async fn crear_actividad_pgcrp(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    input: PgcrpInput,
) -> Result<i32, String> {
    let db = db_pool.lock().await;
    
    let stmt = db.prepare(
        "INSERT INTO pgcrp (nombre, descripcion) VALUES ($1, $2) RETURNING id_pgcrp"
    ).await.map_err(|e| format!("Error preparando inserción: {}", e))?;
    
    let row = db.query_one(&stmt, &[&input.nombre, &input.descripcion])
        .await.map_err(|e| format!("Error creando actividad: {}", e))?;
    
    Ok(row.get("id_pgcrp"))
}

#[tauri::command]
pub async fn actualizar_actividad_pgcrp(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_pgcrp: i32,
    input: PgcrpInput,
) -> Result<(), String> {
    let db = db_pool.lock().await;
    
    let stmt = db.prepare(
        "UPDATE pgcrp SET nombre = $1, descripcion = $2 WHERE id_pgcrp = $3"
    ).await.map_err(|e| format!("Error preparando actualización: {}", e))?;
    
    let affected = db.execute(&stmt, &[&input.nombre, &input.descripcion, &id_pgcrp])
        .await.map_err(|e| format!("Error actualizando actividad: {}", e))?;
    
    if affected == 0 {
        return Err("Actividad no encontrada".to_string());
    }
    
    Ok(())
}

#[tauri::command]
pub async fn eliminar_actividad_pgcrp(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_pgcrp: i32,
) -> Result<(), String> {
    let db = db_pool.lock().await;
    
    // Verificar si la actividad está siendo utilizada
    let check_stmt = db.prepare(
        "SELECT COUNT(*) as count FROM (
            SELECT 1 FROM pgcrp_asignacion_seccion WHERE id_pgcrp = $1 AND activo = true
            UNION ALL
            SELECT 1 FROM pgcrp_asignacion_estudiante WHERE id_pgcrp = $1 AND activo = true
        ) as asignaciones"
    ).await.map_err(|e| format!("Error preparando verificación: {}", e))?;
    
    let count_row = db.query_one(&check_stmt, &[&id_pgcrp])
        .await.map_err(|e| format!("Error verificando uso: {}", e))?;
    
    let count: i64 = count_row.get("count");
    
    if count > 0 {
        return Err("No se puede eliminar la actividad porque está siendo utilizada por estudiantes o secciones".to_string());
    }
    
    // Desactivar en lugar de eliminar para mantener integridad referencial
    let stmt = db.prepare(
        "UPDATE pgcrp SET activo = false WHERE id_pgcrp = $1"
    ).await.map_err(|e| format!("Error preparando desactivación: {}", e))?;
    
    let affected = db.execute(&stmt, &[&id_pgcrp])
        .await.map_err(|e| format!("Error desactivando actividad: {}", e))?;
    
    if affected == 0 {
        return Err("Actividad no encontrada".to_string());
    }
    
    Ok(())
}

// ===== GESTIÓN DE ASIGNACIONES POR SECCIÓN =====

#[tauri::command]
pub async fn obtener_asignaciones_seccion(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_periodo: Option<i32>,
) -> Result<Vec<PgcrpAsignacionSeccion>, String> {
    let db = db_pool.lock().await;
    
    let query = if let Some(periodo) = id_periodo {
        "SELECT pas.id_asignacion_seccion, pas.id_grado_secciones, pas.id_pgcrp, pas.id_periodo, 
                pas.fecha_asignacion, pas.activo, p.nombre as nombre_pgcrp,
                g.nombre_grado as grado, s.nombre_seccion as seccion, m.nombre_modalidad as modalidad
         FROM pgcrp_asignacion_seccion pas
         INNER JOIN pgcrp p ON pas.id_pgcrp = p.id_pgcrp
         INNER JOIN grado_secciones gs ON pas.id_grado_secciones = gs.id_grado_secciones
         INNER JOIN grados g ON gs.id_grado = g.id_grado
         INNER JOIN secciones s ON gs.id_seccion = s.id_seccion
         INNER JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
         WHERE pas.activo = true AND pas.id_periodo = $1
         ORDER BY g.nombre_grado, s.nombre_seccion"
    } else {
        "SELECT pas.id_asignacion_seccion, pas.id_grado_secciones, pas.id_pgcrp, pas.id_periodo, 
                pas.fecha_asignacion, pas.activo, p.nombre as nombre_pgcrp,
                g.nombre_grado as grado, s.nombre_seccion as seccion, m.nombre_modalidad as modalidad
         FROM pgcrp_asignacion_seccion pas
         INNER JOIN pgcrp p ON pas.id_pgcrp = p.id_pgcrp
         INNER JOIN grado_secciones gs ON pas.id_grado_secciones = gs.id_grado_secciones
         INNER JOIN grados g ON gs.id_grado = g.id_grado
         INNER JOIN secciones s ON gs.id_seccion = s.id_seccion
         INNER JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
         WHERE pas.activo = true
         ORDER BY pas.id_periodo DESC, g.nombre_grado, s.nombre_seccion"
    };
    
    let stmt = db.prepare(query).await.map_err(|e| format!("Error preparando consulta: {}", e))?;
    
    let rows = if let Some(periodo) = id_periodo {
        db.query(&stmt, &[&periodo]).await.map_err(|e| format!("Error ejecutando consulta: {}", e))?
    } else {
        db.query(&stmt, &[]).await.map_err(|e| format!("Error ejecutando consulta: {}", e))?
    };
    
    let asignaciones = rows.into_iter().map(|row| PgcrpAsignacionSeccion {
        id_asignacion_seccion: row.get("id_asignacion_seccion"),
        id_grado_secciones: row.get("id_grado_secciones"),
        id_pgcrp: row.get("id_pgcrp"),
        id_periodo: row.get("id_periodo"),
        fecha_asignacion: row.get("fecha_asignacion"),
        activo: row.get("activo"),
        nombre_pgcrp: row.get("nombre_pgcrp"),
        grado: row.get("grado"),
        seccion: row.get("seccion"),
        modalidad: row.get("modalidad"),
    }).collect();
    
    Ok(asignaciones)
}

#[tauri::command]
pub async fn asignar_pgcrp_seccion(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    input: AsignacionSeccionInput,
) -> Result<i32, String> {
    let db = db_pool.lock().await;
    
    // Verificar si ya existe una asignación para esta sección en este período
    let check_stmt = db.prepare(
        "SELECT id_asignacion_seccion FROM pgcrp_asignacion_seccion 
         WHERE id_grado_secciones = $1 AND id_periodo = $2 AND activo = true"
    ).await.map_err(|e| format!("Error preparando verificación: {}", e))?;
    
    let existing = db.query_opt(&check_stmt, &[&input.id_grado_secciones, &input.id_periodo])
        .await.map_err(|e| format!("Error verificando asignación existente: {}", e))?;
    
    if existing.is_some() {
        // Actualizar la asignación existente
        let update_stmt = db.prepare(
            "UPDATE pgcrp_asignacion_seccion SET id_pgcrp = $1, fecha_asignacion = now() 
             WHERE id_grado_secciones = $2 AND id_periodo = $3 AND activo = true
             RETURNING id_asignacion_seccion"
        ).await.map_err(|e| format!("Error preparando actualización: {}", e))?;
        
        let row = db.query_one(&update_stmt, &[&input.id_pgcrp, &input.id_grado_secciones, &input.id_periodo])
            .await.map_err(|e| format!("Error actualizando asignación: {}", e))?;
        
        Ok(row.get("id_asignacion_seccion"))
    } else {
        // Crear nueva asignación
        let insert_stmt = db.prepare(
            "INSERT INTO pgcrp_asignacion_seccion (id_grado_secciones, id_pgcrp, id_periodo) 
             VALUES ($1, $2, $3) RETURNING id_asignacion_seccion"
        ).await.map_err(|e| format!("Error preparando inserción: {}", e))?;
        
        let row = db.query_one(&insert_stmt, &[&input.id_grado_secciones, &input.id_pgcrp, &input.id_periodo])
            .await.map_err(|e| format!("Error creando asignación: {}", e))?;
        
        Ok(row.get("id_asignacion_seccion"))
    }
}

#[tauri::command]
pub async fn eliminar_asignacion_seccion(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_asignacion_seccion: i32,
) -> Result<(), String> {
    let db = db_pool.lock().await;
    
    let stmt = db.prepare(
        "UPDATE pgcrp_asignacion_seccion SET activo = false WHERE id_asignacion_seccion = $1"
    ).await.map_err(|e| format!("Error preparando desactivación: {}", e))?;
    
    let affected = db.execute(&stmt, &[&id_asignacion_seccion])
        .await.map_err(|e| format!("Error desactivando asignación: {}", e))?;
    
    if affected == 0 {
        return Err("Asignación no encontrada".to_string());
    }
    
    Ok(())
}

// ===== GESTIÓN DE ASIGNACIONES POR ESTUDIANTE =====

#[tauri::command]
pub async fn obtener_asignaciones_estudiante(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_periodo: Option<i32>,
    id_grado_secciones: Option<i32>,
) -> Result<Vec<PgcrpAsignacionEstudiante>, String> {
    let db = db_pool.lock().await;
    
    let mut query = "SELECT pae.id_asignacion_estudiante, pae.id_estudiante, pae.id_pgcrp, pae.id_periodo, 
                            pae.fecha_asignacion, pae.activo, pae.observaciones, p.nombre as nombre_pgcrp,
                            e.nombres as nombres_estudiante, e.apellidos as apellidos_estudiante, e.cedula as cedula_estudiante
                     FROM pgcrp_asignacion_estudiante pae
                     INNER JOIN pgcrp p ON pae.id_pgcrp = p.id_pgcrp
                     INNER JOIN estudiantes e ON pae.id_estudiante = e.id
                     WHERE pae.activo = true".to_string();
    
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![];
    let mut param_count = 0;
    
    if let Some(periodo) = &id_periodo {
        param_count += 1;
        query.push_str(&format!(" AND pae.id_periodo = ${}", param_count));
        params.push(periodo);
    }
    
    if let Some(grado_sec) = &id_grado_secciones {
        param_count += 1;
        query.push_str(&format!(" AND e.id IN (
            SELECT hge.id_estudiante FROM historial_grado_estudiantes hge 
            WHERE hge.id_grado_secciones = ${} AND hge.es_actual = true AND hge.estado = 'activo'
        )", param_count));
        params.push(grado_sec);
    }
    
    query.push_str(" ORDER BY e.apellidos, e.nombres");
    
    let stmt = db.prepare(&query).await.map_err(|e| format!("Error preparando consulta: {}", e))?;
    
    let rows = db.query(&stmt, &params).await.map_err(|e| format!("Error ejecutando consulta: {}", e))?;
    
    let asignaciones = rows.into_iter().map(|row| PgcrpAsignacionEstudiante {
        id_asignacion_estudiante: row.get("id_asignacion_estudiante"),
        id_estudiante: row.get("id_estudiante"),
        id_pgcrp: row.get("id_pgcrp"),
        id_periodo: row.get("id_periodo"),
        fecha_asignacion: row.get("fecha_asignacion"),
        activo: row.get("activo"),
        observaciones: row.get("observaciones"),
        nombre_pgcrp: row.get("nombre_pgcrp"),
        nombres_estudiante: row.get("nombres_estudiante"),
        apellidos_estudiante: row.get("apellidos_estudiante"),
        cedula_estudiante: row.get("cedula_estudiante"),
    }).collect();
    
    Ok(asignaciones)
}

#[tauri::command]
pub async fn asignar_pgcrp_estudiante(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    input: AsignacionEstudianteInput,
) -> Result<i32, String> {
    let db = db_pool.lock().await;
    
    // Verificar si ya existe una asignación para este estudiante en este período
    let check_stmt = db.prepare(
        "SELECT id_asignacion_estudiante FROM pgcrp_asignacion_estudiante 
         WHERE id_estudiante = $1 AND id_periodo = $2 AND activo = true"
    ).await.map_err(|e| format!("Error preparando verificación: {}", e))?;
    
    let existing = db.query_opt(&check_stmt, &[&input.id_estudiante, &input.id_periodo])
        .await.map_err(|e| format!("Error verificando asignación existente: {}", e))?;
    
    if existing.is_some() {
        // Actualizar la asignación existente
        let update_stmt = db.prepare(
            "UPDATE pgcrp_asignacion_estudiante SET id_pgcrp = $1, observaciones = $2, fecha_asignacion = now() 
             WHERE id_estudiante = $3 AND id_periodo = $4 AND activo = true
             RETURNING id_asignacion_estudiante"
        ).await.map_err(|e| format!("Error preparando actualización: {}", e))?;
        
        let row = db.query_one(&update_stmt, &[&input.id_pgcrp, &input.observaciones, &input.id_estudiante, &input.id_periodo])
            .await.map_err(|e| format!("Error actualizando asignación: {}", e))?;
        
        Ok(row.get("id_asignacion_estudiante"))
    } else {
        // Crear nueva asignación
        let insert_stmt = db.prepare(
            "INSERT INTO pgcrp_asignacion_estudiante (id_estudiante, id_pgcrp, id_periodo, observaciones) 
             VALUES ($1, $2, $3, $4) RETURNING id_asignacion_estudiante"
        ).await.map_err(|e| format!("Error preparando inserción: {}", e))?;
        
        let row = db.query_one(&insert_stmt, &[&input.id_estudiante, &input.id_pgcrp, &input.id_periodo, &input.observaciones])
            .await.map_err(|e| format!("Error creando asignación: {}", e))?;
        
        Ok(row.get("id_asignacion_estudiante"))
    }
}

#[tauri::command]
pub async fn eliminar_asignacion_estudiante(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_asignacion_estudiante: i32,
) -> Result<(), String> {
    let db = db_pool.lock().await;
    
    let stmt = db.prepare(
        "UPDATE pgcrp_asignacion_estudiante SET activo = false WHERE id_asignacion_estudiante = $1"
    ).await.map_err(|e| format!("Error preparando desactivación: {}", e))?;
    
    let affected = db.execute(&stmt, &[&id_asignacion_estudiante])
        .await.map_err(|e| format!("Error desactivando asignación: {}", e))?;
    
    if affected == 0 {
        return Err("Asignación no encontrada".to_string());
    }
    
    Ok(())
}

// ===== CONSULTAS Y REPORTES =====

#[tauri::command]
pub async fn obtener_estudiantes_con_pgcrp(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_grado_secciones: i32,
    id_periodo: i32,
) -> Result<Vec<PgcrpEstudianteVista>, String> {
    let db = db_pool.lock().await;
    
    let stmt = db.prepare(
        "SELECT * FROM vista_pgcrp_estudiante 
         WHERE id_grado_secciones = $1 AND id_periodo = $2
         ORDER BY apellidos, nombres"
    ).await.map_err(|e| format!("Error preparando consulta: {}", e))?;
    
    let rows = db.query(&stmt, &[&id_grado_secciones, &id_periodo])
        .await.map_err(|e| format!("Error ejecutando consulta: {}", e))?;
    
    let estudiantes = rows.into_iter().map(|row| PgcrpEstudianteVista {
        id_estudiante: row.get("id_estudiante"),
        cedula: row.get("cedula"),
        nombres: row.get("nombres"),
        apellidos: row.get("apellidos"),
        id_grado_secciones: row.get("id_grado_secciones"),
        id_periodo: row.get("id_periodo"),
        id_pgcrp_asignado: row.get("id_pgcrp_asignado"),
        nombre_pgcrp_asignado: row.get("nombre_pgcrp_asignado"),
        tipo_asignacion: row.get("tipo_asignacion"),
        observaciones_individuales: row.get("observaciones_individuales"),
    }).collect();
    
    Ok(estudiantes)
}

#[tauri::command]
pub async fn obtener_estadisticas_pgcrp(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_periodo: i32,
) -> Result<PgcrpEstadisticas, String> {
    let db = db_pool.lock().await;
    
    // Estadísticas generales
    let stats_stmt = db.prepare(
        "SELECT 
            COUNT(*) as total_estudiantes,
            COUNT(CASE WHEN id_pgcrp_asignado IS NOT NULL THEN 1 END) as estudiantes_con_asignacion,
            COUNT(CASE WHEN id_pgcrp_asignado IS NULL THEN 1 END) as estudiantes_sin_asignacion,
            COUNT(CASE WHEN tipo_asignacion = 'seccion' THEN 1 END) as asignaciones_por_seccion,
            COUNT(CASE WHEN tipo_asignacion = 'individual' THEN 1 END) as asignaciones_individuales
         FROM vista_pgcrp_estudiante 
         WHERE id_periodo = $1"
    ).await.map_err(|e| format!("Error preparando consulta de estadísticas: {}", e))?;
    
    let stats_row = db.query_one(&stats_stmt, &[&id_periodo])
        .await.map_err(|e| format!("Error ejecutando consulta de estadísticas: {}", e))?;
    
    // Estadísticas por actividad
    let actividades_stmt = db.prepare(
        "SELECT 
            p.id_pgcrp,
            p.nombre as nombre_actividad,
            COUNT(*) as total_estudiantes,
            COUNT(CASE WHEN vpe.tipo_asignacion = 'seccion' THEN 1 END) as asignaciones_seccion,
            COUNT(CASE WHEN vpe.tipo_asignacion = 'individual' THEN 1 END) as asignaciones_individuales
         FROM pgcrp p
         INNER JOIN vista_pgcrp_estudiante vpe ON p.id_pgcrp = vpe.id_pgcrp_asignado
         WHERE vpe.id_periodo = $1
         GROUP BY p.id_pgcrp, p.nombre
         ORDER BY total_estudiantes DESC, p.nombre"
    ).await.map_err(|e| format!("Error preparando consulta de actividades: {}", e))?;
    
    let actividades_rows = db.query(&actividades_stmt, &[&id_periodo])
        .await.map_err(|e| format!("Error ejecutando consulta de actividades: {}", e))?;
    
    let actividades_utilizadas = actividades_rows.into_iter().map(|row| ActividadEstadistica {
        id_pgcrp: row.get("id_pgcrp"),
        nombre_actividad: row.get("nombre_actividad"),
        total_estudiantes: row.get("total_estudiantes"),
        asignaciones_seccion: row.get("asignaciones_seccion"),
        asignaciones_individuales: row.get("asignaciones_individuales"),
    }).collect();
    
    Ok(PgcrpEstadisticas {
        total_estudiantes: stats_row.get("total_estudiantes"),
        estudiantes_con_asignacion: stats_row.get("estudiantes_con_asignacion"),
        estudiantes_sin_asignacion: stats_row.get("estudiantes_sin_asignacion"),
        asignaciones_por_seccion: stats_row.get("asignaciones_por_seccion"),
        asignaciones_individuales: stats_row.get("asignaciones_individuales"),
        actividades_utilizadas,
    })
}

#[tauri::command]
pub async fn generar_reporte_pgcrp(
    db_pool: tauri::State<'_, Arc<Mutex<Client>>>,
    id_periodo: i32,
    id_grado: Option<i32>,
    id_modalidad: Option<i32>,
) -> Result<Vec<ReportePgcrpEstudiante>, String> {
    let db = db_pool.lock().await;
    
    let mut query = "SELECT 
                        vpe.cedula, vpe.nombres, vpe.apellidos,
                        g.nombre_grado as grado, s.nombre_seccion as seccion,
                        COALESCE(vpe.nombre_pgcrp_asignado, 'Sin asignar') as actividad_pgcrp,
                        vpe.tipo_asignacion, vpe.observaciones_individuales as observaciones
                     FROM vista_pgcrp_estudiante vpe
                     INNER JOIN grado_secciones gs ON vpe.id_grado_secciones = gs.id_grado_secciones
                     INNER JOIN grados g ON gs.id_grado = g.id_grado
                     INNER JOIN secciones s ON gs.id_seccion = s.id_seccion
                     WHERE vpe.id_periodo = $1".to_string();
    
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![&id_periodo];
    let mut param_count = 1;
    
    if let Some(grado) = &id_grado {
        param_count += 1;
        query.push_str(&format!(" AND gs.id_grado = ${}", param_count));
        params.push(grado);
    }
    
    if let Some(modalidad) = &id_modalidad {
        param_count += 1;
        query.push_str(&format!(" AND gs.id_modalidad = ${}", param_count));
        params.push(modalidad);
    }
    
    query.push_str(" ORDER BY g.nombre_grado, s.nombre_seccion, vpe.apellidos, vpe.nombres");
    
    let stmt = db.prepare(&query).await.map_err(|e| format!("Error preparando consulta de reporte: {}", e))?;
    
    let rows = db.query(&stmt, &params).await.map_err(|e| format!("Error ejecutando consulta de reporte: {}", e))?;
    
    let reporte = rows.into_iter().map(|row| ReportePgcrpEstudiante {
        cedula: row.get("cedula"),
        nombres: row.get("nombres"),
        apellidos: row.get("apellidos"),
        grado: row.get("grado"),
        seccion: row.get("seccion"),
        actividad_pgcrp: row.get("actividad_pgcrp"),
        tipo_asignacion: row.get("tipo_asignacion"),
        observaciones: row.get("observaciones"),
    }).collect();
    
    Ok(reporte)
} 