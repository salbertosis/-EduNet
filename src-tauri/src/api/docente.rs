use tauri::State;
use crate::AppState;
use crate::models::docente::{Docente, FiltroDocentes, NuevoDocente};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginacionParams {
    pub pagina: usize,
    pub registros_por_pagina: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginacionInfo {
    pub pagina_actual: usize,
    pub total_paginas: usize,
    pub total_registros: usize,
    pub registros_por_pagina: usize,
}

#[derive(Serialize)]
pub struct ResultadoPaginado<T> {
    pub datos: Vec<T>,
    pub paginacion: PaginacionInfo,
}

#[tauri::command]
pub async fn obtener_docentes(
    filtro: Option<FiltroDocentes>,
    paginacion: PaginacionParams,
    state: State<'_, AppState>,
) -> Result<ResultadoPaginado<Docente>, String> {
    let db = state.db.lock().await;
    let mut query_base = String::from(
        "SELECT id_docente, cedula, apellidos, nombres, especialidad, telefono, correoelectronico FROM docentes WHERE 1=1"
    );
    let mut param_strings: Vec<String> = Vec::new();
    if let Some(f) = &filtro {
        if let Some(cedula) = &f.cedula {
            query_base.push_str(" AND cedula::text ILIKE $");
            query_base.push_str(&(param_strings.len() + 1).to_string());
            param_strings.push(format!("{}%", cedula));
        }
        if let Some(apellidos) = &f.apellidos {
            query_base.push_str(" AND apellidos ILIKE $");
            query_base.push_str(&(param_strings.len() + 1).to_string());
            param_strings.push(format!("{}%", apellidos));
        }
        if let Some(especialidad) = &f.especialidad {
            query_base.push_str(" AND especialidad ILIKE $");
            query_base.push_str(&(param_strings.len() + 1).to_string());
            param_strings.push(format!("%{}%", especialidad));
        }
    }
    // Ahora construye params después de terminar de mutar param_strings
    let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = param_strings.iter().map(|s| s as &(dyn tokio_postgres::types::ToSql + Sync)).collect();
    // Paginación
    let offset = (paginacion.pagina - 1) * paginacion.registros_por_pagina;
    let count_query = format!("SELECT COUNT(*) FROM ({}) as sub", query_base);
    let total_registros: i64 = db.query_one(&count_query, &params).await.map_err(|e| e.to_string())?.get(0);
    let total_paginas = if paginacion.registros_por_pagina > 0 {
        ((total_registros as f64) / (paginacion.registros_por_pagina as f64)).ceil() as usize
    } else {
        1
    };
    query_base.push_str(&format!(" ORDER BY cedula ASC LIMIT {} OFFSET {}", paginacion.registros_por_pagina, offset));
    let rows = db.query(&query_base, &params).await.map_err(|e| e.to_string())?;
    let docentes = rows.iter().map(|row| Docente {
        id_docente: row.get(0),
        cedula: row.get(1),
        apellidos: row.get(2),
        nombres: row.get(3),
        especialidad: row.get(4),
        telefono: row.get(5),
        correoelectronico: row.get(6),
    }).collect::<Vec<_>>();
    let paginacion_info = PaginacionInfo {
        pagina_actual: paginacion.pagina,
        total_paginas,
        total_registros: total_registros as usize,
        registros_por_pagina: paginacion.registros_por_pagina,
    };
    Ok(ResultadoPaginado { datos: docentes, paginacion: paginacion_info })
}

#[tauri::command]
pub async fn crear_docente(
    docente: NuevoDocente,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    db.execute(
        "INSERT INTO docentes (cedula, apellidos, nombres, especialidad, telefono, correoelectronico) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&docente.cedula, &docente.apellidos, &docente.nombres, &docente.especialidad, &docente.telefono, &docente.correoelectronico]
    ).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn actualizar_docente(
    id_docente: i32,
    docente: NuevoDocente,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    db.execute(
        "UPDATE docentes SET cedula=$1, apellidos=$2, nombres=$3, especialidad=$4, telefono=$5, correoelectronico=$6 WHERE id_docente=$7",
        &[&docente.cedula, &docente.apellidos, &docente.nombres, &docente.especialidad, &docente.telefono, &docente.correoelectronico, &id_docente]
    ).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn eliminar_docente(
    id_docente: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    db.execute(
        "DELETE FROM docentes WHERE id_docente=$1",
        &[&id_docente]
    ).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn insertar_docentes_masivo(
    docentes: Vec<NuevoDocente>,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let db = state.db.lock().await;
    let mut insertados = 0;
    let mut duplicados = 0;
    let mut errores = vec![];
    for d in docentes {
        let res = db.execute(
            "INSERT INTO docentes (cedula, apellidos, nombres, especialidad, telefono, correoelectronico) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (cedula) DO NOTHING",
            &[&d.cedula, &d.apellidos, &d.nombres, &d.especialidad, &d.telefono, &d.correoelectronico]
        ).await;
        match res {
            Ok(n) if n == 1 => insertados += 1,
            Ok(_) => duplicados += 1,
            Err(e) => errores.push(format!("{} {}: {}", d.cedula, d.nombres, e)),
        }
    }
    Ok(serde_json::json!({
        "total_registros": insertados + duplicados + errores.len(),
        "insertados": insertados,
        "duplicados": duplicados,
        "errores": errores,
    }))
}

#[tauri::command]
pub async fn contar_docentes(state: State<'_, AppState>) -> Result<i64, String> {
    let db = state.db.lock().await;
    let row = db
        .query_one("SELECT COUNT(*) FROM docentes", &[])
        .await
        .map_err(|e| e.to_string())?;
    let total: i64 = row.get(0);
    Ok(total)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn asignar_docente_guia(
    id_grado_secciones: i32,
    id_docente_guia: i32,
    state: tauri::State<'_, crate::AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    db.execute(
        "UPDATE grado_secciones SET id_docente_guia = $1 WHERE id_grado_secciones = $2",
        &[&id_docente_guia, &id_grado_secciones]
    ).await.map_err(|e| e.to_string())?;
    Ok(())
} 