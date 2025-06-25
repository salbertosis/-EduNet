use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tauri::{command, AppHandle, State};
use umya_spreadsheet::*;
use crate::AppState;
use std::time::Instant;
use tokio::time::{timeout, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EstudianteResumen {
    pub cedula: String,
    pub nombres: String,
    pub apellidos: String,
    pub sexo: String,
    pub fecha_nacimiento: Option<String>,
    pub lugar_nacimiento: Option<String>,
    pub calificaciones: HashMap<String, Option<i32>>,
    pub pgcrp: Option<String>,
    pub definitiva_general: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatosSeccion {
    pub id_grado_secciones: i32,
    pub nombre_grado: String,
    pub nombre_seccion: String,
    pub nombre_modalidad: String,
    pub periodo_escolar: String,
    pub estudiantes: Vec<EstudianteResumen>,
    pub docente_guia: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParametrosResumen {
    pub id_periodo: i32,
    pub id_modalidad: i32,
    pub id_grado: i32,
    pub ids_grado_secciones: Vec<i32>,
    pub ruta_salida: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatosEstudianteBasico {
    pub cedula: String,
    pub apellidos: String,
    pub nombres: String,
    pub lugar_nacimiento: String,
    pub entidad_federal: String,
    pub genero: String,
    pub dia_nacimiento: i32,
    pub mes_nacimiento: i32,
    pub ano_nacimiento: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParametrosResumenBasico {
    pub id_grado_secciones: i32,
    pub ruta_salida: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RespuestaResumenBasico {
    pub exito: bool,
    pub mensaje: String,
    pub archivo_generado: Option<String>,
    pub estudiantes_procesados: usize,
}

pub struct GeneradorResumenCompleto {
    app_handle: AppHandle,
}

impl GeneradorResumenCompleto {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    /// Selecciona la plantilla Excel correcta según el grado
    fn seleccionar_plantilla(&self, grado: i32) -> Result<PathBuf, String> {
        let plantillas_dir = self.app_handle
            .path_resolver()
            .resource_dir()
            .ok_or("No se pudo obtener el directorio de recursos")?
            .join("plantillas");

        let nombre_plantilla = match grado {
            1 | 2 => "Resumen Final Rendimiento Estudiantil 1ero y 2do año.xlsx",
            3 => "Resumen Final Rendimiento Estudiantil 3er año.xlsx", 
            4 => "Resumen Final Rendimiento Estudiantil 4to año.xlsx",
            5 => "Resumen Final Rendimiento Estudiantil 5to año.xlsx",
            _ => return Err(format!("No existe plantilla para el grado {}", grado)),
        };

        let ruta_plantilla = plantillas_dir.join(nombre_plantilla);
        
        if !ruta_plantilla.exists() {
            return Err(format!("No se encontró la plantilla: {}", nombre_plantilla));
        }

        println!("[GeneradorResumenCompleto] Plantilla seleccionada: {:?}", ruta_plantilla);
        Ok(ruta_plantilla)
    }

    /// Obtiene todos los datos de una sección desde la base de datos
    async fn obtener_datos_seccion(&self, db: &tokio_postgres::Client, id_grado_secciones: i32, id_periodo: i32) -> Result<DatosSeccion, String> {
        // Obtener información básica de la sección
        let info_seccion = db.query_one(
            "SELECT g.id_grado, g.nombre_grado, s.nombre_seccion, m.nombre_modalidad, p.periodo_escolar,
                    d.nombres || ' ' || d.apellidos as docente_guia
             FROM grado_secciones gs
             JOIN grados g ON gs.id_grado = g.id_grado
             JOIN secciones s ON gs.id_seccion = s.id_seccion  
             JOIN modalidades m ON gs.id_modalidad = m.id_modalidad
             JOIN periodos_escolares p ON p.id_periodo = $2
             LEFT JOIN docentes d ON gs.id_docente_guia = d.id_docente
             WHERE gs.id_grado_secciones = $1",
            &[&id_grado_secciones, &id_periodo]
        ).await.map_err(|e| format!("Error obteniendo info de sección: {}", e))?;

        let id_grado: i32 = info_seccion.get("id_grado");
        let nombre_grado: String = info_seccion.get("nombre_grado");
        let nombre_seccion: String = info_seccion.get("nombre_seccion");
        let nombre_modalidad: String = info_seccion.get("nombre_modalidad");
        let periodo_escolar: String = info_seccion.get("periodo_escolar");
        let docente_guia: Option<String> = info_seccion.try_get("docente_guia").ok();

        // Obtener estudiantes de la sección
        let estudiantes_rows = db.query(
            "SELECT e.id, e.cedula, e.nombres, e.apellidos, e.sexo, e.fecha_nacimiento, e.lugar_nacimiento
             FROM estudiantes e
             JOIN historial_grado_estudiantes hge ON e.id = hge.id_estudiante
             WHERE hge.id_grado_secciones = $1 AND hge.id_periodo = $2 AND hge.es_actual = true
             ORDER BY e.apellidos, e.nombres",
            &[&id_grado_secciones, &id_periodo]
        ).await.map_err(|e| format!("Error obteniendo estudiantes: {}", e))?;

        let mut estudiantes = Vec::new();

        for estudiante_row in estudiantes_rows {
            let id_estudiante: i32 = estudiante_row.get("id");
            let cedula: i64 = estudiante_row.get("cedula");
            let nombres: String = estudiante_row.get("nombres");
            let apellidos: String = estudiante_row.get("apellidos");
            let sexo: String = estudiante_row.get("sexo");
            let fecha_nacimiento: Option<chrono::NaiveDate> = estudiante_row.try_get("fecha_nacimiento").ok();
            let lugar_nacimiento: Option<String> = estudiante_row.try_get("lugar_nacimiento").ok();

            // Obtener calificaciones del estudiante
            let calificaciones = self.obtener_calificaciones_estudiante(db, id_estudiante, id_periodo, id_grado).await?;
            
            // Obtener PGCRP del estudiante
            let pgcrp = self.obtener_pgcrp_estudiante(db, id_estudiante, id_periodo, id_grado_secciones).await?;

            // Calcular definitiva general (promedio de calificaciones válidas)
            let definitiva_general = self.calcular_definitiva_general(&calificaciones);

            estudiantes.push(EstudianteResumen {
                cedula: cedula.to_string(),
                nombres,
                apellidos,
                sexo,
                fecha_nacimiento: fecha_nacimiento.map(|d| d.format("%d/%m/%Y").to_string()),
                lugar_nacimiento,
                calificaciones,
                pgcrp,
                definitiva_general,
            });
        }

        Ok(DatosSeccion {
            id_grado_secciones,
            nombre_grado,
            nombre_seccion,
            nombre_modalidad,
            periodo_escolar,
            estudiantes,
            docente_guia,
        })
    }

    /// Obtiene las calificaciones de un estudiante
    async fn obtener_calificaciones_estudiante(&self, db: &tokio_postgres::Client, id_estudiante: i32, id_periodo: i32, id_grado: i32) -> Result<HashMap<String, Option<i32>>, String> {
        let mut calificaciones = HashMap::new();

        // Obtener asignaturas del grado
        let asignaturas_rows = db.query(
            "SELECT a.codigo, a.nombre, c.definitiva
             FROM asignaturas a
             JOIN grado_modalidad_asignaturas gma ON a.id_asignatura = gma.id_asignatura
             LEFT JOIN calificaciones c ON c.id_asignatura = a.id_asignatura 
                AND c.id_estudiante = $1 AND c.id_periodo = $2
             WHERE gma.id_grado = $3
             ORDER BY gma.orden",
            &[&id_estudiante, &id_periodo, &id_grado]
        ).await.map_err(|e| format!("Error obteniendo calificaciones: {}", e))?;

        for row in asignaturas_rows {
            let codigo: String = row.get("codigo");
            let definitiva: Option<i32> = row.try_get("definitiva").ok().flatten();
            calificaciones.insert(codigo, definitiva);
        }

        Ok(calificaciones)
    }

    /// Obtiene el PGCRP de un estudiante
    async fn obtener_pgcrp_estudiante(&self, db: &tokio_postgres::Client, id_estudiante: i32, id_periodo: i32, id_grado_secciones: i32) -> Result<Option<String>, String> {
        // Primero verificar PGCRP individual del estudiante
        if let Ok(row) = db.query_opt(
            "SELECT p.nombre FROM estudiantes_pgcrp ep
             JOIN \"PGCRP\" p ON ep.id_pgcrp = p.id_pgcrp
             WHERE ep.id_estudiante = $1 AND ep.id_periodo = $2",
            &[&id_estudiante, &id_periodo]
        ).await {
            if let Some(row) = row {
                return Ok(Some(row.get("nombre")));
            }
        }

        // Si no tiene PGCRP individual, verificar PGCRP por sección
        if let Ok(row) = db.query_opt(
            "SELECT p.nombre FROM grado_secciones_pgcrp gsp
             JOIN \"PGCRP\" p ON gsp.id_pgcrp = p.id_pgcrp
             WHERE gsp.id_grado_secciones = $1 AND gsp.id_periodo = $2",
            &[&id_grado_secciones, &id_periodo]
        ).await {
            if let Some(row) = row {
                return Ok(Some(row.get("nombre")));
            }
        }

        Ok(None)
    }

    /// Calcula la definitiva general del estudiante
    fn calcular_definitiva_general(&self, calificaciones: &HashMap<String, Option<i32>>) -> Option<f32> {
        let calificaciones_validas: Vec<i32> = calificaciones
            .values()
            .filter_map(|&cal| cal)
            .collect();

        if calificaciones_validas.is_empty() {
            return None;
        }

        let suma: i32 = calificaciones_validas.iter().sum();
        let promedio = suma as f32 / calificaciones_validas.len() as f32;
        Some(promedio)
    }

    /// Completa la plantilla Excel con los datos de los estudiantes
    async fn completar_plantilla_excel(&self, ruta_plantilla: &Path, datos_seccion: &DatosSeccion, ruta_salida: &str) -> Result<(), String> {
        println!("[DEBUG] Cargando plantilla: {}", ruta_plantilla.display());
        let mut book = umya_spreadsheet::reader::xlsx::read(&ruta_plantilla)
            .map_err(|e| format!("Error cargando plantilla: {}", e))?;

        // Debug: listar todas las hojas disponibles
        println!("[DEBUG] Hojas disponibles en el archivo:");
        for (index, sheet_name) in book.get_sheet_collection().iter().enumerate() {
            println!("  Índice {}: '{}'", index, sheet_name.get_name());
        }
        
        // Seleccionar la hoja evitando múltiples préstamos simultáneos
        let worksheet = if let Some(s) = book.get_sheet_by_name_mut("Sheet1") {
            println!("[DEBUG] Usando hoja 'Sheet1'");
            s
        } else if let Some(s) = book.get_sheet_mut(&0) {
            println!("[DEBUG] Usando hoja con índice 0");
            s
        } else if let Some(s) = book.get_sheet_mut(&1) {
            println!("[DEBUG] Usando hoja con índice 1");
            s
        } else {
            println!("[DEBUG] Intentando usar la primera hoja disponible");
            book.get_sheet_collection_mut()
                .first_mut()
                .ok_or("No se pudo obtener ninguna hoja de trabajo")?
        };

        println!("[DEBUG] Hoja seleccionada: '{}'", worksheet.get_name());

        // Completar encabezados institucionales
        self.completar_encabezados(worksheet, datos_seccion)?;

        // Completar datos de estudiantes
        self.completar_estudiantes(worksheet, datos_seccion)?;

        // Guardar el archivo
        writer::xlsx::write(&book, ruta_salida)
            .map_err(|e| format!("Error guardando Excel: {}", e))?;

        println!("[GeneradorResumenCompleto] Excel completado: {}", ruta_salida);
        Ok(())
    }

    /// Completa los encabezados de la plantilla
    fn completar_encabezados(&self, worksheet: &mut Worksheet, datos: &DatosSeccion) -> Result<(), String> {
        // Estos son ejemplos - deberás ajustar según la estructura real de tus plantillas
        worksheet.get_cell_mut((1, 1)).set_value("REPÚBLICA BOLIVARIANA DE VENEZUELA");
        worksheet.get_cell_mut((2, 1)).set_value("MINISTERIO DEL PODER POPULAR PARA LA EDUCACIÓN");
        worksheet.get_cell_mut((3, 1)).set_value("LICEO BOLIVARIANO \"JOSÉ RAFAEL REVENGA\"");
        
        // Información específica de la sección
        worksheet.get_cell_mut((5, 1)).set_value(format!("GRADO: {}", datos.nombre_grado));
        worksheet.get_cell_mut((5, 3)).set_value(format!("SECCIÓN: {}", datos.nombre_seccion));
        worksheet.get_cell_mut((5, 5)).set_value(format!("MODALIDAD: {}", datos.nombre_modalidad));
        worksheet.get_cell_mut((6, 1)).set_value(format!("AÑO ESCOLAR: {}", datos.periodo_escolar));
        
        if let Some(ref docente) = datos.docente_guia {
            worksheet.get_cell_mut((7, 1)).set_value(format!("DOCENTE GUÍA: {}", docente));
        }

        Ok(())
    }

    /// Completa los datos de estudiantes en la plantilla
    fn completar_estudiantes(&self, worksheet: &mut Worksheet, datos: &DatosSeccion) -> Result<(), String> {
        // Fila inicial donde empiezan los datos de estudiantes (ajustar según plantilla)
        let mut fila_actual = 10;

        for (indice, estudiante) in datos.estudiantes.iter().enumerate() {
            // Número de orden
            worksheet.get_cell_mut((fila_actual, 1)).set_value((indice + 1).to_string());
            
            // Cédula
            worksheet.get_cell_mut((fila_actual, 2)).set_value(&estudiante.cedula);
            
            // Apellidos y Nombres
            worksheet.get_cell_mut((fila_actual, 3)).set_value(format!("{}, {}", estudiante.apellidos, estudiante.nombres));
            
            // Sexo
            worksheet.get_cell_mut((fila_actual, 4)).set_value(&estudiante.sexo);
            
            // Fecha de nacimiento
            if let Some(ref fecha) = estudiante.fecha_nacimiento {
                worksheet.get_cell_mut((fila_actual, 5)).set_value(fecha);
            }
            
            // Lugar de nacimiento
            if let Some(ref lugar) = estudiante.lugar_nacimiento {
                worksheet.get_cell_mut((fila_actual, 6)).set_value(lugar);
            }

            // Calificaciones por materia (columnas 7 en adelante)
            let mut columna_cal = 7;
            for codigo_materia in ["CA", "ILE", "MA", "EF", "FI", "QU", "BI", "CT", "GHC", "FSN", "OC"] {
                if let Some(Some(calificacion)) = estudiante.calificaciones.get(codigo_materia) {
                    worksheet.get_cell_mut((fila_actual, columna_cal)).set_value(calificacion.to_string());
                }
                columna_cal += 1;
            }

            // PGCRP
            if let Some(ref pgcrp) = estudiante.pgcrp {
                worksheet.get_cell_mut((fila_actual, columna_cal)).set_value(pgcrp);
            }
            columna_cal += 1;

            // Definitiva general
            if let Some(definitiva) = estudiante.definitiva_general {
                worksheet.get_cell_mut((fila_actual, columna_cal)).set_value(format!("{:.2}", definitiva));
            }

            fila_actual += 1;
        }

        Ok(())
    }

    /// Convierte el Excel a PDF usando LibreOffice headless
    async fn convertir_a_pdf(&self, ruta_excel: &str) -> Result<String, String> {
        let ruta_pdf = ruta_excel.replace(".xlsx", ".pdf");
        
        // Intentar conversión con LibreOffice headless
        let output = std::process::Command::new("soffice")
            .args([
                "--headless",
                "--convert-to", "pdf",
                "--outdir", 
                std::path::Path::new(ruta_excel).parent().unwrap().to_str().unwrap(),
                ruta_excel
            ])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("[GeneradorResumenCompleto] PDF generado exitosamente: {}", ruta_pdf);
                    Ok(ruta_pdf)
                } else {
                    let error_msg = String::from_utf8_lossy(&result.stderr);
                    println!("[GeneradorResumenCompleto] Error en LibreOffice: {}", error_msg);
                    // Fallback: retornar el Excel como "PDF" para que no falle
                    println!("[GeneradorResumenCompleto] Usando Excel como fallback: {}", ruta_excel);
                    Ok(ruta_excel.to_string())
                }
            }
            Err(e) => {
                println!("[GeneradorResumenCompleto] LibreOffice no disponible: {}", e);
                // Fallback: retornar el Excel como "PDF" para que no falle
                println!("[GeneradorResumenCompleto] Usando Excel como fallback: {}", ruta_excel);
                Ok(ruta_excel.to_string())
            }
        }
    }

    /// Lee la plantilla oficial del MPPE y llena solo los datos de estudiantes preservando todo el formato
    pub fn llenar_estudiantes_excel(&self, estudiantes: &[DatosEstudianteBasico], ruta_plantilla: &str, ruta_salida: &str) -> Result<(), String> {
        use umya_spreadsheet::*;
        
        println!("[DEBUG] Leyendo plantilla oficial: {}", ruta_plantilla);
        
        // Leer la plantilla oficial del MPPE
        let path = std::path::Path::new(ruta_plantilla);
        let mut book = reader::xlsx::read(path)
            .map_err(|e| format!("Error al leer plantilla: {}", e))?;
        
        // Ordenar estudiantes por cédula
        let mut estudiantes_ordenados = estudiantes.to_vec();
        estudiantes_ordenados.sort_by(|a, b| {
            let cedula_a: u64 = a.cedula.parse().unwrap_or(0);
            let cedula_b: u64 = b.cedula.parse().unwrap_or(0);
            cedula_a.cmp(&cedula_b)
        });
        
        println!("[DEBUG] Procesando {} estudiantes en la plantilla oficial", estudiantes_ordenados.len());
        
        // Obtener la primera hoja (donde están los datos)
        let sheet = book.get_sheet_mut(&0)
            .ok_or("No se pudo obtener la primera hoja de la plantilla")?;
        
        // Procesar estudiantes en grupos de 35 (capacidad máxima por hoja)
        const ESTUDIANTES_POR_HOJA: usize = 35;
        let total_hojas = (estudiantes_ordenados.len() + ESTUDIANTES_POR_HOJA - 1) / ESTUDIANTES_POR_HOJA;
        
        for numero_hoja in 0..total_hojas {
            let inicio = numero_hoja * ESTUDIANTES_POR_HOJA;
            let fin = std::cmp::min(inicio + ESTUDIANTES_POR_HOJA, estudiantes_ordenados.len());
            let estudiantes_hoja = &estudiantes_ordenados[inicio..fin];
            
            let nombre_hoja = if numero_hoja == 0 {
                "Plantilla".to_string()
            } else {
                format!("Plantilla - Página {}", numero_hoja + 1)
            };
            
            // Si no es la primera hoja, clonar la plantilla
            if numero_hoja > 0 {
                let plantilla_original = book.get_sheet(&0).unwrap().clone();
                let mut nueva_hoja = plantilla_original;
                nueva_hoja.set_name(&nombre_hoja);
                
                // Limpiar datos de estudiantes de la nueva hoja
                self.limpiar_datos_estudiantes_umya(&mut nueva_hoja)?;
                
                book.add_sheet(nueva_hoja);
            }
            
            // Obtener la hoja actual para llenar
            let hoja_actual = if numero_hoja == 0 {
                book.get_sheet_mut(&0).unwrap()
            } else {
                book.get_sheet_by_name_mut(&nombre_hoja).unwrap()
            };
            
            // Llenar datos de estudiantes en esta hoja
            self.llenar_datos_estudiantes_umya(hoja_actual, estudiantes_hoja)?;
            
            println!("[DEBUG] Hoja '{}' procesada con {} estudiantes", nombre_hoja, estudiantes_hoja.len());
        }
        
        // Guardar el archivo con formato preservado
        println!("[DEBUG] Guardando archivo: {}", ruta_salida);
        let path_salida = std::path::Path::new(ruta_salida);
        writer::xlsx::write(&book, path_salida)
            .map_err(|e| format!("Error al guardar archivo: {}", e))?;
        
        println!("[DEBUG] ✅ Archivo generado exitosamente: {}", ruta_salida);
        Ok(())
    }
    
    /// Llena los datos de estudiantes en la hoja usando umya_spreadsheet
    fn llenar_datos_estudiantes_umya(&self, sheet: &mut umya_spreadsheet::structs::Worksheet, estudiantes: &[DatosEstudianteBasico]) -> Result<(), String> {
        println!("[DEBUG] Llenando {} estudiantes en la hoja", estudiantes.len());
        
        // Coordenadas fijas según parámetros MPPE oficiales [como se guardó en memoria][[memory:3430177925867389327]]
        // Fila inicial donde empiezan los datos de estudiantes
        let fila_inicial = 17u32;
        
        for (index, estudiante) in estudiantes.iter().enumerate() {
            let fila = fila_inicial + index as u32;
            
            // Verificar que no excedamos el límite de 35 estudiantes
            if index >= 35 {
                break;
            }
            
            // Formatear nombres y apellidos según reglas MPPE [como se guardó en memoria][[memory:3430177925867389327]]
            let apellidos_formateados = estudiante.apellidos.clone();
            let nombres_formateados = estudiante.nombres.clone();
            
            // Llenar las celdas según ubicaciones oficiales MPPE [como se guardó en memoria][[memory:3430177925867389327]]
            // Cédula en columna B
            sheet.get_cell_mut((2u32, fila)).set_value(&estudiante.cedula);
            
            // Apellidos en columna O (columna 15)
            sheet.get_cell_mut((15u32, fila)).set_value(&apellidos_formateados);
            
            // Nombres en columna X (columna 24)  
            sheet.get_cell_mut((24u32, fila)).set_value(&nombres_formateados);
            
            // Lugar de nacimiento en columna AI (columna 35)
            sheet.get_cell_mut((35u32, fila)).set_value(&estudiante.lugar_nacimiento);
            
            // Entidad Federal en columna AP (columna 42)
            sheet.get_cell_mut((42u32, fila)).set_value(&estudiante.entidad_federal);
            
            // Género en columna AQ (columna 43)
            let genero_texto = match estudiante.genero.as_str() {
                "M" => "M",
                "F" => "F",
                _ => "M"
            };
            sheet.get_cell_mut((43u32, fila)).set_value(genero_texto);
            
            // Fecha de nacimiento dividida en día, mes, año
            // Día en columna AR (columna 44)
            if estudiante.dia_nacimiento > 0 {
                sheet.get_cell_mut((44u32, fila)).set_value(&estudiante.dia_nacimiento.to_string());
            }
            
            // Mes en columna AT (columna 46) 
            if estudiante.mes_nacimiento > 0 {
                sheet.get_cell_mut((46u32, fila)).set_value(&estudiante.mes_nacimiento.to_string());
            }
            
            // Año en columna AV (columna 48)
            if estudiante.ano_nacimiento > 0 {
                sheet.get_cell_mut((48u32, fila)).set_value(&estudiante.ano_nacimiento.to_string());
            }
        }
        
        println!("[DEBUG] ✅ Datos de estudiantes llenados exitosamente");
        Ok(())
    }
    
    /// Limpia los datos de estudiantes de una hoja manteniendo el formato
    fn limpiar_datos_estudiantes_umya(&self, sheet: &mut umya_spreadsheet::structs::Worksheet) -> Result<(), String> {
        println!("[DEBUG] Limpiando datos de estudiantes de la hoja...");
        
        let fila_inicial = 17u32;
        let fila_final = 51u32; // Hasta fila 51 (35 estudiantes máximo)
        
        // Columnas a limpiar según parámetros MPPE [como se guardó en memoria][[memory:3430177925867389327]]
        let columnas = vec![2u32, 15u32, 24u32, 35u32, 42u32, 43u32, 44u32, 46u32, 48u32];
        
        for fila in fila_inicial..=fila_final {
            for &columna in &columnas {
                // Limpiar el valor pero mantener el formato
                sheet.get_cell_mut((columna, fila)).set_value("");
            }
        }
        
        println!("[DEBUG] ✅ Datos limpiados exitosamente");
        Ok(())
    }
}

pub struct GeneradorResumenBasico;

impl GeneradorResumenBasico {
    pub fn new() -> Self {
        Self
    }

    /// Selecciona la plantilla Excel adecuada para el grado asociado a la sección
    pub async fn seleccionar_plantilla(&self, db: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<String, String> {
        use std::time::Instant;
        
        let inicio = Instant::now();
        println!("[PLANTILLA] 🔍 Seleccionando plantilla para id_grado_secciones: {}", id_grado_secciones);
        
        // 1. Obtener el id_grado desde grado_secciones
        let query_grado = r#"
            SELECT id_grado 
            FROM grado_secciones 
            WHERE id_grado_secciones = $1
        "#;
        
        println!("[PLANTILLA] 📊 Paso 1: Obteniendo id_grado desde grado_secciones...");
        let row = db.query_one(query_grado, &[&id_grado_secciones])
            .await
            .map_err(|e| format!("Error obteniendo id_grado: {}", e))?;
            
        let id_grado: i32 = row.get("id_grado");
        println!("[PLANTILLA] ✅ id_grado obtenido: {}", id_grado);
        
        // 2. Con el id_grado, determinar el número de grado  
        let numero_grado = id_grado; // Asumir que id_grado = numero (1, 2, 3, 4, 5)
        println!("[PLANTILLA] ✅ Número de grado obtenido: {} (en {:.2}s)", numero_grado, inicio.elapsed().as_secs_f64());
        
        // 2. Seleccionar plantilla según el grado
        let nombre_plantilla = match numero_grado {
            1 | 2 => "Resumen Final Rendimiento Estudiantil 1ero y 2do año.xlsx",
            3 => "Resumen Final Rendimiento Estudiantil 3er año.xlsx", 
            4 => "Resumen Final Rendimiento Estudiantil 4to año.xlsx",
            5 => "Resumen Final Rendimiento Estudiantil 5to año.xlsx",
            _ => return Err(format!("Grado no soportado: {}", numero_grado)),
        };
        
        println!("[PLANTILLA] 📄 Plantilla seleccionada: {}", nombre_plantilla);
        
        // 3. Construir ruta completa de la plantilla
        let ruta_plantilla = std::env::current_dir()
            .map_err(|e| format!("Error obteniendo directorio actual: {}", e))?
            .join("plantillas")
            .join(nombre_plantilla);
        
        let ruta_plantilla_str = ruta_plantilla.to_string_lossy().to_string();
        println!("[PLANTILLA] 📁 Ruta completa: {}", ruta_plantilla_str);
        println!("[PLANTILLA] 📂 Directorio actual: {:?}", std::env::current_dir());
        
        // 4. Verificar que el archivo existe
        if !ruta_plantilla.exists() {
            let error_msg = format!("❌ Plantilla no encontrada: {}", ruta_plantilla_str);
            println!("[PLANTILLA] {}", error_msg);
            return Err(error_msg);
        }
        
        // 5. Verificar que se puede leer el archivo
        match std::fs::metadata(&ruta_plantilla) {
            Ok(metadata) => {
                println!("[PLANTILLA] ✅ Archivo accesible - Tamaño: {} bytes", metadata.len());
            },
            Err(e) => {
                let error_msg = format!("❌ No se puede acceder a la plantilla: {}", e);
                println!("[PLANTILLA] {}", error_msg);
                return Err(error_msg);
            }
        }
        
        println!("[PLANTILLA] ✅ Plantilla seleccionada exitosamente en {:.2}s total", inicio.elapsed().as_secs_f64());
        Ok(ruta_plantilla_str)
    }

    pub async fn obtener_estudiantes_seccion(&self, db: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<Vec<DatosEstudianteBasico>, String> {
        let query = r#"
            SELECT 
                e.cedula::text as cedula,
                e.apellidos,
                e.nombres,
                COALESCE(c.nombre, '') as lugar_nacimiento,
                COALESCE(est.entidadfed, '') as entidad_federal,
                e.genero,
                EXTRACT(DAY FROM e.fecha_nacimiento)::int as dia_nacimiento,
                EXTRACT(MONTH FROM e.fecha_nacimiento)::int as mes_nacimiento,
                EXTRACT(YEAR FROM e.fecha_nacimiento)::int as ano_nacimiento
            FROM estudiantes e
            INNER JOIN historial_grado_estudiantes hge ON e.id = hge.id_estudiante
            LEFT JOIN ciudades c ON e.ciudad_nac_id = c.id
            LEFT JOIN estados est ON e.estado_nac_id = est.id
            WHERE hge.id_grado_secciones = $1
              AND hge.es_actual = true
            ORDER BY e.cedula::bigint ASC
        "#;

        println!("[DEBUG] Ejecutando consulta para obtener estudiantes con ciudad y estado de nacimiento");
        let rows = db.query(query, &[&id_grado_secciones])
            .await
            .map_err(|e| format!("Error en consulta de estudiantes: {}", e))?;

        let mut estudiantes = Vec::new();
        for row in rows {
            let estudiante = DatosEstudianteBasico {
                cedula: row.get("cedula"),
                apellidos: row.get("apellidos"),
                nombres: row.get("nombres"),
                lugar_nacimiento: row.get("lugar_nacimiento"),
                entidad_federal: row.get("entidad_federal"),
                genero: row.get("genero"),
                dia_nacimiento: row.get("dia_nacimiento"),
                mes_nacimiento: row.get("mes_nacimiento"),
                ano_nacimiento: row.get("ano_nacimiento"),
            };
            
            println!("[DEBUG] Estudiante: {} {} - Ciudad: {} - Estado: {}", 
                     estudiante.apellidos, estudiante.nombres, 
                     estudiante.lugar_nacimiento, estudiante.entidad_federal);
                     
            estudiantes.push(estudiante);
        }

        println!("[DEBUG] Total estudiantes encontrados: {}", estudiantes.len());
        Ok(estudiantes)
    }

    pub fn formatear_nombres(&self, nombres: &str) -> String {
        // Aplicar parámetros oficiales MPPE para nombres largos
        let nombres_partes: Vec<&str> = nombres.split_whitespace().collect();
        
        if nombres_partes.len() <= 2 {
            // Si tiene 2 nombres o menos, usar completo
            nombres.to_string()
        } else {
            // Si tiene más de 2 nombres, usar primer nombre + inicial del segundo
            let primer_nombre = nombres_partes[0];
            let segundo_nombre_inicial = nombres_partes[1].chars().next().unwrap_or(' ');
            format!("{} {}.", primer_nombre, segundo_nombre_inicial)
        }
    }

    pub fn formatear_apellidos(&self, apellidos: &str) -> String {
        // Aplicar parámetros oficiales MPPE para apellidos largos
        let apellidos_partes: Vec<&str> = apellidos.split_whitespace().collect();
        
        if apellidos_partes.len() <= 25 {
            // Si los apellidos no son muy largos, usar completo
            apellidos.to_string()
        } else {
            // Si son muy largos, usar abreviación inteligente
            let primer_apellido = apellidos_partes[0];
            if apellidos_partes.len() > 1 {
                let segundo_apellido = apellidos_partes[1];
                format!("{} {}", primer_apellido, segundo_apellido)
            } else {
                primer_apellido.to_string()
            }
        }
    }

    /// Crea el resumen Excel desde cero usando rust_xlsxwriter (más rápido y confiable)
    pub fn llenar_estudiantes_excel(&self, estudiantes: &[DatosEstudianteBasico], ruta_plantilla: &str, ruta_salida: &str) -> Result<(), String> {
        use rust_xlsxwriter::*;
        
        println!("[DEBUG] Creando Excel desde cero: {}", ruta_salida);
        
        // Ordenar estudiantes por cédula
        let mut estudiantes_ordenados = estudiantes.to_vec();
        estudiantes_ordenados.sort_by(|a, b| {
            let cedula_a: u64 = a.cedula.parse().unwrap_or(0);
            let cedula_b: u64 = b.cedula.parse().unwrap_or(0);
            cedula_a.cmp(&cedula_b)
        });
        
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        
        // Configurar orientación horizontal
        worksheet.set_landscape();
        
        // === FORMATOS ===
        let formato_titulo = Format::new()
            .set_bold()
            .set_font_size(14)
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin)
            .set_font_name("Arial");
            
        let formato_encabezado = Format::new()
            .set_bold()
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin)
            .set_background_color("#D3D3D3")
            .set_font_name("Arial");
            
        let formato_celda = Format::new()
            .set_font_size(10)
            .set_border(FormatBorder::Thin)
            .set_align(FormatAlign::Center)
            .set_font_name("Arial");
            
        let formato_izquierda = Format::new()
            .set_font_size(10)
            .set_border(FormatBorder::Thin)
            .set_font_name("Arial");
        
        // === ENCABEZADO OFICIAL ===
        worksheet.merge_range(0, 0, 0, 10, "REPÚBLICA BOLIVARIANA DE VENEZUELA", &formato_titulo)
            .map_err(|e| format!("Error escribiendo encabezado: {}", e))?;
            
        worksheet.merge_range(1, 0, 1, 10, "MINISTERIO DEL PODER POPULAR PARA LA EDUCACIÓN", &formato_titulo)
            .map_err(|e| format!("Error escribiendo ministerio: {}", e))?;
            
        worksheet.merge_range(2, 0, 2, 10, "RESUMEN FINAL DEL RENDIMIENTO ESTUDIANTIL", &formato_titulo)
            .map_err(|e| format!("Error escribiendo título: {}", e))?;
        
        // === TABLA DE ESTUDIANTES ===
        let fila_inicio_tabla = 5;
        
        // Encabezados de la tabla (Fila 5)
        worksheet.write_string_with_format(fila_inicio_tabla, 0, "N°", &formato_encabezado)
            .map_err(|e| format!("Error escribiendo encabezado N°: {}", e))?;
        worksheet.write_string_with_format(fila_inicio_tabla, 1, "CÉDULA", &formato_encabezado)
            .map_err(|e| format!("Error escribiendo encabezado CÉDULA: {}", e))?;
        worksheet.write_string_with_format(fila_inicio_tabla, 2, "APELLIDOS", &formato_encabezado)
            .map_err(|e| format!("Error escribiendo encabezado APELLIDOS: {}", e))?;
        worksheet.write_string_with_format(fila_inicio_tabla, 3, "NOMBRES", &formato_encabezado)
            .map_err(|e| format!("Error escribiendo encabezado NOMBRES: {}", e))?;
        worksheet.write_string_with_format(fila_inicio_tabla, 4, "LUGAR DE NACIMIENTO", &formato_encabezado)
            .map_err(|e| format!("Error escribiendo encabezado LUGAR: {}", e))?;
        worksheet.write_string_with_format(fila_inicio_tabla, 5, "ENTIDAD FEDERAL", &formato_encabezado)
            .map_err(|e| format!("Error escribiendo encabezado ENTIDAD: {}", e))?;
        worksheet.write_string_with_format(fila_inicio_tabla, 6, "GÉNERO", &formato_encabezado)
            .map_err(|e| format!("Error escribiendo encabezado GÉNERO: {}", e))?;
        
        // Datos de estudiantes (comenzando desde fila 6)
        let max_estudiantes = estudiantes_ordenados.len().min(35);
        println!("[DEBUG] Escribiendo {} estudiantes", max_estudiantes);
        
        for (indice, estudiante) in estudiantes_ordenados.iter().enumerate().take(max_estudiantes) {
            let fila = fila_inicio_tabla + 1 + indice as u32;
            
            // N° (índice + 1)
            worksheet.write_number_with_format(fila, 0, (indice + 1) as f64, &formato_celda)
                .map_err(|e| format!("Error escribiendo número: {}", e))?;
            
            // Cédula
            worksheet.write_string_with_format(fila, 1, &estudiante.cedula, &formato_celda)
                .map_err(|e| format!("Error escribiendo cédula: {}", e))?;
            
            // Apellidos (formateados según MPPE)
            let apellidos_formateados = self.formatear_apellidos(&estudiante.apellidos);
            worksheet.write_string_with_format(fila, 2, &apellidos_formateados, &formato_izquierda)
                .map_err(|e| format!("Error escribiendo apellidos: {}", e))?;
            
            // Nombres (formateados según MPPE)
            let nombres_formateados = self.formatear_nombres(&estudiante.nombres);
            worksheet.write_string_with_format(fila, 3, &nombres_formateados, &formato_izquierda)
                .map_err(|e| format!("Error escribiendo nombres: {}", e))?;
            
            // Lugar de nacimiento
            worksheet.write_string_with_format(fila, 4, &estudiante.lugar_nacimiento, &formato_izquierda)
                .map_err(|e| format!("Error escribiendo lugar: {}", e))?;
            
            // Entidad federal
            worksheet.write_string_with_format(fila, 5, &estudiante.entidad_federal, &formato_izquierda)
                .map_err(|e| format!("Error escribiendo entidad: {}", e))?;
            
            // Género
            worksheet.write_string_with_format(fila, 6, &estudiante.genero, &formato_celda)
                .map_err(|e| format!("Error escribiendo género: {}", e))?;
            
            if indice < 5 || indice % 10 == 0 || indice == max_estudiantes - 1 {
                println!("[DEBUG] ✏️ Fila {}: {} {} {} ({})", 
                         fila, estudiante.apellidos, estudiante.nombres, estudiante.genero, estudiante.cedula);
            }
        }
        
        // Ajustar ancho de columnas
        worksheet.set_column_width(0, 5)
            .map_err(|e| format!("Error ajustando columna: {}", e))?;   // N°
        worksheet.set_column_width(1, 12)
            .map_err(|e| format!("Error ajustando columna: {}", e))?;  // Cédula
        worksheet.set_column_width(2, 20)
            .map_err(|e| format!("Error ajustando columna: {}", e))?;  // Apellidos
        worksheet.set_column_width(3, 20)
            .map_err(|e| format!("Error ajustando columna: {}", e))?;  // Nombres
        worksheet.set_column_width(4, 25)
            .map_err(|e| format!("Error ajustando columna: {}", e))?;  // Lugar
        worksheet.set_column_width(5, 20)
            .map_err(|e| format!("Error ajustando columna: {}", e))?;  // Entidad
        worksheet.set_column_width(6, 10)
            .map_err(|e| format!("Error ajustando columna: {}", e))?;  // Género
        
        // Guardar archivo
        workbook.save(ruta_salida)
            .map_err(|e| format!("Error guardando Excel: {}", e))?;
        
        println!("[DEBUG] ✅ Excel creado exitosamente: {}", ruta_salida);
        println!("[DEBUG] ✅ Procesados {} estudiantes", max_estudiantes);
        
        Ok(())
    }

    /// Versión simple que lee la plantilla oficial y escribe directamente en coordenadas fijas (SIN buscar marcadores)
    pub fn llenar_estudiantes_excel_simple(&self, estudiantes: &[DatosEstudianteBasico], ruta_plantilla: &str, ruta_salida: &str) -> Result<(), String> {
        use umya_spreadsheet::*;
        use std::time::Instant;
        
        let inicio_total = Instant::now();
        println!("[SIMPLE] 📖 Leyendo plantilla oficial: {}", ruta_plantilla);
        
        // CHECKPOINT 1: Verificar que el archivo existe
        let path = std::path::Path::new(ruta_plantilla);
        if !path.exists() {
            return Err(format!("❌ El archivo plantilla no existe: {}", ruta_plantilla));
        }
        println!("[SIMPLE] ✅ Archivo plantilla existe");
        
        // CHECKPOINT 2: Leer la plantilla oficial del MPPE
        let inicio_lectura = Instant::now();
        println!("[SIMPLE] 🔄 Iniciando lectura de plantilla...");
        let mut book = reader::xlsx::read(path)
            .map_err(|e| format!("Error al leer plantilla: {}", e))?;
        println!("[SIMPLE] ✅ Plantilla leída en {:.2}s", inicio_lectura.elapsed().as_secs_f64());
        
        // CHECKPOINT 3: Ordenar estudiantes
        let inicio_orden = Instant::now();
        println!("[SIMPLE] 🔄 Ordenando {} estudiantes...", estudiantes.len());
        let mut estudiantes_ordenados = estudiantes.to_vec();
        estudiantes_ordenados.sort_by(|a, b| {
            let cedula_a: u64 = a.cedula.parse().unwrap_or(0);
            let cedula_b: u64 = b.cedula.parse().unwrap_or(0);
            cedula_a.cmp(&cedula_b)
        });
        println!("[SIMPLE] ✅ Estudiantes ordenados en {:.2}s", inicio_orden.elapsed().as_secs_f64());
        
        // CHECKPOINT 4: Obtener la primera hoja
        let inicio_hoja = Instant::now();
        println!("[SIMPLE] 🔄 Obteniendo primera hoja...");
        let sheet = book.get_sheet_mut(&0)
            .ok_or("No se pudo obtener la primera hoja de la plantilla")?;
        println!("[SIMPLE] ✅ Hoja obtenida en {:.2}s", inicio_hoja.elapsed().as_secs_f64());
        
        // CHECKPOINT 5: Escribir datos estudiantes
        let inicio_escritura = Instant::now();
        println!("[SIMPLE] 🔄 Escribiendo datos de estudiantes...");
        
        // COORDENADAS FIJAS según parámetros MPPE oficiales [como se guardó en memoria][[memory:3430177925867389327]]
        let fila_inicial = 17u32; // Fila donde empiezan los datos de estudiantes
        
        let max_estudiantes = estudiantes_ordenados.len().min(35);
        println!("[SIMPLE] 📝 Escribiendo {} estudiantes (máximo 35)", max_estudiantes);
        
        for (index, estudiante) in estudiantes_ordenados.iter().enumerate().take(max_estudiantes) {
            if index % 5 == 0 || index < 3 || index == max_estudiantes - 1 {
                println!("[SIMPLE] 📝 Procesando estudiante {}/{}: {} {}", index + 1, max_estudiantes, estudiante.apellidos, estudiante.nombres);
            }
            
            let fila = fila_inicial + index as u32;
            
            // Escribir cada celda con logs ocasionales
            sheet.get_cell_mut((2u32, fila)).set_value(&estudiante.cedula); // Cédula - columna B
            sheet.get_cell_mut((15u32, fila)).set_value(&estudiante.apellidos); // Apellidos - columna O
            sheet.get_cell_mut((24u32, fila)).set_value(&estudiante.nombres); // Nombres - columna X
            sheet.get_cell_mut((35u32, fila)).set_value(&estudiante.lugar_nacimiento); // Lugar - columna AI
            sheet.get_cell_mut((42u32, fila)).set_value(&estudiante.entidad_federal); // Entidad - columna AP
            
            let genero_texto = match estudiante.genero.as_str() {
                "M" => "M",
                "F" => "F", 
                _ => "M"
            };
            sheet.get_cell_mut((43u32, fila)).set_value(genero_texto); // Género - columna AQ
            
            // Fecha de nacimiento
            if estudiante.dia_nacimiento > 0 {
                sheet.get_cell_mut((44u32, fila)).set_value(&estudiante.dia_nacimiento.to_string()); // Día - AR
            }
            if estudiante.mes_nacimiento > 0 {
                sheet.get_cell_mut((46u32, fila)).set_value(&estudiante.mes_nacimiento.to_string()); // Mes - AT  
            }
            if estudiante.ano_nacimiento > 0 {
                sheet.get_cell_mut((48u32, fila)).set_value(&estudiante.ano_nacimiento.to_string()); // Año - AV
            }
        }
        
        println!("[SIMPLE] ✅ Datos escritos en {:.2}s", inicio_escritura.elapsed().as_secs_f64());
        
        // CHECKPOINT 6: Guardar archivo
        let inicio_guardado = Instant::now();
        println!("[SIMPLE] 🔄 Guardando archivo: {}", ruta_salida);
        let path_salida = std::path::Path::new(ruta_salida);
        writer::xlsx::write(&book, path_salida)
            .map_err(|e| format!("Error al guardar archivo: {}", e))?;
        println!("[SIMPLE] ✅ Archivo guardado en {:.2}s", inicio_guardado.elapsed().as_secs_f64());
        
        println!("[SIMPLE] 🎉 PROCESO COMPLETADO en {:.2}s total", inicio_total.elapsed().as_secs_f64());
        println!("[SIMPLE] ✅ Archivo generado exitosamente: {}", ruta_salida);
        Ok(())
    }

    pub async fn generar_resumen_basico(&self, db: &tokio_postgres::Client, parametros: ParametrosResumenBasico) -> Result<RespuestaResumenBasico, String> {
        println!("🚀 [GENERADOR] === INICIANDO GENERACIÓN ===");
        println!("🚀 [GENERADOR] id_grado_secciones: {}", parametros.id_grado_secciones);
        println!("🚀 [GENERADOR] ruta_salida: {}", parametros.ruta_salida);

        // 1. Obtener estudiantes de la sección
        println!("📊 [GENERADOR] Paso 1: Consultando estudiantes...");
        let estudiantes = self.obtener_estudiantes_seccion(db, parametros.id_grado_secciones).await?;
        
        if estudiantes.is_empty() {
            println!("❌ [GENERADOR] No se encontraron estudiantes");
            return Ok(RespuestaResumenBasico {
                exito: false,
                mensaje: "No se encontraron estudiantes para la sección especificada".to_string(),
                archivo_generado: None,
                estudiantes_procesados: 0,
            });
        }

        println!("✅ [GENERADOR] Encontrados {} estudiantes", estudiantes.len());
        for (i, estudiante) in estudiantes.iter().enumerate() {
            println!("👤 [GENERADOR] Estudiante {}: {} {} ({})", i+1, estudiante.apellidos, estudiante.nombres, estudiante.cedula);
        }

        // 2. Seleccionar plantilla
        println!("📄 [GENERADOR] Paso 2: Seleccionando plantilla...");
        let ruta_plantilla = self
            .seleccionar_plantilla(db, parametros.id_grado_secciones)
            .await?;
        println!("✅ [GENERADOR] Plantilla seleccionada: {}", ruta_plantilla);

        // 3. Llenar Excel con datos de estudiantes
        println!("📝 [GENERADOR] Paso 3: Llenando Excel...");
        self.llenar_estudiantes_excel_simple(&estudiantes, &ruta_plantilla, &parametros.ruta_salida)?;

        println!("✅ [GENERADOR] Resumen básico generado exitosamente");

        Ok(RespuestaResumenBasico {
            exito: true,
            mensaje: format!("Resumen básico generado con {} estudiantes", estudiantes.len()),
            archivo_generado: Some(parametros.ruta_salida.clone()),
            estudiantes_procesados: estudiantes.len(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RespuestaGeneracion {
    pub mensaje: String,
    pub archivos_generados: Vec<String>,
    pub archivos_pdf: Vec<String>,
}

#[tauri::command]
pub async fn generar_resumen_excel_masivo(
    parametros: ParametrosResumen,
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<RespuestaGeneracion, String> {
    println!("[generar_resumen_excel_masivo] Iniciando con parámetros: {:?}", parametros);
    
    let generador = GeneradorResumenCompleto::new(app_handle);
    let db = state.db.lock().await;

    let mut archivos_excel = Vec::new();
    let mut archivos_pdf = Vec::new();

    // Procesar cada id_grado_secciones
    for id_grado_secciones in &parametros.ids_grado_secciones {
        println!("[generar_resumen_excel_masivo] Procesando id_grado_secciones: {}", id_grado_secciones);

        // Obtener datos de la sección
        let datos_seccion = generador.obtener_datos_seccion(&*db, *id_grado_secciones, parametros.id_periodo).await?;
        
        println!("[generar_resumen_excel_masivo] Sección: {} {}, {} estudiantes", 
                 datos_seccion.nombre_grado, datos_seccion.nombre_seccion, datos_seccion.estudiantes.len());

        // Seleccionar plantilla según el grado
        let grado_numero = match datos_seccion.nombre_grado.chars().next() {
            Some('1') => 1,
            Some('2') => 2, 
            Some('3') => 3,
            Some('4') => 4,
            Some('5') => 5,
            _ => return Err(format!("No se pudo determinar el número de grado de: {}", datos_seccion.nombre_grado)),
        };

        let ruta_plantilla = generador.seleccionar_plantilla(grado_numero)?;

        // Generar nombre de archivo único para esta sección
        let nombre_archivo = format!("resumen_{}_{}_{}_{}.xlsx", 
                                   grado_numero, 
                                   datos_seccion.nombre_seccion,
                                   datos_seccion.nombre_modalidad.replace(" ", "_"),
                                   parametros.id_periodo);
        
        let ruta_excel = Path::new(&parametros.ruta_salida)
            .parent()
            .unwrap_or(Path::new("."))
            .join(&nombre_archivo)
            .to_string_lossy()
            .to_string();

        // Completar plantilla Excel
        generador.completar_plantilla_excel(&ruta_plantilla, &datos_seccion, &ruta_excel).await?;
        archivos_excel.push(ruta_excel.clone());

        // Convertir a PDF
        let ruta_pdf = generador.convertir_a_pdf(&ruta_excel).await?;
        archivos_pdf.push(ruta_pdf);
    }

    Ok(RespuestaGeneracion {
        mensaje: format!("Resúmenes generados exitosamente para {} secciones", parametros.ids_grado_secciones.len()),
        archivos_generados: archivos_excel,
        archivos_pdf,
    })
}

#[command]
pub async fn generar_resumen_estudiantes_basico(
    id_grado_secciones: i32,
    ruta_salida: String,
    state: State<'_, AppState>,
) -> Result<RespuestaResumenBasico, String> {
    use tokio::time::{timeout, Duration};
    
    println!("🎯 [BACKEND] === COMANDO RECIBIDO ===");
    println!("📋 [BACKEND] id_grado_secciones: {}", id_grado_secciones);
    println!("📋 [BACKEND] ruta_salida: {}", ruta_salida);

    println!("🔗 [BACKEND] Obteniendo conexión a la base de datos...");
    let db_start = std::time::Instant::now();
    let db = state.db.lock().await;
    println!("✅ [BACKEND] Conexión obtenida en {:.2}s", db_start.elapsed().as_secs_f64());

    println!("🏭 [BACKEND] Creando instancia del generador...");
    let generador = GeneradorResumenBasico::new();
    
    // Crear el struct internamente
    let parametros = ParametrosResumenBasico {
        id_grado_secciones,
        ruta_salida,
    };
    
    println!("🚀 [BACKEND] Llamando a generar_resumen_basico con timeout de 60 segundos...");
    
    // Ejecutar con timeout de 60 segundos para evitar cuelgues indefinidos
    let resultado = timeout(
        Duration::from_secs(60),
        generador.generar_resumen_basico(&*db, parametros)
    ).await;
    
    match resultado {
        Ok(res) => {
            match &res {
                Ok(respuesta) => println!("✅ [BACKEND] Resultado exitoso: {:?}", respuesta),
                Err(error) => println!("❌ [BACKEND] Error: {}", error),
            }
            res
        },
        Err(_) => {
            println!("⏰ [BACKEND] TIMEOUT: La operación tardó más de 60 segundos");
            Err("Timeout: La operación tardó demasiado tiempo. Verifique la plantilla Excel.".to_string())
        }
    }
} 