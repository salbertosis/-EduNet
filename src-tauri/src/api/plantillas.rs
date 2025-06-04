use crate::AppState;
use tauri::State;
use std::fs;
use std::path::Path;
use chrono::Local;
use serde::{Deserialize, Serialize};
use rust_xlsxwriter::{Workbook, Format, XlsxError};
use crate::models::catalogo::{SeccionCompleta, SeccionCatalogo};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct EstudiantePlantilla {
    id: i32,
    cedula: i64,
    nombre: String,
    apellido: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodoEscolar {
    pub id_periodo: i32,
    pub periodo_escolar: String,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modalidad {
    pub id_modalidad: i32,
    pub nombre_modalidad: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grado {
    pub id_grado: i32,
    pub nombre_grado: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asignatura {
    pub id_asignatura: i32,
    pub nombre_asignatura: String,
}

#[derive(serde::Serialize)]
pub struct SeccionTemp {
    pub id_seccion: i32,
    pub nombre_seccion: String,
    pub id_grado_secciones: i32,
}

#[tauri::command]
pub async fn generar_plantilla_acta(
    state: State<'_, AppState>,
    id_grado_secciones: i32,
    id_periodo: i32,
    id_asignatura: i32,
    lapso: String,
) -> Result<String, String> {
    println!("=== INICIO GENERACIÓN PLANTILLA ACTA ===");
    println!("Parámetros recibidos: id_grado_secciones={}, id_periodo={}, id_asignatura={}, lapso={}", 
        id_grado_secciones, id_periodo, id_asignatura, lapso);

    let db = state.db.lock().await;
    println!("Ejecutando consulta SQL para obtener estudiantes...");
    let estudiantes = db.query(
        "SELECT e.id, e.cedula, e.nombres, e.apellidos FROM historial_grado_estudiantes h \
         JOIN estudiantes e ON h.id_estudiante = e.id \
         WHERE h.id_grado_secciones = $1 AND h.id_periodo = $2 AND h.estado = 'activo' AND h.es_actual = true \
         ORDER BY e.apellidos, e.nombres",
        &[&id_grado_secciones, &id_periodo],
    ).await.map_err(|e| {
        println!("Error en la consulta SQL: {}", e);
        e.to_string()
    })?;

    println!("Número de estudiantes encontrados: {}", estudiantes.len());
    if estudiantes.is_empty() {
        println!("ADVERTENCIA: No se encontraron estudiantes para los parámetros dados");
    }

    let estudiantes: Vec<EstudiantePlantilla> = estudiantes
        .iter()
        .map(|row| {
            let estudiante = EstudiantePlantilla {
                id: row.get::<_, i32>(0),
                cedula: row.get::<_, i64>(1),
                nombre: row.get::<_, String>(2),
                apellido: row.get::<_, String>(3),
            };
            println!("Estudiante encontrado: id={}, cedula={}, nombre={}, apellido={}", 
                estudiante.id, estudiante.cedula, estudiante.nombre, estudiante.apellido);
            estudiante
        })
        .collect();

    // Crear directorio de plantillas si no existe
    let plantillas_dir = Path::new("C:/plantillas");
    if !plantillas_dir.exists() {
        fs::create_dir_all(plantillas_dir).map_err(|e| e.to_string())?;
    }

    // Generar nombre del archivo
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!(
        "acta_{}_{}_{}_{}_{}.xlsx",
        id_grado_secciones, id_grado_secciones, id_asignatura, lapso, timestamp
    );
    let filepath = plantillas_dir.join(&filename);

    println!("Generando archivo Excel: {}", filename);

    // Crear archivo Excel
    let mut workbook = Workbook::new();
    
    // Crear hoja principal
    let mut worksheet = workbook.add_worksheet();

    // Escribir encabezados de información general
    worksheet.write_string(0, 0, "ID Asignatura:")
        .map_err(|e: XlsxError| e.to_string())?;
    worksheet.write_number(0, 1, id_asignatura as f64)
        .map_err(|e: XlsxError| e.to_string())?;
    worksheet.write_string(1, 0, "Lapso:")
        .map_err(|e: XlsxError| e.to_string())?;
    worksheet.write_string(1, 1, &lapso)
        .map_err(|e: XlsxError| e.to_string())?;

    // Dejar una fila vacía antes de la tabla
    let tabla_offset = 3u32;

    // Configurar anchos de columna
    worksheet.set_column_width(0, 15.0).map_err(|e: XlsxError| e.to_string())?; // id_estudiante
    worksheet.set_column_width(1, 15.0).map_err(|e: XlsxError| e.to_string())?; // id_asignatura
    worksheet.set_column_width(2, 15.0).map_err(|e: XlsxError| e.to_string())?; // id_periodo
    worksheet.set_column_width(3, 15.0).map_err(|e: XlsxError| e.to_string())?; // calificacion

    // Crear formato para encabezados
    let header_format = Format::new()
        .set_bold()
        .set_background_color("#00B050") // verde profesional
        .set_font_color("#FFFFFF");      // blanco

    // Escribir encabezados de la tabla para carga masiva
    let headers = ["id_estudiante", "id_asignatura", "id_periodo", "calificacion"];
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(tabla_offset, col as u16, *header, &header_format)
            .map_err(|e: XlsxError| e.to_string())?;
    }

    println!("Escribiendo datos de {} estudiantes en el Excel", estudiantes.len());

    // Escribir datos de estudiantes para carga masiva
    for (row, estudiante) in estudiantes.iter().enumerate() {
        let row = (row as u32) + tabla_offset + 1;
        worksheet.write_number(row, 0, estudiante.id as f64)
            .map_err(|e: XlsxError| e.to_string())?;
        worksheet.write_number(row, 1, id_asignatura as f64)
            .map_err(|e: XlsxError| e.to_string())?;
        worksheet.write_number(row, 2, id_periodo as f64)
            .map_err(|e: XlsxError| e.to_string())?;
        worksheet.write_string(row, 3, "") // calificacion vacío
            .map_err(|e: XlsxError| e.to_string())?;
    }

    // Crear hoja de metadatos
    let mut metadata = workbook.add_worksheet();
    metadata.set_name("METADATA").map_err(|e: XlsxError| e.to_string())?;
    
    let metadata_data = [
        ("Grado", id_grado_secciones.to_string()),
        ("Sección", id_grado_secciones.to_string()),
        ("ID Asignatura", id_asignatura.to_string()),
        ("Lapso", lapso),
        ("Fecha Generación", Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
    ];

    for (row, (key, value)) in metadata_data.iter().enumerate() {
        metadata.write_string(row as u32, 0, *key)
            .map_err(|e: XlsxError| e.to_string())?;
        metadata.write_string(row as u32, 1, value)
            .map_err(|e: XlsxError| e.to_string())?;
    }

    // Guardar el archivo
    println!("Guardando archivo Excel...");
    workbook.save(&filepath).map_err(|e: XlsxError| e.to_string())?;
    println!("Archivo Excel guardado exitosamente");

    println!("=== FIN GENERACIÓN PLANTILLA ACTA ===");
    println!("ANTES DE OK(filename)");
    Ok(filename)
}

#[tauri::command]
pub async fn obtener_grados(state: State<'_, AppState>) -> Result<Vec<crate::models::catalogo::Grado>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT id_grado, nombre_grado FROM grados ORDER BY id_grado",
        &[],
    ).await.map_err(|e| e.to_string())?;

    let grados = rows
        .iter()
        .map(|row| crate::models::catalogo::Grado {
            id_grado: row.get(0),
            nombre_grado: row.get(1),
        })
        .collect();

    Ok(grados)
}

#[tauri::command]
pub async fn obtener_secciones(
    state: State<'_, AppState>,
    id_grado: i32,
) -> Result<Vec<SeccionCatalogo>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT id_seccion, nombre_seccion FROM secciones ORDER BY id_seccion",
        &[],
    ).await.map_err(|e| e.to_string())?;

    let secciones = rows
        .iter()
        .map(|row| SeccionCatalogo {
            id_seccion: row.get::<usize, i32>(0),
            nombre_seccion: row.get::<usize, String>(1),
        })
        .collect();

    Ok(secciones)
}

#[tauri::command]
pub async fn obtener_asignaturas(
    state: State<'_, AppState>,
    id_grado: i32,
    id_modalidad: i32,
) -> Result<Vec<crate::models::catalogo::Asignatura>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT DISTINCT a.id_asignatura, a.nombre AS nombre_asignatura, gma.orden \
         FROM asignaturas a \
         JOIN grado_modalidad_asignaturas gma ON a.id_asignatura = gma.id_asignatura \
         WHERE gma.id_grado = $1 AND gma.id_modalidad = $2 \
         ORDER BY gma.orden",
        &[&id_grado, &id_modalidad],
    ).await.map_err(|e| e.to_string())?;

    let asignaturas = rows
        .iter()
        .map(|row| crate::models::catalogo::Asignatura {
            id_asignatura: row.get(0),
            nombre_asignatura: row.get(1),
            id_grado,
            id_modalidad,
        })
        .collect();

    Ok(asignaturas)
}

