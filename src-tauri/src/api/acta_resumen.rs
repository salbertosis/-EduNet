use printpdf::{BuiltinFont, IndirectFontRef, Line, Mm, PdfDocument, PdfLayerReference, Point};
use std::io::BufWriter;
use base64::{engine::general_purpose, Engine as _};
use crate::AppState;
use tauri::State;

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

#[derive(Debug, Clone)]
pub struct AsignaturaInfo {
    pub id: i32,
    pub nombre: String,
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
            c.lapso_3 as lapso3
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
                definitiva: None, // La calcularemos después
            };
            (id_asig, detalle)
        }).collect();

        let mut asignaturas_calificadas = Vec::new();
        for asig_info in asignaturas_grado {
            let calificacion_detalle = calificaciones_por_asignatura.get(&asig_info.id).cloned().unwrap_or(CalificacionDetalle {
                lapso1: None, lapso2: None, lapso3: None, definitiva: None
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

fn iniciales_asignatura(nombre: &str) -> String {
    let nombre = nombre.trim().to_uppercase();
    match nombre.as_str() {
        "CIENCIAS DE LA TIERRA" => "C. TIERRA".to_string(),
        "EDUCACION FISICA" | "EDUCACIÓN FÍSICA" => "ED. FISICA".to_string(),
        "ARTE Y PATRIMONIO" => "ARTE Y\nPATRIM.".to_string(),
        "ORIENTACION Y CONVIVENCIA" | "ORIENTACIÓN Y CONVIVENCIA" => "ORIENT.".to_string(),
        "GRUPOS DE CREACION, RECREACION Y PRODUCCION" => "GCRP".to_string(),
        _ => nombre,
    }
}

fn dibujar_encabezado(
    layer: &mut PdfLayerReference,
    doc_info: &InfoEncabezado,
    font_bold: &IndirectFontRef,
    font_regular: &IndirectFontRef,
) {
    // Títulos principales
    layer.set_font(font_bold, 12.0);
    layer.begin_text_section();
    layer.set_text_cursor(Mm(15.0), Mm(200.0));
    layer.write_text("COMPLEJO EDUCATIVO", font_bold);
    layer.end_text_section();

    layer.begin_text_section();
    layer.set_text_cursor(Mm(15.0), Mm(195.0));
    layer.write_text("PROFESOR JESÚS LÓPEZ CASTRO", font_bold);
    layer.end_text_section();

    // Información del curso
    layer.set_font(font_regular, 10.0);
    let info_texto = format!(
        "GRADO: {}   SECCIÓN: {}   DOCENTE GUÍA: {}   AÑO ESCOLAR: {}",
        doc_info.grado, doc_info.seccion, doc_info.docente_guia, doc_info.periodo_escolar
    );
    layer.begin_text_section();
    layer.set_text_cursor(Mm(15.0), Mm(185.0));
    layer.write_text(info_texto, font_regular);
    layer.end_text_section();
}

fn dibujar_tabla_calificaciones(
    layer: &mut PdfLayerReference,
    font_bold: &IndirectFontRef,
    font_regular: &IndirectFontRef,
    asignaturas: &[AsignaturaInfo],
    estudiantes: &[EstudianteCalificaciones],
) {
    // --- Configuración de la tabla ---
    let y_inicio_tabla = 180.0;
    let x_inicio_tabla = 15.0;
    let ancho_total_tabla = 279.4 - 30.0;
    
    let alto_fila_header_asignatura = 10.0;
    let alto_fila_header_lapsos = 5.0;
    let alto_fila_estudiante = 10.0;

    let num_filas_a_dibujar = estudiantes.len().max(25);

    // Anchos de las columnas
    let ancho_col_n = 8.0;
    let ancho_col_nombres = 65.0;
    let ancho_disponible_asig = ancho_total_tabla - ancho_col_n - ancho_col_nombres;
    let ancho_col_asignatura = ancho_disponible_asig / asignaturas.len() as f32;
    let ancho_subcol_lapso = ancho_col_asignatura / 4.0;

    // --- Coordenadas Y ---
    let y_linea_superior = y_inicio_tabla;
    let y_linea_media = y_inicio_tabla - alto_fila_header_asignatura;
    let y_linea_inferior_header = y_linea_media - alto_fila_header_lapsos;

    // --- Dibujar Cabecera de la Tabla ---
    
    // Texto "ASIGNATURAS" centrado aproximado
    let x_centro_area_asig = x_inicio_tabla + ancho_col_n + ancho_col_nombres + (ancho_disponible_asig / 2.0);
    layer.begin_text_section();
    layer.set_font(font_bold, 8.0);
    layer.set_text_cursor(Mm(x_centro_area_asig - 20.0), Mm(y_linea_media + 3.0));
    layer.write_text("ASIGNATURAS", font_bold);
    layer.end_text_section();

    // Recorrer cada asignatura
    let mut x_actual_asig = x_inicio_tabla + ancho_col_n + ancho_col_nombres;
    for asig in asignaturas {
        let nombre_corto = iniciales_asignatura(&asig.nombre);
        
        // Dibujar nombre de la asignatura (centrado aproximado)
        layer.begin_text_section();
        layer.set_font(font_bold, 7.0);
        layer.set_text_cursor(
            Mm(x_actual_asig + (ancho_col_asignatura / 2.0) - 10.0), // Ajuste manual
            Mm(y_linea_media + 1.0)
        );
        layer.write_text(&nombre_corto, font_bold);
        layer.end_text_section();

        // Dibujar sub-cabeceras "1, 2, 3, DF"
        for i in 0..4 {
            let texto_lapso = match i { 0 => "1", 1 => "2", 2 => "3", 3 => "DF", _ => "" };
            layer.begin_text_section();
            layer.set_font(font_regular, 7.0);
            layer.set_text_cursor(
                Mm(x_actual_asig + (i as f32 * ancho_subcol_lapso) + 2.0), // Ajuste manual
                Mm(y_linea_inferior_header + 1.5)
            );
            layer.write_text(texto_lapso, font_regular);
            layer.end_text_section();
        }
        x_actual_asig += ancho_col_asignatura;
    }

    // --- Dibujar Filas de Estudiantes ---
    let mut y_actual_fila = y_linea_inferior_header;
    layer.set_outline_thickness(0.1);
    
    for (i, estudiante) in estudiantes.iter().enumerate() {
        let y_centro_fila = y_actual_fila - (alto_fila_estudiante / 2.0);
        let nombre_completo = format!("{} {}", estudiante.apellidos, estudiante.nombres);
        
        // Escribir Número y Nombre del estudiante
        layer.set_font(font_regular, 8.0);
        layer.begin_text_section();
        layer.set_text_cursor(Mm(x_inicio_tabla + 2.0), Mm(y_centro_fila - 1.0));
        layer.write_text((i + 1).to_string(), font_regular);
        layer.end_text_section();
        
        layer.begin_text_section();
        layer.set_text_cursor(Mm(x_inicio_tabla + ancho_col_n + 2.0), Mm(y_centro_fila - 1.0));
        layer.write_text(&nombre_completo, font_regular);
        layer.end_text_section();

        // Escribir calificaciones
        let mut x_actual_calif = x_inicio_tabla + ancho_col_n + ancho_col_nombres;
        for asig_calif in &estudiante.asignaturas {
            let calif = &asig_calif.calificaciones;
            let notas = [calif.lapso1, calif.lapso2, calif.lapso3];
            let mut suma = 0;
            let mut cont = 0;
            
            // Lapsos 1, 2, 3
            for (j, nota_opt) in notas.iter().enumerate() {
                if let Some(nota) = nota_opt {
                    suma += nota;
                    cont += 1;
                    let nota_str = nota.to_string();
                    layer.begin_text_section();
                    layer.set_text_cursor(
                        Mm(x_actual_calif + (j as f32 * ancho_subcol_lapso) + 2.0), // Ajuste manual
                        Mm(y_centro_fila - 1.0)
                    );
                    layer.write_text(&nota_str, font_regular);
                    layer.end_text_section();
                }
            }

            // Definitiva
            if cont > 0 {
                let definitiva = (suma as f32 / cont as f32).round() as i32;
                let def_str = definitiva.to_string();
                layer.begin_text_section();
                layer.set_font(font_bold, 8.0);
                layer.set_text_cursor(
                    Mm(x_actual_calif + (3.0 * ancho_subcol_lapso) + 2.0), // Ajuste manual
                    Mm(y_centro_fila - 1.0)
                );
                layer.write_text(&def_str, font_bold);
                layer.end_text_section();
                layer.set_font(font_regular, 8.0); // Volver a fuente regular
            }
            x_actual_calif += ancho_col_asignatura;
        }

        y_actual_fila -= alto_fila_estudiante;
    }

    // --- Dibujar Líneas de la Rejilla ---
    let y_final_tabla = y_linea_inferior_header - (num_filas_a_dibujar as f32 * alto_fila_estudiante);
    
    // Líneas Horizontales
    layer.add_line(Line { points: vec![(Point::new(Mm(x_inicio_tabla), Mm(y_linea_superior)), false), (Point::new(Mm(x_inicio_tabla + ancho_total_tabla), Mm(y_linea_superior)), false)], is_closed: false });
    layer.add_line(Line { points: vec![(Point::new(Mm(x_inicio_tabla), Mm(y_linea_media)), false), (Point::new(Mm(x_inicio_tabla + ancho_total_tabla), Mm(y_linea_media)), false)], is_closed: false });
    
    let mut y_linea = y_linea_inferior_header;
    for _ in 0..=num_filas_a_dibujar {
        layer.add_line(Line { points: vec![(Point::new(Mm(x_inicio_tabla), Mm(y_linea)), false), (Point::new(Mm(x_inicio_tabla + ancho_total_tabla), Mm(y_linea)), false)], is_closed: false });
        y_linea -= alto_fila_estudiante;
    }

    // Líneas Verticales
    let mut x_linea_vert = x_inicio_tabla;
    layer.add_line(Line { points: vec![(Point::new(Mm(x_linea_vert), Mm(y_linea_superior)), false), (Point::new(Mm(x_linea_vert), Mm(y_final_tabla)), false)], is_closed: false });
    x_linea_vert += ancho_col_n;
    layer.add_line(Line { points: vec![(Point::new(Mm(x_linea_vert), Mm(y_linea_superior)), false), (Point::new(Mm(x_linea_vert), Mm(y_final_tabla)), false)], is_closed: false });
    x_linea_vert += ancho_col_nombres;
    
    for _ in 0..asignaturas.len() {
        layer.add_line(Line { points: vec![(Point::new(Mm(x_linea_vert), Mm(y_linea_superior)), false), (Point::new(Mm(x_linea_vert), Mm(y_final_tabla)), false)], is_closed: false });
        
        for i in 1..4 {
            let x_sub_linea = x_linea_vert + i as f32 * ancho_subcol_lapso;
            layer.add_line(Line { points: vec![(Point::new(Mm(x_sub_linea), Mm(y_linea_media)), false), (Point::new(Mm(x_sub_linea), Mm(y_final_tabla)), false)], is_closed: false });
        }
        x_linea_vert += ancho_col_asignatura;
    }
    
    layer.add_line(Line { points: vec![(Point::new(Mm(x_linea_vert), Mm(y_linea_superior)), false), (Point::new(Mm(x_linea_vert), Mm(y_final_tabla)), false)], is_closed: false });
}

// Comando principal de Tauri

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

    // --- Inicio de la generación del PDF ---
    let (doc, page1, layer1) = PdfDocument::new(
        "Acta de Resumen de Calificaciones",
        Mm(279.4), // Ancho de Carta en horizontal
        Mm(215.9), // Alto de Carta en horizontal
        "Layer 1",
    );
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).map_err(|e| e.to_string())?;
    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica).map_err(|e| e.to_string())?;

    dibujar_encabezado(&mut current_layer, &info_encabezado, &font_bold, &font_regular);

    dibujar_tabla_calificaciones(
        &mut current_layer,
        &font_bold,
        &font_regular,
        &asignaturas,
        &estudiantes_con_calificaciones
    );

    // --- Finalización del PDF ---
    let mut buf = BufWriter::new(Vec::new());
    doc.save(&mut buf).map_err(|e| format!("Error guardando el PDF: {}", e))?;
    let pdf_bytes = buf.into_inner().map_err(|e| format!("Error obteniendo bytes del PDF: {}", e))?;
    
    Ok(general_purpose::STANDARD.encode(&pdf_bytes))
}