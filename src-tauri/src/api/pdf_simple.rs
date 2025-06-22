use headless_chrome::{Browser, LaunchOptions};
use std::fs;
use tempfile::NamedTempFile;
use crate::AppState;
use tauri::State;
use base64::{engine::general_purpose, Engine as _};
use chrono;

#[derive(Debug, Clone)]
pub struct ActaData {
    pub periodo_escolar: String,
    pub nombre_institucion: String,
    pub codigo_institucion: String,
    pub direccion: String,
    pub telefono: String,
    pub municipio: String,
    pub estado: String,
    pub director: String,
    pub cedula_director: String,
    pub cdcee: String,
    pub grado: String,
    pub seccion: String,
    pub docente_guia: String,
    pub estudiantes: Vec<EstudianteActa>,
}

#[derive(Debug, Clone)]
pub struct EstudianteActa {
    pub numero: u32,
    pub cedula: String,
    pub apellidos: String,
    pub nombres: String,
    pub lugar_nacimiento: String,
    pub estado_nacimiento: String,
    pub sexo: String,
    pub fecha_nacimiento: (u8, u8, u16), // día, mes, año
    pub calificaciones: Vec<CalificacionAsignatura>,
    pub pgcrp: String,
    pub grupo: String,
}

#[derive(Debug, Clone)]
pub struct CalificacionAsignatura {
    pub asignatura_codigo: String,
    pub calificacion: Option<u8>,
}

