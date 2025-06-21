use printpdf::{BuiltinFont, IndirectFontRef, Line, Mm, PdfDocument, PdfLayerReference, Point, Rgb, Color, Image, ImageTransform};
use std::io::{BufWriter, Cursor};
use ::image::{load, GenericImageView, DynamicImage, imageops::FilterType};
use base64::{engine::general_purpose, Engine as _};
use crate::AppState;
use tauri::State;

// Constantes para el diseño (Oficio horizontal optimizado)
const MARGIN_LEFT: f32 = 8.0; // Margen más estrecho
const MARGIN_RIGHT: f32 = 8.0;
const MARGIN_TOP: f32 = 8.0;
const MARGIN_BOTTOM: f32 = 8.0;
const PAGE_WIDTH: f32 = 355.6; // Oficio horizontal (14 pulgadas)
const PAGE_HEIGHT: f32 = 215.9; // Oficio horizontal (8.5 pulgadas)
const CONTENT_WIDTH: f32 = PAGE_WIDTH - MARGIN_LEFT - MARGIN_RIGHT;

// Funciones para colores modernos
fn primary_color() -> Rgb {
    Rgb::new(41.0/255.0, 128.0/255.0, 185.0/255.0, None)
}

fn secondary_color() -> Rgb {
    Rgb::new(52.0/255.0, 73.0/255.0, 94.0/255.0, None)
}

fn accent_color() -> Rgb {
    Rgb::new(230.0/255.0, 126.0/255.0, 34.0/255.0, None)
}

fn light_gray() -> Rgb {
    Rgb::new(236.0/255.0, 240.0/255.0, 241.0/255.0, None)
}

fn white_color() -> Rgb {
    Rgb::new(1.0, 1.0, 1.0, None)
}

fn green_color() -> Rgb {
    Rgb::new(39.0/255.0, 174.0/255.0, 96.0/255.0, None)
}

fn red_color() -> Rgb {
    Rgb::new(231.0/255.0, 76.0/255.0, 60.0/255.0, None)
}

