// main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio_postgres::{NoTls};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::NaiveDate;
use postgres_types::{ToSql, FromSql};
use serde_json;
use std::collections::HashSet;
// Eliminada importación no usada: bytes::BytesMut

#[derive(Debug, Serialize, Deserialize, Clone, ToSql, FromSql)]
#[postgres(name = "estado_estudiante")]
pub enum EstadoEstudiante {
    #[postgres(name = "activo")]
    Activo,
    #[postgres(name = "retirado")]
    Retirado,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Estudiante {
    pub id: i32,
    pub cedula: i64,
    pub nombres: String,
    pub apellidos: String,
    pub genero: Option<String>,
    pub fecha_nacimiento: NaiveDate,
    pub id_grado_secciones: Option<i32>,
    pub fecha_ingreso: Option<NaiveDate>,
    pub municipionac: Option<String>,
    pub paisnac: Option<String>,
    pub entidadfed: Option<String>,
    pub ciudadnac: Option<String>,
    pub estadonac: Option<String>,
    pub id_grado: Option<i32>,
    pub nombre_grado: Option<String>,
    pub id_seccion: Option<i32>,
    pub nombre_seccion: Option<String>,
    pub id_modalidad: Option<i32>,
    pub nombre_modalidad: Option<String>,
    pub id_periodoactual: Option<i32>,
    pub estado: EstadoEstudiante,
    pub fecha_retiro: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct FiltroEstudiantes {
    pub cedula: Option<String>,
    pub apellidos: Option<String>,
    pub grado: Option<i32>,
    pub modalidad: Option<i32>,
    pub estado: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NuevoEstudiante {
    pub cedula: i64,
    pub nombres: String,
    pub apellidos: String,
    pub genero: Option<String>,
    pub fecha_nacimiento: NaiveDate,
    pub id_grado_secciones: Option<i32>,
    pub fecha_ingreso: Option<NaiveDate>,
    pub municipionac: Option<String>,
    pub paisnac: Option<String>,
    pub entidadfed: Option<String>,
    pub ciudadnac: Option<String>,
    pub estadonac: Option<String>,
    pub id_grado: Option<i32>,
    pub id_seccion: Option<i32>,
    pub id_modalidad: Option<i32>,
    pub id_periodoactual: Option<i32>,
    pub estado: EstadoEstudiante,
    pub fecha_retiro: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeriodoEscolar {
    pub id_periodo: i32,
    pub periodo_escolar: String,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Grado {
    pub id_grado: i32,
    pub nombre_grado: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Modalidad {
    pub id_modalidad: i32,
    pub nombre_modalidad: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asignatura {
    pub id_asignatura: i32,
    pub nombre_asignatura: String,
    pub id_grado: i32,
    pub id_modalidad: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalificacionEstudiante {
    pub id_calificacion: Option<i32>,
    pub id_asignatura: i32,
    pub nombre_asignatura: String,
    pub lapso_1: Option<i32>,
    pub lapso_1_ajustado: Option<i32>,
    pub lapso_2: Option<i32>,
    pub lapso_2_ajustado: Option<i32>,
    pub lapso_3: Option<i32>,
    pub lapso_3_ajustado: Option<i32>,
    pub nota_final: Option<i32>,
    pub revision: Option<i32>,
}

struct AppState {
    db: Arc<Mutex<tokio_postgres::Client>>,
}

enum ParamValue {
    I64(i64),
    String(String),
    I32(i32),
}

impl ParamValue {
    fn as_tosql(&self) -> &(dyn ToSql + Sync) {
        match self {
            ParamValue::I64(v) => v as &(dyn ToSql + Sync),
            ParamValue::String(v) => v as &(dyn ToSql + Sync),
            ParamValue::I32(v) => v as &(dyn ToSql + Sync),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResumenInsercion {
    total_registros: usize,
    insertados: usize,
    duplicados: usize,
    errores: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaginacionParams {
    pub pagina: i32,
    pub registros_por_pagina: i32,
}

#[derive(Debug, Serialize)]
pub struct PaginacionInfo {
    pub pagina_actual: i32,
    pub total_paginas: i32,
    pub total_registros: i64,
    pub registros_por_pagina: i32,
}

#[derive(Debug, Serialize)]
pub struct ResultadoPaginado<T> {
    pub datos: Vec<T>,
    pub paginacion: PaginacionInfo,
}

#[derive(Debug, Deserialize)]
pub struct CalificacionInput {
    pub id_calificacion: Option<i32>,
    pub id_estudiante: i64,
    pub id_asignatura: i32,
    pub id_periodo: i32,
    pub lapso_1: Option<i32>,
    pub lapso_1_ajustado: Option<i32>,
    pub lapso_2: Option<i32>,
    pub lapso_2_ajustado: Option<i32>,
    pub lapso_3: Option<i32>,
    pub lapso_3_ajustado: Option<i32>,
    pub revision: Option<i32>,
    pub nota_final: Option<i32>,
}

#[tauri::command]
async fn obtener_estudiantes(
    filtro: Option<FiltroEstudiantes>,
    paginacion: PaginacionParams,
    state: State<'_, AppState>,
) -> Result<ResultadoPaginado<Estudiante>, String> {
    let mut db = state.db.lock().await;
    
    // Construir la consulta base
    let mut query_base = String::from(
        "SELECT e.id, e.cedula, e.nombres, e.apellidos, e.genero, e.fecha_nacimiento, \
        e.id_grado_secciones, e.fecha_ingreso, e.municipionac, e.paisnac, e.entidadfed, \
        e.ciudadnac, e.estadonac, e.id_grado, g.nombre_grado, e.id_seccion, s.nombre_seccion, \
        e.id_modalidad, m.nombre_modalidad, e.id_periodoactual, e.estado, e.fecha_retiro \
        FROM estudiantes e \
        LEFT JOIN grados g ON e.id_grado = g.id_grado \
        LEFT JOIN secciones s ON e.id_seccion = s.id_seccion \
        LEFT JOIN modalidades m ON e.id_modalidad = m.id_modalidad \
        WHERE 1=1"
    );
    
    let mut param_values: Vec<ParamValue> = Vec::new();

    // Aplicar filtros
    if let Some(f) = filtro {
        if let Some(cedula_str) = f.cedula {
            query_base.push_str(&format!(" AND CAST(e.cedula AS TEXT) LIKE ${}", param_values.len() + 1));
            param_values.push(ParamValue::String(format!("{}%", cedula_str)));
        }
        if let Some(apellidos) = f.apellidos {
            query_base.push_str(&format!(" AND e.apellidos ILIKE ${}", param_values.len() + 1));
            param_values.push(ParamValue::String(format!("{}%", apellidos)));
        }
        if let Some(grado_num) = f.grado {
            query_base.push_str(&format!(" AND e.id_grado = ${}", param_values.len() + 1));
            param_values.push(ParamValue::I32(grado_num));
        }
        if let Some(modalidad_num) = f.modalidad {
            query_base.push_str(&format!(" AND e.id_modalidad = ${}", param_values.len() + 1));
            param_values.push(ParamValue::I32(modalidad_num));
        }
        if let Some(estado) = f.estado {
            query_base.push_str(&format!(" AND TRIM(LOWER(e.estado::text)) = LOWER(TRIM(${}))", param_values.len() + 1));
            let estado_formateado = match estado.to_lowercase().as_str() {
                "activo" => "Activo",
                "retirado" => "Retirado",
                _ => &estado,
            };
            param_values.push(ParamValue::String(estado_formateado.to_string()));
        }
    }

    // Obtener el total de registros
    let query_count = format!("SELECT COUNT(*) FROM ({}) as subquery", query_base);
    let params: Vec<&(dyn ToSql + Sync)> = param_values.iter().map(|v| v.as_tosql()).collect();
    let total_registros: i64 = db.query_one(&query_count, &params).await.map_err(|e| e.to_string())?.get(0);

    // Calcular información de paginación
    let total_paginas = (total_registros as f64 / paginacion.registros_por_pagina as f64).ceil() as i32;
    let offset = (paginacion.pagina - 1) * paginacion.registros_por_pagina;

    // Añadir paginación a la consulta
    let query_final = format!(
        "{} ORDER BY e.apellidos, e.nombres LIMIT {} OFFSET {}",
        query_base,
        paginacion.registros_por_pagina,
        offset
    );

    // LOGS DE DEPURACIÓN
    println!("[DEBUG] Query final: {}", query_final);
    for (i, param) in param_values.iter().enumerate() {
        match param {
            ParamValue::I64(v) => println!("[DEBUG] Param {}: I64({})", i+1, v),
            ParamValue::I32(v) => println!("[DEBUG] Param {}: I32({})", i+1, v),
            ParamValue::String(v) => println!("[DEBUG] Param {}: String({})", i+1, v),
        }
    }

    // Ejecutar la consulta paginada
    let rows = db.query(&query_final, &params).await.map_err(|e| e.to_string())?;

    let estudiantes = rows
        .iter()
        .map(|row| Estudiante {
            id: row.get(0),
            cedula: row.get(1),
            nombres: row.get(2),
            apellidos: row.get(3),
            genero: row.get(4),
            fecha_nacimiento: row.get(5),
            id_grado_secciones: row.get(6),
            fecha_ingreso: row.get(7),
            municipionac: row.get(8),
            paisnac: row.get(9),
            entidadfed: row.get(10),
            ciudadnac: row.get(11),
            estadonac: row.get(12),
            id_grado: row.get(13),
            nombre_grado: row.get(14),
            id_seccion: row.get(15),
            nombre_seccion: row.get(16),
            id_modalidad: row.get(17),
            nombre_modalidad: row.get(18),
            id_periodoactual: row.get(19),
            estado: row.get(20),
            fecha_retiro: row.get(21),
        })
        .collect::<Vec<_>>();

    // Crear la información de paginación
    let paginacion_info = PaginacionInfo {
        pagina_actual: paginacion.pagina,
        total_paginas,
        total_registros,
        registros_por_pagina: paginacion.registros_por_pagina,
    };

    // Devolver el resultado paginado
    Ok(ResultadoPaginado {
        datos: estudiantes,
        paginacion: paginacion_info,
    })
}

#[tauri::command]
async fn verificar_cedula_duplicada(
    db: &tokio_postgres::Client,
    cedula: i64,
    id_excluir: Option<i32>,
) -> Result<bool, String> {
    let query = match id_excluir {
        Some(_) => "SELECT COUNT(*) FROM estudiantes WHERE cedula = $1 AND id != $2",
        None => "SELECT COUNT(*) FROM estudiantes WHERE cedula = $1",
    };
    let row = match id_excluir {
        Some(id) => {
            db.query_one(query, &[&cedula, &id]).await
        },
        None => {
            db.query_one(query, &[&cedula]).await
        }
    }.map_err(|e| e.to_string())?;
    let count: i64 = row.get(0);
    Ok(count > 0)
}

#[tauri::command]
async fn crear_estudiante(
    estudiante: NuevoEstudiante,
    state: State<'_, AppState>,
) -> Result<(), String> {
    println!("[DEBUG] Datos recibidos en crear_estudiante: {:#?}", estudiante);
    let mut db = state.db.lock().await;
    let cedula_duplicada = verificar_cedula_duplicada(&*db, estudiante.cedula, None).await?;
    if cedula_duplicada {
        return Err("Ya existe un estudiante con esta cédula".to_string());
    }
    match db.execute(
        "INSERT INTO estudiantes (cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, municipionac, paisnac, entidadfed, ciudadnac, estadonac, id_grado, id_seccion, id_modalidad, id_periodoactual, estado, fecha_retiro) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)",
        &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.municipionac, &estudiante.paisnac, &estudiante.entidadfed, &estudiante.ciudadnac, &estudiante.estadonac, &estudiante.id_grado, &estudiante.id_seccion, &estudiante.id_modalidad, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro]
    ).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("[ERROR] Error al insertar estudiante: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn actualizar_estudiante(
    id: i32,
    estudiante: NuevoEstudiante,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut db = state.db.lock().await;
    let cedula_duplicada = verificar_cedula_duplicada(&*db, estudiante.cedula, Some(id)).await?;
    if cedula_duplicada {
        return Err("Ya existe otro estudiante con esta cédula".to_string());
    }
    db.execute(
        "UPDATE estudiantes SET cedula=$1, nombres=$2, apellidos=$3, genero=$4, fecha_nacimiento=$5, id_grado_secciones=$6, fecha_ingreso=$7, municipionac=$8, paisnac=$9, entidadfed=$10, ciudadnac=$11, estadonac=$12, id_grado=$13, id_seccion=$14, id_modalidad=$15, id_periodoactual=$16, estado=$17, fecha_retiro=$18 WHERE id=$19",
        &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.municipionac, &estudiante.paisnac, &estudiante.entidadfed, &estudiante.ciudadnac, &estudiante.estadonac, &estudiante.id_grado, &estudiante.id_seccion, &estudiante.id_modalidad, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro, &id]
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn eliminar_estudiante(
    id: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut db = state.db.lock().await;
    db.execute(
        "DELETE FROM estudiantes WHERE id = $1",
        &[&id],
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn contar_estudiantes(state: State<'_, AppState>) -> Result<i64, String> {
    let mut db = state.db.lock().await;
    let row = db
        .query_one("SELECT COUNT(*) FROM estudiantes", &[])
        .await
        .map_err(|e| e.to_string())?;
    let total: i64 = row.get(0);
    Ok(total)
}

#[tauri::command]
async fn listar_periodos_escolares(state: State<'_, AppState>) -> Result<Vec<PeriodoEscolar>, String> {
    let mut db = state.db.lock().await;
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
async fn listar_grados(state: State<'_, AppState>) -> Result<Vec<Grado>, String> {
    let mut db = state.db.lock().await;
    let query = "SELECT id_grado, nombre_grado FROM grados ORDER BY id_grado ASC";
    let rows = db.query(query, &[]).await.map_err(|e| e.to_string())?;
    let grados = rows.iter().map(|row| Grado {
        id_grado: row.get("id_grado"),
        nombre_grado: row.get("nombre_grado"),
    }).collect();
    Ok(grados)
}

#[tauri::command]
async fn listar_modalidades(state: State<'_, AppState>) -> Result<Vec<Modalidad>, String> {
    let mut db = state.db.lock().await;
    let query = "SELECT id_modalidad, nombre_modalidad FROM modalidades ORDER BY id_modalidad ASC";
    let rows = db.query(query, &[]).await.map_err(|e| e.to_string())?;
    let modalidades = rows.iter().map(|row| Modalidad {
        id_modalidad: row.get("id_modalidad"),
        nombre_modalidad: row.get("nombre_modalidad"),
    }).collect();
    Ok(modalidades)
}

// Nueva función para obtener todas las cédulas existentes
async fn obtener_cedulas_existentes(db: &tokio_postgres::Client) -> Result<HashSet<i64>, String> {
    let rows = db.query("SELECT cedula FROM estudiantes", &[]).await.map_err(|e| e.to_string())?;
    Ok(rows.iter().map(|row| row.get::<_, i64>(0)).collect())
}

#[tauri::command]
async fn insertar_estudiantes_masivo(
    estudiantes: Vec<serde_json::Value>,
    state: State<'_, AppState>,
) -> Result<ResumenInsercion, String> {
    println!("[DEBUG] insertando estudiantes masivo, recibidos: {}", estudiantes.len());
    for (i, est) in estudiantes.iter().enumerate() {
        println!("[DEBUG] Estudiante {}: {:?}", i, est);
    }
    let mut db = state.db.lock().await;
    let mut cedulas_existentes = obtener_cedulas_existentes(&*db).await?;
    let trans = db.transaction().await.map_err(|e| {
        println!("[ERROR] Error al iniciar transacción: {}", e);
        e.to_string()
    })?;
    let mut resumen = ResumenInsercion {
        total_registros: estudiantes.len(),
        insertados: 0,
        duplicados: 0,
        errores: Vec::new(),
    };
    for (index, est_json) in estudiantes.iter().enumerate() {
        println!("[DEBUG] Procesando estudiante {}: {:?}", index + 1, est_json);
        let estudiante: NuevoEstudiante = match serde_json::from_value(est_json.clone()) {
            Ok(est) => est,
            Err(e) => {
                let msg = format!("Fila {}: Error al deserializar - {}", index + 1, e);
                println!("[ERROR] {}", msg);
                resumen.errores.push(msg);
                continue;
            }
        };
        println!("[DEBUG] Estudiante deserializado: {:?}", estudiante);
        if cedulas_existentes.contains(&estudiante.cedula) {
            let msg = format!("Fila {}: Cédula duplicada {}", index + 1, estudiante.cedula);
            println!("[WARN] {}", msg);
            resumen.duplicados += 1;
            resumen.errores.push(msg);
            continue;
        }
        println!("[DEBUG] Insertando estudiante en la base de datos...");
        match trans.execute(
            "INSERT INTO estudiantes (cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, municipionac, paisnac, entidadfed, ciudadnac, estadonac, id_grado, id_seccion, id_modalidad, id_periodoactual, estado, fecha_retiro) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)",
            &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.municipionac, &estudiante.paisnac, &estudiante.entidadfed, &estudiante.ciudadnac, &estudiante.estadonac, &estudiante.id_grado, &estudiante.id_seccion, &estudiante.id_modalidad, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro]
        ).await {
            Ok(_) => {
                println!("[DEBUG] Insertado estudiante {}: cedula={}", index + 1, estudiante.cedula);
                resumen.insertados += 1;
                cedulas_existentes.insert(estudiante.cedula);
            },
            Err(e) => {
                let msg = format!("Fila {}: Error al insertar - {}", index + 1, e);
                println!("[ERROR] {}", msg);
                resumen.errores.push(msg);
            },
        }
    }
    if resumen.insertados == 0 {
        println!("[WARN] No se insertó ningún registro, haciendo rollback");
        trans.rollback().await.map_err(|e| {
            println!("[ERROR] Error al hacer rollback: {}", e);
            e.to_string()
        })?;
    } else {
        println!("[DEBUG] Commit de la transacción");
        trans.commit().await.map_err(|e| {
            println!("[ERROR] Error al hacer commit: {}", e);
            e.to_string()
        })?;
    }
    println!("[DEBUG] Resumen: insertados={}, duplicados={}, errores={}", resumen.insertados, resumen.duplicados, resumen.errores.len());
    Ok(resumen)
}

#[tauri::command]
async fn obtener_estudiante_por_id(
    id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<Estudiante, String> {
    let mut db = state.db.lock().await;
    let row = db
        .query_one(
            "SELECT e.id, e.cedula, e.nombres, e.apellidos, e.genero, e.fecha_nacimiento, \
            e.id_grado_secciones, e.fecha_ingreso, e.municipionac, e.paisnac, e.entidadfed, \
            e.ciudadnac, e.estadonac, e.id_grado, g.nombre_grado, e.id_seccion, s.nombre_seccion, \
            e.id_modalidad, m.nombre_modalidad, e.id_periodoactual, e.estado, e.fecha_retiro \
            FROM estudiantes e \
            LEFT JOIN grados g ON e.id_grado = g.id_grado \
            LEFT JOIN secciones s ON e.id_seccion = s.id_seccion \
            LEFT JOIN modalidades m ON e.id_modalidad = m.id_modalidad \
            WHERE e.id = $1",
            &[&id],
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(Estudiante {
        id: row.get(0),
        cedula: row.get(1),
        nombres: row.get(2),
        apellidos: row.get(3),
        genero: row.get(4),
        fecha_nacimiento: row.get(5),
        id_grado_secciones: row.get(6),
        fecha_ingreso: row.get(7),
        municipionac: row.get(8),
        paisnac: row.get(9),
        entidadfed: row.get(10),
        ciudadnac: row.get(11),
        estadonac: row.get(12),
        id_grado: row.get(13),
        nombre_grado: row.get(14),
        id_seccion: row.get(15),
        nombre_seccion: row.get(16),
        id_modalidad: row.get(17),
        nombre_modalidad: row.get(18),
        id_periodoactual: row.get(19),
        estado: row.get(20),
        fecha_retiro: row.get(21),
    })
}

#[tauri::command]
async fn guardar_calificacion(
    calificacion: CalificacionInput,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Validar ajustes
    if let (Some(l1), Some(a1)) = (calificacion.lapso_1, calificacion.lapso_1_ajustado) {
        if a1 < l1 { return Err("El ajuste del 1er lapso no puede ser menor que la nota original".into()); }
        if a1 > l1 + 2 { return Err("El ajuste del 1er lapso no puede ser mayor que la nota original +2".into()); }
    }
    if let (Some(l2), Some(a2)) = (calificacion.lapso_2, calificacion.lapso_2_ajustado) {
        if a2 < l2 { return Err("El ajuste del 2do lapso no puede ser menor que la nota original".into()); }
        if a2 > l2 + 2 { return Err("El ajuste del 2do lapso no puede ser mayor que la nota original +2".into()); }
    }
    if let (Some(l3), Some(a3)) = (calificacion.lapso_3, calificacion.lapso_3_ajustado) {
        if a3 < l3 { return Err("El ajuste del 3er lapso no puede ser menor que la nota original".into()); }
        if a3 > l3 + 2 { return Err("El ajuste del 3er lapso no puede ser mayor que la nota original +2".into()); }
    }

    // Calcular nota final usando ajustes si existen
    let l1 = calificacion.lapso_1_ajustado.or(calificacion.lapso_1).unwrap_or(0);
    let l2 = calificacion.lapso_2_ajustado.or(calificacion.lapso_2).unwrap_or(0);
    let l3 = calificacion.lapso_3_ajustado.or(calificacion.lapso_3).unwrap_or(0);
    let nota_final = ((l1 + l2 + l3) as f32 / 3.0).round() as i32;

    let mut db = state.db.lock().await;
    if let Some(id_calificacion) = calificacion.id_calificacion {
        // UPDATE
        db.execute(
            "UPDATE calificaciones SET lapso_1=$1, lapso_1_ajustado=$2, lapso_2=$3, lapso_2_ajustado=$4, lapso_3=$5, lapso_3_ajustado=$6, revision=$7, nota_final=$8 WHERE id_calificacion=$9",
            &[&calificacion.lapso_1, &calificacion.lapso_1_ajustado, &calificacion.lapso_2, &calificacion.lapso_2_ajustado, &calificacion.lapso_3, &calificacion.lapso_3_ajustado, &calificacion.revision, &nota_final, &id_calificacion]
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
            &[&calificacion.id_estudiante, &calificacion.id_asignatura, &calificacion.id_periodo, &calificacion.lapso_1, &calificacion.lapso_1_ajustado, &calificacion.lapso_2, &calificacion.lapso_2_ajustado, &calificacion.lapso_3, &calificacion.lapso_3_ajustado, &calificacion.revision, &nota_final]
        ).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn obtener_asignaturas_por_grado_modalidad(
    id_grado: i32,
    id_modalidad: i32,
    state: State<'_, AppState>,
) -> Result<Vec<Asignatura>, String> {
    println!("[DEBUG] >>>>> FUNCION obtener_asignaturas_por_grado_modalidad llamada");
    println!("[DEBUG] Parámetros recibidos: id_grado={}, id_modalidad={}", id_grado, id_modalidad);
    let mut db = state.db.lock().await;
    let query = "
        SELECT a.id_asignatura, a.nombre, gma.id_grado, gma.id_modalidad
        FROM asignaturas a
        INNER JOIN grado_modalidad_asignaturas gma ON a.id_asignatura = gma.id_asignatura
        WHERE gma.id_grado = $1 AND gma.id_modalidad = $2
        ORDER BY gma.orden";
    let rows = db.query(query, &[&id_grado, &id_modalidad])
        .await
        .map_err(|e| e.to_string())?;
    println!("[DEBUG] Filas retornadas por la consulta: {}", rows.len());
    let asignaturas = rows.iter().map(|row| Asignatura {
        id_asignatura: row.get(0),
        nombre_asignatura: row.get(1),
        id_grado: row.get(2),
        id_modalidad: row.get(3),
    }).collect();
    Ok(asignaturas)
}

#[tauri::command]
async fn obtener_calificaciones_estudiante(
    id_estudiante: i64,
    id_periodo: i32,
    state: State<'_, AppState>,
) -> Result<Vec<CalificacionEstudiante>, String> {
    println!("[DEBUG][BACKEND] >>>>> FUNCION obtener_calificaciones_estudiante llamada");
    println!("[DEBUG][BACKEND] Parámetros recibidos: id_estudiante={}, id_periodo={}", id_estudiante, id_periodo);
    let mut db = state.db.lock().await;
    let estudiante = db.query_one(
        "SELECT id_grado, id_modalidad FROM estudiantes WHERE id = $1",
        &[&((id_estudiante) as i32)]
    ).await.map_err(|e| format!("Error al verificar estudiante: {}", e))?;
    let id_grado: i32 = estudiante.get(0);
    let id_modalidad: i32 = estudiante.get(1);
    println!("[DEBUG] Estudiante encontrado: id_grado={}, id_modalidad={}", id_grado, id_modalidad);
    
    // Obtenemos las asignaturas y calificaciones en una sola consulta
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
            c.nota_final,
            c.revision
        FROM asignaturas_estudiante ae
        LEFT JOIN calificaciones c 
            ON ae.id_asignatura = c.id_asignatura 
            AND c.id_estudiante = $3 
            AND c.id_periodo = $4
        ORDER BY ae.nombre";
    
    println!("[DEBUG] Ejecutando consulta SQL");
    let rows = db.query(query, &[&id_grado, &id_modalidad, &id_estudiante, &id_periodo])
        .await
        .map_err(|e| format!("Error al obtener calificaciones: {}", e))?;
    
    println!("[DEBUG] Filas obtenidas: {}", rows.len());
    
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
            nota_final: row.get(9),
            revision: row.get(10),
        };
        println!("[DEBUG] Calificación procesada: {:?}", cal);
        cal
    }).collect();
    
    println!("[DEBUG] Calificaciones procesadas exitosamente");
    Ok(calificaciones)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando Tauri...");
    
    println!("Conectando a la base de datos...");
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=Salbertosis password=salbertosis13497674 dbname=EduNet port=5432",
        NoTls,
    )
    .await?;
    println!("Conexión a la base de datos establecida");

    println!("Iniciando conexión en segundo plano...");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("Conexión en segundo plano iniciada");

    let app_state = AppState {
        db: Arc::new(Mutex::new(client)),
    };
    println!("Estado de la aplicación creado");

    println!("Configurando Tauri...");
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            obtener_estudiantes,
            crear_estudiante,
            actualizar_estudiante,
            eliminar_estudiante,
            contar_estudiantes,
            listar_periodos_escolares,
            listar_grados,
            listar_modalidades,
            insertar_estudiantes_masivo,
            obtener_estudiante_por_id,
            guardar_calificacion,
            obtener_asignaturas_por_grado_modalidad,
            obtener_calificaciones_estudiante
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("Tauri iniciado correctamente.");
    Ok(())
}