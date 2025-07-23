//! Modelos de datos específicos para el resumen final

use serde::{Deserialize, Serialize};

/// Configuración para la generación del resumen final
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumenConfig {
    pub estudiantes_por_pagina: usize,
    pub ano_escolar: String,
    pub tipo_evaluacion: String,
    pub mostrar_diagnostico: bool,
    pub incluir_estadisticas: bool,
}

impl Default for ResumenConfig {
    fn default() -> Self {
        Self {
            estudiantes_por_pagina: 35,
            ano_escolar: "2024-2025".to_string(),
            tipo_evaluacion: "FINAL".to_string(),
            mostrar_diagnostico: false,
            incluir_estadisticas: true,
        }
    }
}

/// Información de una fila en la tabla dinámica
#[derive(Debug, Clone)]
pub struct FilaTabla {
    pub numero: i32,
    pub codigo: String,
    pub nombre: String,
    pub tiene_rowspan: bool,
    pub tiene_colspan: bool,
    pub contenido_especial: Option<String>,
}

/// Estadísticas del resumen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadisticasResumen {
    pub total_estudiantes: usize,
    pub estudiantes_por_pagina: usize,
    pub total_paginas: usize,
    pub promedio_general: Option<f64>,
    pub aprobados: usize,
    pub aplazados: usize,
}

/// Información de paginación
#[derive(Debug, Clone)]
pub struct InfoPaginacion {
    pub pagina_actual: usize,
    pub total_paginas: usize,
    pub inicio: usize,
    pub fin: usize,
    pub es_ultima_pagina: bool,
}

impl InfoPaginacion {
    pub fn new(pagina: usize, total_estudiantes: usize, estudiantes_por_pagina: usize) -> Self {
        let total_paginas = (total_estudiantes + estudiantes_por_pagina - 1) / estudiantes_por_pagina;
        let inicio = pagina * estudiantes_por_pagina;
        let fin = std::cmp::min(inicio + estudiantes_por_pagina, total_estudiantes);
        
        Self {
            pagina_actual: pagina,
            total_paginas,
            inicio,
            fin,
            es_ultima_pagina: pagina >= total_paginas.saturating_sub(1),
        }
    }
} 