use crate::AppState;
use tauri::State;
use std::fs;
use std::path::Path;
use chrono::Local;
use serde::{Deserialize, Serialize};
use umya_spreadsheet::{reader, writer};
use umya_spreadsheet::helper::coordinate::CellCoordinates;
use std::collections::HashMap;
use crate::models::catalogo::{SeccionCatalogo, SeccionCompleta};
use regex;
use regex::Regex;
use umya_spreadsheet::CellValue;

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

    // === Obtener datos legibles para la hoja bonita ===
    // Obtener nombre de la asignatura
    let row_asig = db.query_one("SELECT nombre FROM asignaturas WHERE id_asignatura = $1", &[&id_asignatura]).await.map_err(|e| e.to_string())?;
    let nombre_asignatura: String = row_asig.get(0);
    // Obtener grado y sección legibles
    println!("DEBUG: id_grado_secciones recibido: {}", id_grado_secciones);
    let row_grado_seccion = db.query_one(
        "SELECT g.nombre_grado, s.nombre_seccion FROM grado_secciones gs \
         JOIN grados g ON gs.id_grado = g.id_grado \
         JOIN secciones s ON gs.id_seccion = s.id_seccion \
         WHERE gs.id_grado_secciones = $1",
        &[&id_grado_secciones],
    ).await.map_err(|e| e.to_string())?;
    let nombre_grado: String = row_grado_seccion.get(0);
    let nombre_seccion: String = row_grado_seccion.get(1);
    println!("DEBUG: nombre_grado obtenido: '{}', nombre_seccion obtenido: '{}'", nombre_grado, nombre_seccion);

    // Normaliza el nombre de la asignatura para buscar la sigla
    let nombre_asignatura_normalizado = if nombre_asignatura.trim().to_lowercase() == "lengua y literatura" {
        "castellano".to_string()
    } else {
        nombre_asignatura.trim().to_lowercase()
            .replace("á", "a").replace("é", "e").replace("í", "i").replace("ó", "o").replace("ú", "u")
            .replace("à", "a").replace("è", "e").replace("ì", "i").replace("ò", "o").replace("ù", "u")
            .replace(" ", "")
    };

    let siglas = HashMap::from([
        ("castellano".to_string(), "Ca"),
        ("ingles".to_string(), "In"),
        ("matematica".to_string(), "Ma"),
        ("educacionfisica".to_string(), "Ef"),
        ("arteypatrimonio".to_string(), "Ap"),
        ("bat".to_string(), "Ba"),
        ("ghc".to_string(), "Gh"),
        ("fisica".to_string(), "Fi"),
        ("quimica".to_string(), "Qu"),
        ("fsn".to_string(), "Fs"),
        ("cs.de.la.tierra".to_string(), "Cs"),
        ("programacion".to_string(), "Pr"),
        ("proyectodeeconomia".to_string(), "Pe"),
        ("gcrp".to_string(), "Gc"),
        ("orientacion".to_string(), "Or"),
    ]);
    let sigla = match siglas.get(&nombre_asignatura_normalizado) {
        Some(s) => s,
        None => {
            println!("[WARN] Asignatura no encontrada en siglas: '{}'. Normalizado: '{}'", nombre_asignatura, nombre_asignatura_normalizado);
            &"XX"
        }
    };

    // Lapso con símbolo
    let lapso_str = format!("{}º", lapso);

    // === Abrir plantilla bonita existente ===
    let plantilla_path = Path::new("C:/plantillas/acta_pantilla/acta_plantilla.xlsx");
    let mut book = reader::xlsx::read(plantilla_path).map_err(|e| e.to_string())?;
    let sheet = book.get_sheet_by_name_mut("ACTA").ok_or("No se encontró la hoja ACTA en la plantilla")?;

    // Corrige '1er' y '3er' a '1ero' y '3ero' en el grado seleccionado
    let mut grado = nombre_grado.trim().to_string();
    if grado == "1er" { grado = "1ero".to_string(); }
    if grado == "3er" { grado = "3ero".to_string(); }
    let seccion = nombre_seccion.trim();
    // Arma el texto para la celda D6 con el formato correcto
    let texto_d6 = format!("{} Año {}", nombre_grado.trim(), nombre_seccion.trim());
    sheet.get_cell_mut("D6").set_value(texto_d6);

    // Escribir el lapso en D7
    sheet.get_cell_mut("D7").set_value(lapso_str.clone());
    // Escribir el nombre de la asignatura en D8
    sheet.get_cell_mut("D8").set_value(nombre_asignatura.clone());

    // El nombre del acta y archivo, usando los valores seleccionados
    let nombre_acta = format!("{}{}_{}_{}", sigla, grado, seccion, lapso_str);
    sheet.get_cell_mut("D10").set_value(&nombre_acta);

    // Llenar estudiantes en la hoja ACTA usando 'sheet'
    for (i, est) in estudiantes.iter().enumerate() {
        let row = 13 + i;
        let cell_cedula = format!("B{}", row);
        let cell_nombre = format!("C{}", row);
        sheet.get_cell_mut(&*cell_cedula).set_value(est.cedula.to_string());
        sheet.get_cell_mut(&*cell_nombre).set_value(format!("{} {}", est.apellido, est.nombre));
    }

    // 5. En la celda A11 de la hoja ACTA, coloco la modalidad
    let row = db.query_one("SELECT id_modalidad FROM grado_secciones WHERE id_grado_secciones = $1", &[&id_grado_secciones]).await.map_err(|e| e.to_string())?;
    let id_modalidad: i32 = row.get(0);
    let row_modalidad = db.query_one("SELECT nombre_modalidad FROM modalidades WHERE id_modalidad = $1", &[&id_modalidad]).await.map_err(|e| e.to_string())?;
    let modalidad: String = row_modalidad.get(0);
    sheet.get_cell_mut("A11").set_value(&modalidad);

    // === Agregar hoja de carga masiva ===
    let nombre_lapso_campo = match lapso.as_str() {
        "1" | "1º" | "1er" | "1ero" => "lapso_1",
        "2" | "2º" | "2do" => "lapso_2",
        "3" | "3º" | "3er" | "3ero" => "lapso_3",
        _ => "calificacion",
    };

    // 2. Separar grado y sección para mostrar como '2do Año B', '3er Año A', etc.
    // Ejemplo: '2do AñoB' -> grado: '2do Año', seccion: 'B'
    let (nombre_grado_limpio, nombre_seccion_limpia) = {
        let s = nombre_grado.replace("Año", "Año ").replace("año", "año ");
        let s = s.trim();
        if let Some(idx) = s.rfind(char::is_alphabetic) {
            let (grado, seccion) = s.split_at(idx);
            (grado.trim().to_string(), seccion.trim().to_string())
        } else {
            (s.to_string(), String::new())
        }
    };

    // 3. Hoja CARGA_MASIVA visible (la ocultación no es soportada en esta versión de umya-spreadsheet)
    // carga.set_hidden(true); // No disponible en esta versión
    let carga = book.new_sheet("CARGA_MASIVA").map_err(|e| e.to_string())?;
    carga.get_cell_mut("A1").set_value("id_estudiante");
    carga.get_cell_mut("B1").set_value("id_asignatura");
    carga.get_cell_mut("C1").set_value("id_periodo");
    carga.get_cell_mut("D1").set_value(nombre_lapso_campo);
    for (i, est) in estudiantes.iter().enumerate() {
        let row = 2 + i;
        carga.get_cell_mut(&*format!("A{}", row)).set_value(est.id.to_string());
        carga.get_cell_mut(&*format!("B{}", row)).set_value(id_asignatura.to_string());
        carga.get_cell_mut(&*format!("C{}", row)).set_value(id_periodo.to_string());
        let celda_acta = format!("N{}", 13 + i); // N13, N14, ...
        let formula = format!("=ACTA!{}", celda_acta);
        carga.get_cell_mut(&*format!("D{}", row)).set_formula(&formula);
    }

    // === Guardar copia con el nombre del acta ===
    println!("DEBUG FINAL: nombre_acta usado para guardar: '{}'", nombre_acta);
    let save_path_str = format!("C:/plantillas/{}.xlsx", nombre_acta);
    let save_path = Path::new(&save_path_str);
    writer::xlsx::write(&book, save_path).map_err(|e| e.to_string())?;

    println!("=== FIN GENERACIÓN PLANTILLA ACTA ===");
    println!("ANTES DE OK(nombre_acta)");
    Ok(nombre_acta)
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
    _id_grado: i32,
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
pub async fn obtener_lapsos(_state: State<'_, AppState>) -> Result<Vec<String>, String> {
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