#[tauri::command]
pub async fn obtener_lapsos(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    Ok(vec!["1".to_string(), "2".to_string(), "3".to_string()])
}

#[tauri::command]
pub async fn obtener_periodos_escolares(state: State<'_, AppState>) -> Result<Vec<PeriodoEscolar>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT id_periodo, periodo_escolar, activo FROM periodos_escolares ORDER BY fecha_inicio DESC",
        &[],
    ).await.map_err(|e| e.to_string())?;
    let periodos = rows.iter().map(|row| PeriodoEscolar {
        id_periodo: row.get(0),
        periodo_escolar: row.get(1),
        activo: row.get(2),
    }).collect();
    Ok(periodos)
}

#[tauri::command]
pub async fn obtener_modalidades(state: State<'_, AppState>) -> Result<Vec<Modalidad>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT id_modalidad, nombre_modalidad FROM modalidades ORDER BY nombre_modalidad",
        &[],
    ).await.map_err(|e| e.to_string())?;
    let modalidades = rows.iter().map(|row| Modalidad {
        id_modalidad: row.get(0),
        nombre_modalidad: row.get(1),
    }).collect();
    Ok(modalidades)
}

#[tauri::command]
pub async fn obtener_grados_por_modalidad(
    state: State<'_, AppState>,
    id_modalidad: i32
) -> Result<Vec<Grado>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT DISTINCT g.id_grado, g.nombre_grado FROM grados g \
         JOIN grado_modalidad_asignaturas gma ON g.id_grado = gma.id_grado \
         WHERE gma.id_modalidad = $1 ORDER BY g.id_grado",
        &[&id_modalidad],
    ).await.map_err(|e| e.to_string())?;
    let grados = rows.iter().map(|row| Grado {
        id_grado: row.get(0),
        nombre_grado: row.get(1),
    }).collect();
    Ok(grados)
}

