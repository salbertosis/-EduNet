use crate::models::pgcrp_simple::{ActividadPgcrp, AsignacionPgcrpCompleta, AsignacionPgcrpInput};
use crate::AppState;
use tauri::State;

// Obtener todas las actividades PGCRP disponibles
#[tauri::command(rename_all = "snake_case")]
pub async fn obtener_actividades_pgcrp_simple(
    state: State<'_, AppState>,
) -> Result<Vec<ActividadPgcrp>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT id_pgcrp, nombre FROM \"PGCRP\" ORDER BY nombre",
        &[]
    ).await.map_err(|e| e.to_string())?;

    let actividades = rows
        .iter()
        .map(|row| ActividadPgcrp {
            id_pgcrp: row.get("id_pgcrp"),
            nombre: row.get("nombre"),
        })
        .collect();

    Ok(actividades)
}

// Asignar actividad PGCRP a una sección (similar a asignar_docente_guia)
#[tauri::command(rename_all = "snake_case")]
pub async fn asignar_pgcrp_seccion_simple(
    input: AsignacionPgcrpInput,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut client = state.db.lock().await;
    
    // Iniciar transacción
    let transaction = client.transaction().await
        .map_err(|e| format!("Error al iniciar transacción: {}", e))?;
    
    // 1. Primero eliminar asignación existente si existe
    transaction.execute(
        "DELETE FROM grado_secciones_pgcrp 
         WHERE id_grado_secciones = $1 AND id_periodo = $2",
        &[&input.id_grado_secciones, &input.id_periodo]
    ).await.map_err(|e| format!("Error al eliminar asignación previa: {}", e))?;
    
    // 2. Insertar nueva asignación
    transaction.execute(
        "INSERT INTO grado_secciones_pgcrp (id_grado_secciones, id_pgcrp, id_periodo, fecha_asignacion)
         VALUES ($1, $2, $3, CURRENT_TIMESTAMP)",
        &[&input.id_grado_secciones, &input.id_pgcrp, &input.id_periodo]
    ).await.map_err(|e| format!("Error al asignar PGCRP por sección: {}", e))?;
    
    // 3. Insertar todos los estudiantes de la sección específica en estudiantes_pgcrp
    transaction.execute(
        "INSERT INTO estudiantes_pgcrp (id_estudiante, id_pgcrp, id_periodo, id_grado_secciones, tipo_asignacion, observaciones, fecha_asignacion)
         SELECT hge.id_estudiante, $2, $3, $1, 'seccion', $4, CURRENT_TIMESTAMP
         FROM historial_grado_estudiantes hge 
         WHERE hge.id_grado_secciones = $1 
           AND hge.id_periodo = $3
           AND hge.es_actual = true
           AND NOT EXISTS (
               SELECT 1 FROM estudiantes_pgcrp ep 
               WHERE ep.id_estudiante = hge.id_estudiante 
                 AND ep.id_periodo = $3
           )",
        &[&input.id_grado_secciones, &input.id_pgcrp, &input.id_periodo, &input.observaciones]
    ).await.map_err(|e| format!("Error al asignar estudiantes: {}", e))?;
    
    // Confirmar transacción
    transaction.commit().await
        .map_err(|e| format!("Error al confirmar transacción: {}", e))?;

    Ok(())
}

// Obtener la asignación PGCRP actual de una sección para un período
#[tauri::command(rename_all = "snake_case")]
pub async fn obtener_pgcrp_seccion(
    id_grado_secciones: i32,
    id_periodo: i32,
    state: State<'_, AppState>,
) -> Result<Option<AsignacionPgcrpCompleta>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT gsp.id_grado_secciones, gsp.id_pgcrp, gsp.id_periodo,
                p.nombre as nombre_actividad, 
                gsp.fecha_asignacion::text as fecha_asignacion
         FROM grado_secciones_pgcrp gsp
         JOIN \"PGCRP\" p ON gsp.id_pgcrp = p.id_pgcrp
         WHERE gsp.id_grado_secciones = $1 AND gsp.id_periodo = $2",
        &[&id_grado_secciones, &id_periodo]
    ).await.map_err(|e| e.to_string())?;

    if rows.is_empty() {
        return Ok(None);
    }

    let row = &rows[0];
    Ok(Some(AsignacionPgcrpCompleta {
        id_grado_secciones: row.get("id_grado_secciones"),
        id_pgcrp: row.get("id_pgcrp"),
        id_periodo: row.get("id_periodo"),
        nombre_actividad: row.get("nombre_actividad"),
        fecha_asignacion: row.get("fecha_asignacion"),
    }))
}

// Eliminar asignación PGCRP de una sección
#[tauri::command(rename_all = "snake_case")]
pub async fn eliminar_pgcrp_seccion(
    id_grado_secciones: i32,
    id_periodo: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut client = state.db.lock().await;
    
    // Iniciar transacción
    let transaction = client.transaction().await
        .map_err(|e| format!("Error al iniciar transacción: {}", e))?;
    
    // 1. Eliminar estudiantes de la sección que NO tienen asignación individual
    transaction.execute(
        "DELETE FROM estudiantes_pgcrp ep
         WHERE ep.id_periodo = $2
           AND ep.id_estudiante IN (
               SELECT hge.id_estudiante 
               FROM historial_grado_estudiantes hge 
               WHERE hge.id_grado_secciones = $1 
                 AND hge.id_periodo = $2
                 AND hge.es_actual = true
           )
           -- Solo eliminar si no tienen asignación individual previa
           AND NOT EXISTS (
               SELECT 1 FROM estudiantes_pgcrp ep2 
               WHERE ep2.id_estudiante = ep.id_estudiante 
                 AND ep2.id_periodo = ep.id_periodo
                 AND ep2.observaciones IS NOT NULL 
                 AND ep2.observaciones != ''
           )",
        &[&id_grado_secciones, &id_periodo]
    ).await.map_err(|e| format!("Error al eliminar estudiantes PGCRP: {}", e))?;
    
    // 2. Eliminar la asignación por sección
    transaction.execute(
        "DELETE FROM grado_secciones_pgcrp 
         WHERE id_grado_secciones = $1 AND id_periodo = $2",
        &[&id_grado_secciones, &id_periodo]
    ).await.map_err(|e| format!("Error al eliminar PGCRP de sección: {}", e))?;
    
    // Confirmar transacción
    transaction.commit().await
        .map_err(|e| format!("Error al confirmar transacción: {}", e))?;

    Ok(())
}

// Consulta súper simple para cualquier reporte
#[tauri::command(rename_all = "snake_case")]
pub async fn obtener_estudiantes_pgcrp_periodo(
    id_periodo: i32,
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    let db = state.db.lock().await;
    
    // Súper simple para cualquier reporte
    let rows = db.query(
        "SELECT e.nombres, e.apellidos, p.nombre as pgcrp_asignado
         FROM estudiantes e
         JOIN estudiantes_pgcrp ep ON ep.id_estudiante = e.id
         JOIN \"PGCRP\" p ON p.id_pgcrp = ep.id_pgcrp
         WHERE ep.id_periodo = $1
         ORDER BY e.apellidos, e.nombres",
        &[&id_periodo]
    ).await.map_err(|e| e.to_string())?;

    let estudiantes: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "nombres": row.get::<_, String>("nombres"),
                "apellidos": row.get::<_, String>("apellidos"),
                "pgcrp_asignado": row.get::<_, String>("pgcrp_asignado")
            })
        })
        .collect();

    Ok(estudiantes)
} 