use crate::models::estudiante_pgcrp::{AsignacionEstudiantePgcrp, EstudiantePgcrpDetalle, ActividadPgcrp};
use crate::AppState;
use tauri::{command, State};

#[command]
pub async fn obtener_estudiantes_seccion_pgcrp(
    id_grado_secciones: i32,
    id_periodo: i32,
    app_state: State<'_, AppState>,
) -> Result<Vec<EstudiantePgcrpDetalle>, String> {
    let client = app_state.db.lock().await;

    let query = r#"
        SELECT 
            e.id as id_estudiante,
            e.cedula,
            e.nombres,
            e.apellidos,
            g.nombre_grado,
            s.nombre_seccion,
            m.nombre_modalidad,
            -- PGCRP individual del estudiante
            ep.id_pgcrp as id_extra_catedra,
            pgcrp_individual.nombre as actividad_pgcrp,
            ep.tipo_asignacion,
            ep.observaciones,
            ep.fecha_asignacion,
            -- PGCRP por sección
            pgcrp_seccion.nombre as actividad_seccion
        FROM estudiantes e
        JOIN historial_grado_estudiantes hge ON hge.id_estudiante = e.id 
            AND hge.id_periodo = $2 
            AND hge.es_actual = true
        JOIN grado_secciones gs ON gs.id_grado_secciones = hge.id_grado_secciones
        JOIN grados g ON g.id_grado = gs.id_grado
        JOIN secciones s ON s.id_seccion = gs.id_seccion
        JOIN modalidades m ON m.id_modalidad = gs.id_modalidad
        -- LEFT JOIN para PGCRP individual
        LEFT JOIN estudiantes_pgcrp ep ON ep.id_estudiante = e.id 
            AND ep.id_periodo = $2
        LEFT JOIN "PGCRP" pgcrp_individual ON pgcrp_individual.id_pgcrp = ep.id_pgcrp
        -- LEFT JOIN para PGCRP por sección
        LEFT JOIN grado_secciones_pgcrp gsp ON gsp.id_grado_secciones = hge.id_grado_secciones 
            AND gsp.id_periodo = $2
        LEFT JOIN "PGCRP" pgcrp_seccion ON pgcrp_seccion.id_pgcrp = gsp.id_pgcrp
        WHERE hge.id_grado_secciones = $1 
            AND hge.id_periodo = $2 
            AND hge.es_actual = true
        ORDER BY e.apellidos, e.nombres
    "#;

    println!("[DEBUG] Ejecutando consulta estudiantes PGCRP con parámetros: id_grado_secciones={}, id_periodo={}", id_grado_secciones, id_periodo);
    
    let rows = client
        .query(query, &[&id_grado_secciones, &id_periodo])
        .await
        .map_err(|e| format!("Error al obtener estudiantes: {}", e))?;
    
    println!("[DEBUG] Consulta ejecutada, {} filas encontradas", rows.len());

    if rows.is_empty() {
        // Verificar si existen estudiantes en esa sección
        let verificar_estudiantes = client
            .query("SELECT COUNT(*) as count FROM estudiantes WHERE id_grado_secciones = $1", &[&id_grado_secciones])
            .await
            .map_err(|e| format!("Error al verificar estudiantes: {}", e))?;
        
        if let Some(row) = verificar_estudiantes.first() {
            let count: i64 = row.get("count");
            println!("[DEBUG] Estudiantes en la sección {}: {}", id_grado_secciones, count);
        }
    }

    let estudiantes: Vec<EstudiantePgcrpDetalle> = rows
        .iter()
        .map(|row| {
            let tipo_asignacion: Option<String> = row.get("tipo_asignacion");
            println!("[DEBUG] Estudiante: {} - Tipo asignación: {:?}", 
                row.get::<_, String>("nombres"), 
                tipo_asignacion
            );
            
            EstudiantePgcrpDetalle {
                id_estudiante: row.get("id_estudiante"),
                cedula: row.get("cedula"),
                nombres: row.get("nombres"),
                apellidos: row.get("apellidos"),
                nombre_grado: row.get("nombre_grado"),
                nombre_seccion: row.get("nombre_seccion"),
                nombre_modalidad: row.get("nombre_modalidad"),
                id_extra_catedra: row.get::<_, Option<i32>>("id_extra_catedra"),
                actividad_pgcrp: row.get("actividad_pgcrp"),
                tipo_asignacion,
                actividad_seccion: row.get("actividad_seccion"),
                observaciones: row.get("observaciones"),
                fecha_asignacion: row.get::<_, Option<chrono::NaiveDateTime>>("fecha_asignacion")
                    .map(|dt| dt.to_string()),
            }
        })
        .collect();

    println!("[DEBUG] Retornando {} estudiantes con tipos de asignación", estudiantes.len());
    Ok(estudiantes)
}