// Plantilla HTML que replica exactamente el formato oficial del Ministerio
const PLANTILLA_ACTA_OFICIAL: &str = r#"
<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <title>Resumen Final del Rendimiento Estudiantil</title>
    <style>
        @page {
            size: A4 landscape;
            margin: 5mm;
        }
        
        body {
            font-family: Arial, sans-serif;
            font-size: 8pt;
            margin: 0;
            padding: 0;
            line-height: 1.0;
        }

        .document-container {
            width: 100%;
            border: 2px solid #000;
        }

        /* ENCABEZADO PRINCIPAL */
        .header-principal {
            display: flex;
            border-bottom: 1px solid #000;
            min-height: 25mm;
        }

        .logo-section {
            width: 25mm;
            border-right: 1px solid #000;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 2mm;
        }

        .logo-oficial {
            max-width: 20mm;
            max-height: 20mm;
        }

        .titulo-principal {
            flex: 1;
            display: flex;
            flex-direction: column;
            justify-content: center;
            text-align: center;
            padding: 2mm;
            border-right: 1px solid #000;
        }

        .gobierno-bolivariano {
            font-size: 9pt;
            font-weight: bold;
            margin-bottom: 1mm;
        }

        .ministerio-educacion {
            font-size: 8pt;
            font-weight: bold;
        }

        .codigo-section {
            width: 35mm;
            display: flex;
            flex-direction: column;
            justify-content: center;
            text-align: center;
            padding: 2mm;
        }

        .resumen-titulo {
            font-size: 9pt;
            font-weight: bold;
            margin-bottom: 2mm;
            line-height: 1.1;
        }

        .codigo-emg {
            font-size: 14pt;
            font-weight: bold;
            border: 2px solid #000;
            padding: 3mm;
            background-color: #f0f0f0;
        }

        /* INFORMACIÓN DEL AÑO ESCOLAR */
        .info-ano-escolar {
            display: flex;
            border-bottom: 1px solid #000;
            padding: 2mm;
            font-size: 8pt;
            font-weight: bold;
        }

        .ano-escolar {
            flex: 1;
        }

        .tipo-evaluacion {
            flex: 1;
            text-align: center;
        }

        .mes-ano {
            flex: 1;
            text-align: right;
        }

        /* DATOS INSTITUCIÓN */
        .seccion-institucion {
            border-bottom: 1px solid #000;
        }

        .titulo-seccion {
            background-color: #f0f0f0;
            font-weight: bold;
            padding: 2mm;
            border-bottom: 1px solid #000;
            font-size: 8pt;
        }

        .fila-datos {
            display: flex;
            border-bottom: 1px solid #000;
            min-height: 6mm;
        }

        .campo-datos {
            display: flex;
            align-items: center;
            padding: 1mm 2mm;
            font-size: 7pt;
        }

        .label-campo {
            font-weight: bold;
            margin-right: 3mm;
            white-space: nowrap;
        }

        .valor-campo {
            border-bottom: 1px solid #000;
            flex: 1;
            min-height: 3mm;
            padding: 0 2mm;
        }

        .separador-vertical {
            border-right: 1px solid #000;
        }

        /* TABLA DE ESTUDIANTES */
        .tabla-estudiantes {
            width: 100%;
            border-collapse: collapse;
            font-size: 6pt;
        }

        .tabla-estudiantes th,
        .tabla-estudiantes td {
            border: 1px solid #000;
            padding: 1mm;
            text-align: center;
            vertical-align: middle;
        }

        .tabla-estudiantes th {
            background-color: #f0f0f0;
            font-weight: bold;
            font-size: 6pt;
        }

        .header-identificacion {
            background-color: #f0f0f0;
            font-weight: bold;
            text-align: left;
            padding: 2mm;
        }

        .numero-estudiante {
            width: 8mm;
        }

        .cedula-estudiante {
            width: 20mm;
        }

        .apellidos-estudiante {
            width: 35mm;
            text-align: left !important;
            font-size: 6pt;
        }

        .nombres-estudiante {
            width: 25mm;
            text-align: left !important;
            font-size: 6pt;
        }

        .lugar-nacimiento {
            width: 20mm;
            font-size: 6pt;
        }

        .fecha-nacimiento {
            width: 8mm;
        }

        .asignatura-header {
            width: 8mm;
            font-size: 5pt;
            writing-mode: vertical-rl;
            text-orientation: mixed;
        }

        .calificacion-cell {
            font-weight: bold;
            font-size: 7pt;
        }

        .grade-excellent { color: #1B5E20; }
        .grade-good { color: #1565C0; }
        .grade-pass { color: #F57C00; }
        .grade-fail { color: #C62828; }

        .pgcrp-column {
            width: 15mm;
            font-size: 5pt;
        }

        .grupo-column {
            width: 8mm;
        }
    </style>
</head>
<body>
    <div class="document-container">
        <!-- ENCABEZADO PRINCIPAL -->
        <div class="header-principal">
            <div class="logo-section">
                <img src="{{LOGO_BASE64}}" class="logo-oficial" alt="Logo Ministerio">
            </div>
            <div class="titulo-principal">
                <div class="gobierno-bolivariano">GOBIERNO BOLIVARIANO DE VENEZUELA</div>
                <div class="ministerio-educacion">MINISTERIO DEL PODER POPULAR PARA LA EDUCACIÓN</div>
            </div>
            <div class="codigo-section">
                <div class="resumen-titulo">RESUMEN FINAL DEL<br>RENDIMIENTO ESTUDIANTIL</div>
                <div class="codigo-emg">EMG</div>
            </div>
        </div>

        <!-- AÑO ESCOLAR -->
        <div class="info-ano-escolar">
            <div class="ano-escolar">I. Año Escolar: {{PERIODO_ESCOLAR}}</div>
            <div class="tipo-evaluacion">Tipo de Evaluación: Final</div>
            <div class="mes-ano">Mes y Año: {{FECHA_ACTUAL}}</div>
        </div>

        <!-- DATOS INSTITUCIÓN -->
        <div class="seccion-institucion">
            <div class="titulo-seccion">II. Datos de la Institución Educativa:</div>
            
            <div class="fila-datos">
                <div class="campo-datos separador-vertical" style="flex: 2;">
                    <span class="label-campo">Código de la Institución Educativa:</span>
                    <div class="valor-campo">{{CODIGO_INSTITUCION}}</div>
                </div>
                <div class="campo-datos" style="flex: 3;">
                    <span class="label-campo">Denominación y Epónimo:</span>
                    <div class="valor-campo">{{NOMBRE_INSTITUCION}}</div>
                </div>
            </div>

            <div class="fila-datos">
                <div class="campo-datos separador-vertical" style="flex: 2;">
                    <span class="label-campo">Dirección:</span>
                    <div class="valor-campo">{{DIRECCION}}</div>
                </div>
                <div class="campo-datos separador-vertical" style="flex: 1;">
                    <span class="label-campo">Teléfono:</span>
                    <div class="valor-campo">{{TELEFONO}}</div>
                </div>
                <div class="campo-datos separador-vertical" style="flex: 1;">
                    <span class="label-campo">Municipio:</span>
                    <div class="valor-campo">{{MUNICIPIO}}</div>
                </div>
                <div class="campo-datos" style="flex: 1;">
                    <span class="label-campo">Entidad Federal:</span>
                    <div class="valor-campo">{{ESTADO}}</div>
                </div>
            </div>

            <div class="fila-datos">
                <div class="campo-datos separador-vertical" style="flex: 2;">
                    <span class="label-campo">Director(a):</span>
                    <div class="valor-campo">{{DIRECTOR}}</div>
                </div>
                <div class="campo-datos separador-vertical" style="flex: 1;">
                    <span class="label-campo">Cédula de Identidad:</span>
                    <div class="valor-campo">{{CEDULA_DIRECTOR}}</div>
                </div>
                <div class="campo-datos" style="flex: 1;">
                    <span class="label-campo">CDCEE:</span>
                    <div class="valor-campo">{{CDCEE}}</div>
                </div>
            </div>
        </div>

        <!-- TABLA DE ESTUDIANTES -->
        <div style="border-bottom: 1px solid #000;">
            <div class="header-identificacion">III. Identificación del Estudiante:</div>
            <div style="float: right; padding: 2mm; font-weight: bold; font-size: 8pt;">
                IV. Resumen Final del Rendimiento:
            </div>
            <div style="clear: both;"></div>
        </div>

        <table class="tabla-estudiantes">
            <thead>
                <tr>
                    <th rowspan="2" class="numero-estudiante">N°</th>
                    <th rowspan="2" class="cedula-estudiante">Cédula de Identidad</th>
                    <th rowspan="2" class="apellidos-estudiante">Apellidos</th>
                    <th rowspan="2" class="nombres-estudiante">Nombres</th>
                    <th rowspan="2" class="lugar-nacimiento">Lugar de Nacimiento</th>
                    <th rowspan="2" class="fecha-nacimiento">EF</th>
                    <th rowspan="2" class="fecha-nacimiento">SEXO</th>
                    <th colspan="3" style="font-size: 6pt;">FECHA DE NACIMIENTO</th>
                    <th colspan="12" style="font-size: 6pt;">ÁREAS DE FORMACIÓN</th>
                    <th rowspan="2" class="pgcrp-column">PARTICIPACIÓN EN GRUPOS DE CREACIÓN, RECREACIÓN Y PRODUCCIÓN</th>
                    <th rowspan="2" class="grupo-column">GRUPO</th>
                </tr>
                <tr>
                    <th class="fecha-nacimiento">DÍA</th>
                    <th class="fecha-nacimiento">MES</th>
                    <th class="fecha-nacimiento">AÑO</th>
                    <th class="asignatura-header">1<br>CA</th>
                    <th class="asignatura-header">2<br>ILE</th>
                    <th class="asignatura-header">3<br>MA</th>
                    <th class="asignatura-header">4<br>EF</th>
                    <th class="asignatura-header">5<br>FI</th>
                    <th class="asignatura-header">6<br>QU</th>
                    <th class="asignatura-header">7<br>BI</th>
                    <th class="asignatura-header">8<br>CT</th>
                    <th class="asignatura-header">9<br>GHC</th>
                    <th class="asignatura-header">10<br>FSN</th>
                    <th class="asignatura-header">11<br>OC</th>
                    <th class="asignatura-header">12<br>PGCRP</th>
                </tr>
            </thead>
            <tbody>
                {{FILAS_ESTUDIANTES}}
            </tbody>
        </table>
    </div>
</body>
</html>
"#;

// Función para formatear calificación con color
fn formatear_calificacion_html(calificacion: Option<u8>) -> String {
    match calificacion {
        Some(calif) => {
            let (clase, valor) = match calif {
                17..=20 => ("grade-excellent", format!("{:02}", calif)),
                14..=16 => ("grade-good", format!("{:02}", calif)),
                10..=13 => ("grade-pass", format!("{:02}", calif)),
                _ => ("grade-fail", format!("{:02}", calif)),
            };
            format!(r#"<span class="calificacion-cell {}">{}</span>"#, clase, valor)
        }
        None => "--".to_string()
    }
}

// Función para generar las filas de estudiantes
fn generar_filas_estudiantes_oficial(estudiantes: &[EstudianteActa]) -> String {
    let mut filas = Vec::new();
    
    // Generar filas con datos
    for estudiante in estudiantes {
        let calificaciones_html = estudiante.calificaciones.iter()
            .map(|calif| format!("<td class=\"calificacion-cell\">{}</td>", formatear_calificacion_html(calif.calificacion)))
            .collect::<Vec<String>>()
            .join("");

        let fila = format!(
            r#"<tr>
                <td class="numero-estudiante">{:02}</td>
                <td class="cedula-estudiante">{}</td>
                <td class="apellidos-estudiante">{}</td>
                <td class="nombres-estudiante">{}</td>
                <td class="lugar-nacimiento">{}</td>
                <td class="fecha-nacimiento">{}</td>
                <td class="fecha-nacimiento">{}</td>
                <td class="fecha-nacimiento">{:02}</td>
                <td class="fecha-nacimiento">{:02}</td>
                <td class="fecha-nacimiento">{}</td>
                {}
                <td class="pgcrp-column">{}</td>
                <td class="grupo-column">{}</td>
            </tr>"#,
            estudiante.numero,
            estudiante.cedula,
            estudiante.apellidos,
            estudiante.nombres,
            estudiante.lugar_nacimiento,
            estudiante.estado_nacimiento,
            estudiante.sexo,
            estudiante.fecha_nacimiento.0,
            estudiante.fecha_nacimiento.1,
            estudiante.fecha_nacimiento.2,
            calificaciones_html,
            estudiante.pgcrp,
            estudiante.grupo
        );
        filas.push(fila);
    }
    
    // Rellenar hasta 15 filas vacías si hay menos estudiantes
    let total_filas = 15;
    while filas.len() < total_filas {
        let numero_fila = filas.len() + 1;
        let fila_vacia = format!(
            r#"<tr>
                <td class="numero-estudiante">{:02}</td>
                <td class="cedula-estudiante"></td>
                <td class="apellidos-estudiante"></td>
                <td class="nombres-estudiante"></td>
                <td class="lugar-nacimiento"></td>
                <td class="fecha-nacimiento"></td>
                <td class="fecha-nacimiento"></td>
                <td class="fecha-nacimiento"></td>
                <td class="fecha-nacimiento"></td>
                <td class="fecha-nacimiento"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="calificacion-cell"></td>
                <td class="pgcrp-column"></td>
                <td class="grupo-column"></td>
            </tr>"#,
            numero_fila
        );
        filas.push(fila_vacia);
    }
    
    filas.join("\n")
}

// Función para cargar y convertir logo a base64
fn cargar_logo_base64() -> Result<String, String> {
    let logo_path = "imagenes/ministerio_resumen.png";
    let logo_bytes = fs::read(logo_path)
        .map_err(|e| format!("Error leyendo logo {}: {}", logo_path, e))?;
    
    let logo_base64 = general_purpose::STANDARD.encode(&logo_bytes);
    Ok(format!("data:image/png;base64,{}", logo_base64))
}

// Función principal para generar el PDF oficial
#[tauri::command]
pub async fn generar_acta_pdf_simple(
    id_grado_secciones: i32,
    _state: State<'_, AppState>,
) -> Result<String, String> {
    // Datos de example - aquí conectarías con tu base de datos real
    let acta_data = ActaData {
        periodo_escolar: "2023-2024".to_string(),
        nombre_institucion: "COMPLEJO EDUCATIVO PROFESOR JESÚS LÓPEZ CASTRO".to_string(),
        codigo_institucion: "12345678".to_string(),
        direccion: "CALLE PRINCIPAL, SECTOR CENTRO".to_string(),
        telefono: "0414-1234567".to_string(),
        municipio: "LIBERTADOR".to_string(),
        estado: "ARAGUA".to_string(),
        director: "PROF. JUAN PÉREZ".to_string(),
        cedula_director: "12.345.678".to_string(),
        cdcee: "123456".to_string(),
        grado: "5to".to_string(),
        seccion: "A".to_string(),
        docente_guia: "PROF. MARÍA GONZÁLEZ".to_string(),
        estudiantes: vec![
            EstudianteActa {
                numero: 1,
                cedula: "V-30.123.456".to_string(),
                apellidos: "GARCÍA RODRÍGUEZ".to_string(),
                nombres: "MARÍA JOSÉ".to_string(),
                lugar_nacimiento: "MARACAY".to_string(),
                estado_nacimiento: "AR".to_string(),
                sexo: "F".to_string(),
                fecha_nacimiento: (15, 3, 2008),
                calificaciones: vec![
                    CalificacionAsignatura { asignatura_codigo: "CA".to_string(), calificacion: Some(18) },
                    CalificacionAsignatura { asignatura_codigo: "ILE".to_string(), calificacion: Some(16) },
                    CalificacionAsignatura { asignatura_codigo: "MA".to_string(), calificacion: Some(19) },
                    CalificacionAsignatura { asignatura_codigo: "EF".to_string(), calificacion: Some(17) },
                    CalificacionAsignatura { asignatura_codigo: "FI".to_string(), calificacion: Some(15) },
                    CalificacionAsignatura { asignatura_codigo: "QU".to_string(), calificacion: Some(16) },
                    CalificacionAsignatura { asignatura_codigo: "BI".to_string(), calificacion: Some(18) },
                    CalificacionAsignatura { asignatura_codigo: "CT".to_string(), calificacion: Some(14) },
                    CalificacionAsignatura { asignatura_codigo: "GHC".to_string(), calificacion: Some(17) },
                    CalificacionAsignatura { asignatura_codigo: "FSN".to_string(), calificacion: Some(16) },
                    CalificacionAsignatura { asignatura_codigo: "OC".to_string(), calificacion: Some(15) },
                    CalificacionAsignatura { asignatura_codigo: "PGCRP".to_string(), calificacion: Some(19) },
                ],
                pgcrp: "ROBÓTICA Y PROGRAMACIÓN".to_string(),
                grupo: "A".to_string(),
            },
            EstudianteActa {
                numero: 2,
                cedula: "V-30.234.567".to_string(),
                apellidos: "MARTÍNEZ LÓPEZ".to_string(),
                nombres: "CARLOS ANDRÉS".to_string(),
                lugar_nacimiento: "VALENCIA".to_string(),
                estado_nacimiento: "CA".to_string(),
                sexo: "M".to_string(),
                fecha_nacimiento: (22, 7, 2008),
                calificaciones: vec![
                    CalificacionAsignatura { asignatura_codigo: "CA".to_string(), calificacion: Some(15) },
                    CalificacionAsignatura { asignatura_codigo: "ILE".to_string(), calificacion: Some(14) },
                    CalificacionAsignatura { asignatura_codigo: "MA".to_string(), calificacion: Some(16) },
                    CalificacionAsignatura { asignatura_codigo: "EF".to_string(), calificacion: Some(18) },
                    CalificacionAsignatura { asignatura_codigo: "FI".to_string(), calificacion: Some(13) },
                    CalificacionAsignatura { asignatura_codigo: "QU".to_string(), calificacion: Some(14) },
                    CalificacionAsignatura { asignatura_codigo: "BI".to_string(), calificacion: Some(15) },
                    CalificacionAsignatura { asignatura_codigo: "CT".to_string(), calificacion: Some(16) },
                    CalificacionAsignatura { asignatura_codigo: "GHC".to_string(), calificacion: Some(14) },
                    CalificacionAsignatura { asignatura_codigo: "FSN".to_string(), calificacion: Some(15) },
                    CalificacionAsignatura { asignatura_codigo: "OC".to_string(), calificacion: Some(13) },
                    CalificacionAsignatura { asignatura_codigo: "PGCRP".to_string(), calificacion: Some(17) },
                ],
                pgcrp: "PROGRAMACIÓN".to_string(),
                grupo: "B".to_string(),
            },
        ],
    };

    // Cargar el logo oficial
    let logo_base64 = cargar_logo_base64()?;

    // Generar HTML desde la plantilla oficial
    let mut html = PLANTILLA_ACTA_OFICIAL.to_string();
    
    // Reemplazar placeholders
    html = html.replace("{{PERIODO_ESCOLAR}}", &acta_data.periodo_escolar);
    html = html.replace("{{NOMBRE_INSTITUCION}}", &acta_data.nombre_institucion);
    html = html.replace("{{CODIGO_INSTITUCION}}", &acta_data.codigo_institucion);
    html = html.replace("{{DIRECCION}}", &acta_data.direccion);
    html = html.replace("{{TELEFONO}}", &acta_data.telefono);
    html = html.replace("{{MUNICIPIO}}", &acta_data.municipio);
    html = html.replace("{{ESTADO}}", &acta_data.estado);
    html = html.replace("{{DIRECTOR}}", &acta_data.director);
    html = html.replace("{{CEDULA_DIRECTOR}}", &acta_data.cedula_director);
    html = html.replace("{{CDCEE}}", &acta_data.cdcee);
    html = html.replace("{{GRADO}}", &acta_data.grado);
    html = html.replace("{{SECCION}}", &acta_data.seccion);
    html = html.replace("{{DOCENTE_GUIA}}", &acta_data.docente_guia);
    html = html.replace("{{FECHA_ACTUAL}}", &chrono::Utc::now().format("%B %Y").to_string());
    html = html.replace("{{LOGO_BASE64}}", &logo_base64);
    html = html.replace("{{FILAS_ESTUDIANTES}}", &generar_filas_estudiantes_oficial(&acta_data.estudiantes));

    // Crear archivo temporal para el HTML
    let mut temp_html = NamedTempFile::new()
        .map_err(|e| format!("Error creando archivo temporal HTML: {}", e))?;
    
    fs::write(temp_html.path(), html)
        .map_err(|e| format!("Error escribiendo HTML temporal: {}", e))?;

    // Generar PDF usando headless Chrome
    let browser = Browser::new(LaunchOptions::default_builder().build().unwrap())
        .map_err(|e| format!("Error iniciando navegador: {}", e))?;
    
    let tab = browser.new_tab()
        .map_err(|e| format!("Error creando pestaña: {}", e))?;
    
    tab.navigate_to(&format!("file://{}", temp_html.path().display()))
        .map_err(|e| format!("Error navegando a HTML: {}", e))?;
    
    tab.wait_until_navigated()
        .map_err(|e| format!("Error esperando navegación: {}", e))?;

    let pdf_data = tab.print_to_pdf(Some(headless_chrome::types::PrintToPdfOptions {
        landscape: Some(true),
        display_header_footer: Some(false),
        print_background: Some(true),
        scale: Some(0.68), // Ajustado para que quepa mejor
        paper_width: Some(11.7),
        paper_height: Some(8.3),
        margin_top: Some(0.2),
        margin_bottom: Some(0.2),
        margin_left: Some(0.2),
        margin_right: Some(0.2),
        page_ranges: None,
        ignore_invalid_page_ranges: Some(false),
        header_template: None,
        footer_template: None,
        prefer_css_page_size: Some(true),
        generate_document_outline: Some(false),
        generate_tagged_pdf: Some(false),
        transfer_mode: None,
    }))
    .map_err(|e| format!("Error generando PDF: {}", e))?;

    // Guardar PDF
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("acta_resumen_oficial_{}.pdf", timestamp);
    let file_path = std::env::temp_dir().join(&filename);
    
    fs::write(&file_path, pdf_data)
        .map_err(|e| format!("Error guardando PDF: {}", e))?;

    // Retornar el PDF como base64
    Ok(general_purpose::STANDARD.encode(fs::read(&file_path).map_err(|e| format!("Error leyendo PDF: {}", e))?))
} 