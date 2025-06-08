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
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres;
use rust_xlsxwriter::{Workbook, Format, FormatAlign, FormatBorder, XlsxError};

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

#[derive(Debug, serde::Serialize)]
pub struct TarjetaCurso {
    pub id_grado_secciones: i32,
    pub id_grado: i32,
    pub id_seccion: i32,
    pub id_modalidad: i32,
    pub nombre_grado: String,
    pub nombre_seccion: String,
    pub nombre_modalidad: String,
    pub docente_guia: String,
    pub total_estudiantes: i64,
    pub estudiantes_femeninos: i64,
    pub estudiantes_masculinos: i64,
}

pub async fn generar_plantilla_acta(
    db_pool: Arc<Mutex<tokio_postgres::Client>>,
    id_grado_secciones: i32,
    id_periodo: i32,
    id_asignatura: i32,
    lapso: String,
) -> Result<String, String> {
    println!("=== INICIO GENERACIÓN PLANTILLA ACTA ===");
    println!("Parámetros recibidos: id_grado_secciones={}, id_periodo={}, id_asignatura={}, lapso={}", 
        id_grado_secciones, id_periodo, id_asignatura, lapso);

    println!("[ACTA] Ejecutando consulta SQL para obtener estudiantes...");
    let estudiantes = {
        let db = db_pool.lock().await;
        db.query(
            "SELECT e.id, e.cedula, e.nombres, e.apellidos FROM historial_grado_estudiantes h \
             JOIN estudiantes e ON h.id_estudiante = e.id \
             WHERE h.id_grado_secciones = $1 AND h.id_periodo = $2 AND h.estado = 'activo' AND h.es_actual = true \
             ORDER BY e.apellidos, e.nombres",
            &[&id_grado_secciones, &id_periodo],
        ).await.map_err(|e| {
            println!("[ACTA][ERROR] Error en la consulta SQL de estudiantes: {}", e);
            e.to_string()
        })?
    };
    println!("[ACTA] Número de estudiantes encontrados: {}", estudiantes.len());
    if estudiantes.is_empty() {
        println!("[ACTA][WARN] No se encontraron estudiantes para los parámetros dados");
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
            println!("[ACTA] Estudiante encontrado: id={}, cedula={}, nombre={}, apellido={}", 
                estudiante.id, estudiante.cedula, estudiante.nombre, estudiante.apellido);
            estudiante
        })
        .collect();

    println!("[ACTA] Consultando nombre de la asignatura...");
    let row_asig = {
        let db = db_pool.lock().await;
        db.query_one("SELECT nombre FROM asignaturas WHERE id_asignatura = $1", &[&id_asignatura]).await.map_err(|e| {
            println!("[ACTA][ERROR] Error consultando nombre de asignatura: {}", e);
            e.to_string()
        })?
    };
    let nombre_asignatura: String = row_asig.get(0);
    println!("[ACTA] Nombre de asignatura obtenido: {}", nombre_asignatura);

    println!("[ACTA] Consultando grado y sección legibles...");
    let row_grado_seccion = {
        let db = db_pool.lock().await;
        db.query_one(
            "SELECT g.nombre_grado, s.nombre_seccion FROM grado_secciones gs \
             JOIN grados g ON gs.id_grado = g.id_grado \
             JOIN secciones s ON gs.id_seccion = s.id_seccion \
             WHERE gs.id_grado_secciones = $1",
            &[&id_grado_secciones],
        ).await.map_err(|e| {
            println!("[ACTA][ERROR] Error consultando grado y sección: {}", e);
            e.to_string()
        })?
    };
    let nombre_grado: String = row_grado_seccion.get(0);
    let nombre_seccion: String = row_grado_seccion.get(1);
    println!("[ACTA] nombre_grado obtenido: '{}', nombre_seccion obtenido: '{}'", nombre_grado, nombre_seccion);

    let nombre_asignatura_normalizado = if nombre_asignatura.trim().to_lowercase() == "lengua y literatura" {
        "castellano".to_string()
    } else {
        nombre_asignatura.trim().to_lowercase()
            .replace("á", "a").replace("é", "e").replace("í", "i").replace("ó", "o").replace("ú", "u")
            .replace("à", "a").replace("è", "e").replace("ì", "i").replace("ò", "o").replace("ù", "u")
            .replace(" ", "")
    };
    println!("[ACTA] nombre_asignatura_normalizado: {}", nombre_asignatura_normalizado);

    let siglas = HashMap::from([
        ("castellano".to_string(), "Ca"),
        ("lengua".to_string(), "Ca"),
        ("lenguayliteratura".to_string(), "Ca"),
        ("lengua y literatura".to_string(), "Ca"),
        ("ingles".to_string(), "In"),
        ("matematica".to_string(), "Ma"),
        ("educacionfisica".to_string(), "Ef"),
        ("arteypatrimonio".to_string(), "Ap"),
        ("bat".to_string(), "Ba"),
        ("ghc".to_string(), "Gh"),
        ("fisica".to_string(), "Fi"),
        ("quimica".to_string(), "Qu"),
        ("fsn".to_string(), "Fs"),
        ("cienciasdelatierra".to_string(), "Cs"),
        ("ciencias de la tierra".to_string(), "Cs"),
        ("csdelatierra".to_string(), "Cs"),
        ("cs.de.la.tierra".to_string(), "Cs"),
        ("programacion".to_string(), "Pr"),
        ("proyectodeeconomia".to_string(), "Pe"),
        ("gcrp".to_string(), "Gc"),
        ("orientacion".to_string(), "Or"),
    ]);
    let sigla = match siglas.get(&nombre_asignatura_normalizado) {
        Some(s) => s,
        None => {
            println!("[ACTA][WARN] Asignatura no encontrada en siglas: '{}'. Normalizado: '{}'", nombre_asignatura, nombre_asignatura_normalizado);
            &"XX"
        }
    };
    println!("[ACTA] Sigla usada: {}", sigla);

    let lapso_str = format!("{}º", lapso);
    println!("[ACTA] Lapso string: {}", lapso_str);

    let plantilla_path = Path::new("C:/plantillas/acta_pantilla/acta_plantilla.xlsx");
    println!("[ACTA] Intentando abrir plantilla: {:?}", plantilla_path);
    let mut book = match reader::xlsx::read(plantilla_path) {
        Ok(b) => {
            println!("[ACTA] Plantilla abierta correctamente");
            b
        },
        Err(e) => {
            println!("[ACTA][ERROR] Error abriendo plantilla: {}", e);
            return Err(e.to_string());
        }
    };
    let sheet = match book.get_sheet_by_name_mut("ACTA") {
        Some(s) => {
            println!("[ACTA] Hoja ACTA encontrada en la plantilla");
            s
        },
        None => {
            println!("[ACTA][ERROR] No se encontró la hoja ACTA en la plantilla");
            return Err("No se encontró la hoja ACTA en la plantilla".to_string());
        }
    };

    let mut grado = nombre_grado.trim().to_string();
    if grado == "1er" { grado = "1ero".to_string(); }
    if grado == "3er" { grado = "3ero".to_string(); }
    let seccion = nombre_seccion.trim();
    let texto_d6 = format!("{} Año {}", nombre_grado.trim(), nombre_seccion.trim());
    sheet.get_cell_mut("D6").set_value(&texto_d6);
    println!("[ACTA] Escrito en D6: {}", texto_d6);

    sheet.get_cell_mut("D7").set_value(lapso_str.clone());
    println!("[ACTA] Escrito en D7: {}", lapso_str);
    sheet.get_cell_mut("D8").set_value(nombre_asignatura.clone());
    println!("[ACTA] Escrito en D8: {}", nombre_asignatura);

    println!("[ACTA] Consultando modalidad...");
    let id_modalidad: i32 = {
        let db = db_pool.lock().await;
        let row = db.query_one("SELECT id_modalidad FROM grado_secciones WHERE id_grado_secciones = $1", &[&id_grado_secciones]).await.map_err(|e| {
            println!("[ACTA][ERROR] Error consultando id_modalidad: {}", e);
            e.to_string()
        })?;
        row.get(0)
    };
    let modalidad: String = {
        let db = db_pool.lock().await;
        let row_modalidad = db.query_one("SELECT nombre_modalidad FROM modalidades WHERE id_modalidad = $1", &[&id_modalidad]).await.map_err(|e| {
            println!("[ACTA][ERROR] Error consultando nombre_modalidad: {}", e);
            e.to_string()
        })?;
        row_modalidad.get(0)
    };
    sheet.get_cell_mut("A11").set_value(&modalidad);
    println!("[ACTA] Escrito en A11: {}", modalidad);

    let sigla_ext = if modalidad.to_lowercase().contains("tele") {
        format!("{}Tele", sigla)
    } else {
        sigla.to_string()
    };
    let nombre_acta = format!("{}{}_{}_{}", sigla_ext, grado, seccion, lapso_str);
    sheet.get_cell_mut("D10").set_value(&nombre_acta);
    println!("[ACTA] Escrito en D10: {}", nombre_acta);

    for (i, est) in estudiantes.iter().enumerate() {
        let row = 13 + i;
        let cell_cedula = format!("B{}", row);
        let cell_nombre = format!("C{}", row);
        sheet.get_cell_mut(&*cell_cedula).set_value(est.cedula.to_string());
        sheet.get_cell_mut(&*cell_nombre).set_value(format!("{} {}", est.apellido, est.nombre));
        println!("[ACTA] Escrito estudiante en fila {}: cedula={}, nombre={}", row, est.cedula, format!("{} {}", est.apellido, est.nombre));
    }

    let nombre_lapso_campo = match lapso.as_str() {
        "1" | "1º" | "1er" | "1ero" => "lapso_1",
        "2" | "2º" | "2do" => "lapso_2",
        "3" | "3º" | "3er" | "3ero" => "lapso_3",
        _ => "calificacion",
    };
    println!("[ACTA] nombre_lapso_campo: {}", nombre_lapso_campo);

    let carga = match book.new_sheet("CARGA_MASIVA") {
        Ok(c) => {
            println!("[ACTA] Hoja CARGA_MASIVA creada correctamente");
            c
        },
        Err(e) => {
            println!("[ACTA][ERROR] Error creando hoja CARGA_MASIVA: {}", e);
            return Err(e.to_string());
        }
    };
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
        println!("[ACTA] Escrito en CARGA_MASIVA fila {}: id_estudiante={}, id_asignatura={}, id_periodo={}", row, est.id, id_asignatura, id_periodo);
    }

    println!("[ACTA] Guardando archivo en: C:/plantillas/{}.xlsx", nombre_acta);
    let save_path_str = format!("C:/plantillas/{}.xlsx", nombre_acta);
    let save_path = Path::new(&save_path_str);
    match writer::xlsx::write(&book, save_path) {
        Ok(_) => println!("[ACTA] Archivo guardado correctamente"),
        Err(e) => {
            println!("[ACTA][ERROR] Error guardando archivo: {}", e);
            return Err(e.to_string());
        }
    }

    println!("=== FIN GENERACIÓN PLANTILLA ACTA ===");
    println!("[ACTA] ANTES DE OK(nombre_acta)");
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

