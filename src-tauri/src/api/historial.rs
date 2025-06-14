use tauri::State;
use crate::AppState;
use crate::models::historial::HistorialAcademico;
use crate::models::calificacion::CalificacionEstudiante;
use crate::utils::notas::{calcular_nota_final, calcular_promedio_anual};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HistorialInput {
    #[serde(rename = "idEstudiante")]
    pub id_estudiante: i32,
    #[serde(rename = "idPeriodo")]
    pub id_periodo: i32,
    #[serde(rename = "idGradoSecciones")]
    pub id_grado_secciones: i32,
}

#[tauri::command]
pub async fn obtener_historial_academico_estudiante(
    id_estudiante: i32,
    state: State<'_, AppState>,
) -> Result<Vec<HistorialAcademico>, String> {
    println!("[LOG][BACKEND] Iniciando obtención de historial académico para id_estudiante={}", id_estudiante);
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

    println!("[LOG][BACKEND] Ejecutando query de historial académico...");
    let rows_result = db.query(query, &[&id_estudiante]).await;
    match rows_result {
        Ok(rows) => {
            println!("[LOG][BACKEND] Query ejecutada correctamente. Filas obtenidas: {}", rows.len());
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
            }).collect::<Vec<_>>();
            println!("[LOG][BACKEND] Historiales mapeados correctamente. Total: {}", historiales.len());
            Ok(historiales)
        },
        Err(e) => {
            println!("[ERROR][BACKEND] Error al ejecutar query de historial académico: {}", e);
            Err(format!("Error al consultar historial académico: {}", e))
        }
    }
}

async fn guardar_historial_academico_interno(
    input: &HistorialInput,
    db: &tokio_postgres::Client,
) -> Result<(), String> {
    println!("[INTERNAL] Iniciando guardar_historial_academico_interno para estudiante={}, periodo={}, grado_secciones={}", input.id_estudiante, input.id_periodo, input.id_grado_secciones);
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
    println!("[INTERNAL] Ejecutando query de calificaciones para estudiante={}, periodo={}", input.id_estudiante, input.id_periodo);
    let calificaciones_rows = db.query(calificaciones_query, &[&input.id_estudiante, &input.id_periodo])
        .await
        .map_err(|e| {
            println!("[INTERNAL][ERROR] Error al obtener calificaciones: {}", e);
            e.to_string()
        })?;
    println!("[INTERNAL] Calificaciones encontradas: {}", calificaciones_rows.len());
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
    println!("[INTERNAL] Calificaciones procesadas para promedio y estatus.");
    let promedio_anual = calcular_promedio_anual(&calificaciones);
    let estatus = crate::utils::notas::calcular_estatus_academico(&calificaciones);
    println!("[INTERNAL] Promedio anual: {}, Estatus: {}", promedio_anual, estatus);
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
    println!("[INTERNAL] Ejecutando UPSERT en historial_academico para estudiante={}, periodo={}", input.id_estudiante, input.id_periodo);
    db.execute(query, &[&input.id_estudiante, &input.id_periodo, &input.id_grado_secciones, &promedio_anual, &estatus])
        .await
        .map_err(|e| {
            println!("[INTERNAL][ERROR] Error al ejecutar UPSERT: {}", e);
            e.to_string()
        })?;
    println!("[INTERNAL] UPSERT completado para estudiante={}, periodo={}", input.id_estudiante, input.id_periodo);
    Ok(())
}

#[tauri::command]
pub async fn upsert_historial_academico(
    input: HistorialInput,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    guardar_historial_academico_interno(&input, &db).await
}

#[tauri::command]
pub async fn guardar_historial_masivo(
    state: State<'_, AppState>,
    id_modalidad: Option<i32>,
    id_grado: Option<i32>,
    id_seccion: Option<i32>,
) -> Result<String, String> {
    let db = state.db.lock().await;
    println!("[MASIVO] Iniciando guardado masivo de historial académico...");
    // 1. Obtener el período activo
    let row = db.query_one(
        "SELECT id_periodo FROM periodos_escolares WHERE activo = TRUE LIMIT 1",
        &[],
    ).await.map_err(|e| format!("Error al obtener período activo: {}", e))?;
    let id_periodo: i32 = row.get("id_periodo");
    println!("[MASIVO] Período activo: {}", id_periodo);

    // 2. Construir el filtro de estudiantes activos
    let mut query = String::from(
        "SELECT id, id_grado_secciones FROM estudiantes WHERE estado = 'activo'"
    );
    let mut params_owned: Vec<Box<dyn tokio_postgres::types::ToSql + Send + Sync>> = Vec::new();
    let mut idx = 1;
    if let Some(modalidad) = id_modalidad {
        query.push_str(&format!(" AND id_modalidad = ${}", idx));
        params_owned.push(Box::new(modalidad));
        idx += 1;
    }
    if let Some(grado) = id_grado {
        query.push_str(&format!(" AND id_grado = ${}", idx));
        params_owned.push(Box::new(grado));
        idx += 1;
    }
    if let Some(seccion) = id_seccion {
        query.push_str(&format!(" AND id_seccion = ${}", idx));
        params_owned.push(Box::new(seccion));
    }
    let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params_owned.iter().map(|b| b.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync)).collect();

    println!("[MASIVO] Ejecutando query de estudiantes: {}", query);
    let estudiantes_rows = db.query(&query, &params).await.map_err(|e| format!("Error al buscar estudiantes: {}", e))?;
    println!("[MASIVO] Estudiantes encontrados: {}", estudiantes_rows.len());
    if estudiantes_rows.is_empty() {
        println!("[MASIVO] No se encontraron estudiantes activos con los filtros seleccionados.");
        return Err("No se encontraron estudiantes activos con los filtros seleccionados.".to_string());
    }

    let mut exitos = 0;
    let mut errores: Vec<String> = Vec::new();
    for (i, row) in estudiantes_rows.iter().enumerate() {
        let id_estudiante: i32 = row.get("id");
        let id_grado_secciones: i32 = match row.try_get("id_grado_secciones") {
            Ok(val) => val,
            Err(_) => {
                let msg = format!("Estudiante {} no tiene grado/sección asignado.", id_estudiante);
                println!("[MASIVO][ERROR] {}", msg);
                errores.push(msg);
                continue;
            }
        };
        let input = HistorialInput {
            id_estudiante,
            id_periodo,
            id_grado_secciones,
        };
        println!("[MASIVO] ({}/{}) Guardando historial para estudiante {}...", i+1, estudiantes_rows.len(), id_estudiante);
        match guardar_historial_academico_interno(&input, &db).await {
            Ok(_) => {
                exitos += 1;
                println!("[MASIVO] ({}/{}) Historial guardado exitosamente para estudiante {}.", i+1, estudiantes_rows.len(), id_estudiante);
            },
            Err(e) => {
                let msg = format!("Estudiante {}: {}", id_estudiante, e);
                println!("[MASIVO][ERROR] {}", msg);
                errores.push(msg);
            },
        }
    }
    let total = exitos + errores.len();
    println!("[MASIVO] Guardado masivo completado. Total: {}. Exitosos: {}. Errores: {}.", total, exitos, errores.len());
    let mut resumen = format!("Guardado masivo completado. Total: {}. Exitosos: {}. Errores: {}.", total, exitos, errores.len());
    if !errores.is_empty() {
        resumen.push_str("\nErrores principales:\n");
        for err in errores.iter().take(10) {
            resumen.push_str(&format!("- {}\n", err));
        }
        if errores.len() > 10 {
            resumen.push_str(&format!("... y {} errores más.", errores.len() - 10));
        }
    }
    Ok(resumen)
} 