#[tauri::command]
pub async fn obtener_secciones_por_grado_modalidad_periodo(
    state: State<'_, AppState>,
    id_grado: i32,
    id_modalidad: i32,
    id_periodo: i32
) -> Result<Vec<SeccionCompleta>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT DISTINCT \
            gs.id_grado_secciones::integer AS id_grado_secciones, \
            s.id_seccion::integer AS id_seccion, \
            s.nombre_seccion::text AS nombre_seccion \
         FROM secciones s \
         JOIN grado_secciones gs ON s.id_seccion = gs.id_seccion \
         JOIN historial_grado_estudiantes h ON gs.id_grado_secciones = h.id_grado_secciones \
         WHERE gs.id_grado = $1 AND gs.id_modalidad = $2 AND h.id_periodo = $3 AND h.estado = 'activo' AND h.es_actual = true \
         ORDER BY s.id_seccion",
        &[&id_grado, &id_modalidad, &id_periodo],
    ).await.map_err(|e| e.to_string())?;

    let secciones = rows.iter().map(|row| SeccionCompleta {
        id_grado_secciones: row.get::<_, i32>("id_grado_secciones"),
        id_seccion: row.get::<_, i32>("id_seccion"),
        nombre_seccion: row.get::<_, String>("nombre_seccion"),
    }).collect();

    Ok(secciones)
}

#[tauri::command]
pub async fn test_query_minimal(state: State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT 1::integer, 'A'::text, 2::integer",
        &[],
    ).await.map_err(|e| e.to_string())?;

    let mut result = String::new();
    for (i, row) in rows.iter().enumerate() {
        result.push_str(&format!(
            "Fila {}: id_seccion={:?}, nombre_seccion={:?}, id_grado_secciones={:?}\n",
            i,
            row.try_get::<usize, i32>(0),
            row.try_get::<usize, String>(1),
            row.try_get::<usize, i32>(2)
        ));
    }
    Ok(result)
}

#[tauri::command]
pub async fn test_column_types(state: State<'_, AppState>, id_grado: i32, id_modalidad: i32, id_periodo: i32) -> Result<String, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT DISTINCT s.id_seccion::integer, s.nombre_seccion::text, gs.id_grado_secciones::integer FROM secciones s \
         JOIN grado_secciones gs ON s.id_seccion = gs.id_seccion \
         JOIN historial_grado_estudiantes h ON gs.id_grado_secciones = h.id_grado_secciones \
         WHERE gs.id_grado = $1 AND gs.id_modalidad = $2 AND h.id_periodo = $3 AND h.estado = 'activo' AND h.es_actual = true \
         ORDER BY s.id_seccion",
        &[&id_grado, &id_modalidad, &id_periodo],
    ).await.map_err(|e| e.to_string())?;

    let mut result = String::new();
    for (i, row) in rows.iter().enumerate() {
        let id_seccion = row.try_get::<_, i32>("id_seccion");
        let nombre_seccion = row.try_get::<_, String>("nombre_seccion");
        let id_grado_secciones = row.try_get::<_, i32>("id_grado_secciones");
        result.push_str(&format!(
            "Fila {}: id_seccion={:?}, nombre_seccion={:?}, id_grado_secciones={:?}\n",
            i, id_seccion, nombre_seccion, id_grado_secciones
        ));
    }
    Ok(result)
}

#[tauri::command]
pub async fn test_id_seccion_solo(
    state: State<'_, AppState>,
    id_grado: i32,
    id_modalidad: i32,
    id_periodo: i32
) -> Result<Vec<i32>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT s.id_seccion::integer AS id_seccion FROM secciones s \
         JOIN grado_secciones gs ON s.id_seccion = gs.id_seccion \
         JOIN historial_grado_estudiantes h ON gs.id_grado_secciones = h.id_grado_secciones \
         WHERE gs.id_grado = $1 AND gs.id_modalidad = $2 AND h.id_periodo = $3 AND h.estado = 'activo' AND h.es_actual = true \
         ORDER BY s.id_seccion",
        &[&id_grado, &id_modalidad, &id_periodo],
    ).await.map_err(|e| e.to_string())?;

    let ids: Vec<i32> = rows.iter().map(|row| row.get("id_seccion")).collect();
    println!("IDs: {:?}", ids);
    Ok(ids)
} 