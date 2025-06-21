use printpdf::{BuiltinFont, IndirectFontRef, Line, Mm, PdfDocument, PdfLayerReference, Point, Rgb, Color};
use std::io::BufWriter;
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
        "ARTE Y PATRIMONIO" => "ARTE".to_string(),
        "ORIENTACION Y CONVIVENCIA" | "ORIENTACIÓN Y CONVIVENCIA" => "ORIENT".to_string(),
        "CIENCIAS DE LA TIERRA" => "C.TIERRA".to_string(),
        _ => {
            if nombre.len() > 8 {
                nombre.chars().take(8).collect::<String>().to_uppercase()
            } else {
                nombre_upper
            }
        }
    }
}

fn dibujar_encabezado_compacto(
    layer: &mut PdfLayerReference,
    doc_info: &InfoEncabezado,
    font_bold: &IndirectFontRef,
    _font_regular: &IndirectFontRef,
) {
    let y_start = PAGE_HEIGHT - MARGIN_TOP;
    
    // Título principal más compacto
    layer.set_font(font_bold, 10.0);
    layer.set_fill_color(Color::Rgb(secondary_color()));
    layer.begin_text_section();
    layer.set_text_cursor(Mm(MARGIN_LEFT), Mm(y_start - 5.0));
    layer.write_text("COMPLEJO EDUCATIVO PROFESOR JESÚS LÓPEZ CASTRO - ACTA DE RESUMEN DE CALIFICACIONES", font_bold);
    layer.end_text_section();

    // Información del curso en una línea ultra compacta
    layer.set_font(font_bold, 8.0);
    let info_text = format!(
        "GRADO: {}   SECCIÓN: {}   DOCENTE GUÍA: {}   AÑO ESCOLAR: {}",
        doc_info.grado, doc_info.seccion, doc_info.docente_guia, doc_info.periodo_escolar
    );
    layer.begin_text_section();
    layer.set_text_cursor(Mm(MARGIN_LEFT), Mm(y_start - 12.0));
    layer.write_text(&info_text, font_bold);
    layer.end_text_section();
}

