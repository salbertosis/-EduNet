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
    pub async fn generar_tercera_tabla_dinamica(
        &self, 
        html_content: &mut String, 
        asignaturas: &[Asignatura], 
        id_grado: i32, 
        total_estudiantes: usize, 
        estudiantes_en_pagina_actual: usize,
        id_modalidad: i32,
        db: &tokio_postgres::Client,
        id_grado_secciones: i32
    ) -> Result<(), String> {
        let mut filas_tercera_tabla = String::new();
        
        // Obtener año cursado y sección desde la BD
        let (ano_cursado, seccion) = Self::obtener_ano_seccion_bd(db, id_grado_secciones).await?;
        
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
                estudiantes_en_pagina_actual.to_string() 
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
            let contenido_columna_7_8 = if i == 0 { 
                let (plan_estudio, _) = Self::obtener_plan_estudio_por_modalidad(id_modalidad);
                plan_estudio
            } else if i == 1 { 
                "CÓDIGO".to_string() 
            } else if i == 2 { 
                let (_, codigo_plan) = Self::obtener_plan_estudio_por_modalidad(id_modalidad);
                codigo_plan
            } else if i == 3 { 
                "AÑO CURSADO".to_string()
            } else if i == 4 { 
                ano_cursado.clone()
            } else if i == 5 { 
                "SECCIÓN".to_string()
            } else if i == 6 { 
                seccion.clone()
            } else { 
                "".to_string() 
            };
                format!(
                    r#"<tr>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: left; padding: 2px;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 2px;{}"{}>{}</td>
      </tr>"#,
                    numero, codigo_fila, nombre_fila, 
                    if i == 0 || i == 2 || i == 4 || i == 6 { "" } else { " font-weight: bold;" },
                    colspan_7_8, contenido_columna_7_8
                )
            } else if i == 6 {
                // Para la fila 7 (índice 6), usar el bloque combinado (rowspan="2" colspan="2") con el valor de sección
                format!(
                    r#"<tr>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0; font-weight: bold;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: left; padding: 2px;">{}</td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 8px; text-align: center; padding: 0;"></td>
        <td style="border: 1px solid #000; font-size: 10px; text-align: center; padding: 2px;"{}>{}</td>
      </tr>"#,
                    numero, codigo_fila, nombre_fila, rowspan_colspan_7_8, seccion
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
        
        Ok(())
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
    /// Obtiene el código de asignatura completo usando la abreviatura de la BD
    fn obtener_codigo_asignatura_completo(asignatura: &Asignatura, id_grado: i32) -> String {
        if !asignatura.abreviatura.is_empty() {
            // Usar directamente la abreviatura de la BD
            asignatura.abreviatura.clone()
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

    /// Obtiene el plan de estudio y código según la modalidad
    fn obtener_plan_estudio_por_modalidad(id_modalidad: i32) -> (String, String) {
        match id_modalidad {
            1 => ("EDUCACION MEDIA GENERAL".to_string(), "31059".to_string()), // Ciencias
            2 => ("EDUCACION MEDIA GENERAL Y TECNICA".to_string(), "43298".to_string()), // Telemática
            _ => ("EDUCACION MEDIA GENERAL".to_string(), "31059".to_string()), // Por defecto
        }
    }

    /// Obtiene el año cursado y sección desde la base de datos
    async fn obtener_ano_seccion_bd(
        db: &tokio_postgres::Client,
        id_grado_secciones: i32,
    ) -> Result<(String, String), String> {
        const QUERY: &str = "
            SELECT 
                g.nombre_grado_letra as ano_cursado,
                s.nombre_seccion as seccion
            FROM grado_secciones gs
            JOIN grados g ON gs.id_grado = g.id_grado
            JOIN secciones s ON gs.id_seccion = s.id_seccion
            WHERE gs.id_grado_secciones = $1
        ";
        
        let row = db.query_one(QUERY, &[&id_grado_secciones])
            .await
            .map_err(|e| format!("Error consultando año y sección: {}", e))?;
        
        let ano_cursado: String = row.get("ano_cursado");
        let seccion: String = row.get("seccion");
        
        Ok((ano_cursado, seccion))
    }
} 