// Estructuras para los datos del PDF
#[derive(Debug, Clone)]
pub struct InfoEncabezado {
    pub grado: String,
    pub seccion: String,
    pub docente_guia: String,
    pub periodo_escolar: String,
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

async fn obtener_info_encabezado(client: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<InfoEncabezado, String> {
    let stmt = client.prepare(
        "SELECT g.nombre_grado as grado, s.nombre_seccion as seccion, 
                p.periodo_escolar,
                d.nombres || ' ' || d.apellidos as docente_guia
         FROM grado_secciones gs
         INNER JOIN grados g ON gs.id_grado = g.id_grado
         INNER JOIN secciones s ON gs.id_seccion = s.id_seccion
         LEFT JOIN docentes d ON gs.id_docente_guia = d.id_docente
         INNER JOIN periodos_escolares p ON p.activo = true
         WHERE gs.id_grado_secciones = $1
         LIMIT 1"
    ).await.map_err(|e| format!("Error preparando consulta de encabezado: {}", e))?;

    let row = client.query_one(&stmt, &[&id_grado_secciones]).await.map_err(|e| format!("Error ejecutando consulta de encabezado: {}", e))?;

    Ok(InfoEncabezado {
        grado: row.get("grado"),
        seccion: row.get("seccion"),
        docente_guia: row.try_get("docente_guia").unwrap_or_else(|_| "N/A".to_string()),
        periodo_escolar: row.get("periodo_escolar"),
    })
}

async fn obtener_asignaturas_del_grado(client: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<Vec<AsignaturaInfo>, String> {
    let stmt = client.prepare(
        "SELECT a.id_asignatura, a.nombre FROM grado_seccion_asignatura_docente gsad
         INNER JOIN asignaturas a ON gsad.id_asignatura = a.id_asignatura
         INNER JOIN grado_secciones gs ON gsad.id_grado_secciones = gs.id_grado_secciones
         INNER JOIN grado_modalidad_asignaturas gma
            ON gma.id_asignatura = a.id_asignatura
            AND gma.id_grado = gs.id_grado
            AND gma.id_modalidad = gs.id_modalidad
         WHERE gsad.id_grado_secciones = $1
         ORDER BY gma.orden"
    ).await.map_err(|e| format!("Error preparando consulta de asignaturas: {}", e))?;
    
    let rows = client.query(&stmt, &[&id_grado_secciones]).await.map_err(|e| format!("Error ejecutando consulta de asignaturas: {}", e))?;

    Ok(rows.into_iter().map(|row| AsignaturaInfo {
        id: row.get("id_asignatura"),
        nombre: row.get("nombre"),
    }).collect())
}

async fn obtener_calificaciones_estudiantes(
    client: &tokio_postgres::Client,
    id_grado_secciones: i32,
    id_periodo_activo: i32,
    asignaturas_grado: &[AsignaturaInfo],
) -> Result<Vec<EstudianteCalificaciones>, String> {
    let stmt_estudiantes = client.prepare(
        "SELECT e.id, e.apellidos, e.nombres
         FROM estudiantes e
         INNER JOIN historial_grado_estudiantes hge ON e.id = hge.id_estudiante
         WHERE hge.id_grado_secciones = $1 AND hge.es_actual = true
         ORDER BY e.cedula"
    ).await.map_err(|e| format!("Error preparando consulta de estudiantes: {}", e))?;

    let estudiantes_rows = client.query(&stmt_estudiantes, &[&id_grado_secciones]).await.map_err(|e| format!("Error en consulta de estudiantes: {}", e))?;

    let mut resultado_final = Vec::new();

    let stmt_calificaciones = client.prepare(
        "SELECT
            c.id_asignatura,
            c.lapso_1 as lapso1,
            c.lapso_2 as lapso2,
            c.lapso_3 as lapso3,
            c.lapso_1_ajustado,
            c.lapso_2_ajustado,
            c.lapso_3_ajustado
        FROM calificaciones c
        WHERE c.id_estudiante = $1 AND c.id_periodo = $2"
    ).await.map_err(|e| format!("Error preparando consulta de calificaciones: {}", e))?;

    for estudiante_row in estudiantes_rows {
        let id_estudiante: i32 = estudiante_row.get("id");
        let calificaciones_rows = client.query(&stmt_calificaciones, &[&id_estudiante, &id_periodo_activo]).await.map_err(|e| format!("Error en consulta de calificaciones: {}", e))?;
        
        let calificaciones_por_asignatura: std::collections::HashMap<i32, CalificacionDetalle> = calificaciones_rows.into_iter().map(|row| {
            let id_asig: i32 = row.get("id_asignatura");
            let detalle = CalificacionDetalle {
                lapso1: row.get("lapso1"),
                lapso2: row.get("lapso2"),
                lapso3: row.get("lapso3"),
                lapso_ajustado_1: row.get("lapso_1_ajustado"),
                lapso_ajustado_2: row.get("lapso_2_ajustado"),
                lapso_ajustado_3: row.get("lapso_3_ajustado"),
                definitiva: None, // La calcularemos después
            };
            (id_asig, detalle)
        }).collect();

        let mut asignaturas_calificadas = Vec::new();
        for asig_info in asignaturas_grado {
            let calificacion_detalle = calificaciones_por_asignatura.get(&asig_info.id).cloned().unwrap_or(CalificacionDetalle {
                lapso1: None, 
                lapso2: None, 
                lapso3: None, 
                lapso_ajustado_1: None,
                lapso_ajustado_2: None,
                lapso_ajustado_3: None,
                definitiva: None
            });
            asignaturas_calificadas.push(AsignaturaCalificaciones {
                nombre_asignatura: asig_info.nombre.clone(),
                calificaciones: calificacion_detalle,
            });
        }
        
        resultado_final.push(EstudianteCalificaciones {
            apellidos: estudiante_row.get("apellidos"),
            nombres: estudiante_row.get("nombres"),
            asignaturas: asignaturas_calificadas,
        });
    }

    Ok(resultado_final)
}

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

fn formatear_calificacion(nota: i32) -> String {
    if nota < 10 {
        format!("0{}", nota)
    } else {
        nota.to_string()
    }
}

// Función para procesar logos desde base64 (reutilizada de pdf_estudiantes.rs)
fn procesar_logo(base64_str: &str, altura_max_mm: f32) -> Result<(DynamicImage, f32, f32), String> {
    // Decodificar base64 a bytes
    let bytes = general_purpose::STANDARD.decode(base64_str)
        .map_err(|e| format!("Error decodificando base64: {}", e))?;
    
    // Cargar imagen desde bytes (probamos varios formatos)
    let format = image::guess_format(&bytes)
        .map_err(|e| format!("Error adivinando formato de imagen: {}", e))?;
    let img = load(Cursor::new(bytes), format)
        .map_err(|e| format!("Error cargando imagen: {}", e))?;
    
    // Obtener dimensiones originales
    let (ancho_orig, alto_orig) = img.dimensions();
    let relacion_aspecto = ancho_orig as f32 / alto_orig as f32;
    
    // Calcular nuevo tamaño manteniendo relación de aspecto
    let alto_mm = altura_max_mm;
    let ancho_mm = alto_mm * relacion_aspecto;
    
    // Redimensionar imagen (convertir mm a píxeles - 150 DPI)
    let dpi = 150.0;
    let mm_to_px = dpi / 25.4;
    let alto_px = (alto_mm * mm_to_px) as u32;
    let ancho_px = (ancho_mm * mm_to_px) as u32;
    
    let img_redim = img.resize_exact(
        ancho_px,
        alto_px,
        FilterType::Lanczos3, // Filtro de alta calidad
    );
    
    Ok((img_redim, ancho_mm, alto_mm))
}

fn dibujar_encabezado_con_logo(
    layer: &mut PdfLayerReference,
    doc_info: &InfoEncabezado,
    font_bold: &IndirectFontRef,
    _font_regular: &IndirectFontRef,
    logo_base64: &str,
) {
    let y_start = PAGE_HEIGHT - MARGIN_TOP;
    
    // Procesar el logo desde base64
    let (_logo_width, _logo_height, text_start_x) = if let Ok((logo_dyn, ancho_logo, alto_logo)) = procesar_logo(logo_base64, 20.0) {
        // Insertar el logo en la esquina superior izquierda
                 Image::from_dynamic_image(&logo_dyn).add_to_layer(
             layer.clone(),
             ImageTransform {
                 translate_x: Some(Mm(MARGIN_LEFT)),
                 translate_y: Some(Mm(y_start - alto_logo + 3.0)), // Subir el logo 3mm
                 dpi: Some(150.0),
                 ..Default::default()
             }
         );
        
        let text_start_x = MARGIN_LEFT + ancho_logo + 5.0; // Espacio después del logo
        (ancho_logo, alto_logo, text_start_x)
    } else {
        // Si no se puede cargar el logo, usar valores por defecto
        (0.0, 0.0, MARGIN_LEFT)
    };
    
    // Título principal alineado con el logo (ajustado para mejor alineación)
    layer.set_font(font_bold, 11.0);
    layer.set_fill_color(Color::Rgb(secondary_color()));
    layer.begin_text_section();
    layer.set_text_cursor(Mm(text_start_x), Mm(y_start - 2.0)); // Subir el título
    layer.write_text("COMPLEJO EDUCATIVO PROFESOR JESÚS LÓPEZ CASTRO", font_bold);
    layer.end_text_section();
    
    // Subtítulo
    layer.set_font(font_bold, 9.0);
    layer.set_fill_color(Color::Rgb(primary_color()));
    layer.begin_text_section();
    layer.set_text_cursor(Mm(text_start_x), Mm(y_start - 6.5)); // Ajustar posición
    layer.write_text("ACTA DE RESUMEN DE CALIFICACIONES", font_bold);
    layer.end_text_section();

    // Información del curso en una línea compacta
    layer.set_font(font_bold, 8.0);
    layer.set_fill_color(Color::Rgb(secondary_color()));
    let info_text = format!(
        "GRADO: {}   SECCIÓN: {}   DOCENTE GUÍA: {}   AÑO ESCOLAR: {}",
        doc_info.grado, doc_info.seccion, doc_info.docente_guia, doc_info.periodo_escolar
    );
    layer.begin_text_section();
    layer.set_text_cursor(Mm(text_start_x), Mm(y_start - 10.5)); // Ajustar posición
    layer.write_text(&info_text, font_bold);
    layer.end_text_section();
    
    // Línea divisoria debajo del encabezado
    layer.set_outline_color(Color::Rgb(primary_color()));
    layer.set_outline_thickness(0.5);
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(MARGIN_LEFT), Mm(y_start - 17.5)), false), // Bajar la línea otros 0.3mm más
            (Point::new(Mm(PAGE_WIDTH - MARGIN_RIGHT), Mm(y_start - 17.5)), false)
        ],
        is_closed: false,
    });
}

