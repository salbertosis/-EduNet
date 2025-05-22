use tauri::State;
use crate::AppState;
use crate::models::pendiente::{AsignaturaPendiente, AsignaturaPendienteInput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CalificacionesPendiente {
    pub id_pendiente: i32,
    pub cal_momento1: Option<i32>,
    pub cal_momento2: Option<i32>,
    pub cal_momento3: Option<i32>,
    pub cal_momento4: Option<i32>,
}

#[tauri::command]
pub async fn obtener_asignaturas_pendientes_estudiante(
    id_estudiante: i32,
    state: State<'_, AppState>,
) -> Result<Vec<AsignaturaPendiente>, String> {
    let db = state.db.lock().await;
    let query = "
        SELECT 
            ap.id_pendiente,
            ap.id_estudiante,
            ap.id_asignatura,
            ap.id_periodo,
            ap.grado,
            ap.cal_momento1,
            ap.cal_momento2,
            ap.cal_momento3,
            ap.cal_momento4,
            ap.estado,
            ap.fecha_registro,
            ap.id_grado_secciones,
            a.nombre,
            p.periodo_escolar
        FROM asignaturas_pendientes ap
        LEFT JOIN asignaturas a ON ap.id_asignatura = a.id_asignatura
        LEFT JOIN periodos_escolares p ON ap.id_periodo = p.id_periodo
        WHERE ap.id_estudiante = $1
        ORDER BY ap.id_periodo DESC";

    let rows = db.query(query, &[&id_estudiante])
        .await
        .map_err(|e| e.to_string())?;

    let pendientes = rows.iter().map(|row| AsignaturaPendiente {
        id_pendiente: row.get("id_pendiente"),
        id_estudiante: row.get("id_estudiante"),
        id_asignatura: row.get("id_asignatura"),
        id_periodo: row.get("id_periodo"),
        grado: row.get("grado"),
        cal_momento1: row.get("cal_momento1"),
        cal_momento2: row.get("cal_momento2"),
        cal_momento3: row.get("cal_momento3"),
        cal_momento4: row.get("cal_momento4"),
        estado: row.get("estado"),
        fecha_registro: row.get("fecha_registro"),
        id_grado_secciones: row.get("id_grado_secciones"),
        nombre_asignatura: row.get("nombre"),
        periodo_escolar: row.get("periodo_escolar"),
    }).collect();

    Ok(pendientes)
}

#[tauri::command]
pub async fn guardar_asignaturas_pendientes(
    id_estudiante: i32,
    pendientes: Vec<AsignaturaPendienteInput>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    println!("[DEBUG][BACKEND] guardar_asignaturas_pendientes llamado");
    let db = state.db.lock().await;
    
    // Obtener el nombre del grado y el id_grado_secciones del estudiante
    let grado_query = "
        SELECT g.nombre_grado, e.id_grado_secciones 
        FROM estudiantes e
        JOIN grado_secciones gs ON e.id_grado_secciones = gs.id_grado_secciones
        JOIN grados g ON gs.id_grado = g.id_grado
        WHERE e.id = $1";
    let grado_row = db.query_one(grado_query, &[&id_estudiante])
        .await
        .map_err(|e| e.to_string())?;
    let nombre_grado: String = grado_row.get("nombre_grado");
    let id_grado_secciones: i32 = grado_row.get("id_grado_secciones");

    println!("[DEBUG][BACKEND] Recibidos {} pendientes para guardar para id_estudiante={}", pendientes.len(), id_estudiante);
    if pendientes.is_empty() {
        return Err("No hay asignaturas pendientes para guardar.".to_string());
    }
    if pendientes.len() > 2 {
        return Err("No se pueden guardar más de 2 asignaturas pendientes.".to_string());
    }
    let mut insertados = 0;
    for pendiente in pendientes.iter() {
        println!("[DEBUG][BACKEND] Insertando pendiente: id_asignatura={}, id_periodo={:?}, revision={:?}", pendiente.id_asignatura, pendiente.id_periodo, pendiente.revision);
        let insert_query = "
            INSERT INTO asignaturas_pendientes (
                id_estudiante, id_asignatura, id_periodo, grado, estado, id_grado_secciones
            ) VALUES ($1, $2, $3, $4, 'PENDIENTE', $5)";
        let filas = db.execute(insert_query, &[&id_estudiante, &pendiente.id_asignatura, &pendiente.id_periodo, &nombre_grado, &id_grado_secciones])
            .await
            .map_err(|e| {
                println!("[ERROR][BACKEND] Error al insertar pendiente: {}", e);
                e.to_string()
            })?;
        if filas > 0 {
            insertados += 1;
        }
    }
    println!("[DEBUG][BACKEND] Guardado de pendientes completado. Total insertados: {}", insertados);
    if insertados == 0 {
        return Err("No se guardó ningún pendiente".to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn obtener_calificaciones_pendiente(id_pendiente: i32, state: State<'_, AppState>) -> Result<CalificacionesPendiente, String> {
    println!("[DEBUG][BACKEND] obtener_calificaciones_pendiente llamado con id_pendiente={}", id_pendiente);
    let db = state.db.lock().await;
    let query = "SELECT id_pendiente, cal_momento1, cal_momento2, cal_momento3, cal_momento4 FROM asignaturas_pendientes WHERE id_pendiente = $1";
    match db.query_one(query, &[&id_pendiente]).await {
        Ok(row) => {
            let result = CalificacionesPendiente {
                id_pendiente: row.get(0),
                cal_momento1: row.get(1),
                cal_momento2: row.get(2),
                cal_momento3: row.get(3),
                cal_momento4: row.get(4),
            };
            println!("[DEBUG][BACKEND] obtener_calificaciones_pendiente retorna: {:?}", result);
            Ok(result)
        },
        Err(e) => {
            eprintln!("[ERROR][obtener_calificaciones_pendiente] id_pendiente={}: {}", id_pendiente, e);
            Err(format!("ERR_DB: No se pudieron obtener las calificaciones para la asignatura pendiente (id: {}). Detalle: {}", id_pendiente, e))
        }
    }
}

#[derive(Deserialize)]
pub struct UpsertCalificacionesPendienteInput {
    pub id_pendiente: i32,
    pub cal_momento1: i32,
    pub cal_momento2: i32,
    pub cal_momento3: i32,
    pub cal_momento4: i32,
}

#[tauri::command]
pub async fn upsert_calificaciones_pendiente(input: UpsertCalificacionesPendienteInput, state: State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().await;
    let query = "UPDATE asignaturas_pendientes SET cal_momento1 = $1, cal_momento2 = $2, cal_momento3 = $3, cal_momento4 = $4 WHERE id_pendiente = $5";
    let result = db.execute(query, &[&input.cal_momento1, &input.cal_momento2, &input.cal_momento3, &input.cal_momento4, &input.id_pendiente]).await;
    match result {
        Ok(rows) if rows > 0 => Ok("Calificaciones actualizadas correctamente".to_string()),
        Ok(_) => {
            let insert_query = "INSERT INTO asignaturas_pendientes (id_pendiente, cal_momento1, cal_momento2, cal_momento3, cal_momento4) VALUES ($1, $2, $3, $4, $5)";
            match db.execute(insert_query, &[&input.id_pendiente, &input.cal_momento1, &input.cal_momento2, &input.cal_momento3, &input.cal_momento4]).await {
                Ok(_) => Ok("Calificaciones insertadas correctamente".to_string()),
                Err(e) => {
                    eprintln!("[ERROR][insert_calificaciones_pendiente] id_pendiente={}: {}", input.id_pendiente, e);
                    Err(format!("ERR_DB: No se pudieron insertar las calificaciones para la asignatura pendiente (id: {}). Detalle: {}", input.id_pendiente, e))
                }
            }
        },
        Err(e) => {
            eprintln!("[ERROR][upsert_calificaciones_pendiente] id_pendiente={}: {}", input.id_pendiente, e);
            Err(format!("ERR_DB: No se pudieron actualizar las calificaciones para la asignatura pendiente (id: {}). Detalle: {}", input.id_pendiente, e))
        }
    }
}

#[tauri::command]
pub async fn eliminar_asignatura_pendiente(
    id_pendiente: Option<i32>,
    idPendiente: Option<i32>,
    state: State<'_, AppState>
) -> Result<String, String> {
    // Prioridad: id_pendiente, luego idPendiente
    let id = id_pendiente.or(idPendiente).ok_or("Falta el parámetro id_pendiente o idPendiente")?;
    let db = state.db.lock().await;
    let query = "DELETE FROM asignaturas_pendientes WHERE id_pendiente = $1";
    match db.execute(query, &[&id]).await {
        Ok(rows) if rows > 0 => Ok("Asignatura pendiente eliminada correctamente".to_string()),
        Ok(_) => {
            eprintln!("[ERROR][eliminar_asignatura_pendiente] No se encontró el registro id_pendiente={}", id);
            Err(format!("No se encontró la asignatura pendiente a eliminar (id: {})", id))
        },
        Err(e) => {
            eprintln!("[ERROR][eliminar_asignatura_pendiente] id_pendiente={}: {}", id, e);
            Err(format!("No se pudo eliminar la asignatura pendiente (id: {}). Detalle: {}", id, e))
        }
    }
} 