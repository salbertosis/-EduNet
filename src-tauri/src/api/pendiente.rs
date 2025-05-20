use tauri::State;
use crate::AppState;
use crate::models::pendiente::{AsignaturaPendiente, AsignaturaPendienteInput};

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