use tauri::State;
use crate::AppState;
use crate::models::catalogo::{PeriodoEscolar, Grado, Modalidad, Asignatura};

#[tauri::command]
pub async fn listar_periodos_escolares(state: State<'_, AppState>) -> Result<Vec<PeriodoEscolar>, String> {
    let db = state.db.lock().await;
    let query = "SELECT id_periodo, periodo_escolar, activo FROM periodos_escolares ORDER BY id_periodo ASC";
    let rows = db.query(query, &[]).await.map_err(|e| e.to_string())?;

    let periodos = rows.iter().map(|row| PeriodoEscolar {
        id_periodo: row.get("id_periodo"),
        periodo_escolar: row.get("periodo_escolar"),
        activo: row.get("activo"),
    }).collect();

    Ok(periodos)
}

#[tauri::command]
pub async fn listar_grados(state: State<'_, AppState>) -> Result<Vec<Grado>, String> {
    let db = state.db.lock().await;
    let query = "SELECT id_grado, nombre_grado FROM grados ORDER BY id_grado ASC";
    let rows = db.query(query, &[]).await.map_err(|e| e.to_string())?;
    let grados = rows.iter().map(|row| Grado {
        id_grado: row.get("id_grado"),
        nombre_grado: row.get("nombre_grado"),
    }).collect();
    Ok(grados)
}

#[tauri::command]
pub async fn listar_modalidades(state: State<'_, AppState>) -> Result<Vec<Modalidad>, String> {
    let db = state.db.lock().await;
    let query = "SELECT id_modalidad, nombre_modalidad FROM modalidades ORDER BY id_modalidad ASC";
    let rows = db.query(query, &[]).await.map_err(|e| e.to_string())?;
    let modalidades = rows.iter().map(|row| Modalidad {
        id_modalidad: row.get("id_modalidad"),
        nombre_modalidad: row.get("nombre_modalidad"),
    }).collect();
    Ok(modalidades)
}

#[tauri::command]
pub async fn obtener_asignaturas_por_grado_modalidad(
    id_grado: i32,
    id_modalidad: i32,
    state: State<'_, AppState>,
) -> Result<Vec<Asignatura>, String> {
    let db = state.db.lock().await;
    let query = "
        SELECT 
            a.id_asignatura, 
            a.nombre AS nombre_asignatura, 
            gma.id_grado, 
            gma.id_modalidad
        FROM asignaturas a
        INNER JOIN grado_modalidad_asignaturas gma 
            ON a.id_asignatura = gma.id_asignatura
        WHERE gma.id_grado = $1 
            AND gma.id_modalidad = $2
        ORDER BY gma.orden";
    
    let rows = db.query(query, &[&id_grado, &id_modalidad])
        .await
        .map_err(|e| e.to_string())?;

    let asignaturas = rows.iter().map(|row| Asignatura {
        id_asignatura: row.get("id_asignatura"),
        nombre_asignatura: row.get("nombre_asignatura"),
        id_grado: row.get("id_grado"),
        id_modalidad: row.get("id_modalidad"),
    }).collect();

    Ok(asignaturas)
} 