fn dibujar_tabla_acta_completa(
    layer: &mut PdfLayerReference,
    font_bold: &IndirectFontRef,
    font_regular: &IndirectFontRef,
    asignaturas: &[AsignaturaInfo],
    estudiantes: &[EstudianteCalificaciones],
) {
    let table_start_y = PAGE_HEIGHT - MARGIN_TOP - 16.0; // Subir la tabla
    let row_height = 7.0; // Filas más compactas
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
    
    // ENCABEZADOS
    layer.set_fill_color(Color::Rgb(secondary_color()));
    layer.set_font(font_bold, 9.0); // Fuente más grande para encabezados
    
    // Encabezado N°
    layer.begin_text_section();
    layer.set_text_cursor(Mm(MARGIN_LEFT + 1.0), Mm(table_start_y - 6.0));
    layer.write_text("N°", font_bold);
    layer.end_text_section();
    
    // Encabezado ESTUDIANTE
    layer.begin_text_section();
    layer.set_text_cursor(Mm(MARGIN_LEFT + col_num_width + 1.0), Mm(table_start_y - 6.0));
    layer.write_text("ESTUDIANTE", font_bold);
    layer.end_text_section();
    
    // Encabezados de asignaturas
    let mut x_pos = MARGIN_LEFT + col_num_width + col_name_width;
    for asignatura in asignaturas {
        let nombre_corto = abreviar_asignatura(&asignatura.nombre);
        
        // Nombre de la asignatura (centrado, fuente más grande)
        let center_x = x_pos + (col_subject_width / 2.0) - ((nombre_corto.len() as f32 * 1.8) / 2.0);
        layer.set_font(font_bold, 8.0);
        layer.begin_text_section();
        layer.set_text_cursor(Mm(center_x), Mm(table_start_y - 4.0));
        layer.write_text(&nombre_corto, font_bold);
        layer.end_text_section();
        
        // Sub-encabezados de lapsos (1, 2, 3, DF)
        layer.set_font(font_regular, 7.0); // Fuente más grande para lapsos
        let sub_headers = ["1", "2", "3", "DF"];
        for (i, header) in sub_headers.iter().enumerate() {
            let sub_x = x_pos + (i as f32 * col_lapso_width) + (col_lapso_width / 2.0) - 1.5;
            layer.begin_text_section();
            layer.set_text_cursor(Mm(sub_x), Mm(table_start_y - 11.0));
            layer.write_text(*header, font_regular);
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
    
    // Línea horizontal intermedia (entre asignatura y lapsos)
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
    
    for (index, estudiante) in estudiantes.iter().enumerate().take(22) { // Máximo 22 estudiantes
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
        for asig_index in 0..num_asignaturas {
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
        
        // Número de estudiante
        layer.set_font(font_bold, 8.0); // Fuente más grande
        layer.begin_text_section();
        layer.set_text_cursor(Mm(MARGIN_LEFT + 1.0), Mm(current_y - 4.5));
        layer.write_text(&(index + 1).to_string(), font_bold);
        layer.end_text_section();
        
        // Nombre del estudiante
        layer.set_font(font_regular, 7.0); // Fuente más grande
        let nombre_completo = format!("{} {}", estudiante.apellidos, estudiante.nombres);
        let max_chars = if num_asignaturas > 10 { 35 } else { 45 };
        let nombre_truncado = if nombre_completo.len() > max_chars {
            format!("{}...", &nombre_completo[..max_chars-3])
        } else {
            nombre_completo
        };
        
        layer.begin_text_section();
        layer.set_text_cursor(Mm(MARGIN_LEFT + col_num_width + 0.5), Mm(current_y - 4.5));
        layer.write_text(&nombre_truncado, font_regular);
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
                
                // Nota principal
                if let Some(nota) = lapsos[i] {
                    layer.set_font(font_regular, 8.0); // Fuente más grande
                    layer.set_fill_color(Color::Rgb(secondary_color()));
                    layer.begin_text_section();
                    layer.set_text_cursor(Mm(lapso_x), Mm(current_y - 2.5));
                    layer.write_text(&nota.to_string(), font_regular);
                    layer.end_text_section();
                }
                
                // Nota ajustada (debajo en rojo)
                if let Some(nota_ajustada) = lapsos_ajustados[i] {
                    layer.set_font(font_regular, 7.0); // Fuente más grande
                    layer.set_fill_color(Color::Rgb(red_color()));
                    layer.begin_text_section();
                    layer.set_text_cursor(Mm(lapso_x), Mm(current_y - 5.5));
                    layer.write_text(&nota_ajustada.to_string(), font_regular);
                    layer.end_text_section();
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
                
                layer.set_font(font_bold, 8.0); // Fuente más grande
                layer.set_fill_color(Color::Rgb(secondary_color()));
                
                layer.begin_text_section();
                layer.set_text_cursor(Mm(def_x), Mm(current_y - 4.0));
                layer.write_text(&definitiva.to_string(), font_bold);
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
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).map_err(|e| e.to_string())?;
    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica).map_err(|e| e.to_string())?;

    dibujar_encabezado_compacto(&mut current_layer, &info_encabezado, &font_bold, &font_regular);
    dibujar_tabla_acta_completa(&mut current_layer, &font_bold, &font_regular, &asignaturas, &estudiantes_con_calificaciones);

    // --- Finalización del PDF ---
    let mut buf = BufWriter::new(Vec::new());
    doc.save(&mut buf).map_err(|e| format!("Error guardando el PDF: {}", e))?;
    let pdf_bytes = buf.into_inner().map_err(|e| format!("Error obteniendo bytes del PDF: {}", e))?;
    
    Ok(general_purpose::STANDARD.encode(&pdf_bytes))
}