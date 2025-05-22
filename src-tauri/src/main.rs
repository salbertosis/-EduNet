// main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::Mutex;

mod config;
mod db;
mod utils;
mod models;
mod api;

pub struct AppState {
    pub db: Arc<Mutex<tokio_postgres::Client>>,
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
            api::calificaciones::guardar_calificacion,
            api::calificaciones::obtener_calificaciones_estudiante,
            api::estudiantes::obtener_estudiantes,
            api::estudiantes::obtener_estudiante_por_id,
            api::estudiantes::crear_estudiante,
            api::estudiantes::actualizar_estudiante,
            api::estudiantes::eliminar_estudiante,
            api::estudiantes::contar_estudiantes,
            api::estudiantes::insertar_estudiantes_masivo,
            api::historial::obtener_historial_academico_estudiante,
            api::historial::upsert_historial_academico,
            api::pendiente::obtener_asignaturas_pendientes_estudiante,
            api::pendiente::guardar_asignaturas_pendientes,
            api::pendiente::obtener_calificaciones_pendiente,
            api::pendiente::upsert_calificaciones_pendiente,
            api::pendiente::eliminar_asignatura_pendiente,
            api::catalogo::listar_periodos_escolares,
            api::catalogo::listar_grados,
            api::catalogo::listar_modalidades,
            api::catalogo::obtener_asignaturas_por_grado_modalidad,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}