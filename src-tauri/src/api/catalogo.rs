use tauri::State;
use crate::AppState;
use crate::models::catalogo::{PeriodoEscolar, Grado, Modalidad, Asignatura};
use chrono::NaiveDate;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Pais {
    pub id: i32,
    pub nombre: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Estado {
    pub id: i32,
    pub nombre: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Ciudad {
    pub id: i32,
    pub nombre: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Municipio {
    pub id: i32,
    pub nombre: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Seccion {
    pub id: i32,
    pub nombre: String,
}

#[tauri::command]
pub async fn listar_periodos_escolares(state: State<'_, AppState>) -> Result<Vec<PeriodoEscolar>, String> {
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
pub async fn listar_grados(state: tauri::State<'_, crate::AppState>) -> Result<Vec<Grado>, String> {
    let db = state.db.lock().await;
    let rows = db.query("SELECT id_grado, nombre_grado FROM grados ORDER BY nombre_grado", &[])
        .await.map_err(|e| e.to_string())?;
    let grados = rows.iter().map(|row| Grado {
        id_grado: row.get(0),
        nombre_grado: row.get(1),
    }).collect();
    Ok(grados)
}

#[tauri::command]
pub async fn listar_modalidades(state: tauri::State<'_, crate::AppState>) -> Result<Vec<Modalidad>, String> {
    let db = state.db.lock().await;
    let rows = db.query("SELECT id_modalidad, nombre_modalidad FROM modalidades ORDER BY nombre_modalidad", &[])
        .await.map_err(|e| e.to_string())?;
    let modalidades = rows.iter().map(|row| Modalidad {
        id_modalidad: row.get(0),
        nombre_modalidad: row.get(1),
    }).collect();
    Ok(modalidades)
}

#[tauri::command]
pub async fn obtener_asignaturas_por_grado_modalidad(
    idGrado: i32,
    idModalidad: i32,
    state: State<'_, AppState>,
) -> Result<Vec<Asignatura>, String> {
    let db = state.db.lock().await;
    let query = "
        SELECT 
            a.id_asignatura, 
            a.nombre AS nombre_asignatura, 
            gma.id_grado, 
            gma.id_modalidad
        FROM asignaturas a
        INNER JOIN grado_modalidad_asignaturas gma 
            ON a.id_asignatura = gma.id_asignatura
        WHERE gma.id_grado = $1 
            AND gma.id_modalidad = $2
        ORDER BY gma.orden";
    
    let rows = db.query(query, &[&idGrado, &idModalidad])
        .await
        .map_err(|e| e.to_string())?;

    let asignaturas = rows.iter().map(|row| Asignatura {
        id_asignatura: row.get("id_asignatura"),
        nombre_asignatura: row.get("nombre_asignatura"),
        id_grado: row.get("id_grado"),
        id_modalidad: row.get("id_modalidad"),
        id_docente: None,
        nombres_docente: None,
        apellidos_docente: None,
    }).collect();

    Ok(asignaturas)
}

#[tauri::command]
pub async fn crear_periodo_escolar(
    periodo_escolar: String,
    fecha_inicio: String,
    fecha_final: String,
    state: State<'_, AppState>,
) -> Result<i32, String> {
    let db = state.db.lock().await;

    let parsed_fecha_inicio = NaiveDate::parse_from_str(&fecha_inicio, "%Y-%m-%d")
        .map_err(|e| format!("Error al parsear fecha de inicio: {}", e))?;
    let parsed_fecha_final = NaiveDate::parse_from_str(&fecha_final, "%Y-%m-%d")
        .map_err(|e| format!("Error al parsear fecha final: {}", e))?;

    let row = db.query_one(
        "INSERT INTO periodos_escolares (periodo_escolar, fecha_inicio, fecha_final, activo) VALUES ($1, $2, $3, FALSE) RETURNING id_periodo",
        &[&periodo_escolar, &parsed_fecha_inicio, &parsed_fecha_final],
    ).await.map_err(|e| e.to_string())?;

    let id_periodo: i32 = row.get("id_periodo");

    Ok(id_periodo)
}

#[tauri::command]
pub async fn establecer_periodo_activo(
    id_periodo: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;

    // 1. Establecer todos los periodos a inactivos
    db.execute("UPDATE periodos_escolares SET activo = FALSE", &[])
        .await
        .map_err(|e| format!("Error al desactivar periodos existentes: {}", e))?;

    // 2. Establecer el periodo especificado como activo
    db.execute("UPDATE periodos_escolares SET activo = TRUE WHERE id_periodo = $1", &[&id_periodo])
        .await
        .map_err(|e| format!("Error al activar el nuevo periodo: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn listar_paises(state: tauri::State<'_, crate::AppState>) -> Result<Vec<Pais>, String> {
    println!("[DEBUG] listar_paises llamado");
    let db = state.db.lock().await;
    let query = "SELECT id, nombre FROM paises ORDER BY nombre";
    println!("[DEBUG] Ejecutando query: {}", query);
    let rows = db.query(query, &[])
        .await.map_err(|e| {
            println!("[ERROR] Error en query listar_paises: {}", e);
            e.to_string()
        })?;
    println!("[DEBUG] Filas obtenidas: {}", rows.len());
    let paises = rows.iter().map(|row| {
        let pais = Pais {
        id: row.get(0),
        nombre: row.get(1),
        };
        println!("[DEBUG] País encontrado: {:?}", pais);
        pais
    }).collect();
    println!("[DEBUG] Países totales: {:?}", paises);
    Ok(paises)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn listar_estados_por_pais(state: tauri::State<'_, crate::AppState>, id_pais: i32) -> Result<Vec<Estado>, String> {
    println!("[DEBUG] listar_estados_por_pais llamado con id_pais: {}", id_pais);
    let db = state.db.lock().await;
    let query = "SELECT id, nombre FROM estados WHERE pais_id = $1 ORDER BY nombre";
    println!("[DEBUG] Ejecutando query: {}", query);
    let rows = db.query(query, &[&id_pais])
        .await.map_err(|e| {
            println!("[ERROR] Error en query listar_estados_por_pais: {}", e);
            e.to_string()
        })?;
    println!("[DEBUG] Filas obtenidas: {}", rows.len());
    let estados = rows.iter().map(|row| {
        let estado = Estado {
        id: row.get(0),
        nombre: row.get(1),
        };
        println!("[DEBUG] Estado encontrado: {:?}", estado);
        estado
    }).collect();
    println!("[DEBUG] Estados totales: {:?}", estados);
    Ok(estados)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn listar_municipios_por_estado(state: tauri::State<'_, crate::AppState>, id_estado: i32) -> Result<Vec<Municipio>, String> {
    println!("[DEBUG] listar_municipios_por_estado llamado con id_estado: {}", id_estado);
    let db = state.db.lock().await;
    let query = "SELECT id, nombre FROM municipios WHERE estado_id = $1 ORDER BY nombre";
    println!("[DEBUG] Ejecutando query: {}", query);
    let rows = db.query(query, &[&id_estado])
        .await.map_err(|e| {
            println!("[ERROR] Error en query listar_municipios_por_estado: {}", e);
            e.to_string()
        })?;
    println!("[DEBUG] Filas obtenidas: {}", rows.len());
    let municipios = rows.iter().map(|row| {
        let municipio = Municipio {
        id: row.get(0),
        nombre: row.get(1),
        };
        println!("[DEBUG] Municipio encontrado: {:?}", municipio);
        municipio
    }).collect();
    println!("[DEBUG] Municipios totales: {:?}", municipios);
    Ok(municipios)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn listar_ciudades_por_municipio(state: tauri::State<'_, crate::AppState>, id_municipio: i32) -> Result<Vec<Ciudad>, String> {
    println!("[DEBUG] listar_ciudades_por_municipio llamado con id_municipio: {}", id_municipio);
    let db = state.db.lock().await;
    let query = "SELECT id, nombre FROM ciudades WHERE municipio_id = $1 ORDER BY nombre";
    println!("[DEBUG] Ejecutando query: {}", query);
    let rows = db.query(query, &[&id_municipio])
        .await.map_err(|e| {
            println!("[ERROR] Error en query listar_ciudades_por_municipio: {}", e);
            e.to_string()
        })?;
    println!("[DEBUG] Filas obtenidas: {}", rows.len());
    let ciudades = rows.iter().map(|row| {
        let ciudad = Ciudad {
        id: row.get(0),
        nombre: row.get(1),
        };
        println!("[DEBUG] Ciudad encontrada: {:?}", ciudad);
        ciudad
    }).collect();
    println!("[DEBUG] Ciudades totales: {:?}", ciudades);
    Ok(ciudades)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn obtener_id_grado_secciones(state: tauri::State<'_, crate::AppState>, grado_id: i32, seccion_id: i32, modalidad_id: i32) -> Result<i32, String> {
    println!("[DEBUG] Buscando id_grado_secciones para grado={}, seccion={}, modalidad={}", grado_id, seccion_id, modalidad_id);
    let db = state.db.lock().await;
    let query = "SELECT id_grado_secciones FROM grado_secciones WHERE id_grado = $1 AND id_seccion = $2 AND id_modalidad = $3";
    println!("[DEBUG] Ejecutando query: {}", query);
    let row = db.query_one(query, &[&grado_id, &seccion_id, &modalidad_id])
        .await.map_err(|e| {
            println!("[ERROR] Error en query: {}", e);
            e.to_string()
        })?;
    Ok(row.get(0))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn listar_secciones_por_grado_modalidad(state: tauri::State<'_, crate::AppState>, grado_id: i32, modalidad_id: i32) -> Result<Vec<Seccion>, String> {
    println!("[LOG] listar_secciones_por_grado_modalidad: grado_id={}, modalidad_id={}", grado_id, modalidad_id);
    let db = state.db.lock().await;
    let query = "SELECT s.id_seccion, s.nombre_seccion FROM secciones s INNER JOIN grado_secciones gs ON gs.seccion_id = s.id_seccion WHERE gs.grado_id = $1 AND gs.modalidad_id = $2 ORDER BY s.nombre_seccion";
    println!("[LOG] Ejecutando query: {}", query);
    let rows = db.query(query, &[&grado_id, &modalidad_id])
        .await.map_err(|e| {
            println!("[ERROR] Error en query: {}", e);
            e.to_string()
        })?;
    println!("[LOG] Filas obtenidas: {}", rows.len());
    let secciones = rows.iter().map(|row| Seccion {
        id: row.get(0),
        nombre: row.get(1),
    }).collect();
    println!("[LOG] Secciones serializadas: {:?}", secciones);
    Ok(secciones)
}

#[tauri::command]
pub async fn listar_secciones(state: tauri::State<'_, crate::AppState>) -> Result<Vec<Seccion>, String> {
    let db = state.db.lock().await;
    let rows = db.query("SELECT id_seccion, nombre_seccion FROM secciones ORDER BY nombre_seccion", &[])
        .await.map_err(|e| e.to_string())?;
    let secciones = rows.iter().map(|row| Seccion {
        id: row.get(0),
        nombre: row.get(1),
    }).collect();
    Ok(secciones)
} 