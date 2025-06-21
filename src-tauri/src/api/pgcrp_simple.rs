use crate::models::pgcrp_simple::{ActividadPgcrp, AsignacionPgcrpSeccion, AsignacionPgcrpCompleta, AsignacionPgcrpInput};
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
    let db = state.db.lock().await;
    
    // Insertar o actualizar la asignación
    db.execute(
        "INSERT INTO grado_secciones_pgcrp (id_grado_secciones, id_pgcrp, id_periodo)
         VALUES ($1, $2, $3)
         ON CONFLICT (id_grado_secciones, id_pgcrp, id_periodo) 
         DO UPDATE SET fecha_asignacion = CURRENT_TIMESTAMP",
        &[&input.id_grado_secciones, &input.id_pgcrp, &input.id_periodo]
    ).await.map_err(|e| e.to_string())?;

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
    let db = state.db.lock().await;
    db.execute(
        "DELETE FROM grado_secciones_pgcrp 
         WHERE id_grado_secciones = $1 AND id_periodo = $2",
        &[&id_grado_secciones, &id_periodo]
    ).await.map_err(|e| e.to_string())?;

    Ok(())
} 