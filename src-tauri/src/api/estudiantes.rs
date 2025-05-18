use tauri::State;
use crate::models::estudiante::{Estudiante, EstadoEstudiante, FiltroEstudiantes, NuevoEstudiante};
use crate::AppState;
use chrono::NaiveDate;
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
        e.id_grado_secciones, e.fecha_ingreso, e.municipionac, e.paisnac, e.entidadfed, \
        e.ciudadnac, e.estadonac, e.id_grado, g.nombre_grado, e.id_seccion, s.nombre_seccion, \
        e.id_modalidad, m.nombre_modalidad, e.id_periodoactual, e.estado, e.fecha_retiro \
        FROM estudiantes e \
        LEFT JOIN grados g ON e.id_grado = g.id_grado \
        LEFT JOIN secciones s ON e.id_seccion = s.id_seccion \
        LEFT JOIN modalidades m ON e.id_modalidad = m.id_modalidad \
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
            query_base.push_str(&format!(" AND e.id_grado = ${}", param_values.len() + 1));
            param_values.push(ParamValue::I32(grado_num));
        }
        if let Some(modalidad_num) = f.modalidad {
            query_base.push_str(&format!(" AND e.id_modalidad = ${}", param_values.len() + 1));
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
            municipionac: row.get(8),
            paisnac: row.get(9),
            entidadfed: row.get(10),
            ciudadnac: row.get(11),
            estadonac: row.get(12),
            id_grado: row.get(13),
            nombre_grado: row.get(14),
            id_seccion: row.get(15),
            nombre_seccion: row.get(16),
            id_modalidad: row.get(17),
            nombre_modalidad: row.get(18),
            id_periodoactual: row.get(19),
            estado: row.get(20),
            fecha_retiro: row.get(21),
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
        "INSERT INTO estudiantes (cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, municipionac, paisnac, entidadfed, ciudadnac, estadonac, id_grado, id_seccion, id_modalidad, id_periodoactual, estado, fecha_retiro) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)",
        &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.municipionac, &estudiante.paisnac, &estudiante.entidadfed, &estudiante.ciudadnac, &estudiante.estadonac, &estudiante.id_grado, &estudiante.id_seccion, &estudiante.id_modalidad, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro]
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
        "UPDATE estudiantes SET cedula=$1, nombres=$2, apellidos=$3, genero=$4, fecha_nacimiento=$5, id_grado_secciones=$6, fecha_ingreso=$7, municipionac=$8, paisnac=$9, entidadfed=$10, ciudadnac=$11, estadonac=$12, id_grado=$13, id_seccion=$14, id_modalidad=$15, id_periodoactual=$16, estado=$17, fecha_retiro=$18 WHERE id=$19",
        &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.municipionac, &estudiante.paisnac, &estudiante.entidadfed, &estudiante.ciudadnac, &estudiante.estadonac, &estudiante.id_grado, &estudiante.id_seccion, &estudiante.id_modalidad, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro, &id]
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
pub async fn insertar_estudiantes_masivo(
    estudiantes: Vec<serde_json::Value>,
    state: State<'_, AppState>,
) -> Result<ResumenInsercion, String> {
    let mut db = state.db.lock().await;
    let mut cedulas_existentes = obtener_cedulas_existentes(&*db).await?;
    let trans = db.transaction().await.map_err(|e| e.to_string())?;
    let mut resumen = ResumenInsercion {
        total_registros: estudiantes.len(),
        insertados: 0,
        duplicados: 0,
        errores: Vec::new(),
    };
    for (index, est_json) in estudiantes.iter().enumerate() {
        let estudiante: NuevoEstudiante = match serde_json::from_value(est_json.clone()) {
            Ok(est) => est,
            Err(e) => {
                let msg = format!("Fila {}: Error al deserializar - {}", index + 1, e);
                resumen.errores.push(msg);
                continue;
            }
        };
        if cedulas_existentes.contains(&estudiante.cedula) {
            let msg = format!("Fila {}: Cédula duplicada {}", index + 1, estudiante.cedula);
            resumen.duplicados += 1;
            resumen.errores.push(msg);
            continue;
        }
        match trans.execute(
            "INSERT INTO estudiantes (cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, municipionac, paisnac, entidadfed, ciudadnac, estadonac, id_grado, id_seccion, id_modalidad, id_periodoactual, estado, fecha_retiro) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18)",
            &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.municipionac, &estudiante.paisnac, &estudiante.entidadfed, &estudiante.ciudadnac, &estudiante.estadonac, &estudiante.id_grado, &estudiante.id_seccion, &estudiante.id_modalidad, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro]
        ).await {
            Ok(_) => {
                resumen.insertados += 1;
                cedulas_existentes.insert(estudiante.cedula);
            },
            Err(e) => {
                let msg = format!("Fila {}: Error al insertar - {}", index + 1, e);
                resumen.errores.push(msg);
            },
        }
    }
    if resumen.insertados == 0 {
        trans.rollback().await.map_err(|e| e.to_string())?;
    } else {
        trans.commit().await.map_err(|e| e.to_string())?;
    }
    Ok(resumen)
}

#[tauri::command]
pub async fn obtener_estudiante_por_id(
    id: i32,
    state: State<'_, AppState>,
) -> Result<Estudiante, String> {
    let db = state.db.lock().await;
    let row = db
        .query_one(
            "SELECT e.id, e.cedula, e.nombres, e.apellidos, e.genero, e.fecha_nacimiento, \
            e.id_grado_secciones, e.fecha_ingreso, e.municipionac, e.paisnac, e.entidadfed, \
            e.ciudadnac, e.estadonac, e.id_grado, g.nombre_grado, e.id_seccion, s.nombre_seccion, \
            e.id_modalidad, m.nombre_modalidad, e.id_periodoactual, e.estado, e.fecha_retiro \
            FROM estudiantes e \
            LEFT JOIN grados g ON e.id_grado = g.id_grado \
            LEFT JOIN secciones s ON e.id_seccion = s.id_seccion \
            LEFT JOIN modalidades m ON e.id_modalidad = m.id_modalidad \
            WHERE e.id = $1",
            &[&id],
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(Estudiante {
        id: row.get(0),
        cedula: row.get(1),
        nombres: row.get(2),
        apellidos: row.get(3),
        genero: row.get(4),
        fecha_nacimiento: row.get(5),
        id_grado_secciones: row.get(6),
        fecha_ingreso: row.get(7),
        municipionac: row.get(8),
        paisnac: row.get(9),
        entidadfed: row.get(10),
        ciudadnac: row.get(11),
        estadonac: row.get(12),
        id_grado: row.get(13),
        nombre_grado: row.get(14),
        id_seccion: row.get(15),
        nombre_seccion: row.get(16),
        id_modalidad: row.get(17),
        nombre_modalidad: row.get(18),
        id_periodoactual: row.get(19),
        estado: row.get(20),
        fecha_retiro: row.get(21),
    })
}

// Funciones auxiliares necesarias para estudiantes (por ejemplo, verificar_cedula_duplicada, obtener_cedulas_existentes, etc.) 