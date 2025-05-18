use tauri::State;
use crate::AppState;
use crate::models::historial::HistorialAcademico;
use crate::models::calificacion::CalificacionEstudiante;
use crate::utils::notas::{calcular_nota_final, calcular_promedio_anual};

#[tauri::command]
pub async fn obtener_historial_academico_estudiante(
    id_estudiante: i32,
    state: State<'_, AppState>,
) -> Result<Vec<HistorialAcademico>, String> {
    let db = state.db.lock().await;
    let query = "
        SELECT 
            h.id_historial,
            h.id_estudiante,
            h.id_periodo,
            h.id_grado_secciones,
            h.promedio_anual::float8,
            h.estatus,
            h.fecha_registro,
            p.periodo_escolar,
            g.nombre_grado as grado,
            s.nombre_seccion as seccion
        FROM historial_academico h
        LEFT JOIN periodos_escolares p ON h.id_periodo = p.id_periodo
        LEFT JOIN grado_secciones gs ON h.id_grado_secciones = gs.id_grado_secciones
        LEFT JOIN grados g ON gs.id_grado = g.id_grado
        LEFT JOIN secciones s ON gs.id_seccion = s.id_seccion
        WHERE h.id_estudiante = $1
        ORDER BY h.id_periodo DESC";

    let rows = db.query(query, &[&id_estudiante])
        .await
        .map_err(|e| e.to_string())?;

    let historiales = rows.iter().map(|row| HistorialAcademico {
        id_historial: row.get("id_historial"),
        id_estudiante: row.get("id_estudiante"),
        id_periodo: row.get("id_periodo"),
        id_grado_secciones: row.get("id_grado_secciones"),
        promedio_anual: row.get("promedio_anual"),
        estatus: row.get("estatus"),
        fecha_registro: row.get("fecha_registro"),
        periodo_escolar: row.get("periodo_escolar"),
        grado: row.get("grado"),
        seccion: row.get("seccion"),
    }).collect();

    Ok(historiales)
}

#[tauri::command]
pub async fn upsert_historial_academico(
    id_estudiante: i32,
    id_periodo: i32,
    id_grado_secciones: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    
    // Obtener todas las calificaciones del estudiante para el período
    let calificaciones_query = "
        SELECT 
            c.id_calificacion,
            c.id_asignatura,
            a.nombre as nombre_asignatura,
            c.lapso_1,
            c.lapso_1_ajustado,
            c.lapso_2,
            c.lapso_2_ajustado,
            c.lapso_3,
            c.lapso_3_ajustado,
            c.revision
        FROM calificaciones c
        JOIN asignaturas a ON c.id_asignatura = a.id_asignatura
        WHERE c.id_estudiante = $1 AND c.id_periodo = $2";
    
    let calificaciones_rows = db.query(calificaciones_query, &[&id_estudiante, &id_periodo])
        .await
        .map_err(|e| e.to_string())?;
    
    // Convertir las filas a CalificacionEstudiante
    let calificaciones: Vec<CalificacionEstudiante> = calificaciones_rows.iter()
        .map(|row| {
            let cal = CalificacionEstudiante {
                id_calificacion: row.get("id_calificacion"),
                id_asignatura: row.get("id_asignatura"),
                nombre_asignatura: row.get("nombre_asignatura"),
                lapso_1: row.get("lapso_1"),
                lapso_1_ajustado: row.get("lapso_1_ajustado"),
                lapso_2: row.get("lapso_2"),
                lapso_2_ajustado: row.get("lapso_2_ajustado"),
                lapso_3: row.get("lapso_3"),
                lapso_3_ajustado: row.get("lapso_3_ajustado"),
                nota_final: None,
                revision: row.get("revision"),
            };
            let nota_final = calcular_nota_final(&cal);
            CalificacionEstudiante {
                nota_final: Some(nota_final),
                ..cal
            }
        })
        .collect();
    
    // Calcular el promedio anual usando la función centralizada
    let promedio_anual = calcular_promedio_anual(&calificaciones);
    
    // Determinar el estatus basado en el promedio
    let estatus = if promedio_anual >= 10.0 {
        "APROBADO".to_string()
    } else {
        "REPROBADO".to_string()
    };
    
    // Insertar o actualizar el historial académico
    let query = "
        INSERT INTO historial_academico (
            id_estudiante, id_periodo, id_grado_secciones, promedio_anual, estatus
        )
        VALUES ($1, $2, $3, $4::float8, $5)
        ON CONFLICT (id_estudiante, id_periodo) 
        DO UPDATE SET
            id_grado_secciones = EXCLUDED.id_grado_secciones,
            promedio_anual = EXCLUDED.promedio_anual,
            estatus = EXCLUDED.estatus";

    db.execute(query, &[&id_estudiante, &id_periodo, &id_grado_secciones, &promedio_anual, &estatus])
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
} 