#[command]
pub async fn obtener_actividades_pgcrp_estudiante(
    app_state: State<'_, AppState>,
) -> Result<Vec<ActividadPgcrp>, String> {
    let client = app_state.db.lock().await;

    let query = "SELECT id_pgcrp as id_extra_catedra, nombre FROM \"PGCRP\" ORDER BY nombre";

    let rows = client
        .query(query, &[])
        .await
        .map_err(|e| format!("Error al obtener actividades: {}", e))?;

    let actividades: Vec<ActividadPgcrp> = rows
        .iter()
        .map(|row| ActividadPgcrp {
            id_extra_catedra: row.get("id_extra_catedra"),
            nombre: row.get("nombre"),
        })
        .collect();

    Ok(actividades)
}

#[command]
pub async fn asignar_pgcrp_estudiante_individual(
    asignacion: AsignacionEstudiantePgcrp,
    app_state: State<'_, AppState>,
) -> Result<String, String> {
    let client = app_state.db.lock().await;

    // Verificar si ya existe una asignación para este estudiante en este período
    let existe_query = r#"
        SELECT COUNT(*) as count 
        FROM estudiantes_pgcrp 
        WHERE id_estudiante = $1 AND id_periodo = $2
    "#;

    let existe_row = client
        .query_one(existe_query, &[&asignacion.id_estudiante, &asignacion.id_periodo])
        .await
        .map_err(|e| format!("Error al verificar asignación existente: {}", e))?;

    let count: i64 = existe_row.get("count");

    // Si id_extra_catedra es null, significa que queremos quitar la asignación individual
    // y hacer que use la asignación por sección
    if asignacion.id_extra_catedra.is_none() {
        // Obtener la actividad PGCRP asignada a la sección
        let seccion_pgcrp_query = r#"
            SELECT gsp.id_pgcrp
            FROM grado_secciones_pgcrp gsp
            WHERE gsp.id_grado_secciones = $1 AND gsp.id_periodo = $2
        "#;

        let seccion_pgcrp_result = client
            .query_opt(seccion_pgcrp_query, &[&asignacion.id_grado_secciones, &asignacion.id_periodo])
            .await
            .map_err(|e| format!("Error al obtener PGCRP de sección: {}", e))?;

        let id_pgcrp_seccion: Option<i32> = if let Some(row) = seccion_pgcrp_result {
            Some(row.get("id_pgcrp"))
        } else {
            None
        };

        if count > 0 {
            // Actualizar asignación existente para usar sección
            let update_query = r#"
                UPDATE estudiantes_pgcrp 
                SET id_pgcrp = $1,
                    tipo_asignacion = 'seccion',
                    observaciones = $2,
                    fecha_asignacion = CURRENT_TIMESTAMP
                WHERE id_estudiante = $3 AND id_periodo = $4
            "#;

            client
                .execute(
                    update_query,
                    &[
                        &id_pgcrp_seccion,
                        &asignacion.observaciones,
                        &asignacion.id_estudiante,
                        &asignacion.id_periodo,
                    ],
                )
                .await
                .map_err(|e| format!("Error al actualizar asignación: {}", e))?;

            Ok("Asignación PGCRP cambiada a sección exitosamente".to_string())
        } else {
            // Crear nueva asignación de tipo sección
            let insert_query = r#"
                INSERT INTO estudiantes_pgcrp 
                (id_estudiante, id_pgcrp, id_periodo, id_grado_secciones, tipo_asignacion, observaciones, fecha_asignacion)
                VALUES ($1, $2, $3, $4, 'seccion', $5, CURRENT_TIMESTAMP)
            "#;

            client
                .execute(
                    insert_query,
                    &[
                        &asignacion.id_estudiante,
                        &id_pgcrp_seccion,
                        &asignacion.id_periodo,
                        &asignacion.id_grado_secciones,
                        &asignacion.observaciones,
                    ],
                )
                .await
                .map_err(|e| format!("Error al crear asignación: {}", e))?;

            Ok("Asignación PGCRP de sección creada exitosamente".to_string())
        }
    } else {
        // Caso normal: asignar actividad individual específica
        if count > 0 {
            // Actualizar asignación existente
            let update_query = r#"
                UPDATE estudiantes_pgcrp 
                SET id_pgcrp = $1, 
                    id_grado_secciones = $2,
                    tipo_asignacion = 'individual',
                    observaciones = $3,
                    fecha_asignacion = CURRENT_TIMESTAMP
                WHERE id_estudiante = $4 AND id_periodo = $5
            "#;

            client
                .execute(
                    update_query,
                    &[
                        &asignacion.id_extra_catedra,
                        &asignacion.id_grado_secciones,
                        &asignacion.observaciones,
                        &asignacion.id_estudiante,
                        &asignacion.id_periodo,
                    ],
                )
                .await
                .map_err(|e| format!("Error al actualizar asignación: {}", e))?;

            Ok("Asignación PGCRP actualizada exitosamente".to_string())
        } else {
            // Crear nueva asignación
            let insert_query = r#"
                INSERT INTO estudiantes_pgcrp 
                (id_estudiante, id_pgcrp, id_periodo, id_grado_secciones, tipo_asignacion, observaciones, fecha_asignacion)
                VALUES ($1, $2, $3, $4, 'individual', $5, CURRENT_TIMESTAMP)
            "#;

            client
                .execute(
                    insert_query,
                    &[
                        &asignacion.id_estudiante,
                        &asignacion.id_extra_catedra,
                        &asignacion.id_periodo,
                        &asignacion.id_grado_secciones,
                        &asignacion.observaciones,
                    ],
                )
                .await
                .map_err(|e| format!("Error al crear asignación: {}", e))?;

            Ok("Asignación PGCRP creada exitosamente".to_string())
        }
    }
}