// Nueva función para generación masiva eficiente
pub async fn generar_plantilla_acta_desde_datos(
    datos: crate::api::actas_masivas::DatosActa,
    lapso: String,
) -> Result<String, String> {
    use umya_spreadsheet::{reader, writer};
    use std::path::Path;
    use std::collections::HashMap;
    println!("[ACTA][MASIVO] Generando acta desde datos pre-cargados");
    let plantilla_path = Path::new("C:/plantillas/acta_pantilla/acta_plantilla.xlsx");
    let mut book = match reader::xlsx::read(plantilla_path) {
        Ok(b) => b,
        Err(e) => return Err(e.to_string()),
    };
    let sheet = match book.get_sheet_by_name_mut("ACTA") {
        Some(s) => s,
        None => return Err("No se encontró la hoja ACTA en la plantilla".to_string()),
    };
    let mut grado = datos.nombre_grado.trim().to_string();
    if grado == "1er" { grado = "1ero".to_string(); }
    if grado == "3er" { grado = "3ero".to_string(); }
    let seccion = datos.nombre_seccion.trim();
    let texto_d6 = format!("{} Año {}", datos.nombre_grado.trim(), datos.nombre_seccion.trim());
    sheet.get_cell_mut("D6").set_value(&texto_d6);
    let lapso_str = format!("{}º", lapso);
    sheet.get_cell_mut("D7").set_value(lapso_str.clone());
    sheet.get_cell_mut("D8").set_value(datos.nombre_asignatura.clone());
    sheet.get_cell_mut("A11").set_value(&datos.modalidad);
    let siglas = HashMap::from([
        ("castellano".to_string(), "Ca"),
        ("lengua".to_string(), "Ca"),
        ("lenguayliteratura".to_string(), "Ca"),
        ("lengua y literatura".to_string(), "Ca"),
        ("ingles".to_string(), "In"),
        ("matematica".to_string(), "Ma"),
        ("educacionfisica".to_string(), "Ef"),
        ("arteypatrimonio".to_string(), "Ap"),
        ("bat".to_string(), "Ba"),
        ("ghc".to_string(), "Gh"),
        ("fisica".to_string(), "Fi"),
        ("quimica".to_string(), "Qu"),
        ("fsn".to_string(), "Fs"),
        ("cienciasdelatierra".to_string(), "Cs"),
        ("ciencias de la tierra".to_string(), "Cs"),
        ("csdelatierra".to_string(), "Cs"),
        ("cs.de.la.tierra".to_string(), "Cs"),
        ("programacion".to_string(), "Pr"),
        ("proyectodeeconomia".to_string(), "Pe"),
        ("gcrp".to_string(), "Gc"),
        ("orientacion".to_string(), "Or"),
    ]);
    let mut nombre_asignatura_normalizado = datos.nombre_asignatura
        .to_lowercase()
        .replace("á", "a").replace("é", "e").replace("í", "i")
        .replace("ó", "o").replace("ú", "u")
        .replace("à", "a").replace("è", "e").replace("ì", "i")
        .replace("ò", "o").replace("ù", "u")
        .replace(" ", "")
        .replace(".", "")
        .replace("-", "")
        .replace("_", "");
    if nombre_asignatura_normalizado == "lenguayliteratura" {
        nombre_asignatura_normalizado = "castellano".to_string();
    }
    if nombre_asignatura_normalizado == "cienciasdelatierra" || nombre_asignatura_normalizado == "csdelatierra" {
        nombre_asignatura_normalizado = "cienciasdelatierra".to_string();
    }
    let sigla = siglas.get(&nombre_asignatura_normalizado).unwrap_or(&"XX");
    let sigla_ext = if datos.modalidad.to_lowercase().contains("tele") {
        format!("{}Tele", sigla)
    } else {
        sigla.to_string()
    };
    let nombre_acta = format!("{}{}_{}_{}", sigla_ext, grado, seccion, lapso_str);
    sheet.get_cell_mut("D10").set_value(&nombre_acta);
    for (i, est) in datos.estudiantes.iter().enumerate() {
        let row = 13 + i;
        let cell_cedula = format!("B{}", row);
        let cell_nombre = format!("C{}", row);
        sheet.get_cell_mut(&*cell_cedula).set_value(est.1.to_string());
        sheet.get_cell_mut(&*cell_nombre).set_value(format!("{} {}", est.3, est.2));
    }
    let nombre_lapso_campo = match lapso.as_str() {
        "1" | "1º" | "1er" | "1ero" => "lapso_1",
        "2" | "2º" | "2do" => "lapso_2",
        "3" | "3º" | "3er" | "3ero" => "lapso_3",
        _ => "calificacion",
    };
    let carga = match book.new_sheet("CARGA_MASIVA") {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };
    carga.get_cell_mut("A1").set_value("id_estudiante");
    carga.get_cell_mut("B1").set_value("id_asignatura");
    carga.get_cell_mut("C1").set_value("id_periodo");
    carga.get_cell_mut("D1").set_value(nombre_lapso_campo);
    for (i, est) in datos.estudiantes.iter().enumerate() {
        let row = 2 + i;
        carga.get_cell_mut(&*format!("A{}", row)).set_value(est.0.to_string());
        carga.get_cell_mut(&*format!("B{}", row)).set_value(datos.id_asignatura.to_string());
        carga.get_cell_mut(&*format!("C{}", row)).set_value(datos.id_periodo.to_string());
        let celda_acta = format!("N{}", 13 + i);
        let formula = format!("=ACTA!{}", celda_acta);
        carga.get_cell_mut(&*format!("D{}", row)).set_formula(&formula);
    }
    let save_path_str = format!("C:/plantillas/{}.xlsx", nombre_acta);
    let save_path = Path::new(&save_path_str);
    match writer::xlsx::write(&book, save_path) {
        Ok(_) => Ok(nombre_acta),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn obtener_tarjetas_cursos(state: tauri::State<'_, crate::AppState>) -> Result<Vec<TarjetaCurso>, String> {
    let db = state.db.lock().await;
    let rows = db.query(
        "SELECT
            gs.id_grado_secciones,
            gs.id_grado,
            gs.id_seccion,
            gs.id_modalidad,
            g.nombre_grado,
            s.nombre_seccion,
            m.nombre_modalidad,
            CONCAT(d.nombres, ' ', d.apellidos) AS docente_guia,
            COUNT(hge.id_estudiante) AS total_estudiantes,
            SUM(CASE WHEN e.genero = 'F' THEN 1 ELSE 0 END) AS estudiantes_femeninos,
            SUM(CASE WHEN e.genero = 'M' THEN 1 ELSE 0 END) AS estudiantes_masculinos
        FROM historial_grado_estudiantes hge
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
        LEFT JOIN docentes d ON gs.id_docente_guia = d.id_docente
        LEFT JOIN estudiantes e ON hge.id_estudiante = e.id
        WHERE hge.es_actual = true AND hge.estado = 'activo'
        GROUP BY gs.id_grado_secciones, gs.id_grado, gs.id_seccion, gs.id_modalidad, g.nombre_grado, s.nombre_seccion, m.nombre_modalidad, d.nombres, d.apellidos
        ORDER BY gs.id_grado, s.nombre_seccion",
        &[],
    ).await.map_err(|e| e.to_string())?;
    let tarjetas = rows.iter().map(|row| TarjetaCurso {
        id_grado_secciones: row.get(0),
        id_grado: row.get(1),
        id_seccion: row.get(2),
        id_modalidad: row.get(3),
        nombre_grado: row.get(4),
        nombre_seccion: row.get(5),
        nombre_modalidad: row.get(6),
        docente_guia: row.get(7),
        total_estudiantes: row.get(8),
        estudiantes_femeninos: row.get(9),
        estudiantes_masculinos: row.get(10),
    }).collect();
    Ok(tarjetas)
} 