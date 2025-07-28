use crate::models::actividad_reciente::{ActividadReciente, NuevaActividad};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn obtener_actividad_reciente(
    limite: i64,
    state: State<'_, AppState>,
) -> Result<Vec<ActividadReciente>, String> {
    let db = state.db.lock().await;
    
    ActividadReciente::obtener_recientes(&*db, limite)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn registrar_actividad(
    nueva_actividad: NuevaActividad,
    state: State<'_, AppState>,
) -> Result<ActividadReciente, String> {
    let db = state.db.lock().await;
    
    ActividadReciente::crear(&*db, nueva_actividad)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn limpiar_actividad_antigua(
    state: State<'_, AppState>,
) -> Result<u64, String> {
    let db = state.db.lock().await;
    
    ActividadReciente::limpiar_antiguas(&*db)
        .await
        .map_err(|e| e.to_string())
} 