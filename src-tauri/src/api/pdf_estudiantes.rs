use printpdf::{PdfDocument, Mm, Line, Point, Image, ImageTransform, BuiltinFont, PdfLayerReference, IndirectFontRef};
use std::io::{BufWriter, Cursor};
use ::image::{load, GenericImageView, DynamicImage, imageops::FilterType};
use base64::{engine::general_purpose, Engine as _};
use crate::AppState;
use chrono::Local;
use tauri::State;

#[derive(Debug, Clone)]
pub struct EstudianteInfo {
    pub cedula: i64,
    pub apellidos: String,
    pub nombres: String,
    pub genero: String,
}

#[derive(Debug, Clone)]
pub struct DocenteGuiaInfo {
    pub nombres: String,
    pub apellidos: String,
}

#[derive(Debug, Clone)]
pub struct InfoCurso {
    pub grado: String,
    pub seccion: String,
    pub modalidad: String,
    pub periodo: String,
}

async fn obtener_estudiantes_del_curso(client: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<Vec<EstudianteInfo>, String> {
    let stmt = client.prepare(
        "SELECT e.cedula, e.apellidos, e.nombres, e.genero \
         FROM estudiantes e \
         INNER JOIN historial_grado_estudiantes hge ON e.id = hge.id_estudiante \
         WHERE hge.id_grado_secciones = $1 AND hge.es_actual = true \
         ORDER BY e.cedula ASC"
    ).await.map_err(|e| format!("Error preparando consulta: {}", e))?;

    let rows = client.query(&stmt, &[&id_grado_secciones])
        .await
        .map_err(|e| format!("Error ejecutando consulta: {}", e))?;

    let estudiantes = rows.into_iter().map(|row| {
        EstudianteInfo {
            cedula: row.get("cedula"),
            apellidos: row.get("apellidos"),
            nombres: row.get("nombres"),
            genero: row.get::<_, String>("genero"),
        }
    }).collect();

    Ok(estudiantes)
}

async fn obtener_docente_guia(client: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<DocenteGuiaInfo, String> {
    let stmt = client.prepare(
        "SELECT d.nombres, d.apellidos \
         FROM docentes d \
         INNER JOIN grado_secciones gs ON d.id_docente = gs.id_docente_guia \
         WHERE gs.id_grado_secciones = $1"
    ).await.map_err(|e| format!("Error preparando consulta: {}", e))?;

    let row = client.query_opt(&stmt, &[&id_grado_secciones])
        .await
        .map_err(|e| format!("Error ejecutando consulta: {}", e))?
        .ok_or("No se encontró docente guía")?;

    Ok(DocenteGuiaInfo {
        nombres: row.get("nombres"),
        apellidos: row.get("apellidos"),
    })
}

async fn obtener_info_curso(client: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<InfoCurso, String> {
    let stmt = client.prepare(
        "SELECT g.nombre_grado as grado, s.nombre_seccion as seccion, m.nombre_modalidad as modalidad, p.periodo_escolar as periodo 
         FROM grado_secciones gs 
         INNER JOIN grados g ON gs.id_grado = g.id_grado 
         INNER JOIN secciones s ON gs.id_seccion = s.id_seccion 
         INNER JOIN modalidades m ON gs.id_modalidad = m.id_modalidad 
         INNER JOIN historial_grado_estudiantes hge ON gs.id_grado_secciones = hge.id_grado_secciones 
         INNER JOIN periodos_escolares p ON hge.id_periodo = p.id_periodo 
         WHERE gs.id_grado_secciones = $1 AND hge.es_actual = true 
         LIMIT 1"
    ).await.map_err(|e| format!("Error preparando consulta: {}", e))?;

    let row = client.query_one(&stmt, &[&id_grado_secciones])
        .await
        .map_err(|e| format!("Error ejecutando consulta: {}", e))?;

    Ok(InfoCurso {
        grado: row.get("grado"),
        seccion: row.get("seccion"),
        modalidad: row.get("modalidad"),
        periodo: row.get("periodo"),
    })
}

// Limita las iniciales de asignatura a 2 caracteres
fn iniciales_asignatura(nombre: &str) -> String {
    let nombre = nombre.trim().to_uppercase();
    if nombre == "CIENCIAS DE LA TIERRA" {
        return "Cs".to_string();
    }
    let vacias = ["DE", "Y", "EN", "DEL", "LA", "EL"];
    let palabras: Vec<&str> = nombre.split_whitespace().collect();
    if palabras.len() == 1 {
        return palabras[0][..2.min(palabras[0].len())].to_string().to_uppercase();
    }
    let mut ini = String::new();
    for p in palabras {
        if !vacias.contains(&p) {
            ini.push(p.chars().next().unwrap_or(' '));
        }
    }
    ini.chars().take(2).collect::<String>().to_uppercase()
}

// Consulta las asignaturas del grado/sección
async fn obtener_iniciales_asignaturas(client: &tokio_postgres::Client, id_grado_secciones: i32) -> Result<Vec<String>, String> {
    let stmt = client.prepare(
        "SELECT a.nombre FROM grado_seccion_asignatura_docente gsad \
         INNER JOIN asignaturas a ON gsad.id_asignatura = a.id_asignatura \
         INNER JOIN grado_secciones gs ON gsad.id_grado_secciones = gs.id_grado_secciones \
         INNER JOIN grado_modalidad_asignaturas gma \
            ON gma.id_asignatura = a.id_asignatura \
            AND gma.id_grado = gs.id_grado \
            AND gma.id_modalidad = gs.id_modalidad \
         WHERE gsad.id_grado_secciones = $1 \
         ORDER BY gma.orden"
    ).await.map_err(|e| format!("Error preparando consulta: {}", e))?;
    let rows = client.query(&stmt, &[&id_grado_secciones])
        .await
        .map_err(|e| format!("Error ejecutando consulta: {}", e))?;
    let mut iniciales = Vec::new();
    for row in rows {
        let nombre: String = row.get("nombre");
        iniciales.push(iniciales_asignatura(&nombre));
    }
    Ok(iniciales)
}

// Formatea la cédula con puntos
fn formatear_cedula(cedula: i64) -> String {
    let s = cedula.to_string();
    let len = s.len();
    if len <= 3 {
        return s;
    }
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars().rev() {
        if count == 3 || count == 6 {
            result.push('.');
        }
        result.push(c);
        count += 1;
    }
    result.chars().rev().collect()
}

// Función para procesar logos desde base64
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

#[tauri::command]
pub async fn generar_pdf_estudiantes_curso(
    id_grado_secciones: i32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let client = &state.db.lock().await;
    
    let estudiantes = obtener_estudiantes_del_curso(client, id_grado_secciones).await?;
    let docente = obtener_docente_guia(client, id_grado_secciones).await?;
    let info_curso = obtener_info_curso(client, id_grado_secciones).await?;
    let iniciales_asig = obtener_iniciales_asignaturas(client, id_grado_secciones).await?;
    let n_asig = iniciales_asig.len();

    // 1. PROCESAR LOGO DERECHO (EL OTRO LOGO)
    let (logo_dyn, ancho_logo, alto_logo) = procesar_logo(&state.logo_der, 20.0)?;
    
    let (doc, page1, layer1) = PdfDocument::new("Nómina de Estudiantes", Mm(215.9), Mm(279.4), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let margen_izq = 20.0;
    let margen_der = 20.0;
    let ancho_hoja = 215.9;
    let alto_hoja = 279.4;
    let ancho_tabla = ancho_hoja - margen_izq - margen_der;
    
    let ancho_n_col = 7.0;
    let ancho_cedula = 22.0;
    let ancho_gen = 6.0;
    let ancho_asig = 6.0;
    let ancho_nombres = ancho_tabla - (ancho_n_col + ancho_cedula + ancho_gen + (n_asig as f32) * ancho_asig);

    let filas_por_pagina = 45;
    let altura_celda_datos = 5.0;
    let offset_y = 1.6; // Variable única para el desplazamiento (aumentado)

    // --- FUNCIÓN AUXILIAR PARA DIBUJAR ENCABEZADO COMPLETO ---
    let dibujar_cabecera = |layer: &mut PdfLayerReference, y_start: f32, num_filas_en_pagina: usize| -> f32 {
        let mut y = y_start;
        let y_tope_cabecera = y;

        // Dibujar Logo (el correcto) a la izquierda
        Image::from_dynamic_image(&logo_dyn).add_to_layer(layer.clone(), ImageTransform {
            translate_x: Some(Mm(margen_izq)),
            translate_y: Some(Mm(y_tope_cabecera - alto_logo)),
            dpi: Some(150.0), ..Default::default()
        });

        // Dibujar Membrete centrado en el espacio restante
        let x_texto_start = margen_izq + ancho_logo + 5.0; // 5mm padding
        let ancho_texto_disponible = ancho_hoja - x_texto_start - margen_der;
        let centro_texto_x = x_texto_start + ancho_texto_disponible / 2.0;
        
        layer.use_text("COMPLEJO EDUCATIVO PROFESOR JESUS LOPEZ CASTRO", 12.0, Mm(centro_texto_x - 60.0), Mm(y), &font_bold);
        y -= 6.0;
        layer.use_text("COORDINACIÓN DE REGISTRO Y CONTROL ACADÉMICO", 10.0, Mm(centro_texto_x - 45.0), Mm(y), &font_bold);
        y -= 5.0;
        layer.use_text("SAN JOSÉ DE GUANIPA ESTADO ANZOÁTEGUI", 10.0, Mm(centro_texto_x - 35.0), Mm(y), &font_bold);
        
        y = y_tope_cabecera - alto_logo - 5.0;
        
        // Metadatos del curso
        let texto_periodo = format!("Año Escolar: {}", info_curso.periodo);
        let texto_grado = format!("{} Año Sección: {}", info_curso.grado, info_curso.seccion);
        layer.use_text(&texto_periodo, 10.0, Mm(margen_izq), Mm(y), &font_regular);
        layer.use_text(&texto_grado, 10.0, Mm(margen_izq + 45.0), Mm(y), &font_regular);
        layer.use_text("Código DEA: S1371D010", 10.0, Mm(ancho_hoja - margen_der - 40.0), Mm(y), &font_regular);
        y -= 5.0;

        let nombre_completo = format!("{} {}", docente.nombres, docente.apellidos);
        let nombre_truncado = if nombre_completo.len() > 30 { format!("{}...", &nombre_completo[..27]) } else { nombre_completo };
        layer.use_text("Docente Guía:", 10.0, Mm(margen_izq), Mm(y), &font_regular);
        layer.use_text(&nombre_truncado, 10.0, Mm(margen_izq + 25.0), Mm(y), &font_bold);
        let texto_modalidad = format!("Modalidad: {}", info_curso.modalidad);
        layer.use_text(&texto_modalidad, 10.0, Mm(ancho_hoja - margen_der - 40.0), Mm(y), &font_regular);
        y -= 8.0;

        // Cabecera de la tabla
        let tabla_top = y;
        let altura_celda_header = 5.0;
        let y_header_bottom = y - altura_celda_header;
        let ajuste_vertical_header = altura_celda_header - 1.0;
        let mut x = margen_izq;
        let font_encabezado = 9.0;
        
        layer.use_text("N°", font_encabezado, Mm(x + 2.0), Mm(y_header_bottom + ajuste_vertical_header), &font_bold);
        x += ancho_n_col;
        layer.use_text("Cédula", font_encabezado, Mm(x + 5.0), Mm(y_header_bottom + ajuste_vertical_header), &font_bold);
        x += ancho_cedula;
        layer.use_text("APELLIDOS Y NOMBRES", font_encabezado, Mm(x + 1.5), Mm(y_header_bottom + ajuste_vertical_header), &font_bold);
        x += ancho_nombres;
        layer.use_text("G", font_encabezado, Mm(x + 2.0), Mm(y_header_bottom + ajuste_vertical_header), &font_bold);
        x += ancho_gen;

        for ini in iniciales_asig.iter() {
            layer.use_text(ini, font_encabezado, Mm(x + 1.0), Mm(y_header_bottom + ajuste_vertical_header), &font_bold);
            x += ancho_asig;
        }

        // Bordes de la tabla con ajuste
        let tabla_bottom = tabla_top - altura_celda_header - (num_filas_en_pagina as f32 * altura_celda_datos);
        let mut x_bordes = vec![margen_izq];
        x_bordes.push(margen_izq + ancho_n_col);
        x_bordes.push(margen_izq + ancho_n_col + ancho_cedula);
        x_bordes.push(margen_izq + ancho_n_col + ancho_cedula + ancho_nombres);
        x_bordes.push(margen_izq + ancho_n_col + ancho_cedula + ancho_nombres + ancho_gen);
        for i in 0..=n_asig {
            x_bordes.push(margen_izq + ancho_n_col + ancho_cedula + ancho_nombres + ancho_gen + (i as f32) * ancho_asig);
        }

        layer.set_outline_thickness(0.5);
        for x_borde in x_bordes {
            layer.add_line(Line { points: vec![(Point::new(Mm(x_borde), Mm(tabla_top + offset_y)), false), (Point::new(Mm(x_borde), Mm(tabla_bottom + offset_y)), false)], is_closed: false, ..Default::default() });
        }
        layer.add_line(Line { points: vec![(Point::new(Mm(margen_izq), Mm(tabla_top + offset_y)), false), (Point::new(Mm(margen_izq + ancho_tabla), Mm(tabla_top + offset_y)), false)], is_closed: false, ..Default::default() });
        layer.add_line(Line { points: vec![(Point::new(Mm(margen_izq), Mm(y_header_bottom + offset_y)), false), (Point::new(Mm(margen_izq + ancho_tabla), Mm(y_header_bottom + offset_y)), false)], is_closed: false, ..Default::default() });
        
        y_header_bottom
    };

    let y_start_filas = dibujar_cabecera(&mut current_layer, alto_hoja - 15.0, estudiantes.len().min(filas_por_pagina));
    let mut y = y_start_filas;

    for (idx, estudiante) in estudiantes.iter().enumerate() {
        if idx > 0 && idx % filas_por_pagina == 0 {
            let (page, layer) = doc.add_page(Mm(ancho_hoja), Mm(alto_hoja), "Layer 1");
            current_layer = doc.get_page(page).get_layer(layer);
            let filas_restantes = estudiantes.len() - idx;
            y = dibujar_cabecera(&mut current_layer, alto_hoja - 15.0, filas_restantes.min(filas_por_pagina));
        }

        let font_datos = 8.0;
        let ajuste_vertical_datos = altura_celda_datos - 1.5;
        
        let row_bottom = y - altura_celda_datos;
        
        // Dibujar línea horizontal inferior de la fila con el ajuste
        current_layer.add_line(Line { 
            points: vec![
                (Point::new(Mm(margen_izq), Mm(row_bottom + offset_y)), false), 
                (Point::new(Mm(margen_izq + ancho_tabla), Mm(row_bottom + offset_y)), false)
            ], 
            is_closed: false, ..Default::default() 
        });

        let mut x = margen_izq;
        let text_y_pos = row_bottom + ajuste_vertical_datos;
        current_layer.use_text(&format!("{:02}", idx + 1), font_datos, Mm(x + 2.0), Mm(text_y_pos), &font_regular);
        x += ancho_n_col;
        let cedula_text = formatear_cedula(estudiante.cedula);
        current_layer.use_text(&cedula_text, font_datos, Mm(x + 1.5), Mm(text_y_pos), &font_regular);
        x += ancho_cedula;
        current_layer.use_text(&format!("{} {}", estudiante.apellidos, estudiante.nombres), font_datos, Mm(x + 1.5), Mm(text_y_pos), &font_regular);
        x += ancho_nombres;
        current_layer.use_text(&estudiante.genero, font_datos, Mm(x + 2.0), Mm(text_y_pos), &font_regular);
        x += ancho_gen;
        
        for _ in 0..n_asig {
             x += ancho_asig;
        }

        y = row_bottom;
    }
    
    // Guardar PDF
    let mut buf = BufWriter::new(Vec::new());
    doc.save(&mut buf).map_err(|e| format!("Error guardando PDF: {}", e))?;
    let pdf_bytes = buf.into_inner().map_err(|e| format!("Error obteniendo bytes: {}", e))?;
    let base64_pdf = general_purpose::STANDARD.encode(&pdf_bytes);
    
    Ok(base64_pdf)
}