#[command]
pub async fn eliminar_pgcrp_estudiante_individual(
    id_estudiante: i32,
    id_periodo: i32,
    app_state: State<'_, AppState>,
) -> Result<String, String> {
    let client = app_state.db.lock().await;

    let query = r#"
        DELETE FROM estudiantes_pgcrp 
        WHERE id_estudiante = $1 AND id_periodo = $2
    "#;

    let rows_affected = client
        .execute(query, &[&id_estudiante, &id_periodo])
        .await
        .map_err(|e| format!("Error al eliminar asignación: {}", e))?;

    if rows_affected > 0 {
        Ok("Asignación PGCRP eliminada exitosamente".to_string())
    } else {
        Ok("No se encontró asignación para eliminar".to_string())
    }
}

#[command]
pub async fn diagnosticar_pgcrp_estudiantes(
    id_grado_secciones: i32,
    _id_periodo: i32,
    app_state: State<'_, AppState>,
) -> Result<String, String> {
    let client = app_state.db.lock().await;
    
    let mut diagnostico = String::new();
    
    // 1. Verificar estudiantes en la sección
    let estudiantes_query = "SELECT COUNT(*) as count FROM estudiantes WHERE id_grado_secciones = $1";
    let estudiantes_result = client.query_one(estudiantes_query, &[&id_grado_secciones]).await
        .map_err(|e| format!("Error al contar estudiantes: {}", e))?;
    let count_estudiantes: i64 = estudiantes_result.get("count");
    diagnostico.push_str(&format!("Estudiantes en sección {}: {}\n", id_grado_secciones, count_estudiantes));
    
    // 2. Verificar si existe la tabla extra_catedra
    let tabla_extra_catedra = client.query("SELECT COUNT(*) as count FROM extra_catedra", &[]).await;
    match tabla_extra_catedra {
        Ok(rows) => {
            if let Some(row) = rows.first() {
                let count: i64 = row.get("count");
                diagnostico.push_str(&format!("Actividades PGCRP disponibles: {}\n", count));
            }
        }
        Err(e) => {
            diagnostico.push_str(&format!("Error accediendo tabla extra_catedra: {}\n", e));
        }
    }
    
    // 3. Verificar tabla estudiantes_extra_catedra
    let tabla_estudiantes_extra = client.query("SELECT COUNT(*) as count FROM estudiantes_extra_catedra", &[]).await;
    match tabla_estudiantes_extra {
        Ok(rows) => {
            if let Some(row) = rows.first() {
                let count: i64 = row.get("count");
                diagnostico.push_str(&format!("Asignaciones individuales existentes: {}\n", count));
            }
        }
        Err(e) => {
            diagnostico.push_str(&format!("Error accediendo tabla estudiantes_extra_catedra: {}\n", e));
        }
    }
    
    // 4. Verificar tabla grado_secciones_pgcrp
    let tabla_grado_pgcrp = client.query("SELECT COUNT(*) as count FROM grado_secciones_pgcrp", &[]).await;
    match tabla_grado_pgcrp {
        Ok(rows) => {
            if let Some(row) = rows.first() {
                let count: i64 = row.get("count");
                diagnostico.push_str(&format!("Asignaciones por sección existentes: {}\n", count));
            }
        }
        Err(e) => {
            diagnostico.push_str(&format!("Error accediendo tabla grado_secciones_pgcrp: {}\n", e));
        }
    }
    
    // 5. Verificar estructura de grado_secciones
    let grado_seccion_query = "SELECT * FROM grado_secciones WHERE id_grado_secciones = $1";
    let grado_seccion_result = client.query(grado_seccion_query, &[&id_grado_secciones]).await;
    match grado_seccion_result {
        Ok(rows) => {
            if let Some(row) = rows.first() {
                let id_grado: i32 = row.get("id_grado");
                let id_seccion: i32 = row.get("id_seccion");
                let id_modalidad: i32 = row.get("id_modalidad");
                diagnostico.push_str(&format!("Grado-Sección {}: grado={}, sección={}, modalidad={}\n", 
                    id_grado_secciones, id_grado, id_seccion, id_modalidad));
            }
        }
        Err(e) => {
            diagnostico.push_str(&format!("Error obteniendo info grado_secciones: {}\n", e));
        }
    }
    
    Ok(diagnostico)
} 