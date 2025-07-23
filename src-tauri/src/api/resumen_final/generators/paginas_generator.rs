//! Generador de páginas con paginación para el resumen final

use super::super::utils::{constants::*, helpers::*};
use super::super::models::InfoPaginacion;

/// Generador especializado en paginación y generación de páginas múltiples
pub struct PaginasGenerator;

impl PaginasGenerator {
    /// Crea una nueva instancia del generador de páginas
    pub fn new() -> Self {
        Self
    }

    /// Genera múltiples páginas HTML con estudiantes distribuidos
    pub fn generar_paginas_estudiantes<T, F>(
        &self,
        html_base: &str,
        estudiantes: &[T],
        generar_contenido: F
    ) -> Result<String, String>
    where
        F: Fn(&str, &[T], &InfoPaginacion) -> Result<String, String>
    {
        let total_paginas = calcular_total_paginas(estudiantes.len(), ESTUDIANTES_POR_PAGINA);
        let mut paginas_html = String::new();
        
        for pagina in 0..total_paginas {
            let info_paginacion = InfoPaginacion::new(pagina, estudiantes.len(), ESTUDIANTES_POR_PAGINA);
            let estudiantes_pagina = &estudiantes[info_paginacion.inicio..info_paginacion.fin];
            
            // Generar contenido de la página usando la función callback
            let mut pagina_html = generar_contenido(html_base, estudiantes_pagina, &info_paginacion)?;
            
            // Agregar salto de página si no es la última página
            if !info_paginacion.es_ultima_pagina {
                pagina_html = self.aplicar_salto_pagina(&pagina_html);
            }
            
            paginas_html.push_str(&pagina_html);
        }
        
        Ok(paginas_html)
    }

    /// Aplica un salto de página al HTML
    fn aplicar_salto_pagina(&self, html: &str) -> String {
        html.replace("</body>", "<div style='page-break-before: always;'></div></body>")
    }

    /// Genera información de paginación para una página específica
    pub fn obtener_info_paginacion(&self, pagina: usize, total_estudiantes: usize) -> InfoPaginacion {
        InfoPaginacion::new(pagina, total_estudiantes, ESTUDIANTES_POR_PAGINA)
    }

    /// Calcula estadísticas de paginación
    pub fn calcular_estadisticas_paginacion(&self, total_estudiantes: usize) -> (usize, usize, usize) {
        let total_paginas = calcular_total_paginas(total_estudiantes, ESTUDIANTES_POR_PAGINA);
        let estudiantes_ultima_pagina = if total_estudiantes % ESTUDIANTES_POR_PAGINA == 0 {
            ESTUDIANTES_POR_PAGINA
        } else {
            total_estudiantes % ESTUDIANTES_POR_PAGINA
        };
        
        (total_paginas, ESTUDIANTES_POR_PAGINA, estudiantes_ultima_pagina)
    }

    /// Valida los parámetros de paginación
    pub fn validar_paginacion(&self, estudiantes: &[impl std::fmt::Debug]) -> Result<(), String> {
        if estudiantes.is_empty() {
            return Err("No hay estudiantes para paginar".to_string());
        }
        
        if estudiantes.len() > ESTUDIANTES_POR_PAGINA * 100 {
            return Err(format!(
                "Demasiados estudiantes ({}). Máximo recomendado: {}",
                estudiantes.len(),
                ESTUDIANTES_POR_PAGINA * 100
            ));
        }
        
        Ok(())
    }

    /// Genera un resumen de la paginación para logging
    pub fn generar_resumen_paginacion(&self, total_estudiantes: usize) -> String {
        let (total_paginas, estudiantes_por_pagina, estudiantes_ultima_pagina) = 
            self.calcular_estadisticas_paginacion(total_estudiantes);
            
        format!(
            "Paginación: {} estudiantes en {} páginas ({} por página, {} en la última)",
            total_estudiantes, total_paginas, estudiantes_por_pagina, estudiantes_ultima_pagina
        )
    }
}

impl Default for PaginasGenerator {
    fn default() -> Self {
        Self::new()
    }
} 