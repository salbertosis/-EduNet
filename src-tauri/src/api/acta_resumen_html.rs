// Archivo para generar acta de resumen usando HTML con printpdf
// Diseño oficial del gobierno venezolano - réplica exacta del formato EMG

use printpdf::*;
use std::collections::BTreeMap;
use crate::AppState;
use tauri::State;

// Estructuras para los datos del PDF (reutilizando las del archivo original)
#[derive(Debug, Clone)]
pub struct InfoEncabezado {
    pub grado: String,
    pub seccion: String,
    pub docente_guia: String,
    pub periodo_escolar: String,
    pub gcrp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CalificacionDetalle {
    pub lapso1: Option<i32>,
    pub lapso2: Option<i32>,
    pub lapso3: Option<i32>,
    pub lapso_ajustado_1: Option<i32>,
    pub lapso_ajustado_2: Option<i32>,
    pub lapso_ajustado_3: Option<i32>,
    pub definitiva: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct AsignaturaCalificaciones {
    pub nombre_asignatura: String,
    pub calificaciones: CalificacionDetalle,
}

#[derive(Debug, Clone)]
pub struct EstudianteCalificaciones {
    pub apellidos: String,
    pub nombres: String,
    pub asignaturas: Vec<AsignaturaCalificaciones>,
}

#[derive(Debug, Clone)]
pub struct AsignaturaInfo {
    pub id: i32,
    pub nombre: String,
}

// Plantilla HTML que replica exactamente el diseño oficial del Ministerio
pub fn generar_plantilla_html_oficial() -> String {
    r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        @page {
            size: A4 portrait; /* Formato A4 vertical como en la imagen */
            margin: 10mm; /* Márgenes estándar */
        }
        
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 0;
            color: #000;
            font-size: 8pt;
            line-height: 1.2;
        }
        
        /* ENCABEZADO OFICIAL DEL MINISTERIO */
        .header-oficial {
            width: 100%;
            border: 2px solid #000;
            margin-bottom: 3mm;
        }
        
        /* PRIMERA FILA - LOGO Y TÍTULOS PRINCIPALES */
        .fila-principal {
            display: flex;
            border-bottom: 1px solid #000;
            min-height: 25mm;
        }
        
        .logo-venezuela {
            width: 25mm;
            border-right: 1px solid #000;
            display: flex;
            align-items: center;
            justify-content: center;
            background-color: #f8f8f8;
            padding: 2mm;
        }
        
        .logo-placeholder {
            width: 20mm;
            height: 20mm;
            background-color: #e0e0e0;
            border: 1px solid #999;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 6pt;
            color: #666;
            text-align: center;
        }
        
        .titulos-ministerio {
            flex: 1;
            padding: 2mm;
            display: flex;
            flex-direction: column;
            justify-content: center;
        }
        
        .gobierno-bolivariano {
            font-size: 9pt;
            font-weight: bold;
            text-align: center;
            margin-bottom: 1mm;
            text-transform: uppercase;
        }
        
        .ministerio-educacion {
            font-size: 8pt;
            font-weight: bold;
            text-align: center;
            margin-bottom: 2mm;
            text-transform: uppercase;
        }
        
        .codigo-formato {
            width: 35mm;
            border-left: 1px solid #000;
            padding: 2mm;
            display: flex;
            flex-direction: column;
            justify-content: center;
            background-color: #f0f0f0;
        }
        
        .titulo-resumen {
            font-size: 7pt;
            font-weight: bold;
            text-align: center;
            margin-bottom: 1mm;
            text-transform: uppercase;
        }
        
        .codigo-emg {
            font-size: 8pt;
            font-weight: bold;
            text-align: center;
            border: 1px solid #000;
            padding: 1mm;
            background-color: white;
        }
        
        /* SEGUNDA FILA - INFORMACIÓN INSTITUCIONAL */
        .fila-info-institucional {
            display: flex;
            border-bottom: 1px solid #000;
            min-height: 15mm;
        }
        
        .datos-institucion {
            flex: 2;
            padding: 2mm;
            border-right: 1px solid #000;
        }
        
        .info-evaluacion {
            flex: 1;
            padding: 2mm;
        }
        
        .campo-institucional {
            display: flex;
            margin-bottom: 1.5mm;
            font-size: 7pt;
        }
        
        .label-campo {
            font-weight: bold;
            min-width: 35mm;
            margin-right: 2mm;
        }
        
        .valor-campo {
            flex: 1;
            border-bottom: 1px solid #000;
            min-height: 3mm;
            padding-left: 1mm;
        }
        
        /* TERCERA FILA - DATOS ESPECÍFICOS */
        .fila-datos-especificos {
            display: flex;
            min-height: 12mm;
        }
        
        .seccion-izq {
            flex: 1;
            padding: 2mm;
            border-right: 1px solid #000;
        }
        
        .seccion-der {
            flex: 1;
            padding: 2mm;
        }
        
        /* ESTILOS PARA CAMPOS CON DIVISIONES */
        .campo-dividido {
            display: flex;
            align-items: center;
            margin-bottom: 1.5mm;
        }
        
        .subcampo {
            display: flex;
            align-items: center;
            margin-right: 5mm;
        }
        
        .subcampo .label-campo {
            min-width: auto;
            margin-right: 1mm;
        }
        
        .subcampo .valor-campo {
            width: 15mm;
            min-height: 3mm;
        }
        
        /* TABLA DE CALIFICACIONES (placeholder) */
        .tabla-calificaciones {
            width: 100%;
            border-collapse: collapse;
            border: 2px solid #000;
            margin-top: 3mm;
        }
        
        .tabla-calificaciones th,
        .tabla-calificaciones td {
            border: 1px solid #000;
            padding: 1mm;
            text-align: center;
            font-size: 7pt;
        }
        
        .tabla-calificaciones th {
            background-color: #f0f0f0;
            font-weight: bold;
        }
    </style>
</head>
<body>
    <!-- ENCABEZADO OFICIAL DEL MINISTERIO -->
    <div class="header-oficial">
        <!-- PRIMERA FILA: LOGO Y TÍTULOS PRINCIPALES -->
        <div class="fila-principal">
            <div class="logo-venezuela">
                <div class="logo-placeholder">
                    LOGO<br/>VENEZUELA
                </div>
            </div>
            
            <div class="titulos-ministerio">
                <div class="gobierno-bolivariano">
                    Gobierno Bolivariano de Venezuela
                </div>
                <div class="ministerio-educacion">
                    Ministerio del Poder Popular para la Educación
                </div>
            </div>
            
            <div class="codigo-formato">
                <div class="titulo-resumen">
                    Resumen Final del<br/>Rendimiento Estudiantil
                </div>
                <div class="codigo-emg">
                    Código del Formato: EMG
                </div>
            </div>
        </div>
        
        <!-- SEGUNDA FILA: INFORMACIÓN INSTITUCIONAL -->
        <div class="fila-info-institucional">
            <div class="datos-institucion">
                <div class="campo-institucional">
                    <span class="label-campo">II. Datos de la Institución Educativa:</span>
                </div>
                
                <div class="campo-institucional">
                    <span class="label-campo">Código de la Institución Educativa:</span>
                    <div class="valor-campo"></div>
                    <span class="label-campo" style="margin-left: 5mm;">Denominación y Epónimo:</span>
                    <div class="valor-campo"></div>
                </div>
                
                <div class="campo-institucional">
                    <span class="label-campo">Dirección:</span>
                    <div class="valor-campo"></div>
                    <span class="label-campo" style="margin-left: 5mm;">Teléfono:</span>
                    <div class="valor-campo" style="width: 20mm;"></div>
                </div>
                
                <div class="campo-institucional">
                    <span class="label-campo">Municipio:</span>
                    <div class="valor-campo"></div>
                    <span class="label-campo" style="margin-left: 5mm;">Entidad Federal:</span>
                    <div class="valor-campo"></div>
                    <span class="label-campo" style="margin-left: 5mm;">CDCEE:</span>
                    <div class="valor-campo" style="width: 15mm;"></div>
                </div>
                
                <div class="campo-institucional">
                    <span class="label-campo">Director(a):</span>
                    <div class="valor-campo"></div>
                    <span class="label-campo" style="margin-left: 5mm;">Cédula de Identidad:</span>
                    <div class="valor-campo" style="width: 25mm;"></div>
                </div>
            </div>
            
            <div class="info-evaluacion">
                <div class="campo-institucional">
                    <span class="label-campo">I. Año Escolar:</span>
                    <div class="valor-campo"></div>
                </div>
                
                <div class="campo-dividido">
                    <div class="subcampo">
                        <span class="label-campo">Tipo de Evaluación:</span>
                        <div class="valor-campo"></div>
                    </div>
                    <div class="subcampo">
                        <span class="label-campo">Mes y Año:</span>
                        <div class="valor-campo"></div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- TERCERA FILA: DATOS ESPECÍFICOS DEL ESTUDIANTE/CURSO -->
        <div class="fila-datos-especificos">
            <div class="seccion-izq">
                <div class="campo-institucional">
                    <span class="label-campo">III. Datos del Estudiante:</span>
                </div>
                <!-- Aquí irían más campos específicos según el tipo de reporte -->
            </div>
            
            <div class="seccion-der">
                <div class="campo-institucional">
                    <span class="label-campo">IV. Datos Académicos:</span>
                </div>
                <!-- Aquí irían los datos del grado, sección, etc. -->
            </div>
        </div>
    </div>
    
    <!-- TABLA DE CALIFICACIONES (PLACEHOLDER) -->
    <table class="tabla-calificaciones">
        <thead>
            <tr>
                <th>N°</th>
                <th>APELLIDOS Y NOMBRES</th>
                <th>MATEMÁTICA</th>
                <th>CASTELLANO</th>
                <th>INGLÉS</th>
                <!-- Más asignaturas según corresponda -->
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>01</td>
                <td style="text-align: left; padding-left: 2mm;">EJEMPLO ESTUDIANTE, NOMBRE</td>
                <td>18</td>
                <td>16</td>
                <td>15</td>
            </tr>
            <!-- Más estudiantes -->
        </tbody>
    </table>
    
    <!-- PIE DE PÁGINA INFORMATIVO -->
    <div style="margin-top: 10mm; text-align: center; font-size: 7pt; color: #666;">
        <p><strong>Vista Previa del Encabezado Oficial - Formato EMG</strong></p>
        <p>Este diseño replica exactamente el formato oficial del Ministerio del Poder Popular para la Educación</p>
    </div>
</body>
</html>
"#.to_string()
}

// Función para generar datos de prueba del encabezado
pub fn generar_datos_encabezado_oficial() -> BTreeMap<String, String> {
    let mut datos = BTreeMap::new();
    
    // Datos institucionales
    datos.insert("codigo_institucion".to_string(), "12345678".to_string());
    datos.insert("denominacion".to_string(), "COMPLEJO EDUCATIVO PROFESOR JESÚS LÓPEZ CASTRO".to_string());
    datos.insert("direccion".to_string(), "CALLE PRINCIPAL, SECTOR CENTRO".to_string());
    datos.insert("telefono".to_string(), "0424-1234567".to_string());
    datos.insert("municipio".to_string(), "MUNICIPIO EJEMPLO".to_string());
    datos.insert("entidad_federal".to_string(), "ESTADO EJEMPLO".to_string());
    datos.insert("cdcee".to_string(), "001".to_string());
    datos.insert("director".to_string(), "PROF. MARÍA GONZÁLEZ".to_string());
    datos.insert("cedula_director".to_string(), "V-12.345.678".to_string());
    
    // Datos académicos
    datos.insert("ano_escolar".to_string(), "2023-2024".to_string());
    datos.insert("tipo_evaluacion".to_string(), "FINAL".to_string());
    datos.insert("mes_ano".to_string(), "JULIO 2024".to_string());
    datos.insert("grado".to_string(), "5TO AÑO".to_string());
    datos.insert("seccion".to_string(), "A".to_string());
    datos.insert("docente_guia".to_string(), "PROF. CARLOS PÉREZ".to_string());
    
    datos
}

// Función principal para testing
#[tauri::command]
pub async fn generar_vista_previa_html_oficial() -> Result<String, String> {
    Ok(generar_plantilla_html_oficial())
}

// Función para generar encabezados de asignaturas
pub fn generar_encabezados_asignaturas(asignaturas: &[String]) -> (String, String) {
    let mut subject_headers = String::new();
    let mut lapso_headers = String::new();
    
    for asignatura in asignaturas {
        let abreviacion = abreviar_asignatura(asignatura);
        subject_headers.push_str(&format!(
            r#"<th colspan="4" class="subject-header">{}</th>"#,
            abreviacion
        ));
        
        lapso_headers.push_str(r#"
                <th class="sub-header">1°</th>
                <th class="sub-header">2°</th>
                <th class="sub-header">3°</th>
                <th class="sub-header">DF</th>"#);
    }
    
    (subject_headers, lapso_headers)
}

// Función para abreviar nombres de asignaturas (igual que en el archivo original)
fn abreviar_asignatura(nombre: &str) -> String {
    let nombre_upper = nombre.trim().to_uppercase();
    match nombre_upper.as_str() {
        "MATEMATICA" | "MATEMÁTICA" => "MAT".to_string(),
        "CASTELLANO" | "LENGUA Y LITERATURA" => "CAST".to_string(),
        "INGLES" | "INGLÉS" => "ING".to_string(),
        "HISTORIA" | "HISTORIA DE VENEZUELA" => "HIST".to_string(),
        "GEOGRAFIA" | "GEOGRAFÍA" => "GEO".to_string(),
        "BIOLOGIA" | "BIOLOGÍA" => "BIO".to_string(),
        "FISICA" | "FÍSICA" => "FIS".to_string(),
        "QUIMICA" | "QUÍMICA" => "QUIM".to_string(),
        "EDUCACION FISICA" | "EDUCACIÓN FÍSICA" => "ED.FIS".to_string(),
        "ARTE Y PATRIMONIO" => "ART Y PAT".to_string(),
        "ORIENTACION Y CONVIVENCIA" | "ORIENTACIÓN Y CONVIVENCIA" => "ORIENT".to_string(),
        "CIENCIAS DE LA TIERRA" => "CS. TIERRA".to_string(),
        _ => {
            if nombre.len() > 8 {
                nombre.chars().take(8).collect::<String>().to_uppercase()
            } else {
                nombre_upper
            }
        }
    }
}

// Función para generar filas de estudiantes
pub fn generar_filas_estudiantes(
    estudiantes: &[(String, String)], // (apellidos, nombres)
    calificaciones: &[Vec<(Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>)>], // lapso1, lapso2, lapso3, ajuste1, ajuste2, ajuste3 por asignatura
    num_asignaturas: usize,
) -> String {
    let mut filas = String::new();
    
    for (index, (apellidos, nombres)) in estudiantes.iter().enumerate() {
        let numero = index + 1;
        let nombre_completo = format!("{}, {}", apellidos, nombres);
        
        // Truncar nombre si es muy largo (máximo 34 caracteres como en la imagen)
        let nombre_display = if nombre_completo.len() > 34 {
            format!("{}...", &nombre_completo[..31])
        } else {
            nombre_completo
        };
        
        filas.push_str(&format!(
            r#"            <tr class="student-row">
                <td class="student-number">{}</td>
                <td class="student-name truncate">{}</td>"#,
            numero, nombre_display
        ));
        
        // Generar calificaciones para cada asignatura
        if index < calificaciones.len() {
            let estudiante_califs = &calificaciones[index];
            
            for asig_index in 0..num_asignaturas {
                if asig_index < estudiante_califs.len() {
                    let (lapso1, lapso2, lapso3, ajuste1, ajuste2, ajuste3) = estudiante_califs[asig_index];
                    
                    // Formatear cada lapso
                    let lapso1_html = formatear_celda_calificacion(lapso1, ajuste1);
                    let lapso2_html = formatear_celda_calificacion(lapso2, ajuste2);
                    let lapso3_html = formatear_celda_calificacion(lapso3, ajuste3);
                    
                    // Calcular definitiva
                    let definitiva = calcular_definitiva_html(lapso1, lapso2, lapso3, ajuste1, ajuste2, ajuste3);
                    let definitiva_html = formatear_definitiva(definitiva);
                    
                    filas.push_str(&format!(
                        r#"
                <td class="grade-cell">{}</td>
                <td class="grade-cell">{}</td>
                <td class="grade-cell">{}</td>
                <td class="grade-cell">{}</td>"#,
                        lapso1_html, lapso2_html, lapso3_html, definitiva_html
                    ));
                } else {
                    // Celdas vacías si no hay calificaciones
                    filas.push_str(r#"
                <td class="grade-cell"></td>
                <td class="grade-cell"></td>
                <td class="grade-cell"></td>
                <td class="grade-cell"></td>"#);
                }
            }
        }
        
        filas.push_str("\n            </tr>");
    }
    
    filas
}

// Función para formatear una celda de calificación (original y ajustada)
fn formatear_celda_calificacion(original: Option<i32>, ajustada: Option<i32>) -> String {
    match (original, ajustada) {
        (Some(orig), Some(ajust)) => {
            // Mostrar ambas: original arriba pequeña, ajustada abajo grande
            format!(
                r#"<div class="grade-stack">
                    <div class="grade-original">{}</div>
                    <div class="grade-adjustment">{}</div>
                </div>"#,
                formatear_numero(orig),
                formatear_numero(ajust)
            )
        }
        (Some(orig), None) => {
            // Solo original, centrada
            formatear_numero(orig)
        }
        _ => "".to_string()
    }
}

// Función para formatear definitiva con colores
fn formatear_definitiva(definitiva: Option<i32>) -> String {
    match definitiva {
        Some(def) => {
            let clase = if def >= 14 {
                "grade-excellent"
            } else if def >= 10 {
                "grade-pass"
            } else {
                "grade-fail"
            };
            format!(r#"<span class="{}">{}</span>"#, clase, formatear_numero(def))
        }
        None => "".to_string()
    }
}

// Función para formatear números con cero inicial
fn formatear_numero(num: i32) -> String {
    if num < 10 {
        format!("0{}", num)
    } else {
        num.to_string()
    }
}

// Función para calcular definitiva (usar ajustadas si existen)
fn calcular_definitiva_html(
    lapso1: Option<i32>,
    lapso2: Option<i32>, 
    lapso3: Option<i32>,
    ajuste1: Option<i32>,
    ajuste2: Option<i32>,
    ajuste3: Option<i32>
) -> Option<i32> {
    let l1 = ajuste1.or(lapso1);
    let l2 = ajuste2.or(lapso2);
    let l3 = ajuste3.or(lapso3);
    
    match (l1, l2, l3) {
        (Some(a), Some(b), Some(c)) => {
            let promedio = (a + b + c) as f32 / 3.0;
            Some(promedio.round() as i32)
        }
        _ => None
    }
}

// Función principal para generar el HTML completo
pub fn generar_html_completo(
    nombre_institucion: &str,
    grado: &str,
    seccion: &str,
    docente_guia: &str,
    periodo_escolar: &str,
    gcrp: Option<&str>,
    logo_base64: &str,
    asignaturas: &[String],
    estudiantes: &[(String, String)],
    calificaciones: &[Vec<(Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>)>],
) -> String {
    let mut html = generar_plantilla_html_oficial();
    
    // Generar encabezados
    let (subject_headers, lapso_headers) = generar_encabezados_asignaturas(asignaturas);
    
    // Generar filas de estudiantes
    let student_rows = generar_filas_estudiantes(estudiantes, calificaciones, asignaturas.len());
    
    // Formatear grado
    let grado_formateado = match grado {
        "1" => "1er Año",
        "2" => "2do Año", 
        "3" => "3er Año",
        "4" => "4to Año",
        "5" => "5to Año",
        _ => &format!("{}° Año", grado)
    };
    
    // Reemplazar placeholders
    html = html.replace("{{NOMBRE_INSTITUCION}}", nombre_institucion);
    html = html.replace("{{GRADO}}", grado_formateado);
    html = html.replace("{{SECCION}}", seccion);
    html = html.replace("{{DOCENTE_GUIA}}", docente_guia);
    html = html.replace("{{PERIODO_ESCOLAR}}", periodo_escolar);
    html = html.replace("{{GCRP}}", gcrp.unwrap_or("N/A"));
    html = html.replace("{{LOGO_BASE64}}", logo_base64);
    html = html.replace("{{SUBJECT_HEADERS}}", &subject_headers);
    html = html.replace("{{LAPSO_HEADERS}}", &lapso_headers);
    html = html.replace("{{STUDENT_ROWS}}", &student_rows);
    
    html
}

// Función para generar PDF usando printpdf con HTML
pub fn generar_pdf_desde_html(html_content: &str) -> Result<Vec<u8>, String> {
    let options = XmlRenderOptions {
        images: BTreeMap::new(),
        fonts: BTreeMap::new(),
        page_width: Mm(355.6), // Oficio horizontal
        page_height: Mm(215.9),
    };
    
    let pdf_bytes = PdfDocument::new("Acta de Resumen de Calificaciones")
        .with_html(html_content, &options)
        .map_err(|e| format!("Error generando PDF: {}", e))?
        .save(&PdfSaveOptions::default());
    
    Ok(pdf_bytes)
}

// Función principal para generar el PDF usando HTML
pub async fn generar_acta_resumen_html(
    id_grado_secciones: i32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Aquí iría la lógica para obtener los datos de la base de datos
    // Por ahora, creo datos de ejemplo para mostrar la estructura
    
    let info_encabezado = InfoEncabezado {
        grado: "5".to_string(),
        seccion: "A".to_string(),
        docente_guia: "PROF. MARÍA GONZÁLEZ".to_string(),
        periodo_escolar: "2023-2024".to_string(),
        gcrp: Some("ROBÓTICA Y PROGRAMACIÓN".to_string()),
    };
    
    let asignaturas = vec![
        AsignaturaInfo { id: 1, nombre: "MATEMÁTICA".to_string() },
        AsignaturaInfo { id: 2, nombre: "CASTELLANO".to_string() },
        AsignaturaInfo { id: 3, nombre: "INGLÉS".to_string() },
        AsignaturaInfo { id: 4, nombre: "HISTORIA".to_string() },
        AsignaturaInfo { id: 5, nombre: "GEOGRAFÍA".to_string() },
        AsignaturaInfo { id: 6, nombre: "BIOLOGÍA".to_string() },
        AsignaturaInfo { id: 7, nombre: "FÍSICA".to_string() },
        AsignaturaInfo { id: 8, nombre: "QUÍMICA".to_string() },
        AsignaturaInfo { id: 9, nombre: "EDUCACIÓN FÍSICA".to_string() },
        AsignaturaInfo { id: 10, nombre: "ARTE Y PATRIMONIO".to_string() },
    ];
    
    let estudiantes = vec![
        EstudianteCalificaciones {
            apellidos: "GARCÍA RODRÍGUEZ".to_string(),
            nombres: "MARÍA JOSÉ".to_string(),
            asignaturas: vec![
                AsignaturaCalificaciones {
                    nombre_asignatura: "MATEMÁTICA".to_string(),
                    calificaciones: CalificacionDetalle {
                        lapso1: Some(18),
                        lapso2: Some(16),
                        lapso3: Some(19),
                        lapso_ajustado_1: None,
                        lapso_ajustado_2: Some(17),
                        lapso_ajustado_3: None,
                        definitiva: None,
                    },
                },
                // ... más asignaturas
            ],
        },
        // ... más estudiantes
    ];
    
    // Logo de ejemplo (deberías usar el logo real de la institución)
    let logo_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";
    
    // Generar HTML
    let html_content = generar_html_completo(
        "COMPLEJO EDUCATIVO PROFESOR JESÚS LÓPEZ CASTRO",
        &info_encabezado.grado,
        &info_encabezado.seccion,
        &info_encabezado.docente_guia,
        &info_encabezado.periodo_escolar,
        info_encabezado.gcrp.as_deref(),
        logo_base64,
        &asignaturas.iter().map(|a| a.nombre.clone()).collect::<Vec<String>>(),
        &estudiantes.iter().map(|e| (e.apellidos.clone(), e.nombres.clone())).collect::<Vec<(String, String)>>(),
        &estudiantes.iter().map(|e| {
            e.asignaturas.iter().map(|a| {
                (a.calificaciones.lapso1, a.calificaciones.lapso2, a.calificaciones.lapso3, a.calificaciones.lapso_ajustado_1, a.calificaciones.lapso_ajustado_2, a.calificaciones.lapso_ajustado_3)
            }).collect::<Vec<(Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>)>>()
        }).collect::<Vec<Vec<(Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>)>>(),
    );
    
    // Generar PDF usando printpdf
    let pdf_bytes = generar_pdf_desde_html(&html_content)?;
    
    // Guardar archivo
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("acta_resumen_html_{}.pdf", timestamp);
    let file_path = format!("C:\\temp\\{}", filename);
    
    std::fs::write(&file_path, pdf_bytes)
        .map_err(|e| format!("Error guardando archivo: {}", e))?;
    
    Ok(file_path)
} 