fn dibujar_tabla_acta_pagina(
    layer: &mut PdfLayerReference,
    font_bold: &IndirectFontRef,
    _font_regular: &IndirectFontRef,
    asignaturas: &[AsignaturaInfo],
    estudiantes: &[EstudianteCalificaciones],
    inicio_numeracion: usize,
) {
    let table_start_y = PAGE_HEIGHT - MARGIN_TOP - 21.0; // Bajar la tabla para dar más espacio al encabezado mejorado
    let row_height = 7.2; // Filas un poco más altas (+0.2mm)
    let header_height = 14.0; // Encabezado más compacto
    
    // Cálculo optimizado de anchos de columnas
    let available_width = CONTENT_WIDTH;
    let num_asignaturas = asignaturas.len();
    let col_num_width = 6.0; // Columna N° más estrecha
    let col_name_width = if num_asignaturas > 10 { 45.0 } else { 55.0 }; // Nombre adaptativo
    let remaining_width = available_width - col_num_width - col_name_width;
    let col_subject_width = remaining_width / num_asignaturas as f32;
    let col_lapso_width = col_subject_width / 4.0; // Cada lapso dentro de asignatura
    
    // Color negro para bordes
    let black_color = Rgb::new(0.0, 0.0, 0.0, None);
    layer.set_outline_color(Color::Rgb(black_color));
    layer.set_outline_thickness(0.5);
    
    // ENCABEZADOS CON JERARQUÍA VISUAL MEJORADA
    
    // Encabezado N° (centrado)
    layer.set_fill_color(Color::Rgb(secondary_color()));
    layer.set_font(font_bold, 10.0); // Fuente prominente
    let n_text_width = 4.0; // Aproximación del ancho de "N°"
    let center_x_num = MARGIN_LEFT + (col_num_width / 2.0) - (n_text_width / 2.0);
    layer.begin_text_section();
    layer.set_text_cursor(Mm(center_x_num), Mm(table_start_y - 7.0)); // Centrado verticalmente
    layer.write_text("N°", font_bold);
    layer.end_text_section();
    
    // Encabezado principal: APELLIDOS Y NOMBRES (perfectamente centrado y en mayúsculas)
    layer.set_fill_color(Color::Rgb(primary_color())); // Color primario para mayor jerarquía
    layer.set_font(font_bold, 10.0); // Fuente prominente
    let apellidos_nombres_text = "APELLIDOS Y NOMBRES";
    // Cálculo más preciso del centrado basado en el ancho real del texto
    let char_width = 2.5; // Ancho promedio por carácter en fuente 10.0 bold
    let text_width = apellidos_nombres_text.len() as f32 * char_width;
    let center_x_apellidos = MARGIN_LEFT + col_num_width + (col_name_width / 2.0) - (text_width / 2.0);
    layer.begin_text_section();
    layer.set_text_cursor(Mm(center_x_apellidos), Mm(table_start_y - 7.0)); // Centrado verticalmente
    layer.write_text("APELLIDOS Y NOMBRES", font_bold);
    layer.end_text_section();
    
    // Encabezados de asignaturas
    let mut x_pos = MARGIN_LEFT + col_num_width + col_name_width;
    for asignatura in asignaturas {
        let nombre_corto = abreviar_asignatura(&asignatura.nombre);
        
        // Nombre de la asignatura (centrado, negro y negrita)
        let center_x = x_pos + (col_subject_width / 2.0) - ((nombre_corto.len() as f32 * 1.8) / 2.0);
        layer.set_font(font_bold, 8.0);
        layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None))); // Negro
        layer.begin_text_section();
        layer.set_text_cursor(Mm(center_x), Mm(table_start_y - 4.0));
        layer.write_text(&nombre_corto, font_bold);
        layer.end_text_section();
        
        // Sub-encabezados de lapsos (1°, 2°, 3°, DF) - negro y negrita
        layer.set_font(font_bold, 7.5); // Fuente más grande y en negrita para lapsos
        layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None))); // Negro
        let sub_headers = ["1°", "2°", "3°", "DF"];
        for (i, header) in sub_headers.iter().enumerate() {
            let sub_x = x_pos + (i as f32 * col_lapso_width) + (col_lapso_width / 2.0) - 2.0;
            layer.begin_text_section();
            layer.set_text_cursor(Mm(sub_x), Mm(table_start_y - 11.0));
            layer.write_text(*header, font_bold);
            layer.end_text_section();
        }
        
        x_pos += col_subject_width;
    }
    
    // LÍNEAS DE ENCABEZADO
    // Línea horizontal superior
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(MARGIN_LEFT), Mm(table_start_y)), false),
            (Point::new(Mm(MARGIN_LEFT + CONTENT_WIDTH), Mm(table_start_y)), false)
        ],
        is_closed: false,
    });
    
    // Línea horizontal intermedia (entre nombres de asignaturas y lapsos)
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(MARGIN_LEFT + col_num_width + col_name_width), Mm(table_start_y - 7.0)), false),
            (Point::new(Mm(MARGIN_LEFT + CONTENT_WIDTH), Mm(table_start_y - 7.0)), false)
        ],
        is_closed: false,
    });
    
    // Línea horizontal del final del encabezado
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(MARGIN_LEFT), Mm(table_start_y - header_height)), false),
            (Point::new(Mm(MARGIN_LEFT + CONTENT_WIDTH), Mm(table_start_y - header_height)), false)
        ],
        is_closed: false,
    });
    
    // LÍNEAS VERTICALES DEL ENCABEZADO
    let mut x_line = MARGIN_LEFT;
    
    // Borde izquierdo
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(x_line), Mm(table_start_y)), false),
            (Point::new(Mm(x_line), Mm(table_start_y - header_height)), false)
        ],
        is_closed: false,
    });
    
    // Separación después de N°
    x_line += col_num_width;
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(x_line), Mm(table_start_y)), false),
            (Point::new(Mm(x_line), Mm(table_start_y - header_height)), false)
        ],
        is_closed: false,
    });
    
    // Separación después de ESTUDIANTE
    x_line += col_name_width;
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(x_line), Mm(table_start_y)), false),
            (Point::new(Mm(x_line), Mm(table_start_y - header_height)), false)
        ],
        is_closed: false,
    });
    
    // Separaciones entre asignaturas y entre lapsos
    for _ in 0..num_asignaturas {
        // Línea principal de asignatura
        x_line += col_subject_width;
        layer.add_line(Line {
            points: vec![
                (Point::new(Mm(x_line), Mm(table_start_y)), false),
                (Point::new(Mm(x_line), Mm(table_start_y - header_height)), false)
            ],
            is_closed: false,
        });
        
        // Líneas de separación entre lapsos (solo en la parte inferior del encabezado)
        let asig_start_x = x_line - col_subject_width;
        for i in 1..4 {
            let lapso_x = asig_start_x + (i as f32 * col_lapso_width);
            layer.add_line(Line {
                points: vec![
                    (Point::new(Mm(lapso_x), Mm(table_start_y - 7.0)), false),
                    (Point::new(Mm(lapso_x), Mm(table_start_y - header_height)), false)
                ],
                is_closed: false,
            });
        }
    }
    
    // FILAS DE ESTUDIANTES
    layer.set_fill_color(Color::Rgb(secondary_color()));
    let mut current_y = table_start_y - header_height;
    
    for (index, estudiante) in estudiantes.iter().enumerate().take(22) { // Máximo 22 estudiantes por página
        // Línea horizontal de separación
        layer.add_line(Line {
            points: vec![
                (Point::new(Mm(MARGIN_LEFT), Mm(current_y)), false),
                (Point::new(Mm(MARGIN_LEFT + CONTENT_WIDTH), Mm(current_y)), false)
            ],
            is_closed: false,
        });
        
        // Líneas verticales de la fila
        let mut x_line = MARGIN_LEFT;
        
        // Columna N°
        layer.add_line(Line {
            points: vec![
                (Point::new(Mm(x_line), Mm(current_y)), false),
                (Point::new(Mm(x_line), Mm(current_y - row_height)), false)
            ],
            is_closed: false,
        });
        
        x_line += col_num_width;
        layer.add_line(Line {
            points: vec![
                (Point::new(Mm(x_line), Mm(current_y)), false),
                (Point::new(Mm(x_line), Mm(current_y - row_height)), false)
            ],
            is_closed: false,
        });
        
        // Columna ESTUDIANTE
        x_line += col_name_width;
        layer.add_line(Line {
            points: vec![
                (Point::new(Mm(x_line), Mm(current_y)), false),
                (Point::new(Mm(x_line), Mm(current_y - row_height)), false)
            ],
            is_closed: false,
        });
        
        // Columnas de asignaturas
        for _asig_index in 0..num_asignaturas {
            // Líneas de separación entre lapsos
            for i in 1..4 {
                let lapso_x = x_line + (i as f32 * col_lapso_width);
                layer.add_line(Line {
                    points: vec![
                        (Point::new(Mm(lapso_x), Mm(current_y)), false),
                        (Point::new(Mm(lapso_x), Mm(current_y - row_height)), false)
                    ],
                    is_closed: false,
                });
            }
            
            x_line += col_subject_width;
            layer.add_line(Line {
                points: vec![
                    (Point::new(Mm(x_line), Mm(current_y)), false),
                    (Point::new(Mm(x_line), Mm(current_y - row_height)), false)
                ],
                is_closed: false,
            });
        }
        
        // CONTENIDO DE LA FILA
        
        // Número de estudiante (numeración continua)
        layer.set_font(font_bold, 10.0); // Fuente tamaño 10
        layer.begin_text_section();
        layer.set_text_cursor(Mm(MARGIN_LEFT + 1.0), Mm(current_y - 4.5));
        layer.write_text(&(inicio_numeracion + index + 1).to_string(), font_bold);
        layer.end_text_section();
        
        // Nombre del estudiante (negro y negrita, truncado inteligentemente)
        layer.set_font(font_bold, 7.0); // Fuente en negrita
        layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None))); // Negro
        let nombre_completo = format!("{} {}", estudiante.apellidos, estudiante.nombres);
        
        // Cálculo más preciso del truncado basado en el ancho disponible
        let char_width_7pt = 1.59; // Ancho promedio por carácter en fuente 7.0 bold (ajustado para 34 chars)
        let available_width_for_text = col_name_width - 1.0; // Dejar margen de 0.5mm a cada lado
        let max_chars = (available_width_for_text / char_width_7pt) as usize;
        
        let nombre_truncado = if nombre_completo.len() > max_chars && max_chars > 3 {
            format!("{}...", &nombre_completo[..max_chars-3])
        } else if nombre_completo.len() > max_chars {
            nombre_completo[..max_chars].to_string()
        } else {
            nombre_completo
        };
        
        layer.begin_text_section();
        layer.set_text_cursor(Mm(MARGIN_LEFT + col_num_width + 0.5), Mm(current_y - 4.5));
        layer.write_text(&nombre_truncado, font_bold);
        layer.end_text_section();
        
        // Calificaciones por asignatura
        let mut x_pos = MARGIN_LEFT + col_num_width + col_name_width;
        for asig_calif in &estudiante.asignaturas {
            let calif = &asig_calif.calificaciones;
            
            let lapsos = [calif.lapso1, calif.lapso2, calif.lapso3];
            let lapsos_ajustados = [calif.lapso_ajustado_1, calif.lapso_ajustado_2, calif.lapso_ajustado_3];
            
            // Mostrar lapsos 1, 2, 3
            for i in 0..3 {
                let lapso_x = x_pos + (i as f32 * col_lapso_width) + (col_lapso_width / 2.0) - 2.0;
                
                let tiene_ajuste = lapsos_ajustados[i].is_some();
                let tiene_principal = lapsos[i].is_some();
                
                // Si hay ajuste, mostrar formato dual (original arriba pequeña, ajuste abajo grande)
                if tiene_ajuste {
                    // Nota original (arriba, pequeña)
                    if let Some(nota) = lapsos[i] {
                        layer.set_font(font_bold, 8.0); // Fuente más pequeña para original
                        if nota >= 10 {
                            layer.set_fill_color(Color::Rgb(primary_color())); // Azul
                        } else {
                            layer.set_fill_color(Color::Rgb(red_color())); // Rojo
                        }
                        layer.begin_text_section();
                        layer.set_text_cursor(Mm(lapso_x), Mm(current_y - 2.9));
                        layer.write_text(&formatear_calificacion(nota), font_bold);
                        layer.end_text_section();
                    }
                    
                    // Nota ajustada (abajo, grande, verde)
                    if let Some(nota_ajustada) = lapsos_ajustados[i] {
                        layer.set_font(font_bold, 10.0); // Fuente más grande para ajuste
                        layer.set_fill_color(Color::Rgb(green_color())); // Verde para ajustadas
                        layer.begin_text_section();
                        layer.set_text_cursor(Mm(lapso_x), Mm(current_y - 5.9));
                        layer.write_text(&formatear_calificacion(nota_ajustada), font_bold);
                        layer.end_text_section();
                    }
                } else if tiene_principal {
                    // Si solo hay nota principal, centrarla verticalmente
                    if let Some(nota) = lapsos[i] {
                        layer.set_font(font_bold, 10.0);
                        if nota >= 10 {
                            layer.set_fill_color(Color::Rgb(primary_color())); // Azul
                        } else {
                            layer.set_fill_color(Color::Rgb(red_color())); // Rojo
                        }
                        layer.begin_text_section();
                        layer.set_text_cursor(Mm(lapso_x), Mm(current_y - 4.4)); // Centrada verticalmente
                        layer.write_text(&formatear_calificacion(nota), font_bold);
                        layer.end_text_section();
                    }
                }
            }
            
            // Definitiva
            let mut suma_definitiva = 0;
            let mut cont_definitiva = 0;
            
            for i in 0..3 {
                let nota_final = lapsos_ajustados[i].or(lapsos[i]);
                if let Some(nota) = nota_final {
                    suma_definitiva += nota;
                    cont_definitiva += 1;
                }
            }
            
            if cont_definitiva > 0 {
                let definitiva = (suma_definitiva as f32 / cont_definitiva as f32).round() as i32;
                let def_x = x_pos + (3.0 * col_lapso_width) + (col_lapso_width / 2.0) - 2.0;
                
                layer.set_font(font_bold, 10.5); // Fuente más grande para definitiva
                // Color según la calificación definitiva: azul ≥10, rojo <10
                if definitiva >= 10 {
                    layer.set_fill_color(Color::Rgb(primary_color())); // Azul
                } else {
                    layer.set_fill_color(Color::Rgb(red_color())); // Rojo
                }
                
                layer.begin_text_section();
                layer.set_text_cursor(Mm(def_x), Mm(current_y - 4.4)); // Siempre centrada (no hay ajustes en definitiva)
                layer.write_text(&formatear_calificacion(definitiva), font_bold);
                layer.end_text_section();
            }
            
            x_pos += col_subject_width;
        }
        
        current_y -= row_height;
    }
    
    // Línea horizontal final
    layer.add_line(Line {
        points: vec![
            (Point::new(Mm(MARGIN_LEFT), Mm(current_y)), false),
            (Point::new(Mm(MARGIN_LEFT + CONTENT_WIDTH), Mm(current_y)), false)
        ],
        is_closed: false,
    });
}

