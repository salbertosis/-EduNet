use crate::{AppState, models::tipo_evaluacion::{TipoEvaluacion, RespuestaTiposEvaluacion}};
use tauri::State;

#[tauri::command]
pub async fn obtener_tipos_evaluacion(state: State<'_, AppState>) -> Result<RespuestaTiposEvaluacion, String> {
    println!("📊 [TIPOS_EVALUACION] === OBTENIENDO TIPOS DE EVALUACIÓN ===");
    
    let client = state.db.lock().await;
    println!("📊 [TIPOS_EVALUACION] Conexión a BD obtenida");
    
    let query = "
        SELECT id, codigo, nombre
        FROM tipos_evaluacion 
        WHERE activo = true 
        ORDER BY codigo
    ";
    
    println!("📊 [TIPOS_EVALUACION] Ejecutando query: {}", query);
    
    match client.query(query, &[]).await {
        Ok(rows) => {
            println!("📊 [TIPOS_EVALUACION] Query ejecutada exitosamente. Filas obtenidas: {}", rows.len());
            
            let mut tipos_evaluacion = Vec::new();
            
            for row in rows {
                let tipo_evaluacion = TipoEvaluacion {
                    id: row.get("id"),
                    codigo: row.get("codigo"),
                    nombre: row.get("nombre"),
                };
                
                println!("📊 [TIPOS_EVALUACION] Tipo encontrado: {} - {}", tipo_evaluacion.codigo, tipo_evaluacion.nombre);
                tipos_evaluacion.push(tipo_evaluacion);
            }
            
            println!("📊 [TIPOS_EVALUACION] Total de tipos encontrados: {}", tipos_evaluacion.len());
            Ok(RespuestaTiposEvaluacion::exito(tipos_evaluacion))
        }
        Err(e) => {
            println!("❌ [TIPOS_EVALUACION] Error en query: {}", e);
            Err(format!("Error al obtener tipos de evaluación: {}", e))
        }
    }
} 