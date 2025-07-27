//! Servicio principal para la generación de resumen final del rendimiento estudiantil

use crate::api::plantillas::Asignatura;
use super::generators::{TablasGenerator, EstudiantesGenerator, PaginasGenerator};
use super::processors::{HtmlProcessor, PdfProcessor, DataProcessor};
use super::models::{ResumenConfig, EstadisticasResumen};
use super::utils::constants::*;
use tokio_postgres;

// Re-exportar el trait EstudianteData del generador para mantener consistencia
pub use super::generators::estudiantes_generator::EstudianteData;

// Importaciones del archivo original que se mantendrán temporalmente
// TODO: Mover estos tipos al módulo de modelos en una futura iteración
// use crate::api::resumen_final_original::EstudianteResumenFinal;

/// Servicio principal para la generación de resumen final
/// 
/// Este servicio orquesta todos los componentes (generadores y procesadores)
/// para crear reportes de resumen final según las especificaciones del MPPE
pub struct ResumenFinalService {
    /// Generador de tablas dinámicas
    tablas_generator: TablasGenerator,
    /// Generador de filas de estudiantes
    estudiantes_generator: EstudiantesGenerator,
    /// Generador de páginas con paginación
    paginas_generator: PaginasGenerator,
    /// Procesador de HTML
    pub html_processor: HtmlProcessor,
    /// Procesador de PDF
    pdf_processor: PdfProcessor,
    /// Procesador de datos
    data_processor: DataProcessor,
    /// Configuración del servicio
    config: ResumenConfig,
}

impl ResumenFinalService {
    /// Crea una nueva instancia del servicio con configuración por defecto
    pub fn new() -> Self {
        Self {
            tablas_generator: TablasGenerator::new(),
            estudiantes_generator: EstudiantesGenerator::new(),
            paginas_generator: PaginasGenerator::new(),
            html_processor: HtmlProcessor::new(),
            pdf_processor: PdfProcessor::new(),
            data_processor: DataProcessor::new(),
            config: ResumenConfig::default(),
        }
    }

    /// Crea una nueva instancia del servicio con configuración personalizada
    pub fn with_config(config: ResumenConfig) -> Self {
        Self {
            tablas_generator: TablasGenerator::new(),
            estudiantes_generator: EstudiantesGenerator::new(),
            paginas_generator: PaginasGenerator::new(),
            html_processor: HtmlProcessor::new(),
            pdf_processor: PdfProcessor::new(),
            data_processor: DataProcessor::new(),
            config,
        }
    }

    /// Genera el resumen final en formato HTML (método principal con todos los placeholders oficiales)
    /// 
    /// Este es el método de alto nivel que coordina todo el proceso de generación
    pub async fn generar_resumen_final_html<T>(
        &self,
        plantilla_html: &str,
        estudiantes: &[T],
        asignaturas: &[Asignatura],
        _modalidad: &str,
        _grado: &str,
        _seccion: &str,
        id_grado: i32,
        id_modalidad: i32,
        db: &tokio_postgres::Client,
        id_grado_secciones: i32,
    ) -> Result<String, String>
    where
        T: EstudianteData + std::fmt::Debug,
    {
        // 1. Validar datos de entrada
        self.validar_datos_entrada(estudiantes, asignaturas)?;
        
        // 2. Preparar HTML base
        let mut html_content = plantilla_html.to_string();
        
        // 3. Validar estructura HTML
        self.html_processor.validar_estructura_html(&html_content)?;
        
        // 4. Reemplazar placeholders de asignaturas ({{ASIGNATURA_1}} hasta {{ASIGNATURA_12}})
        self.html_processor.reemplazar_placeholders_asignaturas(&mut html_content, asignaturas, &self.config.tipo_evaluacion, id_grado);
        
        // 5. Generar tercera tabla (plan de estudios) - usando tu función mejorada
        let estadisticas = self.data_processor.calcular_estadisticas(
            estudiantes.len(), 
            self.config.estudiantes_por_pagina
        );
        
        // TODO: Necesitamos pasar la conexión a la BD y id_grado_secciones
        // Por ahora usar valores temporales
        self.tablas_generator.generar_tercera_tabla_dinamica(
            &mut html_content,
            asignaturas,
            id_grado,
            estadisticas.total_estudiantes,
            estadisticas.estudiantes_por_pagina,
            id_modalidad,
            &db, // Necesitamos la conexión a la BD
            id_grado_secciones, // Necesitamos el id_grado_secciones
        ).await?;
        
        // 7. Generar páginas de estudiantes
        let paginas_html = self.generar_paginas_con_estudiantes(
            &html_content,
            estudiantes,
            asignaturas,
            id_grado,
        )?;
        
        // 8. Aplicar configuración para PDF si es necesario
        // self.html_processor.aplicar_configuracion_pdf(&mut paginas_html);
        
        Ok(paginas_html)
    }

