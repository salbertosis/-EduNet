use tauri::State;
use crate::AppState;
use crate::models::calificacion::{CalificacionEstudiante, CalificacionInput};
use crate::utils::notas::calcular_nota_final;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CalificacionCargaMasiva {
    pub id_estudiante: i32,
    pub id_asignatura: i32,
    pub id_periodo: i32,
    pub lapso_1: Option<i32>,
    pub lapso_2: Option<i32>,
    pub lapso_3: Option<i32>,
    pub revision: Option<i32>,
    pub lapso_1_ajustado: Option<i32>,
    pub lapso_2_ajustado: Option<i32>,
    pub lapso_3_ajustado: Option<i32>,
    pub nota_final: Option<i32>,
}

#[tauri::command]
pub async fn guardar_calificacion(
    calificacion: CalificacionInput,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    
    if let Some(id_calificacion) = calificacion.id_calificacion {
        // UPDATE
        db.execute(
            "UPDATE calificaciones SET lapso_1=$1, lapso_1_ajustado=$2, lapso_2=$3, lapso_2_ajustado=$4, lapso_3=$5, lapso_3_ajustado=$6, revision=$7, nota_final=$8 WHERE id_calificacion=$9",
            &[&calificacion.lapso_1, &calificacion.lapso_1_ajustado, &calificacion.lapso_2, &calificacion.lapso_2_ajustado, &calificacion.lapso_3, &calificacion.lapso_3_ajustado, &calificacion.revision, &calificacion.nota_final, &id_calificacion]
        ).await.map_err(|e| e.to_string())?;
    } else {
        // UPSERT: INSERT ... ON CONFLICT ... DO UPDATE
        db.execute(
            "INSERT INTO calificaciones (id_estudiante, id_asignatura, id_periodo, lapso_1, lapso_1_ajustado, lapso_2, lapso_2_ajustado, lapso_3, lapso_3_ajustado, revision, nota_final)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
             ON CONFLICT (id_estudiante, id_asignatura, id_periodo)
             DO UPDATE SET
                lapso_1=EXCLUDED.lapso_1,
                lapso_1_ajustado=EXCLUDED.lapso_1_ajustado,
                lapso_2=EXCLUDED.lapso_2,
                lapso_2_ajustado=EXCLUDED.lapso_2_ajustado,
                lapso_3=EXCLUDED.lapso_3,
                lapso_3_ajustado=EXCLUDED.lapso_3_ajustado,
                revision=EXCLUDED.revision,
                nota_final=EXCLUDED.nota_final",
            &[&calificacion.id_estudiante, &calificacion.id_asignatura, &calificacion.id_periodo, &calificacion.lapso_1, &calificacion.lapso_1_ajustado, &calificacion.lapso_2, &calificacion.lapso_2_ajustado, &calificacion.lapso_3, &calificacion.lapso_3_ajustado, &calificacion.revision, &calificacion.nota_final]
        ).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn obtener_calificaciones_estudiante(
    id_estudiante: i32,
    id_periodo: i32,
    state: State<'_, AppState>,
) -> Result<Vec<CalificacionEstudiante>, String> {
    println!("[DEBUG][BACKEND] >>>>> FUNCION obtener_calificaciones_estudiante llamada");
    println!("[DEBUG][BACKEND] Parámetros recibidos: id_estudiante={}, id_periodo={}", id_estudiante, id_periodo);
    let db = state.db.lock().await;
    // Log antes de la consulta de grado y modalidad
    println!("[DEBUG][BACKEND] Consultando grado y modalidad del estudiante...");
    let estudiante = db.query_one(
        "SELECT id_grado, id_modalidad FROM estudiantes WHERE id = $1",
        &[&id_estudiante]
    ).await.map_err(|e| format!("Error al verificar estudiante: {}", e))?;
    let id_grado: i32 = estudiante.get(0);
    let id_modalidad: i32 = estudiante.get(1);
    println!("[DEBUG][BACKEND] Estudiante encontrado: id_grado={}, id_modalidad={}", id_grado, id_modalidad);
    // Log antes de la consulta principal
    println!("[DEBUG][BACKEND] Ejecutando consulta principal de calificaciones con parámetros: id_grado={}, id_modalidad={}, id_estudiante={}, id_periodo={}", id_grado, id_modalidad, id_estudiante, id_periodo);
    let query = "
        WITH asignaturas_estudiante AS (
            SELECT 
                a.id_asignatura,
                a.nombre
            FROM asignaturas a
            INNER JOIN grado_modalidad_asignaturas gma 
                ON a.id_asignatura = gma.id_asignatura
            WHERE gma.id_grado = $1 
                AND gma.id_modalidad = $2
            ORDER BY gma.orden
        )
        SELECT 
            c.id_calificacion,
            ae.id_asignatura,
            ae.nombre,
            c.lapso_1,
            c.lapso_1_ajustado,
            c.lapso_2,
            c.lapso_2_ajustado,
            c.lapso_3,
            c.lapso_3_ajustado,
            c.revision
        FROM asignaturas_estudiante ae
        LEFT JOIN calificaciones c 
            ON ae.id_asignatura = c.id_asignatura 
            AND c.id_estudiante = $3 
            AND c.id_periodo = $4
        ORDER BY ae.nombre";
    println!("[DEBUG][BACKEND] Consulta SQL ejecutada: {}", query);
    let rows = db.query(query, &[&id_grado, &id_modalidad, &id_estudiante, &id_periodo])
        .await
        .map_err(|e| format!("Error al obtener calificaciones: {}", e))?;
    println!("[DEBUG][BACKEND] Filas obtenidas: {}", rows.len());
    for (i, row) in rows.iter().enumerate() {
        println!("[DEBUG][BACKEND] Fila {}: id_asignatura={:?}, id_calificacion={:?}, lapso_1={:?}, lapso_2={:?}, lapso_3={:?}",
            i,
            row.get::<usize, Option<i32>>(1),
            row.get::<usize, Option<i32>>(0),
            row.get::<usize, Option<i32>>(3),
            row.get::<usize, Option<i32>>(5),
            row.get::<usize, Option<i32>>(7),
        );
    }
    let calificaciones = rows.iter().map(|row| {
        let cal = CalificacionEstudiante {
            id_calificacion: row.get(0),
            id_asignatura: row.get(1),
            nombre_asignatura: row.get(2),
            lapso_1: row.get(3),
            lapso_1_ajustado: row.get(4),
            lapso_2: row.get(5),
            lapso_2_ajustado: row.get(6),
            lapso_3: row.get(7),
            lapso_3_ajustado: row.get(8),
            nota_final: None,
            revision: row.get(9),
        };
        let nota_final = calcular_nota_final(&cal);
        CalificacionEstudiante {
            nota_final: Some(nota_final),
            ..cal
        }
    }).collect::<Vec<_>>();
    println!("[DEBUG][BACKEND] Calificaciones construidas para respuesta: {:?}", calificaciones);
    Ok(calificaciones)
}

#[tauri::command]
pub async fn cargar_calificaciones_masivo(
    calificaciones: Vec<CalificacionCargaMasiva>,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let mut db = state.db.lock().await;
    let trans = db.transaction().await.map_err(|e| e.to_string())?;
    let mut insertados = 0;
    let mut actualizados = 0;
    let mut errores = vec![];

    for c in calificaciones {
        let res = trans.execute(
            "INSERT INTO calificaciones (id_estudiante, id_asignatura, id_periodo, lapso_1, lapso_2, lapso_3, revision, lapso_1_ajustado, lapso_2_ajustado, lapso_3_ajustado, nota_final)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
             ON CONFLICT (id_estudiante, id_asignatura, id_periodo)
             DO UPDATE SET
                lapso_1=EXCLUDED.lapso_1,
                lapso_2=EXCLUDED.lapso_2,
                lapso_3=EXCLUDED.lapso_3,
                revision=EXCLUDED.revision,
                lapso_1_ajustado=EXCLUDED.lapso_1_ajustado,
                lapso_2_ajustado=EXCLUDED.lapso_2_ajustado,
                lapso_3_ajustado=EXCLUDED.lapso_3_ajustado,
                nota_final=EXCLUDED.nota_final",
            &[&c.id_estudiante, &c.id_asignatura, &c.id_periodo, &c.lapso_1, &c.lapso_2, &c.lapso_3, &c.revision, &c.lapso_1_ajustado, &c.lapso_2_ajustado, &c.lapso_3_ajustado, &c.nota_final]
        ).await;
        match res {
            Ok(n) if n == 1 => insertados += 1,
            Ok(_) => actualizados += 1,
            Err(e) => errores.push(format!(
                "Est: {} Asig: {} Per: {}: {}", c.id_estudiante, c.id_asignatura, c.id_periodo, e
            )),
        }
    }
    trans.commit().await.map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "total_registros": insertados + actualizados + errores.len(),
        "insertados": insertados,
        "actualizados": actualizados,
        "errores": errores,
    }))
} 