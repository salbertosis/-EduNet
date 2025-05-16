// main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::NaiveDate;
use postgres_types::{ToSql, FromSql};
use serde_json;
use std::collections::HashSet;
// Eliminada importación no usada: bytes::BytesMut

mod config;
mod db;
mod utils;
mod models;
mod api;

use crate::models::estudiante::{Estudiante, EstadoEstudiante, FiltroEstudiantes, NuevoEstudiante};
use crate::api::estudiantes::{obtener_estudiantes, crear_estudiante, actualizar_estudiante, eliminar_estudiante, contar_estudiantes, insertar_estudiantes_masivo, obtener_estudiante_por_id};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistorialAcademico {
    pub id_historial: i32,
    pub id_estudiante: i32,
    pub id_periodo: i32,
    pub id_grado_secciones: i32,
    pub promedio_anual: f64,
    pub estatus: String,
    pub fecha_registro: chrono::NaiveDateTime,
    pub periodo_escolar: Option<String>,
    pub grado: Option<String>,
    pub seccion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AsignaturaPendiente {
    pub id_pendiente: i32,
    pub id_estudiante: i32,
    pub id_asignatura: i32,
    pub id_periodo: i32,
    pub grado: String,
    pub cal_momento1: Option<i32>,
    pub estado: String,
    pub fecha_registro: chrono::NaiveDateTime,
    pub id_grado_secciones: i32,
    pub nombre_asignatura: Option<String>,
    pub periodo_escolar: Option<String>,
}

pub struct AppState {
    pub db: Arc<Mutex<tokio_postgres::Client>>,
}

enum ParamValue {
    String(String),
    I32(i32),
}

impl ParamValue {
    fn as_tosql(&self) -> &(dyn ToSql + Sync) {
        match self {
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

#[derive(Debug, Deserialize)]
pub struct AsignaturaPendienteInput {
    pub id_asignatura: i32,
    pub id_periodo: i32,
    pub revision: Option<i32>,
}

#[tauri::command]
async fn listar_periodos_escolares(state: State<'_, AppState>) -> Result<Vec<PeriodoEscolar>, String> {
    let db = state.db.lock().await;
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
    let db = state.db.lock().await;
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
    let db = state.db.lock().await;
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

    let db = state.db.lock().await;
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
    let db = state.db.lock().await;
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
    let db = state.db.lock().await;
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

#[tauri::command]
async fn obtener_historial_academico_estudiante(
    id_estudiante: Option<i32>,
    id_estudiante_alt: Option<i32>,
    id_estudiante_alias: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<HistorialAcademico>, String> {
    let id = id_estudiante
        .or(id_estudiante_alt)
        .or(id_estudiante_alias)
        .ok_or("Falta el parámetro id_estudiante/idEstudiante")?;
    println!("[DEBUG][BACKEND] >>>>> FUNCION obtener_historial_academico_estudiante llamada");
    println!("[DEBUG][BACKEND] Parámetro recibido: id={}", id);
    let db = state.db.lock().await;
    let query = "
        SELECT 
            ha.id_historial,
            ha.id_estudiante,
            ha.id_periodo,
            ha.id_grado_secciones,
            ha.promedio_anual::float8,
            ha.estatus,
            ha.fecha_registro,
            pe.periodo_escolar,
            g.nombre_grado,
            s.nombre_seccion
        FROM historial_academico ha
        LEFT JOIN periodos_escolares pe ON ha.id_periodo = pe.id_periodo
        LEFT JOIN grado_secciones gs ON ha.id_grado_secciones = gs.id_grado_secciones
        LEFT JOIN grados g ON gs.id_grado = g.id_grado
        LEFT JOIN secciones s ON gs.id_seccion = s.id_seccion
        WHERE ha.id_estudiante = $1
        ORDER BY ha.fecha_registro DESC
    ";
    let rows = db.query(query, &[&id]).await.map_err(|e| e.to_string())?;
    let historial = rows.iter().map(|row| HistorialAcademico {
        id_historial: row.get(0),
        id_estudiante: row.get(1),
        id_periodo: row.get(2),
        id_grado_secciones: row.get(3),
        promedio_anual: row.get(4),
        estatus: row.get(5),
        fecha_registro: row.get(6),
        periodo_escolar: row.get(7),
        grado: row.get(8),
        seccion: row.get(9),
    }).collect();
    Ok(historial)
}

#[tauri::command]
async fn upsert_historial_academico(
    id_estudiante: Option<i32>,
    id_estudiante_alt: Option<i32>,
    id_periodo: Option<i32>,
    id_periodo_alt: Option<i32>,
    id_grado_secciones: Option<i32>,
    id_grado_secciones_alt: Option<i32>,
    promedio_anual: Option<f64>,
    promedio_anual_alt: Option<f64>,
    estatus: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let id = id_estudiante.or(id_estudiante_alt).ok_or("Falta el parámetro id_estudiante/idEstudiante")?;
    let periodo = id_periodo.or(id_periodo_alt).ok_or("Falta el parámetro id_periodo/idPeriodo")?;
    let grado_secciones = id_grado_secciones.or(id_grado_secciones_alt).ok_or("Falta el parámetro id_grado_secciones/idGradoSecciones")?;
    let promedio = promedio_anual.or(promedio_anual_alt).ok_or("Falta el parámetro promedio_anual/promedioAnual")?;
    println!("[DEBUG][BACKEND] >>>>> FUNCION upsert_historial_academico llamada");
    println!("[DEBUG][BACKEND] Parámetros recibidos: id_estudiante={}, id_periodo={}, id_grado_secciones={}, promedio_anual={}, estatus={}", 
        id, periodo, grado_secciones, promedio, estatus);
    let db = state.db.lock().await;
    // Verificar si ya existe un historial para este estudiante y período
    let row = db.query_opt(
        "SELECT id_historial FROM historial_academico WHERE id_estudiante = $1 AND id_periodo = $2",
        &[&id, &periodo]
    ).await.map_err(|e| format!("Error al verificar historial existente: {}", e))?;
    if let Some(row) = row {
        // UPDATE
        let id_historial: i32 = row.get(0);
        println!("[DEBUG][BACKEND] Actualizando historial existente id={}", id_historial);
        db.execute(
            "UPDATE historial_academico SET id_grado_secciones=$1, promedio_anual=$2::float8, estatus=$3 WHERE id_historial=$4",
            &[&grado_secciones, &promedio, &estatus, &id_historial]
        ).await.map_err(|e| format!("Error al actualizar historial: {}", e))?;
    } else {
        // INSERT
        println!("[DEBUG][BACKEND] Insertando nuevo historial");
        db.execute(
            "INSERT INTO historial_academico (id_estudiante, id_periodo, id_grado_secciones, promedio_anual, estatus) VALUES ($1,$2,$3,$4::float8,$5)",
            &[&id, &periodo, &grado_secciones, &promedio, &estatus]
        ).await.map_err(|e| format!("Error al insertar historial: {}", e))?;
    }
    println!("[DEBUG][BACKEND] Operación completada exitosamente");
    Ok(())
}

#[tauri::command]
async fn obtener_asignaturas_pendientes_estudiante(
    id_estudiante: Option<i32>,
    id_estudiante_alt: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<AsignaturaPendiente>, String> {
    let id = id_estudiante.or(id_estudiante_alt).ok_or("Falta el parámetro id_estudiante/idEstudiante")?;
    println!("[DEBUG][BACKEND] obtener_asignaturas_pendientes_estudiante: id={}", id);
    let db = state.db.lock().await;
    let query = r#"
        SELECT ap.id_pendiente, ap.id_estudiante, ap.id_asignatura, ap.id_periodo, ap.grado, ap.cal_momento1, ap.estado, ap.fecha_registro, ap.id_grado_secciones,
               a.nombre AS nombre_asignatura, p.periodo_escolar
        FROM asignaturas_pendientes ap
        LEFT JOIN asignaturas a ON ap.id_asignatura = a.id_asignatura
        LEFT JOIN periodos_escolares p ON ap.id_periodo = p.id_periodo
        WHERE ap.id_estudiante = $1
        ORDER BY ap.id_periodo DESC, ap.grado, a.nombre
    "#;
    let rows = db.query(query, &[&id])
        .await
        .map_err(|e| format!("Error al consultar asignaturas pendientes: {}", e))?;
    let pendientes = rows.iter().map(|row| AsignaturaPendiente {
        id_pendiente: row.get(0),
        id_estudiante: row.get(1),
        id_asignatura: row.get(2),
        id_periodo: row.get(3),
        grado: row.get(4),
        cal_momento1: row.get(5),
        estado: row.get(6),
        fecha_registro: row.get(7),
        id_grado_secciones: row.get(8),
        nombre_asignatura: row.get(9),
        periodo_escolar: row.get(10),
    }).collect::<Vec<_>>();
    println!("[DEBUG][BACKEND] Filas encontradas: {}", pendientes.len());
    Ok(pendientes)
}

#[tauri::command]
async fn guardar_asignaturas_pendientes(
    id_estudiante: i32,
    pendientes: Vec<AsignaturaPendienteInput>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    if pendientes.is_empty() {
        return Err("No hay asignaturas pendientes para guardar.".to_string());
    }
    if pendientes.len() > 2 {
        return Err("No se pueden guardar más de 2 asignaturas pendientes.".to_string());
    }
    let db = state.db.lock().await;
    // Obtener grado y sección del estudiante
    let row = db.query_one(
        "SELECT id_grado_secciones FROM estudiantes WHERE id = $1",
        &[&id_estudiante],
    ).await.map_err(|e| format!("Error al obtener grado/sección: {}", e))?;
    let id_grado_secciones: i32 = row.get(0);
    // Obtener nombre del grado
    let row_grado = db.query_one(
        "SELECT g.nombre_grado FROM grado_secciones gs INNER JOIN grados g ON gs.id_grado = g.id_grado WHERE gs.id_grado_secciones = $1",
        &[&id_grado_secciones],
    ).await.map_err(|e| format!("Error al obtener nombre de grado: {}", e))?;
    let nombre_grado: String = row_grado.get(0);
    for pendiente in pendientes {
        // UPSERT en asignaturas_pendientes
        db.execute(
            "INSERT INTO asignaturas_pendientes (id_estudiante, id_asignatura, id_periodo, grado, cal_momento1, estado, id_grado_secciones) \
            VALUES ($1, $2, $3, $4, $5, 'Pendiente', $6) \
            ON CONFLICT (id_estudiante, id_asignatura, id_periodo) \
            DO UPDATE SET cal_momento1 = EXCLUDED.cal_momento1, estado = 'Pendiente', grado = EXCLUDED.grado, id_grado_secciones = EXCLUDED.id_grado_secciones",
            &[&id_estudiante, &pendiente.id_asignatura, &pendiente.id_periodo, &nombre_grado, &pendiente.revision, &id_grado_secciones]
        ).await.map_err(|e| format!("Error al guardar asignatura pendiente: {}", e))?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando EduNet...");
    
    // Inicializar la base de datos
    let db_pool = db::connection::init_db().await?;
    let app_state = AppState { db: db_pool };
    
    // Configurar Tauri
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            obtener_estudiantes,
            crear_estudiante,
            actualizar_estudiante,
            eliminar_estudiante,
            contar_estudiantes,
            insertar_estudiantes_masivo,
            obtener_estudiante_por_id,
            listar_periodos_escolares,
            listar_grados,
            listar_modalidades,
            guardar_calificacion,
            obtener_asignaturas_por_grado_modalidad,
            obtener_calificaciones_estudiante,
            obtener_historial_academico_estudiante,
            upsert_historial_academico,
            obtener_asignaturas_pendientes_estudiante,
            guardar_asignaturas_pendientes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}