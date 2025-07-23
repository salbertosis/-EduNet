//! Constantes para el módulo de resumen final

// ===== CONFIGURACIÓN DE PAGINACIÓN =====
pub const ESTUDIANTES_POR_PAGINA: usize = 35;
pub const MAX_COLUMNAS_CALIFICACIONES: usize = 12;

// ===== TIMEOUTS =====
pub const TIMEOUT_HTML: u64 = 60;
pub const TIMEOUT_PDF: u64 = 120;

// ===== VALORES POR DEFECTO =====
pub const DEFAULT_ANO_ESCOLAR: &str = "2024-2025";
pub const DEFAULT_TIPO_EVALUACION: &str = "FINAL";

// ===== MEDIDAS EXACTAS OFICIALES =====
pub const ALTURA_FILA_ESTUDIANTE: &str = "0.3142cm";
pub const ANCHO_TABLA_TOTAL: &str = "20.2cm";
pub const PAPEL_ANCHO: f64 = 21.59;  // cm
pub const PAPEL_ALTO: f64 = 27.94;   // cm

// ===== PLANTILLAS HTML OFICIALES =====
pub const TEMPLATE_FILA_ESTUDIANTE_OFICIAL: &str = r#"<tr style="height: 0.3142cm;">
    <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{numero}</td>
    <td style="border: 1px solid #000; font-size: {tamano_cedula}; text-align: left; padding: 0;">{cedula}</td>
    <td style="border: 1px solid #000; font-size: {tamano_apellidos}; text-align: left; padding: 0;">{apellidos}</td>
    <td style="border: 1px solid #000; font-size: {tamano_nombres}; text-align: left; padding: 0;">{nombres}</td>
    <td style="border: 1px solid #000; font-size: {tamano_lugar}; text-align: left; padding: 0;">{lugar_nacimiento}</td>
    <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{entidad_federal}</td>
    <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{genero}</td>
    <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{dia}</td>
    <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{mes}</td>
    <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{ano}</td>
    {calificaciones}
    <td style="border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;">{pgcrp}</td>
</tr>"#;

// ===== ESTILOS CSS OFICIALES =====
pub const ESTILO_CELDA_OFICIAL: &str = "border: 1px solid #000; padding: 0;";
pub const ESTILO_CELDA_NUMERO: &str = "border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;";
pub const ESTILO_CELDA_TEXTO: &str = "border: 1px solid #000; font-size: 9px; text-align: left; padding: 0;";
pub const ESTILO_CELDA_CALIFICACION: &str = "border: 1px solid #000; font-size: 9px; text-align: center; padding: 0;";
pub const ESTILO_NUMERO: &str = "font-size: 9px; text-align: center;";
pub const ESTILO_TEXTO: &str = "text-align: left;";
pub const ESTILO_CENTRO: &str = "font-size: 9px; text-align: center;";
pub const ESTILO_CALIFICACION: &str = "font-size: 9px; text-align: center;";

// ===== FORMATOS =====
pub const FORMATO_FECHA_VENEZUELA: &str = "%d/%m/%Y";
pub const FORMATO_NUMERO_ESTUDIANTE: &str = "{:02}";

// ===== PLACEHOLDERS HTML =====
pub const PLACEHOLDER_TERCERA_TABLA: &str = "{{TERCERA_TABLA_CONTENIDO}}";
pub const PLACEHOLDER_TBODY: &str = "{{FILAS_ESTUDIANTES}}";
pub const PLACEHOLDER_PLAN_ESTUDIO: &str = "{{PLAN_ESTUDIO}}";
pub const PLACEHOLDER_MODALIDAD: &str = "{{MODALIDAD}}";
pub const PLACEHOLDER_GRADO: &str = "{{GRADO}}";
pub const PLACEHOLDER_SECCION: &str = "{{SECCION}}"; 