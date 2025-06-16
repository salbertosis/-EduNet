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
            api::calificaciones::cargar_calificaciones_masivo,
            api::estudiantes::obtener_estudiantes,
            api::estudiantes::obtener_estudiante_por_id,
            api::estudiantes::crear_estudiante,
            api::estudiantes::actualizar_estudiante,
            api::estudiantes::eliminar_estudiante,
            api::estudiantes::contar_estudiantes,
            api::estudiantes::contar_estudiantes_femeninos,
            api::estudiantes::contar_estudiantes_masculinos,
            api::historial::obtener_historial_academico_estudiante,
            api::historial::upsert_historial_academico,
            api::historial::guardar_historial_masivo,
            api::pendiente::obtener_asignaturas_pendientes_estudiante,
            api::pendiente::guardar_asignaturas_pendientes,
            api::pendiente::obtener_calificaciones_pendiente,
            api::pendiente::upsert_calificaciones_pendiente,
            api::pendiente::eliminar_asignatura_pendiente,
            api::catalogo::listar_periodos_escolares,
            api::catalogo::listar_grados,
            api::catalogo::listar_secciones,
            api::catalogo::listar_modalidades,
            api::catalogo::obtener_asignaturas_por_grado_modalidad,
            api::catalogo::crear_periodo_escolar,
            api::catalogo::establecer_periodo_activo,
            api::catalogo::listar_paises,
            api::catalogo::listar_estados_por_pais,
            api::catalogo::listar_municipios_por_estado,
            api::catalogo::listar_ciudades_por_municipio,
            api::docente::obtener_docentes,
            api::docente::crear_docente,
            api::docente::actualizar_docente,
            api::docente::eliminar_docente,
            api::docente::insertar_docentes_masivo,
            api::docente::contar_docentes,
            api::plantillas::obtener_grados,
            api::plantillas::obtener_secciones,
            api::plantillas::obtener_asignaturas,
            api::plantillas::obtener_lapsos,
            api::plantillas::obtener_periodos_escolares,
            api::plantillas::obtener_modalidades,
            api::plantillas::obtener_grados_por_modalidad,
            api::plantillas::obtener_secciones_por_grado_modalidad_periodo,
            api::plantillas::test_query_minimal,
            api::plantillas::test_column_types,
            api::actas_masivas::generar_actas_masivas,
            api::plantillas::obtener_tarjetas_cursos,
            api::migracion::migrar_estudiantes,
            api::plantillas::obtener_secciones_anio_anterior,
            api::plantillas::obtener_grado_secciones_por_id,
            api::catalogo::listar_secciones_por_grado_modalidad,
            api::catalogo::obtener_id_grado_secciones,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}