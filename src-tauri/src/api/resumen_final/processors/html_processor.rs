//! Procesador de HTML para el resumen final

use super::super::utils::{constants::*, helpers::*};
use std::collections::HashMap;
use crate::api::plantillas::Asignatura;

// Estructura de datos de institución
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

/// Procesador especializado en manipulación de HTML
pub struct HtmlProcessor;

impl HtmlProcessor {
    /// Crea una nueva instancia del procesador HTML
    pub fn new() -> Self {
        Self
    }

    /// Reemplaza múltiples placeholders en el HTML
    pub fn reemplazar_placeholders(&self, html: &mut String, placeholders: &HashMap<&str, &str>) {
        for (placeholder, valor) in placeholders {
            *html = html.replace(placeholder, valor);
        }
    }

    /// Reemplaza todos los placeholders institucionales (formato oficial MPPE)
    pub fn reemplazar_placeholders_institucionales(&self, html: &mut String, datos_institucion: &DatosInstitucion, tipo_evaluacion: &str, ano_escolar: &str, logo_base64: &str) {
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

        for (placeholder, valor) in &reemplazos {
            *html = html.replace(placeholder, valor);
        }
    }

    /// Reemplaza los placeholders de asignaturas (1-12) según el tipo de evaluación
    pub fn reemplazar_placeholders_asignaturas(&self, html: &mut String, asignaturas: &[Asignatura], tipo_evaluacion: &str, id_grado: i32) {
        if tipo_evaluacion == "FINAL" && !asignaturas.is_empty() {
            // Reemplazar asignaturas dinámicas
            for (i, asignatura) in asignaturas.iter().take(12).enumerate() {
                let placeholder = format!("{{{{ASIGNATURA_{}}}}}", i + 1);
                let codigo = Self::obtener_codigo_asignatura_completo(asignatura, id_grado);
                *html = html.replace(&placeholder, &codigo);
            }
            
            // Rellenar placeholders vacíos con "*"
            for i in asignaturas.len()..12 {
                let placeholder = format!("{{{{ASIGNATURA_{}}}}}", i + 1);
                *html = html.replace(&placeholder, "*");
            }
        } else {
            // Usar "*" para columnas vacías
            for i in 0..12 {
                let placeholder = format!("{{{{ASIGNATURA_{}}}}}", i + 1);
                *html = html.replace(&placeholder, "*");
            }
        }
    }

    /// Obtiene el código de asignatura completo (copiado exacto del original)
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

    /// Reemplaza el tbody del HTML con nuevo contenido (formato oficial MPPE)
    pub fn reemplazar_tbody(&self, html: &mut String, nuevo_tbody: &str) {
        // Buscar el patrón específico del tbody de estudiantes (plantilla oficial MPPE)
        let tbody_pattern = "<tbody id=\"estudiantes-tbody\">";
        let tbody_end_pattern = "</tbody>";
        
        if let (Some(start_pos), Some(end_pos)) = (html.find(tbody_pattern), html.find(tbody_end_pattern)) {
            // Asegurar que encontramos el </tbody> correcto después del inicio
            if end_pos > start_pos {
                let new_tbody = format!("<tbody id=\"estudiantes-tbody\">{}</tbody>", nuevo_tbody);
                *html = html[..start_pos].to_string() + &new_tbody + &html[end_pos + tbody_end_pattern.len()..];
                return;
            }
        }
        
        // Fallback 1: Buscar tbody genérico
        if let Some(inicio) = html.find("<tbody>") {
            if let Some(fin) = html.find("</tbody>") {
                let inicio_contenido = inicio + "<tbody>".len();
                
                // Construir el nuevo HTML
                let mut nuevo_html = String::with_capacity(html.len() + nuevo_tbody.len());
                nuevo_html.push_str(&html[..inicio_contenido]);
                nuevo_html.push_str(nuevo_tbody);
                nuevo_html.push_str(&html[fin..]);
                
                *html = nuevo_html;
                return;
            }
        }
        
        // Fallback 2: reemplazar placeholder si no hay tbody
        *html = html.replace(PLACEHOLDER_TBODY, nuevo_tbody);
    }

