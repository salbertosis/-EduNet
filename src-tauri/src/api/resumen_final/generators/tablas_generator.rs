//! Generador de tablas dinámicas para el resumen final

use crate::api::plantillas::Asignatura;
use super::super::utils::constants::*;

/// Generador especializado en crear las tablas del resumen final
pub struct TablasGenerator;

impl TablasGenerator {
    /// Crea una nueva instancia del generador de tablas
    pub fn new() -> Self {
        Self
    }

    /// Genera la tercera tabla dinámicamente con asignaturas REALES de la BD
    /// 
    /// ✅ ASIGNATURAS DINÁMICAS: Usa abreviaciones y nombres largos reales de la BD
    /// ✅ ESTRUCTURA ORIGINAL: Mantiene exactamente la misma lógica del archivo original
    /// - Filas 1-6: Columnas 7 y 8 combinadas horizontalmente
    /// - Fila 7: Rowspan=2 y Colspan=2 para "PLAN DE ESTUDIO"
    /// - Fila 8: Omite columnas 7 y 8 (ocupadas por fila 7)
    /// - Fila 9: Datos de estudiantes con rowspan=2
    /// - Fila 10: Sin columnas 7-8 (ocupadas por fila 9)
    /// - Fila 11: PGCRP con rowspan=2
    /// - Fila 12: Sin columnas 7-8 (ocupadas por fila 11)
    pub fn generar_tercera_tabla_dinamica(
        &self, 
        html_content: &mut String, 
        asignaturas: &[Asignatura], 
        id_grado: i32, 
        total_estudiantes: usize, 
        estudiantes_por_pagina: usize
    ) {
        let mut filas_tercera_tabla = String::new();
        
        // Generar exactamente 12 filas como en el original
        for i in 0..12 {
            let numero = format!("{:02}", i + 1);
            
            // Determinar el contenido específico para cada fila
            let (codigo_fila, nombre_fila) = if i < asignaturas.len() {
                // Usar asignaturas REALES de la BD
                let asignatura = &asignaturas[i];
                let codigo = Self::obtener_codigo_asignatura_completo(asignatura, id_grado);
                let nombre = asignatura.nombre_largo.as_ref()
                    .unwrap_or(&asignatura.nombre_asignatura)
                    .clone();
                (codigo, nombre)
            } else {
                // Filas especiales según el original
                match i {
                    11 => ("*".to_string(), "*".to_string()),
                    _ => ("*".to_string(), "*".to_string()),
                }
            };
            
            // Preparar los valores para las columnas de estudiantes (posiciones fijas como el original)
            let columna_estudiantes_1 = if i == 8 { 
                "N° DE ESTUDIANTES POR SECCIÓN".to_string() 
            } else if i == 9 { 
                total_estudiantes.to_string() 
            } else { 
                "".to_string() 
            };
            
            let columna_estudiantes_2 = if i == 8 { 
                "N° DE ESTUDIANTES EN ESTA PÁGINA".to_string() 
            } else if i == 9 { 
                estudiantes_por_pagina.to_string() 
            } else { 
                "".to_string() 
            };
            
            // Determinar si esta fila debe tener rowspan para las columnas de estudiantes (exacto del original)
            let (rowspan_7, rowspan_8) = if i == 6 {
                // Fila 7 (índice 6) - combina filas y columnas (rowspan="2" colspan="2")
                ("", "") // No se usan porque usaremos rowspan_colspan_7_8
            } else if i == 7 {
                // Fila 8 (índice 7) - no incluye columnas 7 y 8 porque están combinadas con fila 7
                ("", "")
            } else if i == 8 {
                (r#" rowspan="2""#, r#" rowspan="2""#)
            } else if i == 9 {
                ("", "") // Esta fila no tendrá las columnas 7 y 8
            } else if i == 10 {
                (r#" rowspan="2""#, r#" rowspan="2""#)
            } else if i == 11 {
                ("", "") // Esta fila no tendrá las columnas 7 y 8
            } else {
                ("", "")
            };
            
            // Determinar si esta fila debe tener colspan para combinar columnas 7 y 8 (exacto del original)
            let colspan_7_8 = if i == 0 || i == 1 || i == 2 || i == 3 || i == 4 || i == 5 {
                r#" colspan="2""#
            } else {
                ""
            };
            
            // Para la fila 7 (índice 6), crear un bloque combinado con rowspan y colspan (exacto del original)
            let rowspan_colspan_7_8 = if i == 6 {
                r#" rowspan="2" colspan="2""#
            } else {
                ""
            };
            
            // Generar HTML exactamente como el original
            let fila = if i == 7 || i == 9 || i == 11 {
                // Para las filas 8 (índice 7), 10 (índice 9) y 12 (índice 11), no incluir las columnas 7 y 8
                format!(
                    r#"<tr>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: left; padding: 2px;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
      </tr>"#,
                    numero, codigo_fila, nombre_fila
                )
            } else if i == 0 || i == 1 || i == 2 || i == 3 || i == 4 || i == 5 {
                // Para las filas 1-6, combinar las columnas 7 y 8
                let contenido_columna_7_8 = if i == 1 { "CÓDIGO" } else if i == 3 { "AÑO CURSADO" } else if i == 5 { "SECCIÓN" } else { "" };
                format!(
                    r#"<tr>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: left; padding: 2px;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 2px; font-weight: bold;"{}>{}</td>
      </tr>"#,
                    numero, codigo_fila, nombre_fila, colspan_7_8, contenido_columna_7_8
                )
            } else if i == 6 {
                // Para la fila 7 (índice 6), usar el bloque combinado (rowspan="2" colspan="2")
                format!(
                    r#"<tr>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: left; padding: 2px;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 2px; font-weight: bold;"{}></td>
      </tr>"#,
                    numero, codigo_fila, nombre_fila, rowspan_colspan_7_8
                )
            } else {
                // Para las demás filas (8, 10), incluir todas las columnas con rowspan cuando corresponda
                format!(
                    r#"<tr>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: left; padding: 2px;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 2px; font-weight: bold;"{}>{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 2px; font-weight: bold;"{}>{}</td>
      </tr>"#,
                    numero, codigo_fila, nombre_fila, rowspan_7, columna_estudiantes_1, rowspan_8, columna_estudiantes_2
                )
            };
                
            filas_tercera_tabla.push_str(&fila);
        }
        
        // Reemplazar el placeholder de la tercera tabla
        *html_content = html_content.replace(PLACEHOLDER_TERCERA_TABLA, &filas_tercera_tabla);
    }

    /// Genera la primera tabla del resumen (información institucional)
    pub fn generar_primera_tabla(&self, html_content: &mut String, modalidad: &str, grado: &str, seccion: &str) {
        *html_content = html_content.replace(PLACEHOLDER_MODALIDAD, modalidad);
        *html_content = html_content.replace(PLACEHOLDER_GRADO, grado);
        *html_content = html_content.replace(PLACEHOLDER_SECCION, seccion);
    }

    /// Genera la segunda tabla del resumen (encabezados de asignaturas)
    pub fn generar_segunda_tabla(&self, html_content: &mut String, asignaturas: &[Asignatura]) {
        // Implementación futura según necesidades específicas
        // Por ahora mantiene compatibilidad con el código existente
    }
}

impl Default for TablasGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl TablasGenerator {
    /// Obtiene el código de asignatura completo (copiado del HtmlProcessor)
    fn obtener_codigo_asignatura_completo(asignatura: &Asignatura, id_grado: i32) -> String {
        // Caso especial para BAT (Biología)
        if asignatura.nombre_asignatura.to_uppercase() == "BAT" || 
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
            Self::obtener_codigo_asignatura_fallback(&asignatura.nombre_asignatura, id_grado)
        }
    }

    /// Función fallback para generar código cuando no hay abreviatura
    fn obtener_codigo_asignatura_fallback(nombre: &str, id_grado: i32) -> String {
        if id_grado <= 2 {
            // Para 1er y 2do año, usar nombres más cortos
            match nombre.to_uppercase().as_str() {
                "CASTELLANO" => "CA".to_string(),
                "MATEMÁTICAS" => "MA".to_string(),
                "INGLÉS" => "ILE".to_string(),
                "EDUCACIÓN FÍSICA" => "EF".to_string(),
                "FÍSICA" => "FI".to_string(),
                "QUÍMICA" => "QU".to_string(),
                "BIOLOGÍA" => "BI".to_string(),
                _ => nombre.chars().take(3).collect::<String>().to_uppercase(),
            }
        } else {
            nombre.chars().take(5).collect::<String>().to_uppercase()
        }
    }
} 