#[tauri::command]
pub async fn generar_acta_resumen(
    id_grado_secciones: i32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let client_guard = state.db.lock().await;
    let client = &*client_guard;

    // 1. Obtener el ID del período activo
    let periodo_activo_row = client.query_one("SELECT id_periodo FROM periodos_escolares WHERE activo = true LIMIT 1", &[])
        .await.map_err(|e| format!("Error obteniendo período activo: {}", e))?;
    let id_periodo_activo: i32 = periodo_activo_row.get("id_periodo");

    // 2. Obtener la información restante
    let info_encabezado = obtener_info_encabezado(client, id_grado_secciones).await?;
    let asignaturas = obtener_asignaturas_del_grado(client, id_grado_secciones).await?;
    let estudiantes_con_calificaciones = obtener_calificaciones_estudiantes(client, id_grado_secciones, id_periodo_activo, &asignaturas).await?;

    // --- Generación del PDF moderno ---
    let (doc, page1, layer1) = PdfDocument::new(
        "Acta de Resumen de Calificaciones",
        Mm(PAGE_WIDTH), 
        Mm(PAGE_HEIGHT), 
        "Layer 1",
    );
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).map_err(|e| e.to_string())?;
    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica).map_err(|e| e.to_string())?;

    // Procesar estudiantes en páginas de 22
    let estudiantes_por_pagina = 22;
    let total_estudiantes = estudiantes_con_calificaciones.len();
    let total_paginas = (total_estudiantes + estudiantes_por_pagina - 1) / estudiantes_por_pagina;

    for pagina in 0..total_paginas {
        let inicio_estudiante = pagina * estudiantes_por_pagina;
        let fin_estudiante = std::cmp::min(inicio_estudiante + estudiantes_por_pagina, total_estudiantes);
        let estudiantes_pagina = &estudiantes_con_calificaciones[inicio_estudiante..fin_estudiante];

        // Crear la página correspondiente
        let mut layer_actual = if pagina == 0 {
            doc.get_page(page1).get_layer(layer1)
        } else {
            let (nueva_pagina, nuevo_layer) = doc.add_page(Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer");
            doc.get_page(nueva_pagina).get_layer(nuevo_layer)
        };

        // Dibujar encabezado en cada página
        dibujar_encabezado_con_logo(&mut layer_actual, &info_encabezado, &font_bold, &font_regular, &state.logo_der);
        
        // Dibujar tabla con numeración continua
        dibujar_tabla_acta_pagina(&mut layer_actual, &font_bold, &font_regular, &asignaturas, estudiantes_pagina, inicio_estudiante);
    }

    // --- Finalización del PDF ---
    let mut buf = BufWriter::new(Vec::new());
    doc.save(&mut buf).map_err(|e| format!("Error guardando el PDF: {}", e))?;
    let pdf_bytes = buf.into_inner().map_err(|e| format!("Error obteniendo bytes del PDF: {}", e))?;
    
    Ok(general_purpose::STANDARD.encode(&pdf_bytes))
}