    /// Valida la estructura básica del HTML
    pub fn validar_estructura_html(&self, html: &str) -> Result<(), String> {
        let validaciones = [
            ("<html", "Etiqueta <html> faltante"),
            ("<head", "Etiqueta <head> faltante"),
            ("<body", "Etiqueta <body> faltante"),
            ("</html>", "Etiqueta </html> faltante"),
            ("</head>", "Etiqueta </head> faltante"),
            ("</body>", "Etiqueta </body> faltante"),
        ];

        for (patron, mensaje) in &validaciones {
            if !html.contains(patron) {
                return Err(formatear_error("Validación HTML", mensaje));
            }
        }

        Ok(())
    }

    /// Limpia y optimiza el HTML
    pub fn optimizar_html(&self, html: &str) -> String {
        html
            // Remover espacios extra entre etiquetas
            .replace("> <", "><")
            // Remover líneas vacías múltiples
            .replace("\n\n\n", "\n\n")
            // Remover espacios al final de líneas
            .split('\n')
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
            // Remover comentarios HTML (opcional)
            .trim()
            .to_string()
    }

    /// Inserta CSS adicional en el HTML
    pub fn insertar_css(&self, html: &mut String, css: &str) {
        if let Some(pos) = html.find("</head>") {
            let css_tag = format!("<style>\n{}\n</style>\n", css);
            html.insert_str(pos, &css_tag);
        }
    }

    /// Reemplaza contenido específico en una tabla
    pub fn reemplazar_contenido_tabla(&self, html: &mut String, selector_tabla: &str, nuevo_contenido: &str) {
        // Implementación básica para identificar tablas por clase o id
        let patron_inicio = format!("<table{}", selector_tabla);
        
        if let Some(inicio) = html.find(&patron_inicio) {
            if let Some(fin) = html[inicio..].find("</table>") {
                let fin_absoluto = inicio + fin + "</table>".len();
                
                // Extraer la estructura de la tabla y reemplazar solo el contenido interno
                if let Some(tbody_inicio) = html[inicio..fin_absoluto].find("<tbody>") {
                    if let Some(tbody_fin) = html[inicio..fin_absoluto].find("</tbody>") {
                        let tbody_inicio_abs = inicio + tbody_inicio + "<tbody>".len();
                        let tbody_fin_abs = inicio + tbody_fin;
                        
                        let mut nuevo_html = String::with_capacity(html.len() + nuevo_contenido.len());
                        nuevo_html.push_str(&html[..tbody_inicio_abs]);
                        nuevo_html.push_str(nuevo_contenido);
                        nuevo_html.push_str(&html[tbody_fin_abs..]);
                        
                        *html = nuevo_html;
                    }
                }
            }
        }
    }

    /// Genera metadatos HTML básicos
    pub fn generar_metadatos(&self, titulo: &str, descripcion: &str) -> String {
        format!(
            r#"<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<meta name="description" content="{}">
<title>{}</title>"#,
            limpiar_html(descripcion),
            limpiar_html(titulo)
        )
    }

    /// Aplica configuraciones específicas para impresión PDF
    pub fn aplicar_configuracion_pdf(&self, html: &mut String) {
        let css_pdf = r#"
@media print {
    body { 
        margin: 0;
        font-size: 9pt;
        line-height: 1.2;
    }
    table { 
        page-break-inside: avoid;
        border-collapse: collapse;
    }
    tr { 
        page-break-inside: avoid;
    }
}"#;
        
        self.insertar_css(html, css_pdf);
    }
}

impl Default for HtmlProcessor {
    fn default() -> Self {
        Self::new()
    }
} 