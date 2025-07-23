//! Procesador de datos para el resumen final

use super::super::utils::{constants::*, helpers::*};
use super::super::models::EstadisticasResumen;

/// Procesador especializado en manipulación y validación de datos
pub struct DataProcessor;

impl DataProcessor {
    /// Crea una nueva instancia del procesador de datos
    pub fn new() -> Self {
        Self
    }

    /// Procesa y valida los datos de estudiantes
    pub fn procesar_datos_estudiantes<T>(&self, estudiantes: &[T]) -> Result<(), String> 
    where 
        T: std::fmt::Debug
    {
        if estudiantes.is_empty() {
            return Err(formatear_error("Data Processor", "Lista de estudiantes vacía"));
        }

        // Validaciones básicas
        self.validar_limite_estudiantes(estudiantes)?;
        
        Ok(())
    }

    /// Calcula estadísticas del resumen
    pub fn calcular_estadisticas(&self, total_estudiantes: usize, estudiantes_por_pagina: usize) -> EstadisticasResumen {
        let total_paginas = calcular_total_paginas(total_estudiantes, estudiantes_por_pagina);
        
        EstadisticasResumen {
            total_estudiantes,
            estudiantes_por_pagina,
            total_paginas,
            promedio_general: None, // Se puede calcular con datos específicos
            aprobados: 0,           // Se puede calcular con calificaciones
            aplazados: 0,           // Se puede calcular con calificaciones
        }
    }

    /// Valida que no se exceda el límite de estudiantes por procesamiento
    fn validar_limite_estudiantes<T>(&self, estudiantes: &[T]) -> Result<(), String> 
    where 
        T: std::fmt::Debug
    {
        const MAX_ESTUDIANTES: usize = 1000;
        
        if estudiantes.len() > MAX_ESTUDIANTES {
            return Err(formatear_error(
                "Data Processor", 
                &format!("Demasiados estudiantes ({}). Máximo: {}", estudiantes.len(), MAX_ESTUDIANTES)
            ));
        }
        
        Ok(())
    }

    /// Normaliza los datos de entrada
    pub fn normalizar_datos<T>(&self, datos: &mut T) 
    where 
        T: Normalizable
    {
        datos.normalizar();
    }

    /// Valida la integridad de los datos
    pub fn validar_integridad_datos<T>(&self, datos: &[T]) -> Result<(), String> 
    where 
        T: Validable
    {
        for (indice, dato) in datos.iter().enumerate() {
            if let Err(error) = dato.validar() {
                return Err(formatear_error(
                    "Data Processor", 
                    &format!("Error en elemento {}: {}", indice, error)
                ));
            }
        }
        
        Ok(())
    }

    /// Genera un resumen de los datos procesados
    pub fn generar_resumen_procesamiento<T>(&self, datos: &[T]) -> String 
    where 
        T: std::fmt::Debug
    {
        format!(
            "Procesamiento completado: {} elementos procesados exitosamente", 
            datos.len()
        )
    }

    /// Filtra datos según criterios específicos
    pub fn filtrar_datos<'a, T, F>(&self, datos: &'a [T], criterio: F) -> Vec<&'a T> 
    where 
        F: Fn(&T) -> bool
    {
        datos.iter().filter(|item| criterio(item)).collect()
    }

    /// Ordena datos según un comparador específico
    pub fn ordenar_datos<T, F>(&self, datos: &mut [T], comparador: F) 
    where 
        F: Fn(&T, &T) -> std::cmp::Ordering
    {
        datos.sort_by(comparador);
    }
}

impl Default for DataProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait para elementos que pueden ser normalizados
pub trait Normalizable {
    fn normalizar(&mut self);
}

/// Trait para elementos que pueden ser validados
pub trait Validable {
    fn validar(&self) -> Result<(), String>;
}

/// Implementación por defecto para tipos básicos que no requieren normalización
impl Normalizable for String {
    fn normalizar(&mut self) {
        *self = self.trim().to_string();
    }
}

/// Implementación por defecto de validación para String
impl Validable for String {
    fn validar(&self) -> Result<(), String> {
        if self.trim().is_empty() {
            Err("String vacío".to_string())
        } else {
            Ok(())
        }
    }
} 