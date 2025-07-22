use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tauri::{command, AppHandle, State};
use tokio::time::timeout;
use base64::Engine as _;
use crate::AppState;

// ===== CONSTANTES Y CONFIGURACIÓN =====
const ESTUDIANTES_POR_PAGINA: usize = 35;
const MAX_COLUMNAS_CALIFICACIONES: usize = 12;
const TIMEOUT_HTML: u64 = 60;
const TIMEOUT_PDF: u64 = 120;
const DEFAULT_ANO_ESCOLAR: &str = "2024-2025";
const DEFAULT_TIPO_EVALUACION: &str = "FINAL";

// ===== TIPOS DE DATOS OPTIMIZADOS =====
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EstudianteResumenFinal {
    pub numero: i32,
    pub cedula: String,
    pub apellidos: String,
    pub nombres: String,
    pub lugar_nacimiento: String,
    pub entidad_federal: String,
    pub genero: String,
    pub dia_nacimiento: i32,
    pub mes_nacimiento: i32,
    pub ano_nacimiento: i32,
    pub calificaciones: HashMap<String, Option<i32>>,
    pub pgcrp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RespuestaResumenFinal {
    pub exito: bool,
    pub mensaje: String,
    pub archivo_generado: Option<String>,
    pub estudiantes_procesados: usize,
}

#[derive(Debug, Clone)]
pub struct DatosInstitucion {
    pub codigo: String,
    pub denominacion: String,
    pub direccion: String,
    pub telefono: String,
    pub municipio: String,
    pub entidad_federal: String,
    pub cdcee: String,
    pub director: String,
    pub cedula_director: String,
}

impl Default for DatosInstitucion {
    fn default() -> Self {
        Self {
            codigo: "SIN DATOS".to_string(),
            denominacion: "INSTITUCIÓN EDUCATIVA".to_string(),
            direccion: "DIRECCIÓN NO CONFIGURADA".to_string(),
            telefono: String::new(),
            municipio: String::new(),
            entidad_federal: String::new(),
            cdcee: String::new(),
            director: String::new(),
            cedula_director: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Asignatura {
    pub nombre: String,
    pub abreviatura: String,
    pub nombre_largo: String,
}

impl Asignatura {
    pub fn new(nombre: String, abreviatura: String, nombre_largo: String) -> Self {
        Self { nombre, abreviatura, nombre_largo }
    }
    
    pub fn from_string(asignatura_completa: &str) -> Self {
        let partes: Vec<&str> = asignatura_completa.split('|').collect();
        let nombre = partes.get(0).unwrap_or(&"").to_string();
        let abreviatura = partes.get(1).unwrap_or(&"").to_string();
        let nombre_largo = nombre.clone(); // Por defecto usa el nombre normal
        Self { nombre, abreviatura, nombre_largo }
    }
}

// ===== GENERADOR OPTIMIZADO =====
pub struct GeneradorResumenFinal {
    app_handle: AppHandle,
}

impl GeneradorResumenFinal {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    // ===== MÉTODOS DE BASE DE DATOS OPTIMIZADOS =====
    
    /// Obtiene los datos de la institución de forma optimizada
    async fn obtener_datos_institucion(&self, db: &tokio_postgres::Client) -> Result<DatosInstitucion, String> {
        const QUERY: &str = "
            SELECT codigo, denominacion, direccion, telefono, municipio, entidad_federal, cdcee, director, cedula_director
            FROM institucion 
            WHERE id = 1
        ";
        
        let rows = db.query(QUERY, &[])
            .await
            .map_err(|e| format!("Error consultando datos de institución: {}", e))?;
        
        if rows.is_empty() {
            return Ok(DatosInstitucion::default());
        }
        
        let row = &rows[0];
        Ok(DatosInstitucion {
            codigo: row.get("codigo"),
            denominacion: row.get("denominacion"),
            direccion: row.get("direccion"),
            telefono: row.get::<_, Option<String>>("telefono").unwrap_or_default(),
            municipio: row.get("municipio"),
            entidad_federal: row.get("entidad_federal"),
            cdcee: row.get::<_, Option<String>>("cdcee").unwrap_or_default(),
            director: row.get("director"),
            cedula_director: row.get("cedula_director"),
        })
    }

    /// Obtiene el año escolar activo de forma optimizada
    async fn obtener_ano_escolar_activo(&self, db: &tokio_postgres::Client) -> Result<String, String> {
        const QUERY: &str = "
            SELECT periodo_escolar
            FROM periodos_escolares 
            WHERE activo = true
            ORDER BY id_periodo DESC
            LIMIT 1
        ";
        
        let rows = db.query(QUERY, &[])
            .await
            .map_err(|e| format!("Error consultando año escolar activo: {}", e))?;
        
        if rows.is_empty() {
            return Ok(DEFAULT_ANO_ESCOLAR.to_string());
        }
        
        Ok(rows[0].get("periodo_escolar"))
    }

    /// Obtiene las asignaturas del grado de forma optimizada
    async fn obtener_asignaturas_grado(&self, db: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<Vec<Asignatura>, String> {
        // Obtener grado y modalidad en una sola consulta
        let (id_grado, id_modalidad) = self.obtener_grado_modalidad(db, id_grado_secciones).await?;
        
        const QUERY_ASIGNATURAS: &str = "
            SELECT 
                a.nombre, 
                COALESCE(a.abreviatura, UPPER(LEFT(a.nombre, 2))) as abreviatura,
                COALESCE(a.nombre_largo, a.nombre) as nombre_largo
            FROM asignaturas a
            INNER JOIN grado_modalidad_asignaturas gma ON a.id_asignatura = gma.id_asignatura
            WHERE gma.id_grado = $1 AND gma.id_modalidad = $2
            ORDER BY gma.orden
        ";
        
        let rows = db.query(QUERY_ASIGNATURAS, &[&id_grado, &id_modalidad])
            .await
            .map_err(|e| format!("Error consultando asignaturas: {}", e))?;
        
        Ok(rows.iter()
            .map(|row| Asignatura::new(
                row.get("nombre"),
                row.get("abreviatura"),
                row.get("nombre_largo")
            ))
            .collect())
    }

    /// Obtiene grado y modalidad de forma optimizada
    async fn obtener_grado_modalidad(&self, db: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<(i32, i32), String> {
        const QUERY: &str = "
            SELECT id_grado, id_modalidad
            FROM grado_secciones 
            WHERE id_grado_secciones = $1
        ";
        
        let rows = db.query(QUERY, &[&id_grado_secciones])
            .await
            .map_err(|e| format!("Error consultando grado_secciones: {}", e))?;
        
        if rows.is_empty() {
            return Err(format!("No se encontró grado_secciones con ID: {}", id_grado_secciones));
        }
        
        let row = &rows[0];
        Ok((row.get("id_grado"), row.get("id_modalidad")))
    }

    /// Obtiene el tipo de evaluación de forma optimizada
    async fn obtener_tipo_evaluacion(&self, db: &tokio_postgres::Client, id_tipo_evaluacion: i32) -> Result<String, String> {
        const QUERY: &str = "
            SELECT nombre
            FROM tipos_evaluacion 
            WHERE id = $1 AND activo = true
        ";
        
        let rows = db.query(QUERY, &[&id_tipo_evaluacion])
            .await
            .map_err(|e| format!("Error consultando tipo de evaluación: {}", e))?;
        
        if rows.is_empty() {
            return Ok(DEFAULT_TIPO_EVALUACION.to_string());
        }
        
        Ok(rows[0].get("nombre"))
    }

    /// Obtiene estudiantes de forma optimizada
    async fn obtener_estudiantes_resumen_final(&self, db: &tokio_postgres::Client, id_grado_secciones: i32, id_tipo_evaluacion: i32) -> Result<Vec<EstudianteResumenFinal>, String> {
        // Primero obtener el tipo de evaluación
        let tipo_evaluacion = self.obtener_tipo_evaluacion(db, id_tipo_evaluacion).await?;
        
        if tipo_evaluacion == "FINAL" {
            // Para evaluación FINAL, obtener estudiantes con sus calificaciones
            self.obtener_estudiantes_con_calificaciones_finales(db, id_grado_secciones).await
        } else {
            // Para otros tipos de evaluación, obtener solo datos básicos
            self.obtener_estudiantes_basicos(db, id_grado_secciones).await
        }
    }

    /// Obtiene estudiantes con calificaciones finales
    async fn obtener_estudiantes_con_calificaciones_finales(&self, db: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<Vec<EstudianteResumenFinal>, String> {
        // Primero obtener el grado para las asignaturas
        let (id_grado, _) = self.obtener_grado_modalidad(db, id_grado_secciones).await?;
        
        const QUERY: &str = "
            SELECT 
                hge.id_estudiante,
                e.cedula,
                e.apellidos,
                e.nombres,
                COALESCE(m.nombre, '') as lugar_nacimiento,
                COALESCE(est.entidadfed, '') as entidad_federal,
                e.genero,
                EXTRACT(DAY FROM e.fecha_nacimiento)::integer as dia_nacimiento,
                EXTRACT(MONTH FROM e.fecha_nacimiento)::integer as mes_nacimiento,
                EXTRACT(YEAR FROM e.fecha_nacimiento)::integer as ano_nacimiento,
                c.id_asignatura,
                c.nota_final,
                COALESCE(pg.nombre, '') as pgcrp_nombre
            FROM estudiantes e
            INNER JOIN historial_grado_estudiantes hge ON e.id = hge.id_estudiante
            LEFT JOIN municipios m ON e.municipio_nac_id = m.id
            LEFT JOIN estados est ON e.estado_nac_id = est.id
            LEFT JOIN calificaciones c ON e.id = c.id_estudiante
            LEFT JOIN estudiantes_pgcrp ep ON e.id = ep.id_estudiante AND ep.id_periodo = hge.id_periodo
            LEFT JOIN \"PGCRP\" pg ON ep.id_pgcrp = pg.id_pgcrp
            WHERE hge.id_grado_secciones = $1 AND hge.es_actual = true
            ORDER BY e.cedula, c.id_asignatura
        ";
        
        let rows = db.query(QUERY, &[&id_grado_secciones])
            .await
            .map_err(|e| format!("Error consultando estudiantes con calificaciones: {}", e))?;
        
        // Agrupar calificaciones por estudiante
        let mut estudiantes_map: HashMap<i32, EstudianteResumenFinal> = HashMap::new();
        
        for row in rows {
            let id_estudiante: i32 = row.get("id_estudiante");
            let cedula: i64 = row.get("cedula");
            
            // Obtener o crear el estudiante
            let estudiante = estudiantes_map.entry(id_estudiante).or_insert_with(|| {
                let pgcrp_nombre: String = row.get("pgcrp_nombre");
                EstudianteResumenFinal {
                    numero: 0, // Se asignará después
                    cedula: cedula.to_string(),
                    apellidos: row.get("apellidos"),
                    nombres: row.get("nombres"),
                    lugar_nacimiento: row.get("lugar_nacimiento"),
                    entidad_federal: row.get("entidad_federal"),
                    genero: row.get("genero"),
                    dia_nacimiento: row.get("dia_nacimiento"),
                    mes_nacimiento: row.get("mes_nacimiento"),
                    ano_nacimiento: row.get("ano_nacimiento"),
                    calificaciones: HashMap::new(),
                    pgcrp: if pgcrp_nombre.is_empty() { None } else { Some(pgcrp_nombre) },
                }
            });
            
            // Agregar calificación si existe
            if let Some(id_asignatura) = row.get::<_, Option<i32>>("id_asignatura") {
                if let Some(nota_final) = row.get::<_, Option<i32>>("nota_final") {
                    // Obtener el código de asignatura basado en el id_asignatura
                    let codigo_asignatura = self.obtener_codigo_asignatura_por_id(db, id_asignatura, id_grado).await?;
                    estudiante.calificaciones.insert(codigo_asignatura, Some(nota_final));
                }
            }
        }
        
        // Convertir a vector y asignar números
        let mut estudiantes: Vec<EstudianteResumenFinal> = estudiantes_map.into_values().collect();
        estudiantes.sort_by(|a, b| a.cedula.cmp(&b.cedula));
        
        for (i, estudiante) in estudiantes.iter_mut().enumerate() {
            estudiante.numero = (i + 1) as i32;
        }
        
        Ok(estudiantes)
    }

    /// Obtiene estudiantes sin calificaciones (para otros tipos de evaluación)
    async fn obtener_estudiantes_basicos(&self, db: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<Vec<EstudianteResumenFinal>, String> {
        const QUERY: &str = "
            SELECT 
                hge.id_estudiante,
                e.cedula,
                e.apellidos,
                e.nombres,
                COALESCE(m.nombre, '') as lugar_nacimiento,
                COALESCE(est.entidadfed, '') as entidad_federal,
                e.genero,
                EXTRACT(DAY FROM e.fecha_nacimiento)::integer as dia_nacimiento,
                EXTRACT(MONTH FROM e.fecha_nacimiento)::integer as mes_nacimiento,
                EXTRACT(YEAR FROM e.fecha_nacimiento)::integer as ano_nacimiento,
                COALESCE(pg.nombre, '') as pgcrp_nombre
            FROM estudiantes e
            INNER JOIN historial_grado_estudiantes hge ON e.id = hge.id_estudiante
            LEFT JOIN municipios m ON e.municipio_nac_id = m.id
            LEFT JOIN estados est ON e.estado_nac_id = est.id
            LEFT JOIN estudiantes_pgcrp ep ON e.id = ep.id_estudiante AND ep.id_periodo = hge.id_periodo
            LEFT JOIN \"PGCRP\" pg ON ep.id_pgcrp = pg.id_pgcrp
            WHERE hge.id_grado_secciones = $1 AND hge.es_actual = true
            ORDER BY e.cedula
        ";
        
        let rows = db.query(QUERY, &[&id_grado_secciones])
            .await
            .map_err(|e| format!("Error consultando estudiantes: {}", e))?;
        
        Ok(rows.iter().enumerate().map(|(i, row)| {
            let cedula: i64 = row.get("cedula");
            let pgcrp_nombre: String = row.get("pgcrp_nombre");
            EstudianteResumenFinal {
                numero: (i + 1) as i32,
                cedula: cedula.to_string(),
                apellidos: row.get("apellidos"),
                nombres: row.get("nombres"),
                lugar_nacimiento: row.get("lugar_nacimiento"),
                entidad_federal: row.get("entidad_federal"),
                genero: row.get("genero"),
                dia_nacimiento: row.get("dia_nacimiento"),
                mes_nacimiento: row.get("mes_nacimiento"),
                ano_nacimiento: row.get("ano_nacimiento"),
                calificaciones: HashMap::new(),
                pgcrp: if pgcrp_nombre.is_empty() { None } else { Some(pgcrp_nombre) },
            }
        }).collect())
    }

    /// Obtiene el código de asignatura por ID
    async fn obtener_codigo_asignatura_por_id(&self, db: &tokio_postgres::Client, id_asignatura: i32, id_grado: i32) -> Result<String, String> {
        const QUERY: &str = "
            SELECT nombre, COALESCE(abreviatura, UPPER(LEFT(nombre, 2))) as abreviatura, COALESCE(nombre_largo, nombre) as nombre_largo
            FROM asignaturas 
            WHERE id_asignatura = $1
        ";
        
        let rows = db.query(QUERY, &[&id_asignatura])
            .await
            .map_err(|e| format!("Error consultando asignatura por ID: {}", e))?;
        
        if rows.is_empty() {
            return Ok("OT".to_string()); // Código por defecto
        }
        
        let row = &rows[0];
        let asignatura = Asignatura::new(
            row.get("nombre"),
            row.get("abreviatura"),
            row.get("nombre_largo")
        );
        
        Ok(Self::obtener_codigo_asignatura_completo(&asignatura, id_grado))
    }

    // ===== MÉTODOS DE UTILIDAD OPTIMIZADOS =====
    
    /// Obtiene el código de asignatura de forma optimizada
    fn obtener_codigo_asignatura(nombre: &str, id_grado: Option<i32>) -> &'static str {
        let nombre_upper = nombre.to_uppercase();
        
        // Mapeo optimizado usando match
        match () {
            _ if nombre_upper.contains("BIOLOGÍA") || nombre_upper.contains("BIOLOGIA") => {
                match id_grado {
                    Some(grado) if grado <= 2 => "CN",
                    _ => "BI"
                }
            },
            _ if nombre_upper.contains("CASTELLANO") || nombre_upper.contains("LENGUA") => "CA",
            _ if nombre_upper.contains("INGLÉS") || nombre_upper.contains("INGLES") || nombre_upper.contains("LENGUA EXTRANJERA") => "ILE",
            _ if nombre_upper.contains("MATEMÁTICA") || nombre_upper.contains("MATEMATICA") => "MA",
            _ if nombre_upper.contains("EDUCACIÓN FÍSICA") || nombre_upper.contains("EDUCACION FISICA") => "EF",
            _ if nombre_upper.contains("FÍSICA") && !nombre_upper.contains("EDUCACIÓN") => "FI",
            _ if nombre_upper.contains("QUÍMICA") || nombre_upper.contains("QUIMICA") => "QU",
            _ if nombre_upper.contains("GEOGRAFÍA") || nombre_upper.contains("GEOGRAFIA") || nombre_upper.contains("HISTORIA") || nombre_upper.contains("CIUDADANÍA") => "GH",
            _ if nombre_upper.contains("FORMACIÓN") || nombre_upper.contains("FORMACION") || nombre_upper.contains("SOBERANÍA") => "FN",
            _ if nombre_upper.contains("ORIENTACIÓN") || nombre_upper.contains("ORIENTACION") || nombre_upper.contains("CONVIVENCIA") => "OC",
            _ if nombre_upper.contains("PARTICIPACIÓN") || nombre_upper.contains("PARTICIPACION") || nombre_upper.contains("GRUPOS") || nombre_upper.contains("CREACIÓN") => "PG",
            _ => {
                let codigo = nombre_upper.chars().take(2).collect::<String>();
                match codigo.as_str() {
                    "CS" => "CS",
                    "IN" => "IN",
                    _ => "OT"
                }
            }
        }
    }

    /// Obtiene el código de asignatura considerando abreviatura y lógica especial
    fn obtener_codigo_asignatura_completo(asignatura: &Asignatura, id_grado: i32) -> String {
        // Caso especial para BAT (Biología)
        if asignatura.nombre.to_uppercase() == "BAT" || 
           asignatura.abreviatura.to_uppercase() == "BI" {
            // Para 1ero y 2do año usar CN, para 3ero en adelante usar BI
            if id_grado <= 2 {
                "CN".to_string() // 1er y 2do año: Ciencias Naturales
            } else {
                "BI".to_string() // 3er año en adelante: Biología
            }
        } else if !asignatura.abreviatura.is_empty() {
            // Para otras asignaturas, usar la abreviatura de la BD
            match asignatura.abreviatura.to_uppercase().as_str() {
                "CA" => "CA".to_string(),
                "ILE" => "ILE".to_string(), 
                "MA" => "MA".to_string(),
                "EF" => "EF".to_string(),
                "FI" => "FI".to_string(),
                "QU" => "QU".to_string(),
                "BI" => "BI".to_string(),
                "GH" => "GH".to_string(),
                "FN" => "FN".to_string(),
                "OC" => "OC".to_string(),
                "PG" => "PG".to_string(),
                _ => asignatura.abreviatura.clone() // Usar la abreviatura tal como está
            }
        } else {
            // Si no hay abreviatura, generar una basada en el nombre
            Self::obtener_codigo_asignatura(&asignatura.nombre, Some(id_grado)).to_string()
        }
    }

    /// Formatea número con ceros adelante
    fn formatear_numero(num: i32) -> String {
        if num < 10 {
            format!("0{}", num)
        } else {
            num.to_string()
        }
    }

    /// Determina el tamaño de fuente basado en la longitud del texto
    fn determinar_tamano_fuente(texto: &str) -> &'static str {
        if texto.len() > 18 { "8px" } else { "9px" }
    }

    /// Formatea cédula con formato "V 00000000"
    fn formatear_cedula(cedula: &str) -> (String, &'static str) {
        let cedula_limpia = cedula.trim();
        let cedula_formateada = format!("V {}", cedula_limpia);
        let tamano_fuente = if cedula_limpia.len() > 8 { "7px" } else { "9px" };
        (cedula_formateada, tamano_fuente)
    }

    // ===== MÉTODOS DE GENERACIÓN HTML OPTIMIZADOS =====
    
    /// Genera el resumen final HTML de forma optimizada
    pub async fn generar_resumen_final_html_directo(&self, db: &tokio_postgres::Client, id_grado_secciones: i32, id_tipo_evaluacion: i32) -> Result<String, String> {
        // Obtener todos los datos en paralelo usando join!
        let (datos_institucion, ano_escolar, tipo_evaluacion, estudiantes, asignaturas, id_grado) = tokio::try_join!(
            self.obtener_datos_institucion(db),
            self.obtener_ano_escolar_activo(db),
            self.obtener_tipo_evaluacion(db, id_tipo_evaluacion),
            self.obtener_estudiantes_resumen_final(db, id_grado_secciones, id_tipo_evaluacion),
            async {
                let tipo = self.obtener_tipo_evaluacion(db, id_tipo_evaluacion).await?;
                if tipo == "FINAL" {
                    self.obtener_asignaturas_grado(db, id_grado_secciones).await
                } else {
                    Ok(vec![])
                }
            },
            async {
                let (id_grado, _) = self.obtener_grado_modalidad(db, id_grado_secciones).await?;
                Ok(id_grado)
            }
        )?;
        
        if estudiantes.is_empty() {
            return Err("No se encontraron estudiantes para la sección especificada".to_string());
        }

        self.generar_html_resumen_final(&estudiantes, &datos_institucion, &tipo_evaluacion, &ano_escolar, &asignaturas, id_grado)
    }

    /// Genera el HTML del resumen final de forma optimizada
    fn generar_html_resumen_final(&self, estudiantes: &[EstudianteResumenFinal], datos_institucion: &DatosInstitucion, tipo_evaluacion: &str, ano_escolar: &str, asignaturas: &[Asignatura], id_grado: i32) -> Result<String, String> {
        // Cargar plantilla y logo de forma optimizada
        let mut html_content = self.cargar_plantilla_html()?;
        let logo_base64 = self.cargar_logo_base64()?;
        
        // Reemplazar placeholders de forma optimizada
        self.reemplazar_placeholders_basicos(&mut html_content, datos_institucion, tipo_evaluacion, ano_escolar, &logo_base64);
        self.reemplazar_placeholders_asignaturas(&mut html_content, asignaturas, tipo_evaluacion, id_grado, datos_institucion);
        
        // Generar páginas de forma optimizada
        self.generar_paginas_estudiantes(&mut html_content, estudiantes, asignaturas, tipo_evaluacion, id_grado)
    }

    /// Carga la plantilla HTML de forma optimizada
    fn cargar_plantilla_html(&self) -> Result<String, String> {
        let plantilla_path = self.obtener_ruta_plantilla()?;
        std::fs::read_to_string(&plantilla_path)
            .map_err(|e| format!("Error leyendo plantilla HTML: {}", e))
    }

    /// Carga el logo en base64 de forma optimizada
    fn cargar_logo_base64(&self) -> Result<String, String> {
        let logo_path = self.obtener_ruta_logo()?;
        
        if !logo_path.exists() {
            return Ok(String::new());
        }
        
        let logo_data = std::fs::read(&logo_path)
            .map_err(|e| format!("Error leyendo logo: {}", e))?;
        
        Ok(base64::engine::general_purpose::STANDARD.encode(&logo_data))
    }

    /// Obtiene la ruta de la plantilla de forma optimizada
    fn obtener_ruta_plantilla(&self) -> Result<PathBuf, String> {
        Ok(std::env::current_dir()
            .map_err(|e| format!("Error obteniendo directorio actual: {}", e))?
            .join("plantillas")
            .join("resumen_final_app.html"))
    }

    /// Obtiene la ruta del logo de forma optimizada
    fn obtener_ruta_logo(&self) -> Result<PathBuf, String> {
        Ok(std::env::current_dir()
            .map_err(|e| format!("Error obteniendo directorio actual: {}", e))?
            .join("imagenes")
            .join("Logo_ministerio_resumen.png"))
    }

    /// Reemplaza placeholders básicos de forma optimizada
    fn reemplazar_placeholders_basicos(&self, html_content: &mut String, datos_institucion: &DatosInstitucion, tipo_evaluacion: &str, ano_escolar: &str, logo_base64: &str) {
        let reemplazos = [
            ("{{LOGO_BASE64}}", logo_base64),
            ("{{CODIGO_INSTITUCION}}", &datos_institucion.codigo),
            ("{{DENOMINACION_INSTITUCION}}", &datos_institucion.denominacion),
            ("{{DIRECCION_INSTITUCION}}", &datos_institucion.direccion),
            ("{{TELEFONO_INSTITUCION}}", &datos_institucion.telefono),
            ("{{MUNICIPIO_INSTITUCION}}", &datos_institucion.municipio),
            ("{{ENTIDAD_FEDERAL_INSTITUCION}}", &datos_institucion.entidad_federal),
            ("{{CDCEE_INSTITUCION}}", &datos_institucion.cdcee),
            ("{{DIRECTOR_INSTITUCION}}", &datos_institucion.director),
            ("{{CEDULA_DIRECTOR_INSTITUCION}}", &datos_institucion.cedula_director),
            ("{{TIPO_EVALUACION}}", tipo_evaluacion),
            ("{{ANO_ESCOLAR}}", ano_escolar),
        ];

        for (placeholder, valor) in reemplazos {
            *html_content = html_content.replace(placeholder, valor);
        }
    }

    /// Reemplaza placeholders de asignaturas de forma optimizada
    fn reemplazar_placeholders_asignaturas(&self, html_content: &mut String, asignaturas: &[Asignatura], tipo_evaluacion: &str, id_grado: i32, datos_institucion: &DatosInstitucion) {
        if tipo_evaluacion == "FINAL" && !asignaturas.is_empty() {
            // Reemplazar asignaturas dinámicas
            for (i, asignatura) in asignaturas.iter().take(MAX_COLUMNAS_CALIFICACIONES).enumerate() {
                let placeholder = format!("{{{{ASIGNATURA_{}}}}}", i + 1);
                let codigo = Self::obtener_codigo_asignatura_completo(asignatura, id_grado);
                *html_content = html_content.replace(&placeholder, &codigo);
            }
            
            // Rellenar placeholders vacíos con "*"
            for i in asignaturas.len()..MAX_COLUMNAS_CALIFICACIONES {
                let placeholder = format!("{{{{ASIGNATURA_{}}}}}", i + 1);
                *html_content = html_content.replace(&placeholder, "*");
            }
        } else {
            // Usar "*" para columnas vacías
            for i in 0..MAX_COLUMNAS_CALIFICACIONES {
                let placeholder = format!("{{{{ASIGNATURA_{}}}}}", i + 1);
                *html_content = html_content.replace(&placeholder, "*");
            }
        }
        
        // Generar tercera tabla dinámicamente
        self.generar_tercera_tabla_dinamica(html_content, asignaturas, id_grado);
        
        // Generar cuarta tabla dinámicamente
        self.generar_cuarta_tabla_dinamica(html_content, &datos_institucion);
    }
    
    /// Genera la cuarta tabla dinámicamente con los datos del director
    fn generar_cuarta_tabla_dinamica(&self, html_content: &mut String, datos_institucion: &DatosInstitucion) {
        // Reemplazar los placeholders del director
        *html_content = html_content.replace("{{DIRECTOR_NOMBRE}}", &datos_institucion.director);
        *html_content = html_content.replace("{{DIRECTOR_CEDULA}}", &datos_institucion.cedula_director);
    }
    
    /// Genera la tercera tabla dinámicamente con las asignaturas
    fn generar_tercera_tabla_dinamica(&self, html_content: &mut String, asignaturas: &[Asignatura], id_grado: i32) {
        let mut filas_tercera_tabla = String::new();
        
        // Generar exactamente 12 filas
        for i in 0..12 {
            let numero = format!("{:02}", i + 1);
            
            if i < asignaturas.len() {
                // Fila con asignatura real
                let asignatura = &asignaturas[i];
                let codigo = Self::obtener_codigo_asignatura_completo(asignatura, id_grado);
                
                let fila = format!(
                    r#"<tr>
            <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
            <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
            <td style="border: 1px solid #000; font-size: 8px; text-align: left; padding: 2px;">{}</td>
            <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
            <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
            <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
            <td colspan="2" style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
          </tr>"#,
                    numero, codigo, asignatura.nombre_largo
                );
                
                filas_tercera_tabla.push_str(&fila);
            } else {
                // Fila vacía con "*"
                let fila = format!(
                    r#"<tr>
            <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
            <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">*</td>
            <td style="border: 1px solid #000; font-size: 8px; text-align: left; padding: 2px;">*</td>
            <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
            <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
            <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
            <td colspan="2" style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
          </tr>"#,
                    numero
                );
                
                filas_tercera_tabla.push_str(&fila);
            }
        }
        
        // Reemplazar el placeholder de la tercera tabla
        *html_content = html_content.replace("{{TERCERA_TABLA_CONTENIDO}}", &filas_tercera_tabla);
    }

    /// Genera páginas de estudiantes de forma optimizada
    fn generar_paginas_estudiantes(&self, html_content: &mut String, estudiantes: &[EstudianteResumenFinal], asignaturas: &[Asignatura], tipo_evaluacion: &str, id_grado: i32) -> Result<String, String> {
        let total_paginas = (estudiantes.len() + ESTUDIANTES_POR_PAGINA - 1) / ESTUDIANTES_POR_PAGINA;
        let mut paginas_html = String::new();
        
        for pagina in 0..total_paginas {
            let inicio = pagina * ESTUDIANTES_POR_PAGINA;
            let fin = std::cmp::min(inicio + ESTUDIANTES_POR_PAGINA, estudiantes.len());
            let estudiantes_pagina = &estudiantes[inicio..fin];
            
            let mut pagina_html = html_content.clone();
            let filas_html = self.generar_filas_estudiantes(estudiantes_pagina, asignaturas, tipo_evaluacion, id_grado);
            
            // Reemplazar tbody de forma optimizada
            self.reemplazar_tbody(&mut pagina_html, &filas_html);
            
            // Agregar salto de página si no es la última
            if pagina < total_paginas - 1 {
                pagina_html = pagina_html.replace("</body>", "<div style='page-break-before: always;'></div></body>");
            }
            
            paginas_html.push_str(&pagina_html);
        }
        
        Ok(paginas_html)
    }

    /// Genera filas de estudiantes de forma optimizada
    fn generar_filas_estudiantes(&self, estudiantes_pagina: &[EstudianteResumenFinal], asignaturas: &[Asignatura], tipo_evaluacion: &str, id_grado: i32) -> String {
        let mut filas_html = String::new();
        
        for i in 0..ESTUDIANTES_POR_PAGINA {
            let numero_estudiante = i + 1;
            
            if i < estudiantes_pagina.len() {
                let estudiante = &estudiantes_pagina[i];
                let fila = self.generar_fila_estudiante(estudiante, numero_estudiante, asignaturas, tipo_evaluacion, id_grado);
                filas_html.push_str(&fila);
            } else {
                let fila = self.generar_fila_vacia(numero_estudiante);
                filas_html.push_str(&fila);
            }
        }
        
        filas_html
    }

    /// Genera una fila de estudiante de forma optimizada
    fn generar_fila_estudiante(&self, estudiante: &EstudianteResumenFinal, numero_estudiante: usize, asignaturas: &[Asignatura], tipo_evaluacion: &str, id_grado: i32) -> String {
        let (cedula_formateada, tamano_fuente_cedula) = Self::formatear_cedula(&estudiante.cedula);
        let tamano_fuente_apellidos = Self::determinar_tamano_fuente(&estudiante.apellidos);
        let tamano_fuente_nombres = Self::determinar_tamano_fuente(&estudiante.nombres);
        let tamano_fuente_lugar = Self::determinar_tamano_fuente(&estudiante.lugar_nacimiento);
        
        let calificaciones = self.obtener_calificaciones_dinamicas(estudiante, asignaturas, tipo_evaluacion, id_grado);
        let pgcrp = estudiante.pgcrp.clone().unwrap_or_default();
        
        format!(
            r#"<tr style="height: 0.3142cm;">
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: {}; text-align: left; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: {}; text-align: left; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: {}; text-align: left; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: {}; text-align: left; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
            </tr>"#,
            Self::formatear_numero(numero_estudiante as i32),
            tamano_fuente_cedula, cedula_formateada,
            tamano_fuente_apellidos, estudiante.apellidos,
            tamano_fuente_nombres, estudiante.nombres,
            tamano_fuente_lugar, estudiante.lugar_nacimiento,
            estudiante.entidad_federal,
            estudiante.genero,
            Self::formatear_numero(estudiante.dia_nacimiento),
            Self::formatear_numero(estudiante.mes_nacimiento),
            estudiante.ano_nacimiento,
            calificaciones[0], calificaciones[1], calificaciones[2], calificaciones[3],
            calificaciones[4], calificaciones[5], calificaciones[6], calificaciones[7],
            calificaciones[8], calificaciones[9], calificaciones[10], calificaciones[11],
            pgcrp
        )
    }

    /// Genera una fila vacía de forma optimizada
    fn generar_fila_vacia(&self, numero_estudiante: usize) -> String {
        format!(
            r#"<tr style="height: 0.3142cm;">
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{}</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: left; padding: 0;">V *</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: left; padding: 0;">*</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: left; padding: 0;">*</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: left; padding: 0;">*</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">*</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">*</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">*</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">*</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">*</td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
                <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;"></td>
            </tr>"#,
            Self::formatear_numero(numero_estudiante as i32)
        )
    }

    /// Obtiene calificaciones dinámicas de forma optimizada
    fn obtener_calificaciones_dinamicas(&self, estudiante: &EstudianteResumenFinal, asignaturas: &[Asignatura], tipo_evaluacion: &str, id_grado: i32) -> Vec<String> {
        let mut calificaciones = vec![String::new(); MAX_COLUMNAS_CALIFICACIONES];
        
        if tipo_evaluacion == "FINAL" && !asignaturas.is_empty() {
            // Para evaluación FINAL, usar las calificaciones ya cargadas del estudiante
            for (i, asignatura) in asignaturas.iter().take(MAX_COLUMNAS_CALIFICACIONES).enumerate() {
                let codigo = Self::obtener_codigo_asignatura_completo(asignatura, id_grado);
                
                // Buscar la calificación en el HashMap del estudiante
                calificaciones[i] = estudiante.calificaciones.get(&codigo)
                    .unwrap_or(&None)
                    .map(|c| c.to_string())
                    .unwrap_or_default();
            }
        } else if tipo_evaluacion == "FINAL" {
            // Para evaluación FINAL sin asignaturas específicas, usar todas las calificaciones disponibles
            let mut i = 0;
            for (codigo, calificacion) in &estudiante.calificaciones {
                if i < MAX_COLUMNAS_CALIFICACIONES {
                    calificaciones[i] = calificacion
                        .map(|c| c.to_string())
                        .unwrap_or_default();
                    i += 1;
                } else {
                    break;
                }
            }
        } else {
            // Para otros tipos de evaluación, usar códigos hardcodeados como fallback
            let codigos = ["CA", "ILE", "MA", "EF", "FI", "IN", "QU", "BI", "GH", "FN", "OC", "PG"];
            for (i, codigo) in codigos.iter().enumerate() {
                calificaciones[i] = estudiante.calificaciones.get(*codigo)
                    .unwrap_or(&None)
                    .map(|c| c.to_string())
                    .unwrap_or_default();
            }
        }
        
        calificaciones
    }

    /// Reemplaza el tbody de forma optimizada
    fn reemplazar_tbody(&self, pagina_html: &mut String, filas_html: &str) {
        let tbody_pattern = "<tbody id=\"estudiantes-tbody\">";
        let tbody_end_pattern = "</tbody>";
        
        if let (Some(start_pos), Some(end_pos)) = (pagina_html.find(tbody_pattern), pagina_html.find(tbody_end_pattern)) {
            let new_tbody = format!("<tbody id=\"estudiantes-tbody\">{}</tbody>", filas_html);
            *pagina_html = pagina_html[..start_pos].to_string() + &new_tbody + &pagina_html[end_pos + tbody_end_pattern.len()..];
        }
    }

    // ===== MÉTODOS DE CONVERSIÓN PDF OPTIMIZADOS =====
    
    /// Convierte HTML a PDF de forma optimizada
    async fn convertir_html_a_pdf(&self, html_content: &str, ruta_salida: &str) -> Result<(), String> {
        use headless_chrome::{Browser, LaunchOptionsBuilder};
        use std::fs;
        use tempfile::NamedTempFile;
        
        // Crear archivo temporal
        let temp_html = NamedTempFile::new()
            .map_err(|e| format!("Error creando archivo temporal: {}", e))?;
        
        fs::write(&temp_html, html_content)
            .map_err(|e| format!("Error escribiendo HTML temporal: {}", e))?;
        
        // Configurar y lanzar navegador
        let launch_options = LaunchOptionsBuilder::default()
            .headless(true)
            .idle_browser_timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| format!("Error configurando opciones de lanzamiento: {}", e))?;
        
        let browser = Browser::new(launch_options)
            .map_err(|e| format!("Error lanzando navegador: {}", e))?;
        
        let tab = browser.new_tab()
            .map_err(|e| format!("Error creando pestaña: {}", e))?;
        
        // Navegar y esperar
        tab.navigate_to(&format!("file://{}", temp_html.path().display()))
            .map_err(|e| format!("Error navegando a HTML: {}", e))?;
        
        tab.wait_until_navigated()
            .map_err(|e| format!("Error esperando navegación: {}", e))?;
        
        // Esperar renderizado
        tokio::time::sleep(Duration::from_millis(5000)).await;
        
        // Generar PDF
        let pdf_data = tab.print_to_pdf(Some(headless_chrome::types::PrintToPdfOptions {
            landscape: Some(false),
            display_header_footer: Some(false),
            print_background: Some(true),
            scale: Some(1.0),
            paper_width: Some(21.59),
            paper_height: Some(27.94),
            margin_top: Some(0.1),
            margin_bottom: Some(0.1),
            margin_left: Some(0.1),
            margin_right: Some(0.1),
            page_ranges: None,
            ignore_invalid_page_ranges: Some(false),
            header_template: None,
            footer_template: None,
            prefer_css_page_size: Some(false),
            generate_document_outline: Some(false),
            generate_tagged_pdf: Some(false),
            transfer_mode: None,
        }))
        .map_err(|e| format!("Error generando PDF: {}", e))?;
        
        // Guardar PDF
        fs::write(ruta_salida, pdf_data)
            .map_err(|e| format!("Error guardando PDF: {}", e))?;
        
        Ok(())
    }
}

// ===== COMANDOS TAURI OPTIMIZADOS =====

#[command]
pub async fn generar_resumen_final_html_directo(
    id_grado_secciones: i32,
    id_tipo_evaluacion: i32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let db = state.db.lock().await;
    let generador = GeneradorResumenFinal::new(state.app_handle.clone());
    
    timeout(
        Duration::from_secs(TIMEOUT_HTML),
        generador.generar_resumen_final_html_directo(&*db, id_grado_secciones, id_tipo_evaluacion)
    ).await
    .map_err(|_| "Timeout: La generación del HTML tardó demasiado".to_string())?
}

#[command]
pub async fn generar_resumen_final_pdf_directo(
    id_grado_secciones: i32,
    id_tipo_evaluacion: i32,
    ruta_salida: String,
    state: State<'_, AppState>,
) -> Result<RespuestaResumenFinal, String> {
    let db = state.db.lock().await;
    let generador = GeneradorResumenFinal::new(state.app_handle.clone());
    
    // Generar HTML
    let html_content = generador.generar_resumen_final_html_directo(&*db, id_grado_secciones, id_tipo_evaluacion).await?;
    
    // Convertir a PDF
    timeout(
        Duration::from_secs(TIMEOUT_PDF),
        generador.convertir_html_a_pdf(&html_content, &ruta_salida)
    ).await
    .map_err(|_| "Timeout: La generación del PDF tardó demasiado".to_string())??;
    
    Ok(RespuestaResumenFinal {
        exito: true,
        mensaje: "PDF generado exitosamente".to_string(),
        archivo_generado: Some(ruta_salida),
        estudiantes_procesados: 0,
    })
} 