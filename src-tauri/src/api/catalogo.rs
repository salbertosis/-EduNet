use tauri::State;
use crate::AppState;
use crate::models::catalogo::{PeriodoEscolar, Grado, Modalidad, Asignatura};
use chrono::NaiveDate;

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

#[tauri::command]
pub async fn crear_periodo_escolar(
    periodo_escolar: String,
    fecha_inicio: String,
    fecha_final: String,
    state: State<'_, AppState>,
) -> Result<i32, String> {
    let db = state.db.lock().await;

    let parsed_fecha_inicio = NaiveDate::parse_from_str(&fecha_inicio, "%Y-%m-%d")
        .map_err(|e| format!("Error al parsear fecha de inicio: {}", e))?;
    let parsed_fecha_final = NaiveDate::parse_from_str(&fecha_final, "%Y-%m-%d")
        .map_err(|e| format!("Error al parsear fecha final: {}", e))?;

    let row = db.query_one(
        "INSERT INTO periodos_escolares (periodo_escolar, fecha_inicio, fecha_final, activo) VALUES ($1, $2, $3, FALSE) RETURNING id_periodo",
        &[&periodo_escolar, &parsed_fecha_inicio, &parsed_fecha_final],
    ).await.map_err(|e| e.to_string())?;

    let id_periodo: i32 = row.get("id_periodo");

    Ok(id_periodo)
}

#[tauri::command]
pub async fn establecer_periodo_activo(
    id_periodo: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;

    // 1. Establecer todos los periodos a inactivos
    db.execute("UPDATE periodos_escolares SET activo = FALSE", &[])
        .await
        .map_err(|e| format!("Error al desactivar periodos existentes: {}", e))?;

    // 2. Establecer el periodo especificado como activo
    db.execute("UPDATE periodos_escolares SET activo = TRUE WHERE id_periodo = $1", &[&id_periodo])
        .await
        .map_err(|e| format!("Error al activar el nuevo periodo: {}", e))?;

    Ok(())
} 