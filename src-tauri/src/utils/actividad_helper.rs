use crate::models::actividad_reciente::{ActividadReciente, NuevaActividad};
use crate::models::estudiante::Estudiante;
use crate::models::docente::Docente;
use tokio_postgres::Client;
use serde_json::json;

// Función helper para obtener nombres de asignación
async fn obtener_nombres_asignacion(
    db: &Client,
    id_grado_secciones: Option<i32>,
    id_asignatura: Option<i32>,
    id_periodo: Option<i32>,
) -> Result<(String, String, String), tokio_postgres::Error> {
    let mut nombre_grado_seccion = "N/A".to_string();
    let mut nombre_asignatura = "N/A".to_string();
    let mut nombre_periodo = "N/A".to_string();

    // Obtener nombre de grado/sección
    if let Some(id_gs) = id_grado_secciones {
        println!("[DEBUG] Buscando grado/sección con ID: {}", id_gs);
        if let Ok(row) = db.query_opt(
            "SELECT g.nombre_grado, s.nombre_seccion 
             FROM grado_secciones gs 
             JOIN grados g ON gs.id_grado = g.id_grado 
             JOIN secciones s ON gs.id_seccion = s.id_seccion 
             WHERE gs.id_grado_secciones = $1",
            &[&id_gs]
        ).await {
            if let Some(row) = row {
                let nombre_grado: String = row.get("nombre_grado");
                let nombre_seccion: String = row.get("nombre_seccion");
                nombre_grado_seccion = format!("{} Año - Sección {}", nombre_grado, nombre_seccion);
                println!("[DEBUG] Grado/sección encontrado: {}", nombre_grado_seccion);
            } else {
                println!("[DEBUG] No se encontró grado/sección con ID: {}", id_gs);
            }
        } else {
            println!("[DEBUG] Error al buscar grado/sección con ID: {}", id_gs);
        }
    }

    // Obtener nombre de asignatura
    if let Some(id_asig) = id_asignatura {
        println!("[DEBUG] Buscando asignatura con ID: {}", id_asig);
        if let Ok(row) = db.query_opt(
            "SELECT nombre FROM asignaturas WHERE id_asignatura = $1",
            &[&id_asig]
        ).await {
            if let Some(row) = row {
                nombre_asignatura = row.get("nombre");
                println!("[DEBUG] Asignatura encontrada: {}", nombre_asignatura);
            } else {
                println!("[DEBUG] No se encontró asignatura con ID: {}", id_asig);
            }
        } else {
            println!("[DEBUG] Error al buscar asignatura con ID: {}", id_asig);
        }
    }

    // Obtener nombre de período
    if let Some(id_per) = id_periodo {
        if let Ok(row) = db.query_opt(
            "SELECT periodo_escolar FROM periodos_escolares WHERE id_periodo = $1",
            &[&id_per]
        ).await {
            if let Some(row) = row {
                nombre_periodo = row.get("periodo_escolar");
            }
        }
    }

    Ok((nombre_grado_seccion, nombre_asignatura, nombre_periodo))
}

pub async fn registrar_actividad_estudiante(
    db: &Client,
    accion: &str,
    estudiante: &Estudiante,
    usuario: &str,
) -> Result<(), tokio_postgres::Error> {
    println!("[DEBUG] Iniciando registro de actividad para estudiante: {} {}", estudiante.nombres, estudiante.apellidos);
    
    let descripcion = match accion {
        "crear" => format!("Estudiante {} {} agregado al sistema", estudiante.nombres, estudiante.apellidos),
        "actualizar" => format!("Datos del estudiante {} {} modificados", estudiante.nombres, estudiante.apellidos),
        "eliminar" => format!("Estudiante {} {} eliminado del sistema", estudiante.nombres, estudiante.apellidos),
        _ => format!("Acción '{}' realizada en estudiante {} {}", accion, estudiante.nombres, estudiante.apellidos),
    };

    let metadata = json!({
        "accion": accion,
        "cedula": estudiante.cedula,
        "id_estudiante": estudiante.id,
        "grado_seccion": estudiante.id_grado_secciones
    });

    let nueva_actividad = NuevaActividad {
        tipo_actividad: "estudiante".to_string(),
        descripcion: descripcion.clone(),
        usuario: Some(usuario.to_string()),
        metadata: Some(metadata),
        id_estudiante: Some(estudiante.id),
        id_docente: None,
        id_periodo: None,
    };

    println!("[DEBUG] Intentando registrar actividad: {}", descripcion);
    match ActividadReciente::crear(db, nueva_actividad).await {
        Ok(actividad) => {
            println!("[DEBUG] Actividad registrada exitosamente con ID: {}", actividad.id);
            Ok(())
        },
        Err(e) => {
            println!("[ERROR] Error registrando actividad: {}", e);
            Err(e)
        }
    }
}

