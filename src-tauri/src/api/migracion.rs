use tauri::State;
use crate::AppState;
use postgres_types::ToSql;
use std::collections::HashMap;

#[tauri::command]
pub async fn migrar_estudiantes(
    modalidad: Option<i32>,
    grado: Option<i32>,
    seccion: Option<i32>,
    tipo: String, // "promovidos" o "repitientes"
    state: State<'_, AppState>,
) -> Result<String, String> {
    let db = state.db.lock().await;
    let mut errores: Vec<String> = Vec::new();
    let mut migrados = 0;
    let mut repitentes = 0;
    let mut promovidos = 0;

    // 1. Obtener el periodo activo (nuevo)
    let row = db.query_one(
        "SELECT id_periodo FROM periodos_escolares WHERE activo = TRUE LIMIT 1",
        &[],
    ).await.map_err(|e| format!("Error al obtener período activo: {}", e))?;
    let id_periodo_nuevo: i32 = row.get("id_periodo");

    // 2. Obtener el periodo anterior (el de mayor id menor al activo)
    let row_prev = db.query_opt(
        "SELECT id_periodo FROM periodos_escolares WHERE id_periodo < $1 ORDER BY id_periodo DESC LIMIT 1",
        &[&id_periodo_nuevo],
    ).await.map_err(|e| format!("Error al obtener período anterior: {}", e))?;
    let id_periodo_anterior = match row_prev {
        Some(r) => r.get::<_, i32>("id_periodo"),
        None => return Err("No se encontró período anterior al activo".to_string()),
    };

    // 3. Construir el filtro de estudiantes activos en el periodo anterior
    let mut query = String::from(
        "SELECT hge.id_estudiante, hge.id_grado_secciones FROM historial_grado_estudiantes hge \
         JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones \
         WHERE hge.id_periodo = $1 AND hge.es_actual = TRUE AND hge.estado = 'activo'"
    );
    let mut params_owned: Vec<Box<dyn ToSql + Send + Sync>> = Vec::new();
    params_owned.push(Box::new(id_periodo_anterior));
    let mut idx = 2;
    if let Some(m) = modalidad {
        query.push_str(&format!(" AND gs.id_modalidad = ${}", idx));
        params_owned.push(Box::new(m));
        idx += 1;
    }
    if let Some(g) = grado {
        query.push_str(&format!(" AND gs.id_grado = ${}", idx));
        params_owned.push(Box::new(g));
        idx += 1;
    }
    if let Some(s) = seccion {
        query.push_str(&format!(" AND gs.id_seccion = ${}", idx));
        params_owned.push(Box::new(s));
    }
    let params: Vec<&(dyn ToSql + Sync)> = params_owned.iter().map(|b| b.as_ref() as &(dyn ToSql + Sync)).collect();
    println!("[MIGRACION] Parámetros recibidos - modalidad: {:?}, grado: {:?}, seccion: {:?}, tipo: {}", modalidad, grado, seccion, tipo);
    println!("[MIGRACION] id_periodo_anterior: {}", id_periodo_anterior);
    let estudiantes_rows = db.query(&query, &params).await.map_err(|e| format!("Error al buscar estudiantes: {}", e))?;
    println!("[MIGRACION] Filas encontradas en la consulta: {}", estudiantes_rows.len());
    if estudiantes_rows.is_empty() {
        return Err("No se encontraron estudiantes activos con los filtros seleccionados en el período anterior.".to_string());
    }

    // 4. Mapear id_estudiante -> id_grado_secciones
    let mut estudiantes: HashMap<i32, i32> = HashMap::new();
    for row in estudiantes_rows.iter() {
        estudiantes.insert(row.get("id_estudiante"), row.get("id_grado_secciones"));
    }
    println!("[MIGRACION] Estudiantes encontrados: {:?}", estudiantes.keys().collect::<Vec<_>>());

    // 5. Para cada estudiante, buscar su estatus en historial_academico del periodo anterior
    for (&id_estudiante, &id_grado_secciones) in estudiantes.iter() {
        let row_hist = db.query_opt(
            "SELECT estatus FROM historial_academico WHERE id_estudiante = $1 AND id_periodo = $2 ORDER BY id_historial DESC LIMIT 1",
            &[&id_estudiante, &id_periodo_anterior],
        ).await.map_err(|e| format!("Error al consultar historial académico: {}", e))?;
        let estatus = match row_hist {
            Some(r) => r.get::<_, String>("estatus"),
            None => {
                println!("[MIGRACION] Estudiante {} no tiene historial académico en el período anterior", id_estudiante);
                errores.push(format!("Estudiante {} no tiene historial académico en el período anterior", id_estudiante));
                continue;
            }
        };
        println!("[MIGRACION] Estudiante {} - estatus: {}", id_estudiante, estatus);
        let es_promovido = estatus.trim().to_uppercase() == "APROBADO";
        let es_repitiente = estatus.trim().to_uppercase() == "REPITE";
        // Filtrar según tipo
        if tipo == "promovidos" && !es_promovido {
            continue;
        }
        if tipo == "repitientes" && !es_repitiente {
            continue;
        }
        // Buscar datos de grado_secciones actual
        let row_gs = db.query_one(
            "SELECT id_grado, id_seccion, id_modalidad FROM grado_secciones WHERE id_grado_secciones = $1",
            &[&id_grado_secciones],
        ).await.map_err(|e| format!("Error al consultar grado_secciones: {}", e))?;
        let id_grado: i32 = row_gs.get("id_grado");
        let id_seccion: i32 = row_gs.get("id_seccion");
        let id_modalidad: i32 = row_gs.get("id_modalidad");
        let nuevo_id_grado = if es_promovido { id_grado + 1 } else { id_grado };
        // Buscar el nuevo id_grado_secciones
        let row_nuevo_gs = db.query_opt(
            "SELECT id_grado_secciones FROM grado_secciones WHERE id_grado = $1 AND id_seccion = $2 AND id_modalidad = $3",
            &[&nuevo_id_grado, &id_seccion, &id_modalidad],
        ).await.map_err(|e| format!("Error al buscar nuevo grado_secciones: {}", e))?;
        let nuevo_id_grado_secciones = match row_nuevo_gs {
            Some(r) => r.get::<_, i32>("id_grado_secciones"),
            None => {
                errores.push(format!("No existe grado_secciones para grado {} sección {} modalidad {}", nuevo_id_grado, id_seccion, id_modalidad));
                continue;
            }
        };
        // Desactivar el registro anterior en historial_grado_estudiantes
        let res_update = db.execute(
            "UPDATE historial_grado_estudiantes SET es_actual = FALSE WHERE id_estudiante = $1 AND id_periodo = $2 AND es_actual = TRUE",
            &[&id_estudiante, &id_periodo_anterior],
        ).await;
        if let Err(e) = res_update {
            errores.push(format!("Error al actualizar es_actual para estudiante {}: {}", id_estudiante, e));
            continue;
        }
        // Insertar el nuevo registro
        let res_insert = db.execute(
            "INSERT INTO historial_grado_estudiantes (id_estudiante, id_grado_secciones, id_periodo, fecha_inicio, es_actual, estado) VALUES ($1, $2, $3, NOW(), TRUE, 'activo')",
            &[&id_estudiante, &nuevo_id_grado_secciones, &id_periodo_nuevo],
        ).await;
        match res_insert {
            Ok(_) => {
                migrados += 1;
                if es_promovido { promovidos += 1; } else { repitentes += 1; }
            },
            Err(e) => {
                errores.push(format!("Error al insertar nuevo historial para estudiante {}: {}", id_estudiante, e));
            }
        }
    }
    let mut resumen = format!("Migración completada. Total migrados: {}. Promovidos: {}. Repitientes: {}. Errores: {}.", migrados, promovidos, repitentes, errores.len());
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

#[tauri::command(rename_all = "snake_case")]
pub async fn migrar_estudiantes_nuevos(
    id_grado_secciones: i32,
    id_periodo: i32,
    state: tauri::State<'_, crate::AppState>,
) -> Result<usize, String> {
    let db = state.db.lock().await;

    // 1. Obtener fechas del periodo escolar
    let row_periodo = db.query_one(
        "SELECT fecha_inicio, fecha_final FROM periodos_escolares WHERE id_periodo = $1",
        &[&id_periodo]
    ).await.map_err(|e| format!("No se encontró el período escolar: {}", e))?;
    let fecha_inicio: chrono::NaiveDate = row_periodo.get(0);
    let fecha_fin: chrono::NaiveDate = row_periodo.get(1);

    // 2. Buscar estudiantes activos con el id_grado_secciones dado, que no tengan historial en ese periodo
    let estudiantes = db.query(
        "SELECT id FROM estudiantes WHERE estado = 'activo' AND id_grado_secciones = $1 AND id NOT IN (
            SELECT id_estudiante FROM historial_grado_estudiantes WHERE id_periodo = $2
        )",
        &[&id_grado_secciones, &id_periodo]
    ).await.map_err(|e| format!("Error al buscar estudiantes: {}", e))?;

    let mut migrados = 0;
    for row in estudiantes {
        let id_estudiante: i32 = row.get(0);
        // Insertar en historial_grado_estudiantes
        let res = db.execute(
            "INSERT INTO historial_grado_estudiantes (id_estudiante, id_grado_secciones, id_periodo, fecha_inicio, fecha_fin, es_actual, estado)
             VALUES ($1, $2, $3, $4, $5, TRUE, 'activo')",
            &[&id_estudiante, &id_grado_secciones, &id_periodo, &fecha_inicio, &fecha_fin]
        ).await;
        if res.is_ok() {
            migrados += 1;
        }
    }
    Ok(migrados)
} 