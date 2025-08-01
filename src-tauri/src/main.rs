// main.rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _};
use std::fs;
use app::{AppState, db, api}; // Importar todo lo necesario desde la librería
use tauri::Manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando EduNet...");
    // Usar la función de inicialización de la librería
    let db_pool = db::connection::init_db().await?;

    // Cargar logos
    println!("Intentando cargar logo izquierdo: imagenes/Logo_ministerio_resumen.png");
    let logo_izq_base64 = match fs::read("imagenes/Logo_ministerio_resumen.png") {
        Ok(data) => {
            println!("✅ Logo izquierdo cargado exitosamente: {} bytes", data.len());
            let base64 = general_purpose::STANDARD.encode(&data);
            println!("✅ Logo izquierdo codificado en base64: {} caracteres", base64.len());
            base64
        },
        Err(e) => {
            println!("❌ Error cargando logo izquierdo: {}", e);
            String::new()
        }
    };
    
    println!("Intentando cargar logo derecho: imagenes/Logo_liceo.jpg");
    let logo_der_base64 = match fs::read("imagenes/Logo_liceo.jpg") {
        Ok(data) => {
            println!("✅ Logo derecho cargado exitosamente: {} bytes", data.len());
            let base64 = general_purpose::STANDARD.encode(&data);
            println!("✅ Logo derecho codificado en base64: {} caracteres", base64.len());
            base64
        },
        Err(e) => {
            println!("❌ Error cargando logo derecho: {}", e);
            String::new()
        }
    };
    
    // Configurar Tauri
    tauri::Builder::default()
        .setup(move |app| {
            // Crear la instancia de AppState con el app_handle
            let app_state = AppState { 
                db: db_pool,
                logo_izq: logo_izq_base64.clone(),
                logo_der: logo_der_base64.clone(),
                app_handle: app.handle(),
            };
            app.manage(app_state);
            Ok(())
        })
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
            api::estudiantes::obtener_estadisticas_estudiantes_por_modalidad,
            api::estudiantes::obtener_estadisticas_telematica,
            api::estudiantes::obtener_estadisticas_ciencias,
            api::estudiantes::obtener_estadisticas_cursos_por_grado_modalidad,
            api::estudiantes::obtener_estadisticas_cursos_telematica_por_grado,
            api::estudiantes::obtener_estadisticas_cursos_ciencias_por_grado,
            api::estudiantes::obtener_estadisticas_cursos_compacta,
            api::estudiantes::obtener_estadisticas_cursos_resumen,
            api::estudiantes::obtener_estadisticas_cursos_simple,
            api::estudiantes::debug_estadisticas_cursos,
            api::estudiantes::obtener_estadisticas_cursos_sin_filtros,
            api::estudiantes::probar_datos_estudiantes,
            api::estudiantes::probar_grado_secciones,
            api::estudiantes::debug_estadisticas_modalidad,
            api::estudiantes::insertar_estudiantes_masivo,
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
        api::catalogo::obtener_estadisticas_docentes,
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
            api::docente::asignar_docente_guia,
            api::docente::asignar_docente_asignatura,
            api::docente::obtener_docentes_por_asignatura_seccion,
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
            api::acta_resumen::generar_acta_resumen,
            api::plantillas::obtener_tarjetas_cursos,
            api::migracion::migrar_estudiantes,
            api::migracion::migrar_estudiantes_nuevos,
            api::plantillas::obtener_secciones_anio_anterior,
            api::plantillas::obtener_grado_secciones_por_id,
            api::plantillas::obtener_grado_secciones_por_filtros,
            api::catalogo::listar_secciones_por_grado_modalidad,
            api::catalogo::obtener_id_grado_secciones,
            api::pdf_estudiantes::generar_pdf_estudiantes_curso,
            // Comandos PGCRP Simple
            api::pgcrp_simple::obtener_actividades_pgcrp_simple,
            api::pgcrp_simple::asignar_pgcrp_seccion_simple,
            api::pgcrp_simple::obtener_pgcrp_seccion,
            api::pgcrp_simple::eliminar_pgcrp_seccion,
            api::pgcrp_simple::obtener_estudiantes_pgcrp_periodo,
            // Comandos PGCRP por estudiante
            api::estudiante_pgcrp::obtener_estudiantes_seccion_pgcrp,
            api::estudiante_pgcrp::obtener_actividades_pgcrp_estudiante,
            api::estudiante_pgcrp::asignar_pgcrp_estudiante_individual,
            api::estudiante_pgcrp::eliminar_pgcrp_estudiante_individual,
            // Comandos Resumen Final (sistema refactorizado)
            api::resumen_final_bridge::generar_resumen_final_html_directo_v2,
            api::resumen_final_bridge::generar_resumen_final_pdf_directo_v2,
            // Comandos de Institución
                    api::institucion::obtener_datos_institucion,
        api::institucion::guardar_datos_institucion,
        api::tipos_evaluacion::obtener_tipos_evaluacion,
        // Comandos de Actividad Reciente
        api::actividad_reciente::obtener_actividad_reciente,
        api::actividad_reciente::registrar_actividad,
        api::actividad_reciente::limpiar_actividad_antigua,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}