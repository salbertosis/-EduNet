use tauri::State;
use crate::models::estudiante::{Estudiante, FiltroEstudiantes, NuevoEstudiante};
use crate::AppState;
use postgres_types::ToSql;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

// Paginación y resumen
#[derive(Debug, Serialize)]
pub struct ResumenInsercion {
    pub total_registros: usize,
    pub insertados: usize,
    pub duplicados: usize,
    pub errores: Vec<String>,
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

// ParamValue auxiliar
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

#[tauri::command]
pub async fn obtener_estudiantes(
    filtro: Option<FiltroEstudiantes>,
    paginacion: PaginacionParams,
    state: State<'_, AppState>,
) -> Result<ResultadoPaginado<Estudiante>, String> {
    let db = state.db.lock().await;
    let mut query_base = String::from(
        "SELECT e.id, e.cedula, e.nombres, e.apellidos, e.genero, e.fecha_nacimiento, \
        e.id_grado_secciones, e.fecha_ingreso, e.id_periodoactual, e.estado, e.fecha_retiro, \
        g.nombre_grado, s.nombre_seccion, m.nombre_modalidad \
        FROM estudiantes e \
        LEFT JOIN grado_secciones gs ON e.id_grado_secciones = gs.id_grado_secciones \
        LEFT JOIN grados g ON gs.id_grado = g.id_grado \
        LEFT JOIN secciones s ON gs.id_seccion = s.id_seccion \
        LEFT JOIN modalidades m ON gs.id_modalidad = m.id_modalidad \
        WHERE 1=1"
    );
    let mut param_values: Vec<ParamValue> = Vec::new();
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
            query_base.push_str(&format!(" AND gs.id_grado = ${}", param_values.len() + 1));
            param_values.push(ParamValue::I32(grado_num));
        }
        if let Some(modalidad_num) = f.modalidad {
            query_base.push_str(&format!(" AND gs.id_modalidad = ${}", param_values.len() + 1));
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
    let query_count = format!("SELECT COUNT(*) FROM ({}) as subquery", query_base);
    let params: Vec<&(dyn ToSql + Sync)> = param_values.iter().map(|v| v.as_tosql()).collect();
    let total_registros: i64 = db.query_one(&query_count, &params).await.map_err(|e| e.to_string())?.get(0);
    let total_paginas = (total_registros as f64 / paginacion.registros_por_pagina as f64).ceil() as i32;
    let offset = (paginacion.pagina - 1) * paginacion.registros_por_pagina;
    let query_final = format!(
        "{} ORDER BY e.cedula LIMIT {} OFFSET {}",
        query_base,
        paginacion.registros_por_pagina,
        offset
    );
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
            id_periodoactual: row.get(8),
            estado: row.get(9),
            fecha_retiro: row.get(10),
            nombre_grado: row.get(11),
            nombre_seccion: row.get(12),
            nombre_modalidad: row.get(13),
            municipionac: None,
            paisnac: None,
            entidadfed: None,
            ciudadnac: None,
            estadonac: None,
            id_grado: None,
            id_seccion: None,
            id_modalidad: None,
        })
        .collect::<Vec<_>>();
    let paginacion_info = PaginacionInfo {
        pagina_actual: paginacion.pagina,
        total_paginas,
        total_registros,
        registros_por_pagina: paginacion.registros_por_pagina,
    };
    Ok(ResultadoPaginado {
        datos: estudiantes,
        paginacion: paginacion_info,
    })
}

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
pub async fn crear_estudiante(
    estudiante: NuevoEstudiante,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    let cedula_duplicada = verificar_cedula_duplicada(&*db, estudiante.cedula, None).await?;
    if cedula_duplicada {
        return Err("Ya existe un estudiante con esta cédula".to_string());
    }
    match db.execute(
        "INSERT INTO estudiantes (cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, paisnac_id, estado_nac_id, municipio_nac_id, ciudad_nac_id, id_periodoactual, estado, fecha_retiro) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)",
        &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.paisnac_id, &estudiante.estado_nac_id, &estudiante.municipio_nac_id, &estudiante.ciudad_nac_id, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro]
    ).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("[ERROR] Error al insertar estudiante: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn actualizar_estudiante(
    id: i32,
    estudiante: NuevoEstudiante,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    let cedula_duplicada = verificar_cedula_duplicada(&*db, estudiante.cedula, Some(id)).await?;
    if cedula_duplicada {
        return Err("Ya existe otro estudiante con esta cédula".to_string());
    }
    db.execute(
        "UPDATE estudiantes SET cedula=$1, nombres=$2, apellidos=$3, genero=$4, fecha_nacimiento=$5, id_grado_secciones=$6, fecha_ingreso=$7, paisnac_id=$8, estado_nac_id=$9, municipio_nac_id=$10, ciudad_nac_id=$11, id_periodoactual=$12, estado=$13, fecha_retiro=$14 WHERE id=$15",
        &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.paisnac_id, &estudiante.estado_nac_id, &estudiante.municipio_nac_id, &estudiante.ciudad_nac_id, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro, &id]
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn eliminar_estudiante(
    id: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    db.execute(
        "DELETE FROM estudiantes WHERE id = $1",
        &[&id],
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn contar_estudiantes(state: State<'_, AppState>) -> Result<i64, String> {
    let db = state.db.lock().await;
    let row = db
        .query_one("SELECT COUNT(*) FROM estudiantes", &[])
        .await
        .map_err(|e| e.to_string())?;
    let total: i64 = row.get(0);
    Ok(total)
}

async fn obtener_cedulas_existentes(db: &tokio_postgres::Client) -> Result<HashSet<i64>, String> {
    let rows = db.query("SELECT cedula FROM estudiantes", &[]).await.map_err(|e| e.to_string())?;
    Ok(rows.iter().map(|row| row.get::<_, i64>(0)).collect())
}

#[tauri::command]
pub async fn obtener_estudiante_por_id(
    id: i32,
    state: State<'_, AppState>,
) -> Result<Estudiante, String> {
    let db = state.db.lock().await;
    let row_opt = db
        .query_opt(
            "SELECT e.id, e.cedula, e.nombres, e.apellidos, e.genero, e.fecha_nacimiento, \
            e.id_grado_secciones, e.fecha_ingreso, e.id_periodoactual, e.estado, e.fecha_retiro, \
            gs.id_grado, g.nombre_grado, gs.id_seccion, s.nombre_seccion, \
            gs.id_modalidad, m.nombre_modalidad \
            FROM estudiantes e \
            LEFT JOIN grado_secciones gs ON e.id_grado_secciones = gs.id_grado_secciones \
            LEFT JOIN grados g ON gs.id_grado = g.id_grado \
            LEFT JOIN secciones s ON gs.id_seccion = s.id_seccion \
            LEFT JOIN modalidades m ON gs.id_modalidad = m.id_modalidad \
            WHERE e.id = $1",
            &[&id],
        )
        .await
        .map_err(|e| e.to_string())?;
    let row = match row_opt {
        Some(r) => r,
        None => return Err(format!("No se encontró el estudiante con id {}", id)),
    };
    Ok(Estudiante {
        id: row.get(0),
        cedula: row.get(1),
        nombres: row.get(2),
        apellidos: row.get(3),
        genero: row.get(4),
        fecha_nacimiento: row.get(5),
        id_grado_secciones: row.get(6),
        fecha_ingreso: row.get(7),
        municipionac: None,
        paisnac: None,
        entidadfed: None,
        ciudadnac: None,
        estadonac: None,
        id_grado: row.get(11),
        nombre_grado: row.get(12),
        id_seccion: row.get(13),
        nombre_seccion: row.get(14),
        id_modalidad: row.get(15),
        nombre_modalidad: row.get(16),
        id_periodoactual: row.get(8),
        estado: row.get(9),
        fecha_retiro: row.get(10),
    })
}

#[tauri::command]
pub async fn contar_estudiantes_femeninos(state: State<'_, AppState>) -> Result<i64, String> {
    let db = state.db.lock().await;
    let row = db
        .query_one("SELECT COUNT(*) FROM estudiantes WHERE genero = 'F'", &[])
        .await
        .map_err(|e| e.to_string())?;
    let total: i64 = row.get(0);
    Ok(total)
}

#[tauri::command]
pub async fn contar_estudiantes_masculinos(state: State<'_, AppState>) -> Result<i64, String> {
    let db = state.db.lock().await;
    let row = db
        .query_one("SELECT COUNT(*) FROM estudiantes WHERE genero = 'M'", &[])
        .await
        .map_err(|e| e.to_string())?;
    let total: i64 = row.get(0);
    Ok(total)
}

#[tauri::command]
pub async fn insertar_estudiantes_masivo(
    estudiantes: Vec<serde_json::Value>,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let db = state.db.lock().await;
    let mut insertados = 0;
    let mut duplicados = 0;
    let mut errores = vec![];
    
    println!("[DEBUG] Recibidos {} estudiantes para inserción masiva", estudiantes.len());
    
    for (i, estudiante_json) in estudiantes.iter().enumerate() {
        println!("[DEBUG] Procesando estudiante {}: {:?}", i + 1, estudiante_json);
        
        // Extraer y validar campos requeridos
        let cedula = match estudiante_json.get("cedula") {
            Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0),
            Some(serde_json::Value::String(s)) => s.parse::<i64>().unwrap_or(0),
            _ => {
                errores.push(format!("Estudiante {}: cédula inválida o faltante", i + 1));
                continue;
            }
        };
        
        let nombres = match estudiante_json.get("nombres") {
            Some(serde_json::Value::String(s)) => s.clone(),
            _ => {
                errores.push(format!("Estudiante {}: nombres faltantes", i + 1));
                continue;
            }
        };
        
        let apellidos = match estudiante_json.get("apellidos") {
            Some(serde_json::Value::String(s)) => s.clone(),
            _ => {
                errores.push(format!("Estudiante {}: apellidos faltantes", i + 1));
                continue;
            }
        };
        
        let genero = estudiante_json.get("genero")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let fecha_nacimiento = match estudiante_json.get("fecha_nacimiento") {
            Some(serde_json::Value::String(s)) => {
                match chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                    Ok(date) => date,
                    Err(_) => {
                        errores.push(format!("Estudiante {}: fecha de nacimiento inválida: {}", i + 1, s));
                        continue;
                    }
                }
            },
            _ => {
                errores.push(format!("Estudiante {}: fecha de nacimiento faltante", i + 1));
                continue;
            }
        };
        
        let id_grado_secciones = estudiante_json.get("id_grado_secciones")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        
        let fecha_ingreso = estudiante_json.get("fecha_ingreso")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
        
        let paisnac_id = estudiante_json.get("paisnac_id")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        
        let estado_nac_id = estudiante_json.get("estado_nac_id")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        
        let municipio_nac_id = estudiante_json.get("municipio_nac_id")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        
        let ciudad_nac_id = estudiante_json.get("ciudad_nac_id")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        
        let id_periodoactual = estudiante_json.get("id_periodoactual")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        
        let estado = match estudiante_json.get("estado").and_then(|v| v.as_str()) {
            Some("Activo") | Some("activo") => crate::models::estudiante::EstadoEstudiante::Activo,
            Some("Retirado") | Some("retirado") => crate::models::estudiante::EstadoEstudiante::Retirado,
            _ => crate::models::estudiante::EstadoEstudiante::Activo, // Por defecto
        };
        
        let fecha_retiro = estudiante_json.get("fecha_retiro")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
        
        // Insertar en la base de datos
        let res = db.execute(
            "INSERT INTO estudiantes (cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, paisnac_id, estado_nac_id, municipio_nac_id, ciudad_nac_id, id_periodoactual, estado, fecha_retiro) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) 
             ON CONFLICT (cedula) DO NOTHING",
            &[&cedula, &nombres, &apellidos, &genero, &fecha_nacimiento, &id_grado_secciones, &fecha_ingreso, &paisnac_id, &estado_nac_id, &municipio_nac_id, &ciudad_nac_id, &id_periodoactual, &estado, &fecha_retiro]
        ).await;
        
        match res {
            Ok(n) if n == 1 => {
                insertados += 1;
                println!("[DEBUG] Estudiante {} insertado exitosamente", cedula);
            },
            Ok(_) => {
                duplicados += 1;
                println!("[DEBUG] Estudiante {} ya existe (duplicado)", cedula);
            },
            Err(err) => {
                let error_msg = format!("{} {}: {}", cedula, nombres, err);
                errores.push(error_msg.clone());
                println!("[ERROR] {}", error_msg);
            }
        }
    }
    
    println!("[DEBUG] Resumen: Total: {}, Insertados: {}, Duplicados: {}, Errores: {}", 
             insertados + duplicados + errores.len(), insertados, duplicados, errores.len());
    
    Ok(serde_json::json!({
        "total_registros": insertados + duplicados + errores.len(),
        "insertados": insertados,
        "duplicados": duplicados,
        "errores": errores,
    }))
} 