//! Generador de filas de estudiantes para el resumen final

use crate::api::plantillas::Asignatura;
use super::super::utils::constants::*;

// Importar EstudianteResumenFinal del archivo original
// TODO: Mover estos tipos al módulo de modelos en una futura iteración

/// Generador especializado en crear filas de estudiantes
pub struct EstudiantesGenerator;

impl EstudiantesGenerator {
    /// Crea una nueva instancia del generador de estudiantes
    pub fn new() -> Self {
        Self
    }

    /// Genera las filas HTML para un conjunto de estudiantes
    pub fn generar_filas_estudiantes<T>(
        &self, 
        estudiantes_pagina: &[T], 
        asignaturas: &[Asignatura], 
        _tipo_evaluacion: &str, 
        _id_grado: i32
    ) -> String 
    where 
        T: EstudianteData
    {
        let mut filas_html = String::new();
        
        for i in 0..ESTUDIANTES_POR_PAGINA {
            let numero_estudiante = i + 1;
            
            if i < estudiantes_pagina.len() {
                let estudiante = &estudiantes_pagina[i];
                let fila = self.generar_fila_estudiante_individual(estudiante, numero_estudiante, asignaturas);
                filas_html.push_str(&fila);
            } else {
                // Fila vacía para mantener formato uniforme
                let fila_vacia = self.generar_fila_vacia(numero_estudiante, asignaturas.len());
                filas_html.push_str(&fila_vacia);
            }
        }
        
        filas_html
    }

    /// Genera una fila individual para un estudiante (formato oficial MPPE)
    fn generar_fila_estudiante_individual<T>(&self, estudiante: &T, numero: usize, asignaturas: &[Asignatura]) -> String 
    where 
        T: EstudianteData
    {
        let calificaciones = self.generar_calificaciones_completas(estudiante, asignaturas);
        
        // Aplicar formateo dinámico para evitar 2 líneas
        let (cedula_formateada, tamano_fuente_cedula) = Self::formatear_cedula(estudiante.get_cedula());
        let tamano_fuente_apellidos = Self::determinar_tamano_fuente(estudiante.get_apellidos());
        let tamano_fuente_nombres = Self::determinar_tamano_fuente(estudiante.get_nombres());
        let tamano_fuente_lugar = Self::determinar_tamano_fuente(estudiante.get_lugar_nacimiento());
        
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
            Self::formatear_numero(numero as i32),
            tamano_fuente_cedula, cedula_formateada,
            tamano_fuente_apellidos, estudiante.get_apellidos(),
            tamano_fuente_nombres, estudiante.get_nombres(),
            tamano_fuente_lugar, estudiante.get_lugar_nacimiento(),
            estudiante.get_entidad_federal(),
            estudiante.get_genero(),
            Self::formatear_numero(estudiante.get_dia_nacimiento()),
            Self::formatear_numero(estudiante.get_mes_nacimiento()),
            estudiante.get_ano_nacimiento(),
            calificaciones[0], calificaciones[1], calificaciones[2], calificaciones[3],
            calificaciones[4], calificaciones[5], calificaciones[6], calificaciones[7],
            calificaciones[8], calificaciones[9], calificaciones[10], calificaciones[11],
            estudiante.get_pgcrp()
        )
    }

    /// Genera las 12 calificaciones para un estudiante (formato oficial MPPE)
    fn generar_calificaciones_completas<T>(&self, estudiante: &T, asignaturas: &[Asignatura]) -> Vec<String> 
    where 
        T: EstudianteData
    {
        let mut calificaciones = vec![String::new(); MAX_COLUMNAS_CALIFICACIONES];
        
        // Llenar las calificaciones según las asignaturas disponibles
        for (i, asignatura) in asignaturas.iter().take(MAX_COLUMNAS_CALIFICACIONES).enumerate() {
            calificaciones[i] = estudiante.get_calificacion(asignatura.id_asignatura)
                .unwrap_or_default();
        }
        
        calificaciones
    }

    /// Genera una fila vacía para mantener el formato (formato oficial MPPE)
    fn generar_fila_vacia(&self, numero: usize, _num_asignaturas: usize) -> String {
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
            </tr>"#,
            Self::formatear_numero(numero as i32)
        )
    }

    // ===== FUNCIONES DE FORMATEO CRÍTICAS (COPIADAS DEL ORIGINAL) =====
    
    /// Formatea número con ceros adelante (formato oficial MPPE)
    fn formatear_numero(num: i32) -> String {
        if num < 10 {
            format!("0{}", num)
        } else {
            num.to_string()
        }
    }

    /// Determina el tamaño de fuente basado en la longitud del texto (evita 2 líneas)
    fn determinar_tamano_fuente(texto: &str) -> &'static str {
        if texto.len() > 18 { "8px" } else { "9px" }
    }

    /// Formatea cédula con formato "V 00000000" y tamaño dinámico
    fn formatear_cedula(cedula: &str) -> (String, &'static str) {
        let cedula_limpia = cedula.trim();
        let cedula_formateada = format!("V {}", cedula_limpia);
        let tamano_fuente = if cedula_limpia.len() > 8 { "7px" } else { "9px" };
        (cedula_formateada, tamano_fuente)
    }

    /// Formatea calificación negativa con cero a la izquierda
    fn formatear_calificacion_negativa(calificacion: i32) -> String {
        if calificacion < 0 {
            // Para calificaciones negativas, mostrar con cero a la izquierda
            format!("{:02}", calificacion.abs())
        } else {
            // Para calificaciones positivas, mostrar normal
            calificacion.to_string()
        }
    }
}

impl Default for EstudiantesGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait para abstraer el acceso a datos de estudiante
/// Esto permite flexibilidad con diferentes estructuras de datos
pub trait EstudianteData {
    fn get_cedula(&self) -> &str;
    fn get_apellidos(&self) -> &str;
    fn get_nombres(&self) -> &str;
    fn get_lugar_nacimiento(&self) -> &str;
    fn get_entidad_federal(&self) -> &str;
    fn get_genero(&self) -> &str;
    fn get_dia_nacimiento(&self) -> i32;
    fn get_mes_nacimiento(&self) -> i32;
    fn get_ano_nacimiento(&self) -> i32;
    fn get_calificacion(&self, id_asignatura: i32) -> Option<String>;
    fn get_pgcrp(&self) -> &str;
} 