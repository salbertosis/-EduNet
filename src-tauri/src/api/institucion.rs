use crate::{AppState, models::institucion::Institucion};
use tauri::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RespuestaInstitucion {
    pub exito: bool,
    pub mensaje: String,
    pub datos: Option<Institucion>,
}

#[tauri::command]
pub async fn obtener_datos_institucion(
    state: State<'_, AppState>,
) -> Result<RespuestaInstitucion, String> {
    println!("🏫 [INSTITUCION] === OBTENIENDO DATOS DE INSTITUCIÓN ===");
    
    println!("🔒 [INSTITUCION] Obteniendo lock de la base de datos...");
    let db = state.db.lock().await;
    println!("✅ [INSTITUCION] Lock de BD obtenido exitosamente");
    
    println!("🔍 [INSTITUCION] Ejecutando consulta SQL...");
    match db.query_one(
        "SELECT id, codigo, denominacion, direccion, telefono, municipio, entidad_federal, cdcee, director, cedula_director, created_at, updated_at FROM institucion ORDER BY id DESC LIMIT 1",
        &[]
    ).await {
        Ok(row) => {
            println!("✅ [INSTITUCION] Consulta exitosa, procesando datos...");
            let institucion = Institucion {
                id: Some(row.get("id")),
                codigo: row.get("codigo"),
                denominacion: row.get("denominacion"),
                direccion: row.get("direccion"),
                telefono: row.get("telefono"),
                municipio: row.get("municipio"),
                entidad_federal: row.get("entidad_federal"),
                cdcee: row.get("cdcee"),
                director: row.get("director"),
                cedula_director: row.get("cedula_director"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            
            println!("✅ [INSTITUCION] Datos procesados exitosamente");
            println!("✅ [INSTITUCION] Código: {}", institucion.codigo);
            println!("✅ [INSTITUCION] Denominación: {}", institucion.denominacion);
            Ok(RespuestaInstitucion {
                exito: true,
                mensaje: "Datos obtenidos exitosamente".to_string(),
                datos: Some(institucion),
            })
        }
        Err(e) => {
            println!("❌ [INSTITUCION] Error en consulta SQL: {}", e);
            println!("❌ [INSTITUCION] Tipo de error: {:?}", e);
            Ok(RespuestaInstitucion {
                exito: false,
                mensaje: "No se encontraron datos de la institución".to_string(),
                datos: None,
            })
        }
    }
}

#[tauri::command]
pub async fn guardar_datos_institucion(
    institucion: Institucion,
    state: State<'_, AppState>,
) -> Result<RespuestaInstitucion, String> {
    println!("🏫 [INSTITUCION] === COMANDO INVOCADO ===");
    println!("🏫 [INSTITUCION] === INICIANDO GUARDADO DE DATOS ===");
    println!("🏫 [INSTITUCION] Código: {}", institucion.codigo);
    println!("🏫 [INSTITUCION] Denominación: {}", institucion.denominacion);
    println!("🏫 [INSTITUCION] Dirección: {}", institucion.direccion);
    println!("🏫 [INSTITUCION] Municipio: {}", institucion.municipio);
    println!("🏫 [INSTITUCION] Entidad Federal: {}", institucion.entidad_federal);
    println!("🏫 [INSTITUCION] Director: {}", institucion.director);
    println!("🏫 [INSTITUCION] Cédula Director: {}", institucion.cedula_director);
    
    println!("🔒 [INSTITUCION] Obteniendo lock de la base de datos...");
    let db = state.db.lock().await;
    println!("✅ [INSTITUCION] Lock de BD obtenido exitosamente");
    
    // Verificar si ya existe una institución
    println!("🔍 [INSTITUCION] Verificando si existe institución en la BD...");
    let existe = db.query_opt(
        "SELECT id FROM institucion LIMIT 1",
        &[]
    ).await.map_err(|e| {
        println!("❌ [INSTITUCION] Error verificando existencia: {}", e);
        format!("Error verificando existencia: {}", e)
    })?;
    
    println!("🔍 [INSTITUCION] Resultado de verificación: {}", if existe.is_some() { "EXISTE" } else { "NO EXISTE" });
    
    let resultado = if existe.is_some() {
        // Actualizar datos existentes
        println!("🔄 [INSTITUCION] Actualizando datos existentes...");
        println!("🔄 [INSTITUCION] Ejecutando UPDATE...");
        let update_result = db.execute(
            "UPDATE institucion SET codigo = $1, denominacion = $2, direccion = $3, telefono = $4, municipio = $5, entidad_federal = $6, cdcee = $7, director = $8, cedula_director = $9, updated_at = NOW() WHERE id = (SELECT id FROM institucion ORDER BY id DESC LIMIT 1)",
            &[
                &institucion.codigo,
                &institucion.denominacion,
                &institucion.direccion,
                &institucion.telefono,
                &institucion.municipio,
                &institucion.entidad_federal,
                &institucion.cdcee,
                &institucion.director,
                &institucion.cedula_director,
            ]
        ).await;
        println!("🔄 [INSTITUCION] Resultado UPDATE: {:?}", update_result);
        update_result
    } else {
        // Insertar nuevos datos
        println!("➕ [INSTITUCION] Insertando nuevos datos...");
        println!("➕ [INSTITUCION] Ejecutando INSERT...");
        let insert_result = db.execute(
            "INSERT INTO institucion (codigo, denominacion, direccion, telefono, municipio, entidad_federal, cdcee, director, cedula_director, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
            &[
                &institucion.codigo,
                &institucion.denominacion,
                &institucion.direccion,
                &institucion.telefono,
                &institucion.municipio,
                &institucion.entidad_federal,
                &institucion.cdcee,
                &institucion.director,
                &institucion.cedula_director,
            ]
        ).await;
        println!("➕ [INSTITUCION] Resultado INSERT: {:?}", insert_result);
        insert_result
    };
    
    match resultado {
        Ok(rows_affected) => {
            println!("✅ [INSTITUCION] Datos guardados exitosamente");
            println!("✅ [INSTITUCION] Filas afectadas: {}", rows_affected);
            Ok(RespuestaInstitucion {
                exito: true,
                mensaje: "Datos de la institución guardados exitosamente".to_string(),
                datos: Some(institucion),
            })
        }
        Err(e) => {
            println!("❌ [INSTITUCION] Error guardando datos: {}", e);
            println!("❌ [INSTITUCION] Tipo de error: {:?}", e);
            Err(format!("Error guardando datos de la institución: {}", e))
        }
    }
} 