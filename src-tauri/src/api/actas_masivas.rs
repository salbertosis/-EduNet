use crate::AppState;
use tauri::State;
use tokio::sync::Semaphore;
use std::sync::Arc;
use futures::future::join_all;

#[derive(Clone)]
pub struct DatosActa {
    pub id_grado_secciones: i32,
    pub id_seccion: i32,
    pub id_asignatura: i32,
    pub id_periodo: i32,
    pub estudiantes: Vec<(i32, i64, String, String)>, // id, cedula, nombres, apellidos
    pub nombre_asignatura: String,
    pub nombre_grado: String,
    pub nombre_seccion: String,
    pub modalidad: String,
}

#[tauri::command]
pub async fn generar_actas_masivas(
    state: State<'_, AppState>,
    id_periodo: i32,
    id_modalidad: i32,
    id_grado: i32,
    ids_asignatura: Option<Vec<i32>>,
    ids_seccion: Option<Vec<i32>>,
    lapso: String,
) -> Result<Vec<String>, String> {
    println!("[MASIVO] Iniciando generación masiva de actas");
    println!("[MASIVO] Parámetros: id_periodo={}, id_modalidad={}, id_grado={}, ids_asignatura={:?}, ids_seccion={:?}, lapso={}", id_periodo, id_modalidad, id_grado, ids_asignatura, ids_seccion, lapso);
    let db_pool = state.db.clone();
    let db = db_pool.lock().await;
    // 1. Obtener secciones activas
    println!("[MASIVO] Consultando secciones activas...");
    let secciones_rows = db.query(
        "SELECT DISTINCT gs.id_grado_secciones, s.id_seccion FROM secciones s \
         JOIN grado_secciones gs ON s.id_seccion = gs.id_seccion \
         JOIN historial_grado_estudiantes h ON gs.id_grado_secciones = h.id_grado_secciones \
         WHERE gs.id_grado = $1 AND gs.id_modalidad = $2 AND h.id_periodo = $3 AND h.estado = 'activo' AND h.es_actual = true \
         ORDER BY s.id_seccion",
        &[&id_grado, &id_modalidad, &id_periodo],
    ).await.map_err(|e| {
        println!("[MASIVO][ERROR] Error consultando secciones: {}", e);
        e.to_string()
    })?;
    let secciones: Vec<(i32, i32)> = secciones_rows
        .iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect();
    let secciones_filtradas: Vec<(i32, i32)> = match &ids_seccion {
        Some(ids) => secciones.iter().cloned().filter(|(_, id_seccion)| ids.contains(id_seccion)).collect(),
        None => secciones.clone(),
    };
    if secciones_filtradas.is_empty() {
        return Err("No hay secciones activas para los parámetros dados".to_string());
    }
    // 2. Obtener asignaturas activas
    let asignaturas_rows = db.query(
        "SELECT DISTINCT a.id_asignatura, gma.orden FROM asignaturas a \
         JOIN grado_modalidad_asignaturas gma ON a.id_asignatura = gma.id_asignatura \
         WHERE gma.id_grado = $1 AND gma.id_modalidad = $2 \
         ORDER BY gma.orden",
        &[&id_grado, &id_modalidad],
    ).await.map_err(|e| e.to_string())?;
    let asignaturas: Vec<i32> = asignaturas_rows.iter().map(|row| row.get(0)).collect();
    let asignaturas_filtradas: Vec<i32> = match &ids_asignatura {
        Some(ids) => asignaturas.iter().cloned().filter(|id| ids.contains(id)).collect(),
        None => asignaturas.clone(),
    };
    if asignaturas_filtradas.is_empty() {
        return Err("No hay asignaturas activas para los parámetros dados".to_string());
    }
    // 3. Consultar todos los datos necesarios para cada acta (en batch)
    let mut datos_actas: Vec<DatosActa> = Vec::new();
    for (id_grado_secciones, id_seccion) in &secciones_filtradas {
        // Obtener nombre_grado, nombre_seccion, modalidad
        let row_grado_seccion = db.query_one(
            "SELECT g.nombre_grado, s.nombre_seccion, m.nombre_modalidad FROM grado_secciones gs \
             JOIN grados g ON gs.id_grado = g.id_grado \
             JOIN secciones s ON gs.id_seccion = s.id_seccion \
             JOIN modalidades m ON gs.id_modalidad = m.id_modalidad \
             WHERE gs.id_grado_secciones = $1",
            &[id_grado_secciones],
        ).await.map_err(|e| e.to_string())?;
        let nombre_grado: String = row_grado_seccion.get(0);
        let nombre_seccion: String = row_grado_seccion.get(1);
        let modalidad: String = row_grado_seccion.get(2);
        for id_asignatura in &asignaturas_filtradas {
            // Obtener estudiantes
            let estudiantes_rows = db.query(
                "SELECT e.id, e.cedula, e.nombres, e.apellidos FROM historial_grado_estudiantes h \
                 JOIN estudiantes e ON h.id_estudiante = e.id \
                 WHERE h.id_grado_secciones = $1 AND h.id_periodo = $2 AND h.estado = 'activo' AND h.es_actual = true \
                 ORDER BY \
                    CASE \
                        WHEN e.cedula < 100000000 THEN 0 \
                        ELSE 1 \
                    END, \
                    e.cedula",
                &[id_grado_secciones, &id_periodo],
            ).await.map_err(|e| e.to_string())?;
            let estudiantes: Vec<(i32, i64, String, String)> = estudiantes_rows
                .iter()
                .map(|row| (row.get(0), row.get(1), row.get(2), row.get(3)))
                .collect();
            // Obtener nombre de la asignatura
            let row_asig = db.query_one(
                "SELECT nombre FROM asignaturas WHERE id_asignatura = $1",
                &[id_asignatura],
            ).await.map_err(|e| e.to_string())?;
            let nombre_asignatura: String = row_asig.get(0);
            datos_actas.push(DatosActa {
                id_grado_secciones: *id_grado_secciones,
                id_seccion: *id_seccion,
                id_asignatura: *id_asignatura,
                id_periodo,
                estudiantes: estudiantes.clone(),
                nombre_asignatura,
                nombre_grado: nombre_grado.clone(),
                nombre_seccion: nombre_seccion.clone(),
                modalidad: modalidad.clone(),
            });
        }
    }
    drop(db); // Liberar lock de la BD antes de la generación
    // 4. Procesamiento concurrente limitado SOLO para la generación de archivos
    let semaphore = Arc::new(Semaphore::new(4));
    let mut handles = vec![];
    for datos in datos_actas {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let lapso = lapso.clone();
        let datos_clon = datos.clone();
        let id_grado_secciones = datos_clon.id_grado_secciones;
        let id_asignatura = datos_clon.id_asignatura;
        let handle = tokio::spawn(async move {
            let res = crate::api::plantillas::generar_plantilla_acta_desde_datos(datos_clon, lapso).await;
            drop(permit);
            match res {
                Ok(nombre) => nombre,
                Err(e) => format!("Error: {} (seccion {}, asignatura {})", e, id_grado_secciones, id_asignatura),
            }
        });
        handles.push(handle);
    }
    let resultados: Vec<String> = join_all(handles).await
        .into_iter()
        .map(|res| res.unwrap_or_else(|e| format!("Error en tarea: {}", e)))
        .collect();
    Ok(resultados)
} 