    /// Genera el resumen final en formato PDF
    pub async fn generar_resumen_final_pdf<T>(
        &self,
        plantilla_html: &str,
        estudiantes: &[T],
        asignaturas: &[Asignatura],
        modalidad: &str,
        grado: &str,
        seccion: &str,
        id_grado: i32,
        id_modalidad: i32,
        db: &tokio_postgres::Client,
        id_grado_secciones: i32,
        ruta_salida: &str,
    ) -> Result<(), String>
    where
        T: EstudianteData + std::fmt::Debug,
    {
        // 1. Generar HTML
        let html_content = self.generar_resumen_final_html(
            plantilla_html,
            estudiantes,
            asignaturas,
            modalidad,
            grado,
            seccion,
            id_grado,
            id_modalidad,
            db,
            id_grado_secciones,
        ).await?;
        
        // 2. Preparar HTML para PDF
        let html_preparado = self.pdf_processor.preparar_html_para_pdf(&html_content);
        
        // 3. Convertir a PDF
        self.pdf_processor.convertir_html_a_pdf(&html_preparado, ruta_salida).await?;
        
        Ok(())
    }

    /// Valida los datos de entrada
    fn validar_datos_entrada<T>(&self, estudiantes: &[T], asignaturas: &[Asignatura]) -> Result<(), String>
    where
        T: std::fmt::Debug,
    {
        // Validar estudiantes
        self.data_processor.procesar_datos_estudiantes(estudiantes)?;
        
        // Validar asignaturas
        if asignaturas.is_empty() {
            return Err("Lista de asignaturas vacía".to_string());
        }
        
        // Validar límite de asignaturas
        if asignaturas.len() > MAX_COLUMNAS_CALIFICACIONES {
            return Err(format!(
                "Demasiadas asignaturas ({}). Máximo: {}",
                asignaturas.len(),
                MAX_COLUMNAS_CALIFICACIONES
            ));
        }
        
        Ok(())
    }

    /// Genera páginas con estudiantes distribuidos
    fn generar_paginas_con_estudiantes<T>(
        &self,
        html_base: &str,
        estudiantes: &[T],
        asignaturas: &[Asignatura],
        id_grado: i32,
    ) -> Result<String, String>
    where
        T: EstudianteData + std::fmt::Debug,
    {
        self.paginas_generator.generar_paginas_estudiantes(
            html_base,
            estudiantes,
            |html, estudiantes_pagina, info_paginacion| {
                let mut pagina_html = html.to_string();
                
                // Generar filas de estudiantes para esta página
                let filas_html = self.estudiantes_generator.generar_filas_estudiantes(
                    estudiantes_pagina,
                    asignaturas,
                    &self.config.tipo_evaluacion,
                    id_grado,
                );
                
                // Reemplazar el contenido en el HTML
                self.html_processor.reemplazar_tbody(&mut pagina_html, &filas_html);
                
                Ok(pagina_html)
            }
        )
    }

    /// Obtiene las estadísticas del último procesamiento
    pub fn obtener_estadisticas(&self, total_estudiantes: usize) -> EstadisticasResumen {
        self.data_processor.calcular_estadisticas(total_estudiantes, self.config.estudiantes_por_pagina)
    }

    /// Actualiza la configuración del servicio
    pub fn actualizar_configuracion(&mut self, nueva_config: ResumenConfig) {
        self.config = nueva_config;
    }

    /// Obtiene la configuración actual
    pub fn obtener_configuracion(&self) -> &ResumenConfig {
        &self.config
    }
}

impl Default for ResumenFinalService {
    fn default() -> Self {
        Self::new()
    }
}

// Trait para abstraer el acceso a datos de estudiante
// Se mantiene la definición local para evitar conflictos 