pub async fn registrar_actividad_docente(
    db: &Client,
    accion: &str,
    docente: &Docente,
    usuario: &str,
) -> Result<(), tokio_postgres::Error> {
    let descripcion = match accion {
        "crear" => format!("Docente {} {} agregado al sistema", docente.nombres, docente.apellidos),
        "actualizar" => format!("Datos del docente {} {} modificados", docente.nombres, docente.apellidos),
        "eliminar" => format!("Docente {} {} eliminado del sistema", docente.nombres, docente.apellidos),
        _ => format!("Acción '{}' realizada en docente {} {}", accion, docente.nombres, docente.apellidos),
    };

    let metadata = json!({
        "accion": accion,
        "cedula": docente.cedula,
        "id_docente": docente.id_docente,
        "especialidad": docente.especialidad
    });

    let nueva_actividad = NuevaActividad {
        tipo_actividad: "docente".to_string(),
        descripcion,
        usuario: Some(usuario.to_string()),
        metadata: Some(metadata),
        id_estudiante: None,
        id_docente: Some(docente.id_docente),
        id_periodo: None,
    };

    ActividadReciente::crear(db, nueva_actividad).await?;
    Ok(())
}

pub async fn registrar_actividad_asignacion(
    db: &Client,
    tipo_asignacion: &str,
    docente: &Docente,
    detalles: &str,
    usuario: &str,
    id_grado_secciones: Option<i32>,
    id_asignatura: Option<i32>,
    id_periodo: Option<i32>,
) -> Result<(), tokio_postgres::Error> {
    // Obtener nombres descriptivos
    let (nombre_grado_seccion, nombre_asignatura, nombre_periodo) = 
        obtener_nombres_asignacion(db, id_grado_secciones, id_asignatura, id_periodo).await?;

    let descripcion = match tipo_asignacion {
        "asignar_guia" => format!("Docente {} {} asignado como docente guía de {}", 
            docente.nombres, docente.apellidos, nombre_grado_seccion),
        "crear_asignacion" | "actualizar_asignacion" => format!("Docente {} {} asignado a {} en {} (Período {})", 
            docente.nombres, docente.apellidos, nombre_asignatura, nombre_grado_seccion, nombre_periodo),
        _ => format!("Docente {} {} asignado como {}", 
            docente.nombres, docente.apellidos, detalles),
    };

    let metadata = json!({
        "tipo_asignacion": tipo_asignacion,
        "id_docente": docente.id_docente,
        "detalles": detalles,
        "grado_seccion": nombre_grado_seccion,
        "asignatura": nombre_asignatura,
        "periodo": nombre_periodo
    });

    let nueva_actividad = NuevaActividad {
        tipo_actividad: "asignacion".to_string(),
        descripcion,
        usuario: Some(usuario.to_string()),
        metadata: Some(metadata),
        id_estudiante: None,
        id_docente: Some(docente.id_docente),
        id_periodo,
    };

    ActividadReciente::crear(db, nueva_actividad).await?;
    Ok(())
}

pub async fn registrar_actividad_calificaciones(
    db: &Client,
    accion: &str,
    cantidad: i32,
    detalles: &str,
    usuario: &str,
) -> Result<(), tokio_postgres::Error> {
    let descripcion = match accion {
        "carga_masiva" => format!("Carga masiva de calificaciones: {} registros procesados", cantidad),
        "individual" => format!("Calificación {} registrada", detalles),
        _ => format!("Acción '{}' en calificaciones: {}", accion, detalles),
    };

    let metadata = json!({
        "accion": accion,
        "cantidad": cantidad,
        "detalles": detalles
    });

    let nueva_actividad = NuevaActividad {
        tipo_actividad: "calificacion".to_string(),
        descripcion,
        usuario: Some(usuario.to_string()),
        metadata: Some(metadata),
        id_estudiante: None,
        id_docente: None,
        id_periodo: None,
    };

    ActividadReciente::crear(db, nueva_actividad).await?;
    Ok(())
}

pub async fn registrar_actividad_periodo(
    db: &Client,
    accion: &str,
    nombre_periodo: &str,
    id_periodo: i32,
    usuario: &str,
) -> Result<(), tokio_postgres::Error> {
    let descripcion = match accion {
        "crear" => format!("Nuevo período escolar '{}' creado", nombre_periodo),
        "activar" => format!("Período escolar '{}' activado", nombre_periodo),
        _ => format!("Acción '{}' en período '{}'", accion, nombre_periodo),
    };

    let metadata = json!({
        "accion": accion,
        "nombre_periodo": nombre_periodo,
        "id_periodo": id_periodo
    });

    let nueva_actividad = NuevaActividad {
        tipo_actividad: "periodo".to_string(),
        descripcion,
        usuario: Some(usuario.to_string()),
        metadata: Some(metadata),
        id_estudiante: None,
        id_docente: None,
        id_periodo: Some(id_periodo),
    };

    ActividadReciente::crear(db, nueva_actividad).await?;
    Ok(())
}

pub async fn registrar_actividad_general(
    db: &Client,
    tipo: &str,
    descripcion: &str,
    usuario: &str,
    metadata: Option<serde_json::Value>,
) -> Result<(), tokio_postgres::Error> {
    let nueva_actividad = NuevaActividad {
        tipo_actividad: tipo.to_string(),
        descripcion: descripcion.to_string(),
        usuario: Some(usuario.to_string()),
        metadata: metadata,
        id_estudiante: None,
        id_docente: None,
        id_periodo: None,
    };

    ActividadReciente::crear(db, nueva_actividad).await?;
    Ok(())
} 