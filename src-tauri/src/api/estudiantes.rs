use tauri::State;
use crate::models::estudiante::{Estudiante, FiltroEstudiantes, NuevoEstudiante};
use crate::AppState;
use crate::utils::actividad_helper;
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
            // Campos legacy para compatibilidad
            municipionac: None,
            paisnac: None,
            entidadfed: None,
            ciudadnac: None,
            estadonac: None,
            // Datos académicos
            id_grado: None,
            id_seccion: None,
            id_modalidad: None,
            // Datos de nacimiento con IDs
            paisnac_id: None,
            estado_nac_id: None,
            municipio_nac_id: None,
            ciudad_nac_id: None,
            // Datos de nacimiento con nombres
            pais_nombre: None,
            estado_nombre: None,
            municipio_nombre: None,
            ciudad_nombre: None,
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
    let resultado = db.execute(
        "INSERT INTO estudiantes (cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, paisnac_id, estado_nac_id, municipio_nac_id, ciudad_nac_id, id_periodoactual, estado, fecha_retiro) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)",
        &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.paisnac_id, &estudiante.estado_nac_id, &estudiante.municipio_nac_id, &estudiante.ciudad_nac_id, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro]
    ).await;

    match resultado {
        Ok(_) => {
            // Obtener el ID del estudiante insertado
            let estudiante_insertado = db
                .query_one(
                    "SELECT id, cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, id_periodoactual, estado, fecha_retiro FROM estudiantes WHERE cedula = $1 ORDER BY id DESC LIMIT 1",
                    &[&estudiante.cedula]
                )
                .await
                .map_err(|e| e.to_string())?;

            let estado_str: String = estudiante_insertado.get("estado");
            let estado = if estado_str == "Activo" {
                crate::models::estudiante::EstadoEstudiante::Activo
            } else {
                crate::models::estudiante::EstadoEstudiante::Retirado
            };

            let estudiante_creado = Estudiante {
                id: estudiante_insertado.get("id"),
                cedula: estudiante_insertado.get("cedula"),
                nombres: estudiante_insertado.get("nombres"),
                apellidos: estudiante_insertado.get("apellidos"),
                genero: estudiante_insertado.get("genero"),
                fecha_nacimiento: estudiante_insertado.get("fecha_nacimiento"),
                id_grado_secciones: estudiante_insertado.get("id_grado_secciones"),
                fecha_ingreso: estudiante_insertado.get("fecha_ingreso"),
                id_periodoactual: estudiante_insertado.get("id_periodoactual"),
                estado,
                fecha_retiro: estudiante_insertado.get("fecha_retiro"),
                // Campos legacy
                municipionac: None,
                paisnac: None,
                entidadfed: None,
                ciudadnac: None,
                estadonac: None,
                // Datos académicos
                id_grado: None,
                nombre_grado: None,
                id_seccion: None,
                nombre_seccion: None,
                id_modalidad: None,
                nombre_modalidad: None,
                // Datos de nacimiento con IDs
                paisnac_id: None,
                estado_nac_id: None,
                municipio_nac_id: None,
                ciudad_nac_id: None,
                // Datos de nacimiento con nombres
                pais_nombre: None,
                estado_nombre: None,
                municipio_nombre: None,
                ciudad_nombre: None,
            };

            // Registrar actividad (no bloqueante)
            let _ = actividad_helper::registrar_actividad_estudiante(&*db, "crear", &estudiante_creado, "Admin").await;
            
            Ok(())
        },
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
    
    // 1. Verificar cédula duplicada
    let cedula_duplicada = verificar_cedula_duplicada(&*db, estudiante.cedula, Some(id)).await?;
    if cedula_duplicada {
        return Err("Ya existe otro estudiante con esta cédula".to_string());
    }
    
    // 2. Obtener datos actuales del estudiante para comparar
    let estudiante_actual = db
        .query_opt(
            "SELECT id_grado_secciones, estado FROM estudiantes WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|e| e.to_string())?;
    
    let (id_grado_secciones_anterior, estado_anterior) = match estudiante_actual {
        Some(row) => (row.get::<_, Option<i32>>(0), row.get::<_, crate::models::estudiante::EstadoEstudiante>(1)),
        None => return Err("No se encontró el estudiante".to_string()),
    };
    
    // 3. Actualizar tabla estudiantes
    db.execute(
        "UPDATE estudiantes SET cedula=$1, nombres=$2, apellidos=$3, genero=$4, fecha_nacimiento=$5, id_grado_secciones=$6, fecha_ingreso=$7, paisnac_id=$8, estado_nac_id=$9, municipio_nac_id=$10, ciudad_nac_id=$11, id_periodoactual=$12, estado=$13, fecha_retiro=$14 WHERE id=$15",
        &[&estudiante.cedula, &estudiante.nombres, &estudiante.apellidos, &estudiante.genero, &estudiante.fecha_nacimiento, &estudiante.id_grado_secciones, &estudiante.fecha_ingreso, &estudiante.paisnac_id, &estudiante.estado_nac_id, &estudiante.municipio_nac_id, &estudiante.ciudad_nac_id, &estudiante.id_periodoactual, &estudiante.estado, &estudiante.fecha_retiro, &id]
    )
    .await
    .map_err(|e| e.to_string())?;
    
    // 4. Manejar cambios en historial_grado_estudiantes
    let estado_actual = estudiante.estado.clone();
    match (estado_actual, estado_anterior) {
        // Caso: Cambio de estado a Retirado
        (crate::models::estudiante::EstadoEstudiante::Retirado, crate::models::estudiante::EstadoEstudiante::Activo) => {
            // Marcar como retirado en historial
            db.execute(
                "UPDATE historial_grado_estudiantes 
                 SET estado = 'retirado' 
                 WHERE id_estudiante = $1 AND es_actual = TRUE",
                &[&id]
            )
            .await
            .map_err(|e| e.to_string())?;
        },
        
        // Caso: Cambio de estado a Activo (reactivación)
        (crate::models::estudiante::EstadoEstudiante::Activo, crate::models::estudiante::EstadoEstudiante::Retirado) => {
            // Buscar registro retirado y reactivarlo
            let registro_retirado = db
                .query_opt(
                    "SELECT id_historial_grado FROM historial_grado_estudiantes 
                     WHERE id_estudiante = $1 AND estado = 'retirado' 
                     ORDER BY id_historial_grado DESC LIMIT 1",
                    &[&id]
                )
                .await
                .map_err(|e| e.to_string())?;
            
            match registro_retirado {
                Some(_) => {
                    // Reactivar el registro anterior
                    db.execute(
                        "UPDATE historial_grado_estudiantes 
                         SET estado = 'activo', es_actual = TRUE 
                         WHERE id_estudiante = $1 AND estado = 'retirado'",
                        &[&id]
                    )
                    .await
                    .map_err(|e| e.to_string())?;
                },
                None => {
                    // Crear nuevo registro si no existe uno anterior
                    if let Some(id_grado_secciones) = estudiante.id_grado_secciones {
                        if let Some(id_periodo) = estudiante.id_periodoactual {
                            db.execute(
                                "INSERT INTO historial_grado_estudiantes 
                                 (id_estudiante, id_grado_secciones, id_periodo, fecha_inicio, es_actual, estado) 
                                 VALUES ($1, $2, $3, CURRENT_DATE, TRUE, 'activo')",
                                &[&id, &id_grado_secciones, &id_periodo]
                            )
                            .await
                            .map_err(|e| e.to_string())?;
                        }
                    }
                }
            }
        },
        
        // Caso: Cambio de grado/sección (estado sigue siendo Activo)
        (crate::models::estudiante::EstadoEstudiante::Activo, crate::models::estudiante::EstadoEstudiante::Activo) => {
            // Verificar si cambió el id_grado_secciones
            if id_grado_secciones_anterior != estudiante.id_grado_secciones {
                if let Some(nuevo_id_grado_secciones) = estudiante.id_grado_secciones {
                    // Actualizar el registro activo en historial
                    db.execute(
                        "UPDATE historial_grado_estudiantes 
                         SET id_grado_secciones = $1 
                         WHERE id_estudiante = $2 AND es_actual = TRUE",
                        &[&nuevo_id_grado_secciones, &id]
                    )
                    .await
                    .map_err(|e| e.to_string())?;
                }
            }
        },
        
            // Otros casos: no hacer nada especial
    _ => {}
}

// Registrar actividad de actualización
let estudiante_actualizado = Estudiante {
    id,
    cedula: estudiante.cedula,
    nombres: estudiante.nombres.clone(),
    apellidos: estudiante.apellidos.clone(),
    genero: estudiante.genero.clone(),
    fecha_nacimiento: estudiante.fecha_nacimiento,
    id_grado_secciones: estudiante.id_grado_secciones,
    fecha_ingreso: estudiante.fecha_ingreso,
    id_periodoactual: estudiante.id_periodoactual,
    estado: estudiante.estado,
    fecha_retiro: estudiante.fecha_retiro,
    // Campos legacy
    municipionac: None,
    paisnac: None,
    entidadfed: None,
    ciudadnac: None,
    estadonac: None,
    // Datos académicos
    id_grado: None,
    nombre_grado: None,
    id_seccion: None,
    nombre_seccion: None,
    id_modalidad: None,
    nombre_modalidad: None,
    // Datos de nacimiento con IDs
    paisnac_id: None,
    estado_nac_id: None,
    municipio_nac_id: None,
    ciudad_nac_id: None,
    // Datos de nacimiento con nombres
    pais_nombre: None,
    estado_nombre: None,
    municipio_nombre: None,
    ciudad_nombre: None,
};

// Registrar actividad (no bloqueante)
let _ = actividad_helper::registrar_actividad_estudiante(&*db, "actualizar", &estudiante_actualizado, "Admin").await;

Ok(())
}

#[tauri::command]
pub async fn eliminar_estudiante(
    id: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    
    // Obtener datos del estudiante antes de eliminar
    let estudiante_eliminado = db
        .query_opt(
            "SELECT id, cedula, nombres, apellidos, genero, fecha_nacimiento, id_grado_secciones, fecha_ingreso, id_periodoactual, estado, fecha_retiro FROM estudiantes WHERE id = $1",
            &[&id]
        )
        .await
        .map_err(|e| e.to_string())?;
    
    // Eliminar el estudiante
    db.execute(
        "DELETE FROM estudiantes WHERE id = $1",
        &[&id],
    )
    .await
    .map_err(|e| e.to_string())?;
    
    // Registrar actividad si se encontró el estudiante
    if let Some(row) = estudiante_eliminado {
        let estado_str: String = row.get("estado");
        let estado = if estado_str == "Activo" {
            crate::models::estudiante::EstadoEstudiante::Activo
        } else {
            crate::models::estudiante::EstadoEstudiante::Retirado
        };

        let estudiante = Estudiante {
            id: row.get("id"),
            cedula: row.get("cedula"),
            nombres: row.get("nombres"),
            apellidos: row.get("apellidos"),
            genero: row.get("genero"),
            fecha_nacimiento: row.get("fecha_nacimiento"),
            id_grado_secciones: row.get("id_grado_secciones"),
            fecha_ingreso: row.get("fecha_ingreso"),
            id_periodoactual: row.get("id_periodoactual"),
            estado,
            fecha_retiro: row.get("fecha_retiro"),
            // Campos legacy
            municipionac: None,
            paisnac: None,
            entidadfed: None,
            ciudadnac: None,
            estadonac: None,
            // Datos académicos
            id_grado: None,
            nombre_grado: None,
            id_seccion: None,
            nombre_seccion: None,
            id_modalidad: None,
            nombre_modalidad: None,
            // Datos de nacimiento con IDs
            paisnac_id: None,
            estado_nac_id: None,
            municipio_nac_id: None,
            ciudad_nac_id: None,
            // Datos de nacimiento con nombres
            pais_nombre: None,
            estado_nombre: None,
            municipio_nombre: None,
            ciudad_nombre: None,
        };
        
        // Registrar actividad (no bloqueante)
        let _ = actividad_helper::registrar_actividad_estudiante(&*db, "eliminar", &estudiante, "Admin").await;
    }
    
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
    println!("[DEBUG] Obteniendo estudiante con ID: {}", id);
    let db = state.db.lock().await;
    let row_opt = db
        .query_opt(
            "SELECT e.id, e.cedula, e.nombres, e.apellidos, e.genero, e.fecha_nacimiento, \
            e.id_grado_secciones, e.fecha_ingreso, e.id_periodoactual, e.estado, e.fecha_retiro, \
            gs.id_grado, g.nombre_grado, gs.id_seccion, s.nombre_seccion, \
            gs.id_modalidad, m.nombre_modalidad, \
            e.paisnac_id, e.estado_nac_id, e.municipio_nac_id, e.ciudad_nac_id, \
            p.nombre as pais_nombre, est.nombre as estado_nombre, \
            mun.nombre as municipio_nombre, c.nombre as ciudad_nombre \
            FROM estudiantes e \
            LEFT JOIN grado_secciones gs ON e.id_grado_secciones = gs.id_grado_secciones \
            LEFT JOIN grados g ON gs.id_grado = g.id_grado \
            LEFT JOIN secciones s ON gs.id_seccion = s.id_seccion \
            LEFT JOIN modalidades m ON gs.id_modalidad = m.id_modalidad \
            LEFT JOIN paises p ON e.paisnac_id = p.id \
            LEFT JOIN estados est ON e.estado_nac_id = est.id \
            LEFT JOIN municipios mun ON e.municipio_nac_id = mun.id \
            LEFT JOIN ciudades c ON e.ciudad_nac_id = c.id \
            WHERE e.id = $1",
            &[&id],
        )
        .await
        .map_err(|e| e.to_string())?;
    let row = match row_opt {
        Some(r) => {
            println!("[DEBUG] Estudiante encontrado: cedula={}, nombres={}, apellidos={}", 
                r.get::<_, i64>(1), r.get::<_, String>(2), r.get::<_, String>(3));
            println!("[DEBUG] Datos de nacimiento: paisnac_id={:?}, estado_nac_id={:?}, municipio_nac_id={:?}, ciudad_nac_id={:?}", 
                r.get::<_, Option<i32>>(17), r.get::<_, Option<i32>>(18), 
                r.get::<_, Option<i32>>(19), r.get::<_, Option<i32>>(20));
            println!("[DEBUG] Nombres de nacimiento: pais={:?}, estado={:?}, municipio={:?}, ciudad={:?}", 
                r.get::<_, Option<String>>(21), r.get::<_, Option<String>>(22), 
                r.get::<_, Option<String>>(23), r.get::<_, Option<String>>(24));
            r
        },
        None => return Err(format!("No se encontró el estudiante con id {}", id)),
    };
    let estudiante = Estudiante {
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
        id_grado: row.get(11),
        nombre_grado: row.get(12),
        id_seccion: row.get(13),
        nombre_seccion: row.get(14),
        id_modalidad: row.get(15),
        nombre_modalidad: row.get(16),
        // Datos de nacimiento con IDs
        paisnac_id: row.get(17),
        estado_nac_id: row.get(18),
        municipio_nac_id: row.get(19),
        ciudad_nac_id: row.get(20),
        // Datos de nacimiento con nombres
        pais_nombre: row.get(21),
        estado_nombre: row.get(22),
        municipio_nombre: row.get(23),
        ciudad_nombre: row.get(24),
        // Campos legacy para compatibilidad
        municipionac: row.get(23), // municipio_nombre
        paisnac: row.get(21),      // pais_nombre
        entidadfed: row.get(22),   // estado_nombre
        ciudadnac: row.get(24),    // ciudad_nombre
        estadonac: row.get(22),    // estado_nombre
    };
    
    // Verificar que los datos se asignaron correctamente
    println!("[DEBUG] ===== ESTUDIANTE CONSTRUIDO EN BACKEND =====");
    println!("[DEBUG] ID: {}", estudiante.id);
    println!("[DEBUG] Datos de nacimiento - IDs:");
    println!("[DEBUG]   paisnac_id: {:?}", estudiante.paisnac_id);
    println!("[DEBUG]   estado_nac_id: {:?}", estudiante.estado_nac_id);
    println!("[DEBUG]   municipio_nac_id: {:?}", estudiante.municipio_nac_id);
    println!("[DEBUG]   ciudad_nac_id: {:?}", estudiante.ciudad_nac_id);
    println!("[DEBUG] Datos de nacimiento - Nombres:");
    println!("[DEBUG]   pais_nombre: {:?}", estudiante.pais_nombre);
    println!("[DEBUG]   estado_nombre: {:?}", estudiante.estado_nombre);
    println!("[DEBUG]   municipio_nombre: {:?}", estudiante.municipio_nombre);
    println!("[DEBUG]   ciudad_nombre: {:?}", estudiante.ciudad_nombre);
    println!("[DEBUG] ============================================");
    println!("[DEBUG] Estudiante construido: {:?}", estudiante);
    Ok(estudiante)
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

#[derive(Debug, serde::Serialize)]
pub struct EstadisticasModalidad {
    pub total_estudiantes: i64,
    pub estudiantes_femeninos: i64,
    pub estudiantes_masculinos: i64,
}

#[tauri::command]
pub async fn obtener_estadisticas_estudiantes_por_modalidad(state: State<'_, AppState>) -> Result<Vec<EstadisticasModalidad>, String> {
    let db = state.db.lock().await;
    
    // Query para obtener estadísticas por modalidad
    let query = "
        SELECT 
            gs.id_modalidad,
            COUNT(DISTINCT hge.id_estudiante) as total_estudiantes,
            SUM(CASE WHEN e.genero = 'F' THEN 1 ELSE 0 END) as estudiantes_femeninos,
            SUM(CASE WHEN e.genero = 'M' THEN 1 ELSE 0 END) as estudiantes_masculinos
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        WHERE hge.es_actual = true 
            AND hge.estado = 'activo'
            AND e.estado = 'Activo'
        GROUP BY gs.id_modalidad
        ORDER BY gs.id_modalidad
    ";
    
    let rows = db.query(query, &[])
        .await
        .map_err(|e| format!("Error obteniendo estadísticas por modalidad: {}", e))?;
    
    let estadisticas = rows.iter().map(|row| EstadisticasModalidad {
        total_estudiantes: row.get("total_estudiantes"),
        estudiantes_femeninos: row.get("estudiantes_femeninos"),
        estudiantes_masculinos: row.get("estudiantes_masculinos"),
    }).collect();
    
    Ok(estadisticas)
}

#[tauri::command]
pub async fn obtener_estadisticas_telematica(state: State<'_, AppState>) -> Result<EstadisticasModalidad, String> {
    let db = state.db.lock().await;
    
    // Query corregida para Telemática (id_modalidad = 2) - solo es_actual = true
    let query = "
        SELECT 
            COUNT(DISTINCT hge.id_estudiante) as total_estudiantes,
            SUM(CASE WHEN e.genero = 'F' THEN 1 ELSE 0 END) as estudiantes_femeninos,
            SUM(CASE WHEN e.genero = 'M' THEN 1 ELSE 0 END) as estudiantes_masculinos
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        WHERE hge.es_actual = true 
            AND gs.id_modalidad = 2
    ";
    
    let row = db.query_one(query, &[])
        .await
        .map_err(|e| format!("Error obteniendo estadísticas de Telemática: {}", e))?;
    
    Ok(EstadisticasModalidad {
        total_estudiantes: row.get("total_estudiantes"),
        estudiantes_femeninos: row.get("estudiantes_femeninos"),
        estudiantes_masculinos: row.get("estudiantes_masculinos"),
    })
}

#[tauri::command]
pub async fn obtener_estadisticas_ciencias(state: State<'_, AppState>) -> Result<EstadisticasModalidad, String> {
    let db = state.db.lock().await;
    
    // Query específica para Ciencias (id_modalidad = 1) - solo es_actual = true
    let query = "
        SELECT 
            COUNT(DISTINCT hge.id_estudiante) as total_estudiantes,
            SUM(CASE WHEN e.genero = 'F' THEN 1 ELSE 0 END) as estudiantes_femeninos,
            SUM(CASE WHEN e.genero = 'M' THEN 1 ELSE 0 END) as estudiantes_masculinos
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        WHERE hge.es_actual = true 
            AND gs.id_modalidad = 1
    ";
    
    let row = db.query_one(query, &[])
        .await
        .map_err(|e| format!("Error obteniendo estadísticas de Ciencias: {}", e))?;
    
    Ok(EstadisticasModalidad {
        total_estudiantes: row.get("total_estudiantes"),
        estudiantes_femeninos: row.get("estudiantes_femeninos"),
        estudiantes_masculinos: row.get("estudiantes_masculinos"),
    })
} 

#[derive(Debug, serde::Serialize)]
pub struct EstadisticasCursoPorGrado {
    pub id_grado: i32,
    pub nombre_grado: String,
    pub id_modalidad: i32,
    pub nombre_modalidad: String,
    pub total_estudiantes: i64,
    pub secciones: Vec<String>,
}

#[tauri::command]
pub async fn obtener_estadisticas_cursos_por_grado_modalidad(state: State<'_, AppState>) -> Result<Vec<EstadisticasCursoPorGrado>, String> {
    let db = state.db.lock().await;
    
    // Query para obtener estadísticas de cursos por grado y modalidad
    let query = "
        SELECT 
            g.id_grado,
            g.nombre_grado,
            gs.id_modalidad,
            m.nombre_modalidad,
            COUNT(DISTINCT hge.id_estudiante) as total_estudiantes,
            STRING_AGG(DISTINCT s.nombre_seccion, ',' ORDER BY s.nombre_seccion) as secciones
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        WHERE hge.es_actual = true 
            AND hge.estado = 'activo'
            AND e.estado = 'Activo'
        GROUP BY g.id_grado, g.nombre_grado, gs.id_modalidad, m.nombre_modalidad
        ORDER BY g.id_grado, gs.id_modalidad
    ";
    
    let rows = db.query(query, &[])
        .await
        .map_err(|e| format!("Error obteniendo estadísticas de cursos por grado: {}", e))?;
    
    let estadisticas = rows.iter().map(|row| {
        let secciones_str: String = row.get("secciones");
        let secciones = secciones_str.split(',').map(|s| s.trim().to_string()).collect();
        
        EstadisticasCursoPorGrado {
            id_grado: row.get("id_grado"),
            nombre_grado: row.get("nombre_grado"),
            id_modalidad: row.get("id_modalidad"),
            nombre_modalidad: row.get("nombre_modalidad"),
            total_estudiantes: row.get("total_estudiantes"),
            secciones,
        }
    }).collect();
    
    Ok(estadisticas)
}

#[tauri::command]
pub async fn obtener_estadisticas_cursos_telematica_por_grado(state: State<'_, AppState>) -> Result<Vec<EstadisticasCursoPorGrado>, String> {
    let db = state.db.lock().await;
    
    // Query específica para Telemática (id_modalidad = 2) por grado
    let query = "
        SELECT 
            g.id_grado,
            g.nombre_grado,
            gs.id_modalidad,
            m.nombre_modalidad,
            COUNT(DISTINCT hge.id_estudiante) as total_estudiantes,
            STRING_AGG(DISTINCT s.nombre_seccion, ',' ORDER BY s.nombre_seccion) as secciones
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        WHERE hge.es_actual = true 
            AND hge.estado = 'activo'
            AND e.estado = 'Activo'
            AND gs.id_modalidad = 2
        GROUP BY g.id_grado, g.nombre_grado, gs.id_modalidad, m.nombre_modalidad
        ORDER BY g.id_grado
    ";
    
    println!("[DEBUG] Ejecutando query Telemática: {}", query);
    
    let rows = db.query(query, &[])
        .await
        .map_err(|e| format!("Error obteniendo estadísticas de cursos Telemática por grado: {}", e))?;
    
    println!("[DEBUG] Filas obtenidas para Telemática: {}", rows.len());
    
    let estadisticas = rows.iter().map(|row| {
        let secciones_str: String = row.get("secciones");
        let secciones = secciones_str.split(',').map(|s| s.trim().to_string()).collect();
        
        let estadistica = EstadisticasCursoPorGrado {
            id_grado: row.get("id_grado"),
            nombre_grado: row.get("nombre_grado"),
            id_modalidad: row.get("id_modalidad"),
            nombre_modalidad: row.get("nombre_modalidad"),
            total_estudiantes: row.get("total_estudiantes"),
            secciones,
        };
        
        println!("[DEBUG] Estadística Telemática: {:?}", estadistica);
        estadistica
    }).collect();
    
    Ok(estadisticas)
}

#[tauri::command]
pub async fn obtener_estadisticas_cursos_ciencias_por_grado(state: State<'_, AppState>) -> Result<Vec<EstadisticasCursoPorGrado>, String> {
    let db = state.db.lock().await;
    
    // Query específica para Ciencias (id_modalidad = 1) por grado
    let query = "
        SELECT 
            g.id_grado,
            g.nombre_grado,
            gs.id_modalidad,
            m.nombre_modalidad,
            COUNT(DISTINCT hge.id_estudiante) as total_estudiantes,
            STRING_AGG(DISTINCT s.nombre_seccion, ',' ORDER BY s.nombre_seccion) as secciones
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        WHERE hge.es_actual = true 
            AND hge.estado = 'activo'
            AND e.estado = 'Activo'
            AND gs.id_modalidad = 1
        GROUP BY g.id_grado, g.nombre_grado, gs.id_modalidad, m.nombre_modalidad
        ORDER BY g.id_grado
    ";
    
    println!("[DEBUG] Ejecutando query Ciencias: {}", query);
    
    let rows = db.query(query, &[])
        .await
        .map_err(|e| format!("Error obteniendo estadísticas de cursos Ciencias por grado: {}", e))?;
    
    println!("[DEBUG] Filas obtenidas para Ciencias: {}", rows.len());
    
    let estadisticas = rows.iter().map(|row| {
        let secciones_str: String = row.get("secciones");
        let secciones = secciones_str.split(',').map(|s| s.trim().to_string()).collect();
        
        let estadistica = EstadisticasCursoPorGrado {
            id_grado: row.get("id_grado"),
            nombre_grado: row.get("nombre_grado"),
            id_modalidad: row.get("id_modalidad"),
            nombre_modalidad: row.get("nombre_modalidad"),
            total_estudiantes: row.get("total_estudiantes"),
            secciones,
        };
        
        println!("[DEBUG] Estadística Ciencias: {:?}", estadistica);
        estadistica
    }).collect();
    
    Ok(estadisticas)
} 

#[tauri::command]
pub async fn obtener_estadisticas_cursos_sin_filtros(state: State<'_, AppState>) -> Result<Vec<EstadisticasCursoPorGrado>, String> {
    let db = state.db.lock().await;
    
    // Query sin filtros para debug
    let query = "
        SELECT 
            g.id_grado,
            g.nombre_grado,
            gs.id_modalidad,
            m.nombre_modalidad,
            COUNT(DISTINCT hge.id_estudiante) as total_estudiantes,
            STRING_AGG(DISTINCT s.nombre_seccion, ',' ORDER BY s.nombre_seccion) as secciones
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        GROUP BY g.id_grado, g.nombre_grado, gs.id_modalidad, m.nombre_modalidad
        ORDER BY g.id_grado, gs.id_modalidad
    ";
    
    println!("[DEBUG] Ejecutando query sin filtros: {}", query);
    
    let rows = db.query(query, &[])
        .await
        .map_err(|e| format!("Error obteniendo estadísticas sin filtros: {}", e))?;
    
    println!("[DEBUG] Filas obtenidas sin filtros: {}", rows.len());
    
    let estadisticas = rows.iter().map(|row| {
        let secciones_str: String = row.get("secciones");
        let secciones = secciones_str.split(',').map(|s| s.trim().to_string()).collect();
        
        let estadistica = EstadisticasCursoPorGrado {
            id_grado: row.get("id_grado"),
            nombre_grado: row.get("nombre_grado"),
            id_modalidad: row.get("id_modalidad"),
            nombre_modalidad: row.get("nombre_modalidad"),
            total_estudiantes: row.get("total_estudiantes"),
            secciones,
        };
        
        println!("[DEBUG] Estadística sin filtros: {:?}", estadistica);
        estadistica
    }).collect();
    
    Ok(estadisticas)
}

#[tauri::command]
pub async fn probar_datos_estudiantes(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let db = state.db.lock().await;
    
    // Consulta para ver qué datos tenemos
    let query = "
        SELECT 
            COUNT(*) as total_historial,
            COUNT(DISTINCT hge.id_estudiante) as estudiantes_unicos,
            COUNT(DISTINCT hge.id_grado_secciones) as grado_secciones_unicos,
            COUNT(CASE WHEN hge.es_actual = true THEN 1 END) as actuales,
            COUNT(CASE WHEN hge.estado = 'activo' THEN 1 END) as activos
        FROM historial_grado_estudiantes hge
    ";
    
    let row = db.query_one(query, &[])
        .await
        .map_err(|e| format!("Error en consulta de prueba: {}", e))?;
    
    let resultado = serde_json::json!({
        "total_historial": row.get::<_, i64>("total_historial"),
        "estudiantes_unicos": row.get::<_, i64>("estudiantes_unicos"),
        "grado_secciones_unicos": row.get::<_, i64>("grado_secciones_unicos"),
        "actuales": row.get::<_, i64>("actuales"),
        "activos": row.get::<_, i64>("activos")
    });
    
    Ok(resultado)
}

#[tauri::command]
pub async fn probar_grado_secciones(state: State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let db = state.db.lock().await;
    
    // Consulta para ver los datos de grado_secciones
    let query = "
        SELECT 
            gs.id_grado_secciones,
            gs.id_grado,
            gs.id_seccion,
            gs.id_modalidad,
            g.nombre_grado,
            s.nombre_seccion,
            m.nombre_modalidad
        FROM grado_secciones gs
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
        ORDER BY gs.id_grado_secciones
        LIMIT 10
    ";
    
    let rows = db.query(query, &[])
        .await
        .map_err(|e| format!("Error en consulta grado_secciones: {}", e))?;
    
    let resultado = rows.iter().map(|row| {
        serde_json::json!({
            "id_grado_secciones": row.get::<_, i32>("id_grado_secciones"),
            "id_grado": row.get::<_, i32>("id_grado"),
            "id_seccion": row.get::<_, i32>("id_seccion"),
            "id_modalidad": row.get::<_, i32>("id_modalidad"),
            "nombre_grado": row.get::<_, String>("nombre_grado"),
            "nombre_seccion": row.get::<_, String>("nombre_seccion"),
            "nombre_modalidad": row.get::<_, String>("nombre_modalidad")
        })
    }).collect();
    
    Ok(resultado)
} 

#[derive(Debug, serde::Serialize)]
pub struct EstadisticasCursosCompacta {
    pub modalidad: String,
    pub total_secciones: i64,
    pub total_estudiantes: i64,
    pub promedio_por_seccion: f64,
    pub seccion_mayor_matricula: String,
    pub estudiantes_seccion_mayor: i64,
    pub seccion_menor_matricula: String,
    pub estudiantes_seccion_menor: i64,
}

#[tauri::command]
pub async fn obtener_estadisticas_cursos_compacta(state: State<'_, AppState>) -> Result<Vec<EstadisticasCursosCompacta>, String> {
    let db = state.db.lock().await;
    
    // Query para obtener estadísticas compactas por modalidad
    let query = "
        WITH estadisticas_secciones AS (
            SELECT 
                m.nombre_modalidad,
                g.nombre_grado,
                s.nombre_seccion,
                COUNT(DISTINCT hge.id_estudiante) as estudiantes_seccion,
                CONCAT(g.nombre_grado, ' ', s.nombre_seccion) as nombre_completo_seccion
            FROM historial_grado_estudiantes hge
            JOIN estudiantes e ON hge.id_estudiante = e.id
            JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
            JOIN grados g ON gs.id_grado = g.id_grado
            JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
            JOIN secciones s ON gs.id_seccion = s.id_seccion
            WHERE hge.es_actual = true 
                AND hge.estado = 'activo'
                AND e.estado = 'Activo'
            GROUP BY m.nombre_modalidad, g.nombre_grado, s.nombre_seccion
        ),
        resumen_modalidad AS (
            SELECT 
                nombre_modalidad,
                COUNT(*) as total_secciones,
                SUM(estudiantes_seccion) as total_estudiantes,
                ROUND(AVG(estudiantes_seccion), 1) as promedio_por_seccion,
                MAX(estudiantes_seccion) as max_estudiantes,
                MIN(estudiantes_seccion) as min_estudiantes
            FROM estadisticas_secciones
            GROUP BY nombre_modalidad
        )
        SELECT 
            rm.nombre_modalidad,
            rm.total_secciones,
            rm.total_estudiantes,
            rm.promedio_por_seccion,
            (SELECT nombre_completo_seccion FROM estadisticas_secciones es 
             WHERE es.nombre_modalidad = rm.nombre_modalidad 
             AND es.estudiantes_seccion = rm.max_estudiantes LIMIT 1) as seccion_mayor_matricula,
            rm.max_estudiantes as estudiantes_seccion_mayor,
            (SELECT nombre_completo_seccion FROM estadisticas_secciones es 
             WHERE es.nombre_modalidad = rm.nombre_modalidad 
             AND es.estudiantes_seccion = rm.min_estudiantes LIMIT 1) as seccion_menor_matricula,
            rm.min_estudiantes as estudiantes_seccion_menor
        FROM resumen_modalidad rm
        ORDER BY rm.nombre_modalidad
    ";
    
    let rows = db.query(query, &[])
        .await
        .map_err(|e| format!("Error obteniendo estadísticas compactas: {}", e))?;
    
    let estadisticas = rows.iter().map(|row| EstadisticasCursosCompacta {
        modalidad: row.get("nombre_modalidad"),
        total_secciones: row.get("total_secciones"),
        total_estudiantes: row.get("total_estudiantes"),
        promedio_por_seccion: row.get("promedio_por_seccion"),
        seccion_mayor_matricula: row.get("seccion_mayor_matricula"),
        estudiantes_seccion_mayor: row.get("estudiantes_seccion_mayor"),
        seccion_menor_matricula: row.get("seccion_menor_matricula"),
        estudiantes_seccion_menor: row.get("estudiantes_seccion_menor"),
    }).collect();
    
    Ok(estadisticas)
} 

#[derive(Debug, serde::Serialize)]
pub struct EstadisticasCursosResumen {
    pub seccion_mayor_matricula: String,
    pub estudiantes_mayor: i64,
    pub seccion_menor_matricula: String,
    pub estudiantes_menor: i64,
    pub promedio_estudiantes_por_seccion: f64,
}

#[tauri::command]
pub async fn obtener_estadisticas_cursos_resumen(state: State<'_, AppState>) -> Result<EstadisticasCursosResumen, String> {
    let db = state.db.lock().await;
    
    println!("[DEBUG] Iniciando consulta de estadísticas resumen");
    
    // Primero, obtener todas las secciones con sus conteos
    let query_secciones = "
        SELECT 
            CONCAT(g.nombre_grado, ' sección ', s.nombre_seccion) as nombre_completo_seccion,
            COUNT(DISTINCT hge.id_estudiante) as estudiantes_seccion
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        WHERE hge.es_actual = true 
            AND hge.estado = 'activo'
            AND e.estado = 'Activo'
        GROUP BY g.nombre_grado, s.nombre_seccion
        ORDER BY estudiantes_seccion DESC
    ";
    
    let rows = db.query(query_secciones, &[])
        .await
        .map_err(|e| {
            println!("[DEBUG] Error en consulta de secciones: {}", e);
            format!("Error obteniendo secciones: {}", e)
        })?;
    
    println!("[DEBUG] Secciones encontradas: {}", rows.len());
    
    if rows.is_empty() {
        println!("[DEBUG] No se encontraron secciones con datos");
        return Ok(EstadisticasCursosResumen {
            seccion_mayor_matricula: "Sin datos".to_string(),
            estudiantes_mayor: 0,
            seccion_menor_matricula: "Sin datos".to_string(),
            estudiantes_menor: 0,
            promedio_estudiantes_por_seccion: 0.0,
        });
    }
    
    // Obtener estadísticas
    let total_estudiantes: i64 = rows.iter().map(|row| row.get::<_, i64>("estudiantes_seccion")).sum();
    let promedio = total_estudiantes as f64 / rows.len() as f64;
    
    let seccion_mayor = &rows[0]; // Primera fila (ordenada DESC)
    let seccion_menor = &rows[rows.len() - 1]; // Última fila
    
    let estadisticas = EstadisticasCursosResumen {
        seccion_mayor_matricula: seccion_mayor.get("nombre_completo_seccion"),
        estudiantes_mayor: seccion_mayor.get("estudiantes_seccion"),
        seccion_menor_matricula: seccion_menor.get("nombre_completo_seccion"),
        estudiantes_menor: seccion_menor.get("estudiantes_seccion"),
        promedio_estudiantes_por_seccion: promedio,
    };
    
    println!("[DEBUG] Estadísticas calculadas: {:?}", estadisticas);
    
    Ok(estadisticas)
} 

#[tauri::command]
pub async fn debug_estadisticas_cursos(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let db = state.db.lock().await;
    
    // Debug 1: Verificar datos en historial_grado_estudiantes
    let query1 = "
        SELECT 
            COUNT(*) as total_historial,
            COUNT(DISTINCT id_grado_secciones) as grado_secciones_unicos,
            COUNT(CASE WHEN es_actual = true THEN 1 END) as actuales,
            COUNT(CASE WHEN estado = 'activo' THEN 1 END) as activos
        FROM historial_grado_estudiantes
    ";
    
    let row1 = db.query_one(query1, &[])
        .await
        .map_err(|e| format!("Error en debug 1: {}", e))?;
    
    // Debug 2: Verificar algunos registros de historial_grado_estudiantes
    let query2 = "
        SELECT 
            hge.id_grado_secciones,
            hge.es_actual,
            hge.estado,
            e.estado as estado_estudiante
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        LIMIT 5
    ";
    
    let rows2 = db.query(query2, &[])
        .await
        .map_err(|e| format!("Error en debug 2: {}", e))?;
    
    // Debug 3: Verificar grado_secciones
    let query3 = "
        SELECT 
            gs.id_grado_secciones,
            g.nombre_grado,
            s.nombre_seccion,
            m.nombre_modalidad
        FROM grado_secciones gs
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
        LIMIT 5
    ";
    
    let rows3 = db.query(query3, &[])
        .await
        .map_err(|e| format!("Error en debug 3: {}", e))?;
    
    let resultado = serde_json::json!({
        "historial_grado_estudiantes": {
            "total": row1.get::<_, i64>("total_historial"),
            "grado_secciones_unicos": row1.get::<_, i64>("grado_secciones_unicos"),
            "actuales": row1.get::<_, i64>("actuales"),
            "activos": row1.get::<_, i64>("activos")
        },
        "muestras_historial": rows2.iter().map(|row| {
            serde_json::json!({
                "id_grado_secciones": row.get::<_, i32>("id_grado_secciones"),
                "es_actual": row.get::<_, bool>("es_actual"),
                "estado": row.get::<_, String>("estado"),
                "estado_estudiante": row.get::<_, String>("estado_estudiante")
            })
        }).collect::<Vec<_>>(),
        "muestras_grado_secciones": rows3.iter().map(|row| {
            serde_json::json!({
                "id_grado_secciones": row.get::<_, i32>("id_grado_secciones"),
                "nombre_grado": row.get::<_, String>("nombre_grado"),
                "nombre_seccion": row.get::<_, String>("nombre_seccion"),
                "nombre_modalidad": row.get::<_, String>("nombre_modalidad")
            })
        }).collect::<Vec<_>>()
    });
    
    Ok(resultado)
} 

#[tauri::command]
pub async fn debug_estadisticas_modalidad(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let db = state.db.lock().await;
    
    // Consulta para diagnosticar el problema con las estadísticas por modalidad
    let query = "
        SELECT 
            'total_estudiantes' as tipo,
            COUNT(*) as valor
        FROM estudiantes
        
        UNION ALL
        
        SELECT 
            'estudiantes_en_historial' as tipo,
            COUNT(DISTINCT hge.id_estudiante) as valor
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        
        UNION ALL
        
        SELECT 
            'estudiantes_actuales' as tipo,
            COUNT(DISTINCT hge.id_estudiante) as valor
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        WHERE hge.es_actual = true
        
        UNION ALL
        
        SELECT 
            'estudiantes_telematica' as tipo,
            COUNT(DISTINCT hge.id_estudiante) as valor
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        WHERE hge.es_actual = true 
            AND gs.id_modalidad = 2
            
        UNION ALL
        
        SELECT 
            'estudiantes_ciencias' as tipo,
            COUNT(DISTINCT hge.id_estudiante) as valor
        FROM historial_grado_estudiantes hge
        JOIN estudiantes e ON hge.id_estudiante = e.id
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        WHERE hge.es_actual = true 
            AND gs.id_modalidad = 1
            
        UNION ALL
        
        SELECT 
            'grado_secciones_telematica' as tipo,
            COUNT(*) as valor
        FROM grado_secciones gs
        WHERE gs.id_modalidad = 2
        
        UNION ALL
        
        SELECT 
            'grado_secciones_ciencias' as tipo,
            COUNT(*) as valor
        FROM grado_secciones gs
        WHERE gs.id_modalidad = 1
        
        UNION ALL
        
        SELECT 
            'historial_con_grado_secciones' as tipo,
            COUNT(*) as valor
        FROM historial_grado_estudiantes hge
        JOIN grado_secciones gs ON hge.id_grado_secciones = gs.id_grado_secciones
        WHERE hge.es_actual = true AND hge.estado = 'activo'
    ";
    
    let rows = db.query(query, &[])
        .await
        .map_err(|e| format!("Error en consulta de debug modalidad: {}", e))?;
    
    let mut resultado = serde_json::Map::new();
    for row in rows {
        let tipo: String = row.get("tipo");
        let valor: i64 = row.get("valor");
        resultado.insert(tipo, serde_json::Value::Number(serde_json::Number::from(valor)));
    }
    
    Ok(serde_json::Value::Object(resultado))
}

#[tauri::command]
pub async fn obtener_estadisticas_cursos_simple(state: State<'_, AppState>) -> Result<EstadisticasCursosResumen, String> {
    let db = state.db.lock().await;
    
    println!("[DEBUG] Iniciando consulta simple");
    
    // 1. Contar cuántas veces aparece cada id_grado_secciones
    let query_conteo = "
        SELECT 
            id_grado_secciones,
            COUNT(*) as total_estudiantes
        FROM historial_grado_estudiantes
        GROUP BY id_grado_secciones
        ORDER BY total_estudiantes DESC
    ";
    
    let rows_conteo = db.query(query_conteo, &[])
        .await
        .map_err(|e| {
            println!("[DEBUG] Error en conteo: {}", e);
            format!("Error contando id_grado_secciones: {}", e)
        })?;
    
    println!("[DEBUG] Secciones encontradas: {}", rows_conteo.len());
    
    if rows_conteo.is_empty() {
        println!("[DEBUG] No hay datos en historial_grado_estudiantes");
        return Ok(EstadisticasCursosResumen {
            seccion_mayor_matricula: "Sin datos".to_string(),
            estudiantes_mayor: 0,
            seccion_menor_matricula: "Sin datos".to_string(),
            estudiantes_menor: 0,
            promedio_estudiantes_por_seccion: 0.0,
        });
    }
    
    // 2. Obtener información de la sección con más estudiantes
    let id_mayor = rows_conteo[0].get::<_, i32>("id_grado_secciones");
    let estudiantes_mayor = rows_conteo[0].get::<_, i64>("total_estudiantes");
    
    // 3. Obtener información de la sección con menos estudiantes
    let id_menor = rows_conteo[rows_conteo.len() - 1].get::<_, i32>("id_grado_secciones");
    let estudiantes_menor = rows_conteo[rows_conteo.len() - 1].get::<_, i64>("total_estudiantes");
    
    // 4. Hacer JOIN para obtener nombre de grado y sección del mayor
    let query_mayor = "
        SELECT 
            CONCAT(g.nombre_grado, ' sección ', s.nombre_seccion) as nombre_completo_seccion
        FROM grado_secciones gs
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        WHERE gs.id_grado_secciones = $1
    ";
    
    let row_mayor = db.query_one(query_mayor, &[&id_mayor])
        .await
        .map_err(|e| format!("Error obteniendo datos del mayor: {}", e))?;
    
    // 5. Hacer JOIN para obtener nombre de grado y sección del menor
    let query_menor = "
        SELECT 
            CONCAT(g.nombre_grado, ' sección ', s.nombre_seccion) as nombre_completo_seccion
        FROM grado_secciones gs
        JOIN grados g ON gs.id_grado = g.id_grado
        JOIN secciones s ON gs.id_seccion = s.id_seccion
        WHERE gs.id_grado_secciones = $1
    ";
    
    let row_menor = db.query_one(query_menor, &[&id_menor])
        .await
        .map_err(|e| format!("Error obteniendo datos del menor: {}", e))?;
    
    // 6. Calcular promedio
    let total_estudiantes: i64 = rows_conteo.iter().map(|row| row.get::<_, i64>("total_estudiantes")).sum();
    let promedio = total_estudiantes as f64 / rows_conteo.len() as f64;
    
    let estadisticas = EstadisticasCursosResumen {
        seccion_mayor_matricula: row_mayor.get("nombre_completo_seccion"),
        estudiantes_mayor,
        seccion_menor_matricula: row_menor.get("nombre_completo_seccion"),
        estudiantes_menor,
        promedio_estudiantes_por_seccion: promedio,
    };
    
    println!("[DEBUG] Estadísticas calculadas: {:?}", estadisticas);
    
    Ok(estadisticas)
} 