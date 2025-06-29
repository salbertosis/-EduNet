use rust_xlsxwriter::*;
use std::path::Path;

#[tauri::command]
pub async fn generar_plantilla_mppe_completa(ruta_salida: String) -> Result<String, String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    
    // Configurar página para hoja OFICIO VERTICAL
    worksheet.set_paper_size(5); // Papel oficio (8.5" x 14")
    worksheet.set_portrait(); // Orientación vertical
    worksheet.set_margins(0.3, 0.3, 0.3, 0.3, 0.3, 0.3); // Márgenes: izq, der, sup, inf, header, footer
    worksheet.set_print_fit_to_pages(1, 1); // Ajustar para que ocupe toda la hoja (1 página de ancho x 1 de alto)
    worksheet.set_print_center_horizontally(true); // Centrar horizontalmente
    worksheet.set_print_center_vertically(true); // Centrar verticalmente
    
    // Configurar área de impresión A1:BP77 (todo el rango de celdas)
    worksheet.set_print_area(0, 0, 76, 67).map_err(|e| e.to_string())?; // Filas 0-76, Columnas 0-67
    
    // Aplicar todas las configuraciones
    aplicar_dimensiones_columnas(worksheet)?;
    aplicar_alturas_filas(worksheet)?;
    aplicar_contenido_plantilla(worksheet)?;
    ocultar_areas_extra(worksheet)?;
    
    // Guardar el archivo
    workbook.save(&ruta_salida).map_err(|e| e.to_string())?;
    Ok(format!("Plantilla MPPE generada exitosamente en: {}", ruta_salida))
}

fn aplicar_dimensiones_columnas(worksheet: &mut Worksheet) -> Result<(), String> {
    // COLUMNAS 0-20 (A-U) - MEDIDAS REALES EXACTAS
    worksheet.set_column_width(0, 2.33).map_err(|e| e.to_string())?; // A
    worksheet.set_column_width(1, 6.11).map_err(|e| e.to_string())?; // B
    worksheet.set_column_width(2, 0.0).map_err(|e| e.to_string())?;  // C - OCULTA
    worksheet.set_column_width(3, 0.63).map_err(|e| e.to_string())?; // D
    worksheet.set_column_width(4, 0.63).map_err(|e| e.to_string())?; // E
    worksheet.set_column_width(5, 0.63).map_err(|e| e.to_string())?; // F
    worksheet.set_column_width(6, 0.5).map_err(|e| e.to_string())?;  // G
    worksheet.set_column_width(7, 0.5).map_err(|e| e.to_string())?;  // H
    worksheet.set_column_width(8, 0.38).map_err(|e| e.to_string())?; // I
    worksheet.set_column_width(9, 0.56).map_err(|e| e.to_string())?; // J
    worksheet.set_column_width(10, 0.5).map_err(|e| e.to_string())?; // K
    worksheet.set_column_width(11, 0.63).map_err(|e| e.to_string())?; // L
    worksheet.set_column_width(12, 1.22).map_err(|e| e.to_string())?; // M
    worksheet.set_column_width(13, 0.0).map_err(|e| e.to_string())?; // N - OCULTA
    worksheet.set_column_width(14, 1.11).map_err(|e| e.to_string())?; // O
    worksheet.set_column_width(15, 1.67).map_err(|e| e.to_string())?; // P
    worksheet.set_column_width(16, 4.56).map_err(|e| e.to_string())?; // Q
    worksheet.set_column_width(17, 1.11).map_err(|e| e.to_string())?; // R
    worksheet.set_column_width(18, 1.22).map_err(|e| e.to_string())?; // S
    worksheet.set_column_width(19, 1.11).map_err(|e| e.to_string())?; // T
    worksheet.set_column_width(20, 2.89).map_err(|e| e.to_string())?; // U
    
    // COLUMNAS 21-30 (V-AD) - MEDIDAS CORREGIDAS SEGÚN TABLA OFICIAL
    worksheet.set_column_width(21, 2.89).map_err(|e| e.to_string())?; // V - CORREGIDO: 2.56 → 2.89
    worksheet.set_column_width(22, 0.75).map_err(|e| e.to_string())?; // W
    worksheet.set_column_width(23, 1.11).map_err(|e| e.to_string())?; // X
    worksheet.set_column_width(24, 1.11).map_err(|e| e.to_string())?; // Y - CORREGIDO: 0.81 → 1.11
    worksheet.set_column_width(25, 1.14).map_err(|e| e.to_string())?; // Z - CORREGIDO: 1.11 → 1.14
    worksheet.set_column_width(26, 0.88).map_err(|e| e.to_string())?; // AA
    worksheet.set_column_width(27, 0.81).map_err(|e| e.to_string())?; // AB
    worksheet.set_column_width(28, 1.11).map_err(|e| e.to_string())?; // AC
    worksheet.set_column_width(29, 1.11).map_err(|e| e.to_string())?; // AD
    
    // COLUMNAS 31-40 (AE-AN) - MEDIDAS REALES EXACTAS
    worksheet.set_column_width(30, 0.88).map_err(|e| e.to_string())?; // AE
    worksheet.set_column_width(31, 0.88).map_err(|e| e.to_string())?; // AF
    worksheet.set_column_width(32, 0.88).map_err(|e| e.to_string())?; // AG
    worksheet.set_column_width(33, 0.88).map_err(|e| e.to_string())?; // AH
    worksheet.set_column_width(34, 1.56).map_err(|e| e.to_string())?; // AI
    worksheet.set_column_width(35, 0.88).map_err(|e| e.to_string())?; // AJ
    worksheet.set_column_width(36, 0.5).map_err(|e| e.to_string())?;  // AK
    worksheet.set_column_width(37, 1.56).map_err(|e| e.to_string())?; // AL
    worksheet.set_column_width(38, 1.22).map_err(|e| e.to_string())?; // AM
    worksheet.set_column_width(39, 0.81).map_err(|e| e.to_string())?; // AN
    
    // COLUMNAS 41-50 (AO-AX) - MEDIDAS REALES EXACTAS
    worksheet.set_column_width(40, 0.88).map_err(|e| e.to_string())?; // AO
    worksheet.set_column_width(41, 2.33).map_err(|e| e.to_string())?; // AP
    worksheet.set_column_width(42, 1.89).map_err(|e| e.to_string())?; // AQ
    worksheet.set_column_width(43, 2.67).map_err(|e| e.to_string())?; // AR
    worksheet.set_column_width(44, 0.0).map_err(|e| e.to_string())?;  // AS
    worksheet.set_column_width(45, 1.67).map_err(|e| e.to_string())?; // AT
    worksheet.set_column_width(46, 0.75).map_err(|e| e.to_string())?; // AU
    worksheet.set_column_width(47, 1.33).map_err(|e| e.to_string())?; // AV
    worksheet.set_column_width(48, 1.33).map_err(|e| e.to_string())?; // AW
    worksheet.set_column_width(49, 0.63).map_err(|e| e.to_string())?; // AX
    
    // COLUMNAS 51-60 (AY-BH) - MEDIDAS CORREGIDAS SEGÚN TABLA OFICIAL
    worksheet.set_column_width(50, 1.89).map_err(|e| e.to_string())?; // AY
    worksheet.set_column_width(51, 0.36).map_err(|e| e.to_string())?; // AZ - CORREGIDO: 0.38 → 0.36
    worksheet.set_column_width(52, 1.78).map_err(|e| e.to_string())?; // BA
    worksheet.set_column_width(53, 1.22).map_err(|e| e.to_string())?; // BB
    worksheet.set_column_width(54, 0.88).map_err(|e| e.to_string())?; // BC
    worksheet.set_column_width(55, 0.75).map_err(|e| e.to_string())?; // BD
    worksheet.set_column_width(56, 1.11).map_err(|e| e.to_string())?; // BE
    worksheet.set_column_width(57, 1.11).map_err(|e| e.to_string())?; // BF
    worksheet.set_column_width(58, 0.81).map_err(|e| e.to_string())?; // BG
    worksheet.set_column_width(59, 1.56).map_err(|e| e.to_string())?; // BH
    
    // COLUMNAS 61-68 FINALES (BI-BP) - MEDIDAS REALES EXACTAS
    worksheet.set_column_width(60, 0.75).map_err(|e| e.to_string())?; // BI
    worksheet.set_column_width(61, 1.78).map_err(|e| e.to_string())?; // BJ
    worksheet.set_column_width(62, 1.11).map_err(|e| e.to_string())?; // BK
    worksheet.set_column_width(63, 2.78).map_err(|e| e.to_string())?; // BL
    worksheet.set_column_width(64, 6.22).map_err(|e| e.to_string())?; // BM
    worksheet.set_column_width(65, 3.89).map_err(|e| e.to_string())?; // BN
    worksheet.set_column_width(66, 4.22).map_err(|e| e.to_string())?; // BO
    worksheet.set_column_width(67, 1.89).map_err(|e| e.to_string())?; // BP
    
    Ok(())
}

fn aplicar_alturas_filas(worksheet: &mut Worksheet) -> Result<(), String> {
    // ALTURAS DE FILAS EXACTAS SEGÚN ANÁLISIS MPPE (77 filas total)
    let alturas = [13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 6.8,
                  13.2, 10.5, 15.8, 18.0, 14.1, 13.2, 13.2, 13.2, 13.2, 13.2,
                  13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2,
                  13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2,
                  13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2,
                  13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2,
                  13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 13.2,
                  13.2, 13.2, 13.2, 13.2, 13.2, 13.2, 10.2]; // 77 filas total
    
    for (i, altura) in alturas.iter().enumerate() {
        worksheet.set_row_height(i as u32, *altura).map_err(|e| e.to_string())?;
    }
    
    // === ALTURA DE FILAS 17 A 51 ===
    for fila in 16..51 {
        worksheet.set_row_height(fila, 14.1).map_err(|e| e.to_string())?;
    }
    
    // === ALTURA DE FILAS 52 A 57 (específicamente 14.1) ===
    for fila in 51..=57 {
        worksheet.set_row_height(fila, 14.1).map_err(|e| e.to_string())?;
    }
    
    // === ALTURAS ESPECÍFICAS FILAS 59-68 ===
    worksheet.set_row_height(58, 14.1).map_err(|e| e.to_string())?; // Fila 59: 14.1
    worksheet.set_row_height(59, 23.1).map_err(|e| e.to_string())?; // Fila 60: 23.1
    
    // Filas 61-64: 14.1
    for fila in 60..64 {
        worksheet.set_row_height(fila, 14.1).map_err(|e| e.to_string())?;
    }
    
    worksheet.set_row_height(64, 12.0).map_err(|e| e.to_string())?; // Fila 65: 12.0
    worksheet.set_row_height(65, 32.3).map_err(|e| e.to_string())?; // Fila 66: 32.3
    worksheet.set_row_height(66, 35.3).map_err(|e| e.to_string())?; // Fila 67: 35.3
    worksheet.set_row_height(67, 13.2).map_err(|e| e.to_string())?; // Fila 68: 13.2
    
    Ok(())
}

fn aplicar_contenido_plantilla(worksheet: &mut Worksheet) -> Result<(), String> {
    // Formatos según parámetros MPPE - TODO EN ARIAL 9
    let formato_titulo = Format::new()
        .set_bold()
        .set_font_size(9)
        .set_align(FormatAlign::Center)
        .set_font_name("Arial")
        .set_underline(FormatUnderline::Single); // SUBRAYADO SIN BORDES
    
    let formato_encabezado = Format::new()
        .set_bold()
        .set_font_size(9)
        .set_font_name("Arial")
        .set_align(FormatAlign::Center); // SIN BORDES PARA EMG
    
    let formato_datos = Format::new()
        .set_font_size(9)
        .set_font_name("Arial");
    
    let formato_negrita = Format::new()
        .set_bold()
        .set_font_size(9)
        .set_font_name("Arial");
    
    let formato_borde_inferior = Format::new()
        .set_font_size(9)
        .set_font_name("Arial")
        .set_border_bottom(FormatBorder::Thin);
    
    let formato_borde_superior = Format::new()
        .set_font_size(9)
        .set_font_name("Arial")
        .set_border_top(FormatBorder::Thin);
    
    let formato_sin_borde_superior = Format::new()
        .set_bold()
        .set_font_size(9)
        .set_font_name("Arial");
    
    let formato_sin_borde_inferior = Format::new()
        .set_font_size(9)
        .set_font_name("Arial");
    
    let formato_bordes_completos = Format::new()
        .set_bold()
        .set_font_size(9)
        .set_font_name("Arial")
        .set_border_top(FormatBorder::Thin)
        .set_border_bottom(FormatBorder::Thin)
        .set_border_left(FormatBorder::Thin)
        .set_border_right(FormatBorder::Thin);
        
    let formato_con_todos_bordes = Format::new()
        .set_font_size(9)
        .set_font_name("Arial")
        .set_border_top(FormatBorder::Thin)
        .set_border_bottom(FormatBorder::Thin)
        .set_border_left(FormatBorder::Thin)
        .set_border_right(FormatBorder::Thin);
    
    // ÁREAS COMBINADAS SEGÚN ANÁLISIS MPPE
    let formato_logo = Format::new()
        .set_align(FormatAlign::Center)
        .set_font_size(9)
        .set_font_name("Arial");
    
    // Área combinada: $A$1:$AH$3 (Logo del Ministerio) - SIN BORDES
    worksheet.merge_range(0, 0, 2, 33, "", &formato_logo).map_err(|e| e.to_string())?; // A1:AH3
    
    // INSERTAR LA IMAGEN DEL LOGO DEL MINISTERIO
    // Probar diferentes rutas posibles
    let rutas_posibles = [
        "src-tauri/imagenes/ministerio_resumen.png",
        "imagenes/ministerio_resumen.png", 
        "./src-tauri/imagenes/ministerio_resumen.png",
        "./imagenes/ministerio_resumen.png"
    ];
    
    let mut imagen_cargada = false;
    for ruta_logo in &rutas_posibles {
        if Path::new(ruta_logo).exists() {
            match Image::new(ruta_logo) {
                                 Ok(mut imagen) => {
                     // Ajustar medidas: alto reducido 0.2mm
                     // Ancho: 0.887 (sin cambio), Alto: 1.148 - ajuste ≈ 1.130
                     imagen.set_scale_width(0.887);
                     imagen.set_scale_height(1.130);
                    // Posicionar en A1 sin offset para empezar
                    if let Err(e) = worksheet.insert_image(0, 0, &imagen) {
                        println!("Error insertando imagen: {}", e);
                    } else {
                        println!("✅ Logo insertado exitosamente desde: {}", ruta_logo);
                        imagen_cargada = true;
                        break;
                    }
                },
                Err(e) => {
                    println!("Error cargando imagen desde {}: {}", ruta_logo, e);
                }
            }
        } else {
            println!("❌ No existe el archivo: {}", ruta_logo);
        }
    }
    
    if !imagen_cargada {
        // Si no se pudo cargar ninguna imagen, mostrar texto alternativo
        println!("⚠️ No se pudo cargar el logo, usando texto alternativo");
        worksheet.write_string_with_format(1, 16, "MINISTERIO DEL PODER POPULAR PARA LA EDUCACIÓN", &formato_encabezado).map_err(|e| e.to_string())?;
    }
    
    // AO1: "RESUMEN FINAL DEL RENDIMIENTO ESTUDIANTIL" - SUBRAYADO Y SIN BORDES
    worksheet.merge_range(0, 40, 0, 67, "RESUMEN FINAL DEL RENDIMIENTO ESTUDIANTIL", &formato_titulo).map_err(|e| e.to_string())?; // AO1:BP1
    
    // AO2: "Código del Formato: EMG" - SIN BORDES
    worksheet.merge_range(1, 40, 1, 67, "Código del Formato: EMG", &formato_encabezado).map_err(|e| e.to_string())?; // AO2:BP2
    
    // AM3: "I. Año Escolar:" - EN NEGRITA SIN BORDES
    worksheet.write_string_with_format(2, 38, "I. Año Escolar:", &formato_negrita).map_err(|e| e.to_string())?; // AM3
    
    // Combinar AT3:BP3 - SOLO BORDE INFERIOR
    worksheet.merge_range(2, 45, 2, 67, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // AT3:BP3
    
    // AM4: "Tipo de Evaluación:" - EN NEGRITA SIN BORDES
    worksheet.write_string_with_format(3, 38, "Tipo de Evaluación:", &formato_negrita).map_err(|e| e.to_string())?; // AM4
    
    // Combinar AU4:BC4 - SOLO BORDE INFERIOR
    worksheet.merge_range(3, 46, 3, 54, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // AU4:BC4
    
    // BD4: "Mes y Año:" - EN NEGRITA SIN BORDES
    worksheet.write_string_with_format(3, 55, "Mes y Año:", &formato_negrita).map_err(|e| e.to_string())?; // BD4
    
    // Combinar BJ4:BP4 - SOLO BORDE INFERIOR
    worksheet.merge_range(3, 61, 3, 67, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // BJ4:BP4
    
    // A5: "II. Datos de la Institución Educativa:" - SIN BORDE SUPERIOR
    worksheet.merge_range(4, 0, 4, 34, "II. Datos de la Institución Educativa:", &formato_sin_borde_superior).map_err(|e| e.to_string())?; // A5:AI5
    
    // A6: "Código de la institución Educativa:" - SIN BORDE INFERIOR
    worksheet.write_string_with_format(5, 0, "Código de la institución Educativa:", &formato_sin_borde_inferior).map_err(|e| e.to_string())?; // A6
    
    // Combinar R6:AC6 - SOLO BORDE INFERIOR
    worksheet.merge_range(5, 17, 5, 28, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // R6:AC6
    
    // AD6: "Denominación y Epónimo:" - SIN BORDES
    worksheet.write_string_with_format(5, 29, "Denominación y Epónimo:", &formato_datos).map_err(|e| e.to_string())?; // AD6
    
    // AQ6:BP6 - SOLO BORDE INFERIOR
    worksheet.merge_range(5, 42, 5, 67, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // AQ6:BP6
    
    // A7: "Dirección:" - SIN BORDE INFERIOR
    worksheet.write_string_with_format(6, 0, "Dirección:", &formato_sin_borde_inferior).map_err(|e| e.to_string())?; // A7
    worksheet.merge_range(6, 4, 6, 47, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // E7:AV7 - SOLO BORDE INFERIOR
    
    // BG7: "Teléfono:" - SIN BORDES
    worksheet.write_string_with_format(6, 58, "Teléfono:", &formato_datos).map_err(|e| e.to_string())?; // BG7
    worksheet.merge_range(6, 62, 6, 67, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // BK7:BP7 - SOLO BORDE INFERIOR
    
    // A8: "Municipio:" - SIN BORDE INFERIOR
    worksheet.write_string_with_format(7, 0, "Municipio:", &formato_sin_borde_inferior).map_err(|e| e.to_string())?; // A8
    worksheet.merge_range(7, 4, 7, 20, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // E8:U8 - SOLO BORDE INFERIOR
    
    // V8:AD8 "Entidad Federal:" - SIN BORDES
    worksheet.merge_range(7, 21, 7, 29, "Entidad Federal:", &formato_datos).map_err(|e| e.to_string())?; // V8:AD8
    
    // AP8:AV8 "CDCEE:" - SOLO BORDE SUPERIOR
    worksheet.merge_range(7, 41, 7, 47, "CDCEE:", &formato_borde_superior).map_err(|e| e.to_string())?; // AP8:AV8
    
    // AW8:BP8 - SOLO BORDE INFERIOR (campo de entrada)
    worksheet.merge_range(7, 48, 7, 67, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // AW8:BP8
    
    // A9: "Director(a):" - SIN BORDE INFERIOR
    worksheet.write_string_with_format(8, 0, "Director(a):", &formato_sin_borde_inferior).map_err(|e| e.to_string())?; // A9
    worksheet.merge_range(8, 4, 8, 40, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // E9:AO9 - SOLO BORDE INFERIOR
    
    // AP9: "Cédula de Identidad:" - SIN BORDES
    worksheet.write_string_with_format(8, 41, "Cédula de Identidad:", &formato_datos).map_err(|e| e.to_string())?; // AP9
    worksheet.merge_range(8, 49, 8, 67, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // AX9:BP9 - SOLO BORDE INFERIOR
    
    // A10:BP10 - BORDE INFERIOR EN TODA LA FILA 10
    worksheet.merge_range(9, 0, 9, 67, "", &formato_borde_inferior).map_err(|e| e.to_string())?; // A10:BP10 - SOLO BORDE INFERIOR
    
    // FILA 11 - SECCIONES COMBINADAS CON BORDES COMPLETOS
    // A11:AW11 - "III. Identificación del Estudiante:" EN NEGRITA CON BORDES
    worksheet.merge_range(10, 0, 10, 48, "III. Identificación del Estudiante:", &formato_bordes_completos).map_err(|e| e.to_string())?; // A11:AW11
    
    // AX11:BP11 - "IV. Resumen Final del Rendimiento:" EN NEGRITA CON BORDES
    worksheet.merge_range(10, 49, 10, 67, "IV. Resumen Final del Rendimiento:", &formato_bordes_completos).map_err(|e| e.to_string())?; // AX11:BP11
    
    // === BORDES COMPLETOS FILA 12 (refuerzo) ===
    for col in 0..=67 {
        worksheet.write_with_format(11, col, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?;
    }

    // === FILAS 18 A 51 (índices 17 a 50) ===
    let mut numero = 2;
    for fila in 17..51 {
        // Columna A: enumeración (con ceros a la izquierda si es menor a 10)
        let num_str = if numero < 10 {
            format!("0{}", numero)
        } else {
            format!("{}", numero)
        };
        worksheet.write_with_format(fila, 0, &num_str, &formato_con_todos_bordes).map_err(|e| e.to_string())?;
        // El resto de las columnas (B a BP) con bordes completos
        for col in 1..=67 {
            worksheet.write_with_format(fila, col, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?;
        }
        // Rangos combinados igual que la fila 17
        worksheet.merge_range(fila, 1, fila, 12, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // B:M
        worksheet.merge_range(fila, 14, fila, 22, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // O:W
        worksheet.merge_range(fila, 23, fila, 33, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // X:AH
        worksheet.merge_range(fila, 34, fila, 40, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AI:AO
        worksheet.merge_range(fila, 45, fila, 46, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AT:AU
        worksheet.merge_range(fila, 47, fila, 48, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AV:AW
        worksheet.merge_range(fila, 49, fila, 50, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX:AY
        worksheet.merge_range(fila, 51, fila, 52, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AZ:BA
        worksheet.merge_range(fila, 53, fila, 54, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BB:BC
        worksheet.merge_range(fila, 55, fila, 56, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BD:BE
        worksheet.merge_range(fila, 57, fila, 58, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BF:BG
        worksheet.merge_range(fila, 59, fila, 60, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH:BI
        worksheet.merge_range(fila, 61, fila, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BJ:BK
        worksheet.write_with_format(fila, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BL
        worksheet.merge_range(fila, 65, fila, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BN:BP
        numero += 1;
    }
    
    // COMBINACIONES VERTICALES FILAS 12-16 CON BORDES COMPLETOS
    // B12:M16 - Primer bloque vertical (columnas 1-12, filas 11-15)
    worksheet.merge_range(11, 1, 15, 12, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // B12:M16
    
    // O12:W16 - Segundo bloque vertical (columnas 14-22, filas 11-15)
    worksheet.merge_range(11, 14, 15, 22, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // O12:W16
    
    // X12:AH16 - Tercer bloque vertical (columnas 23-33, filas 11-15)
    worksheet.merge_range(11, 23, 15, 33, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // X12:AH16
    
    // AI12:AO16 - Cuarto bloque vertical (columnas 34-40, filas 11-15)
    worksheet.merge_range(11, 34, 15, 40, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AI12:AO16
    
    // COLUMNAS ADICIONALES VERTICALES FILAS 12-16
    // A12:A16 - Columna A vertical (columna 0, filas 11-15)
    worksheet.merge_range(11, 0, 15, 0, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // A12:A16
    
    // AP12:AP16 - Columna AP vertical (columna 41, filas 11-15)
    worksheet.merge_range(11, 41, 15, 41, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AP12:AP16
    
    // AQ12:AQ16 - Columna AQ vertical (columna 42, filas 11-15)
    worksheet.merge_range(11, 42, 15, 42, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AQ12:AQ16
    
    // COMBINACIONES AR:AW ACTUALIZADAS
    // AR12:AW14 - Filas 12,13,14 combinadas (columnas 43-48, filas 11-13)
    worksheet.merge_range(11, 43, 13, 48, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AR12:AW14
    
    // FILAS 15-16: ESTRUCTURA COMPLETA CORREGIDA
    // AR15:AR16 - Columna AR combinada para filas 15-16
    worksheet.merge_range(14, 43, 15, 43, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AR15:AR16
    
    // AT15:AU16 - Columnas AT y AU combinadas para filas 15-16
    worksheet.merge_range(14, 45, 15, 46, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AT15:AU16
    
    // AV15:AW16 - Columnas AV y AW combinadas para filas 15-16
    worksheet.merge_range(14, 47, 15, 48, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AV15:AW16
    
    // SECCIÓN AX:BL - FILAS 12-13 COMBINADAS
    worksheet.merge_range(11, 49, 12, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX12:BL13
    // AX14:BL14 - Bloque para columnas AX hasta BL en fila 14
    worksheet.merge_range(13, 49, 13, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX14:BL14
    // BM12:BP14 - Bloque rectangular para columnas BM hasta BP en filas 12, 13 y 14
    worksheet.merge_range(11, 63, 13, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BM12:BP14
    
    // BN15:BP16 - Bloque combinado para columnas BN hasta BP en filas 15 y 16
    worksheet.merge_range(14, 65, 15, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BN15:BP16
    
    // BL15 y BL16 - Solo bordes, sin combinar
    worksheet.write_with_format(14, 62, "8", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BL15
    worksheet.write_with_format(15, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BL16
    
    // BM15 y BM16 - Bordes completos
    worksheet.write_with_format(14, 63, "9", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BM15
    worksheet.write_with_format(15, 63, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BM16

    // BN15 y BN16 - Bordes completos
    worksheet.write_with_format(14, 64, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BN15
    worksheet.write_with_format(15, 64, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BN16
    
    // FILAS 15-16: COMBINACIONES DE A 2 COLUMNAS DESDE AX HASTA BK
    // FILA 15 - Combinaciones de a 2 columnas
    worksheet.merge_range(14, 49, 14, 50, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX15:AY15
    worksheet.merge_range(14, 51, 14, 52, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AZ15:BA15
    worksheet.merge_range(14, 53, 14, 54, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BB15:BC15
    worksheet.merge_range(14, 55, 14, 56, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BD15:BE15
    worksheet.merge_range(14, 57, 14, 58, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BF15:BG15
    worksheet.merge_range(14, 59, 14, 60, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH15:BI15
    worksheet.merge_range(14, 61, 14, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BJ15:BK15
    
    // FILA 16 - Combinaciones de a 2 columnas (misma estructura que fila 15)
    worksheet.merge_range(15, 49, 15, 50, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX16:AY16
    worksheet.merge_range(15, 51, 15, 52, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AZ16:BA16
    worksheet.merge_range(15, 53, 15, 54, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BB16:BC16
    worksheet.merge_range(15, 55, 15, 56, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BD16:BE16
    worksheet.merge_range(15, 57, 15, 58, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BF16:BG16
    worksheet.merge_range(15, 59, 15, 60, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH16:BI16
    worksheet.merge_range(15, 61, 15, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BJ16:BK16
    
    // === FILA 17 ===
    // Escribir '01' en la columna A y aplicar bordes a toda la fila
    worksheet.write_with_format(16, 0, "01", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // A17
    for col in 1..=67 {
        worksheet.write_with_format(16, col, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?;
    }
    // Combinaciones de columnas igual que en la fila 16
    worksheet.merge_range(16, 49, 16, 50, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX17:AY17
    worksheet.merge_range(16, 51, 16, 52, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AZ17:BA17
    worksheet.merge_range(16, 53, 16, 54, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BB17:BC17
    worksheet.merge_range(16, 55, 16, 56, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BD17:BE17
    worksheet.merge_range(16, 57, 16, 58, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BF17:BG17
    worksheet.merge_range(16, 59, 16, 60, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH17:BI17
    worksheet.merge_range(16, 61, 16, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BJ17:BK17
    worksheet.merge_range(16, 65, 16, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BN17:BP17
    // BL17 solo bordes, sin combinar
    worksheet.write_with_format(16, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BL17
    // Combinaciones adicionales según la imagen
    worksheet.merge_range(16, 1, 16, 12, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // B17:M17
    worksheet.merge_range(16, 14, 16, 22, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // O17:W17
    worksheet.merge_range(16, 23, 16, 33, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // X17:AH17
    worksheet.merge_range(16, 34, 16, 40, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AI17:AO17
    worksheet.merge_range(16, 45, 16, 46, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AT17:AU17
    worksheet.merge_range(16, 47, 16, 48, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AV17:AW17
    
    // === FORMATOS DE TEXTO ACTUALIZADOS ===
    let formato_centrado_negrita = Format::new()
        .set_bold()
        .set_font_size(9)
        .set_font_name("Arial")
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap();

    let formato_centrado = Format::new()
        .set_font_size(9)
        .set_font_name("Arial")
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap();

    // NUEVO: Formato para texto rotado hacia arriba (DÍA, MES, AÑO)
    let formato_rotado_arriba = Format::new()
        .set_font_size(9)
        .set_font_name("Arial")
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_rotation(90);

    // NUEVO: Formato para SEXO rotado hacia abajo con interlineado reducido
    let formato_sexo_rotado = Format::new()
        .set_font_size(8)
        .set_font_name("Arial")
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_rotation(90)
        .set_text_wrap();

    // === ENCABEZADOS PRINCIPALES ===
    worksheet.write_string_with_format(11, 0, "N°", &formato_centrado_negrita).map_err(|e| e.to_string())?; // A12:A16
    worksheet.write_string_with_format(11, 1, "Cédula de Identidad", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B12:M16
    worksheet.write_string_with_format(11, 14, "Apellidos", &formato_centrado_negrita).map_err(|e| e.to_string())?; // O12:W16
    worksheet.write_string_with_format(11, 23, "Nombres", &formato_centrado_negrita).map_err(|e| e.to_string())?; // X12:AH16
    worksheet.write_string_with_format(11, 34, "Lugar de Nacimiento", &formato_centrado_negrita).map_err(|e| e.to_string())?; // AI12:AO16
    worksheet.write_string_with_format(11, 41, "EF", &formato_centrado_negrita).map_err(|e| e.to_string())?; // AP12:AP16
    worksheet.write_string_with_format(11, 42, "SEXO", &formato_sexo_rotado).map_err(|e| e.to_string())?; // AQ12:AQ16
    worksheet.write_string_with_format(11, 43, "FECHA DE\nNACIMIENTO", &formato_centrado_negrita).map_err(|e| e.to_string())?; // AR12:AW14
    worksheet.write_string_with_format(11, 49, "ÁREAS DE FORMACIÓN", &formato_centrado_negrita).map_err(|e| e.to_string())?; // AX12:BL13
    worksheet.write_string_with_format(13, 49, "ÁREA COMÚN", &formato_centrado_negrita).map_err(|e| e.to_string())?; // AX14:BL14
    worksheet.write_string_with_format(11, 63, "PARTICIPACIÓN EN GRUPOS DE CREACIÓN, RECREACIÓN Y PRODUCCIÓN", &formato_centrado_negrita).map_err(|e| e.to_string())?; // BM12:BP14
    worksheet.write_string_with_format(14, 65, "GRUPO", &formato_centrado_negrita).map_err(|e| e.to_string())?; // BN15:BP16

    // === SUBENCABEZADOS FECHA DE NACIMIENTO ===
    worksheet.write_string_with_format(14, 43, "DÍA", &formato_rotado_arriba).map_err(|e| e.to_string())?; // AR15:AR16
    worksheet.write_string_with_format(14, 45, "MES", &formato_rotado_arriba).map_err(|e| e.to_string())?; // AT15:AU16
    worksheet.write_string_with_format(14, 47, "AÑO", &formato_rotado_arriba).map_err(|e| e.to_string())?; // AV15:AW16

    // === // SUBENCABEZADOS ÁREA COMÚN ===
worksheet.write_string_with_format(14, 49, "1", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX15:AY16
worksheet.write_string_with_format(14, 51, "2", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AZ15:BA16
worksheet.write_string_with_format(14, 53, "3", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BB15:BC16
worksheet.write_string_with_format(14, 55, "4", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BD15:BE16
worksheet.write_string_with_format(14, 57, "5", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BF15:BG16
worksheet.write_string_with_format(14, 59, "6", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH15:BI16
worksheet.write_string_with_format(14, 61, "7", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BJ15:BK16
worksheet.write_string_with_format(14, 63, "8", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BL15:BM16
worksheet.write_string_with_format(14, 64, "9", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BM15:BN16

    // === SUBENCABEZADOS ÁREA COMÚN (NOMBRES) ===
    worksheet.write_string_with_format(15, 49, "CA", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX16:AY16
    worksheet.write_string_with_format(15, 51, "ILE", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AZ16:BA16
    worksheet.write_string_with_format(15, 53, "MA", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BB16:BC16
    worksheet.write_string_with_format(15, 55, "EF", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BD16:BE16
    worksheet.write_string_with_format(15, 57, "AP", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BF16:BG16
    worksheet.write_string_with_format(15, 59, "CN", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH16:BI16
    worksheet.write_string_with_format(15, 61, "GHC", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BJ16:BK16
    worksheet.write_string_with_format(15, 63, "OC", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BL16
    worksheet.write_string_with_format(15, 64, "PGCRP", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BM16

    // === BORDES COMPLETOS FILAS 52-55 ===
    for fila in 51..=54 {
        for col in 0..=67 {
            worksheet.write_with_format(fila, col, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?;
        }
    }

    // === BLOQUE VERTICAL A52:W56 ===
    worksheet.merge_range(51, 0, 55, 22, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // A52:W56
    // === BLOQUES HORIZONTALES X52:AW52, ..., X56:AW56 ===
    for fila in 51..=55 {
        worksheet.merge_range(fila, 23, fila, 48, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?;
    }

    // === COMBINACIONES AX:BP EN FILAS 52 A 56 IGUAL QUE FILA 51 ===
    for fila in 51..=55 {
        worksheet.merge_range(fila, 49, fila, 50, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX:AY
        worksheet.merge_range(fila, 51, fila, 52, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AZ:BA
        worksheet.merge_range(fila, 53, fila, 54, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BB:BC
        worksheet.merge_range(fila, 55, fila, 56, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BD:BE
        worksheet.merge_range(fila, 57, fila, 58, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BF:BG
        worksheet.merge_range(fila, 59, fila, 60, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH:BI
        worksheet.merge_range(fila, 61, fila, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BJ:BK
        worksheet.write_with_format(fila, 62, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BL
        worksheet.write_with_format(fila, 63, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BM
        worksheet.merge_range(fila, 65, fila, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BN:BP
    }

    // === COMBINACIONES VERTICALES FILAS 57 Y 58 (sin BH58:BP58 para evitar solapamiento) ===
    worksheet.merge_range(56, 20, 57, 35, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // U57:AJ58
    worksheet.merge_range(56, 36, 57, 48, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AK57:AW58
    worksheet.merge_range(56, 49, 57, 58, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX57:BG58
    worksheet.merge_range(56, 59, 56, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH57:BP57

    // === COMBINACIONES REPETIDAS DE FILA 59 A 67 ===
    for fila in 58..=66 {
        // NO combinar columna B (iniciales) - debe estar separada
        worksheet.write_with_format(fila, 1, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // Columna B individual
        worksheet.merge_range(fila, 3, fila, 19, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // D:T (nombres de asignaturas)
        worksheet.merge_range(fila, 20, fila, 35, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // U:AJ (nombres profesores)
        worksheet.merge_range(fila, 36, fila, 48, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AK:AW (cédula)
        worksheet.merge_range(fila, 49, fila, 58, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AX:BG (firma)
    }

    // === COMBINAR A57:T57 Y A58:T58 ===
    worksheet.merge_range(56, 0, 56, 19, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // A57:T57
    worksheet.merge_range(57, 0, 57, 19, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // A58:T58

    // === BORDES COMPLETOS FILAS 57 A 67 ===
    for fila in 56..=66 {
        for col in 0..=67 {
            worksheet.write_with_format(fila, col, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?;
        }
    }

    // === COMBINAR BH:BP EN FILAS 58 A 64 (no incluir 65-66) ===
    for fila in 57..=64 {
        worksheet.merge_range(fila, 59, fila, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?;
    }

    // === COMBINACIONES ESPECÍFICAS FILAS 66-67 ===
    // BH66:BM66 (BH=59, BI=60, BJ=61, BK=62, BL=63, BM=64) y BN66:BP66 (BN=65, BO=66, BP=67)
    worksheet.merge_range(65, 59, 65, 64, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH66:BM66 (columnas 59-64)
    worksheet.merge_range(65, 65, 65, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BN66:BP66 (columnas 65-67)
    
    // Repetir en fila 67
    worksheet.merge_range(66, 59, 66, 64, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BH67:BM67 (columnas 59-64)
    worksheet.merge_range(66, 65, 66, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BN67:BP67 (columnas 65-67)

    // === TEXTOS FILAS 52-56 ===
    worksheet.write_string_with_format(51, 23, "Inscritos", &formato_centrado_negrita).map_err(|e| e.to_string())?; // X52
    worksheet.write_string_with_format(52, 23, "Inasistentes", &formato_centrado_negrita).map_err(|e| e.to_string())?; // X53
    worksheet.write_string_with_format(53, 0, "Total de Áreas de Formación", &formato_centrado_negrita).map_err(|e| e.to_string())?; // A54
    worksheet.write_string_with_format(53, 23, "Aprobados", &formato_centrado_negrita).map_err(|e| e.to_string())?; // X54
    worksheet.write_string_with_format(54, 23, "No Aprobados", &formato_centrado_negrita).map_err(|e| e.to_string())?; // X55
    worksheet.write_string_with_format(55, 23, "No Cursantes", &formato_centrado_negrita).map_err(|e| e.to_string())?; // X56

    // === TEXTOS FILA 57 ===
    worksheet.write_string_with_format(56, 0, "V. Profesores por Áreas:", &formato_negrita).map_err(|e| e.to_string())?; // A57
    worksheet.write_string_with_format(56, 20, "Apellidos y Nombres del Profesor", &formato_centrado_negrita).map_err(|e| e.to_string())?; // U57
    worksheet.write_string_with_format(56, 36, "Cédula de Identidad", &formato_centrado_negrita).map_err(|e| e.to_string())?; // AK57
    worksheet.write_string_with_format(56, 49, "Firma", &formato_centrado_negrita).map_err(|e| e.to_string())?; // AX57
    worksheet.write_string_with_format(56, 59, "VI. Identificación del Curso:", &formato_negrita).map_err(|e| e.to_string())?; // BH57

    // === TEXTOS TABLA PROFESORES FILAS 58-66 ===
    worksheet.write_string_with_format(57, 0, "N°", &formato_centrado_negrita).map_err(|e| e.to_string())?; // A58
    worksheet.write_string_with_format(57, 3, "Áreas de Formación", &formato_centrado_negrita).map_err(|e| e.to_string())?; // D58

    // Filas de áreas de formación - INICIALES EN COLUMNA B
    worksheet.write_string_with_format(58, 0, "1", &formato_centrado).map_err(|e| e.to_string())?; // A59
    worksheet.write_string_with_format(58, 1, "CA", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B59
    worksheet.write_string_with_format(58, 3, "Castellano", &formato_datos).map_err(|e| e.to_string())?; // D59
    worksheet.write_string_with_format(58, 59, "PLAN DE ESTUDIO:", &formato_negrita).map_err(|e| e.to_string())?; // BH59

    worksheet.write_string_with_format(59, 0, "2", &formato_centrado).map_err(|e| e.to_string())?; // A60
    worksheet.write_string_with_format(59, 1, "ILE", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B60
    worksheet.write_string_with_format(59, 3, "Inglés y otras Lenguas\nExtranjeras", &formato_datos).map_err(|e| e.to_string())?; // D60
    worksheet.write_string_with_format(59, 59, "EDUCACIÓN MEDIA GENERAL", &formato_datos).map_err(|e| e.to_string())?; // BH60

    worksheet.write_string_with_format(60, 0, "3", &formato_centrado).map_err(|e| e.to_string())?; // A61
    worksheet.write_string_with_format(60, 1, "MA", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B61
    worksheet.write_string_with_format(60, 3, "Matemáticas", &formato_datos).map_err(|e| e.to_string())?; // D61
    worksheet.write_string_with_format(60, 59, "CÓDIGO:", &formato_negrita).map_err(|e| e.to_string())?; // BH61

    worksheet.write_string_with_format(61, 0, "4", &formato_centrado).map_err(|e| e.to_string())?; // A62
    worksheet.write_string_with_format(61, 1, "EF", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B62
    worksheet.write_string_with_format(61, 3, "Educación Física", &formato_datos).map_err(|e| e.to_string())?; // D62
    worksheet.write_string_with_format(61, 59, "31059", &formato_datos).map_err(|e| e.to_string())?; // BH62

    worksheet.write_string_with_format(62, 0, "5", &formato_centrado).map_err(|e| e.to_string())?; // A63
    worksheet.write_string_with_format(62, 1, "AP", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B63
    worksheet.write_string_with_format(62, 3, "Arte y Patrimonio", &formato_datos).map_err(|e| e.to_string())?; // D63
    worksheet.write_string_with_format(62, 59, "AÑO CURSADO", &formato_negrita).map_err(|e| e.to_string())?; // BH63

    worksheet.write_string_with_format(63, 0, "6", &formato_centrado).map_err(|e| e.to_string())?; // A64
    worksheet.write_string_with_format(63, 1, "CN", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B64
    worksheet.write_string_with_format(63, 3, "Ciencias Naturales", &formato_datos).map_err(|e| e.to_string())?; // D64
    worksheet.write_string_with_format(63, 59, "PRIMERO", &formato_datos).map_err(|e| e.to_string())?; // BH64

    worksheet.write_string_with_format(64, 0, "7", &formato_centrado).map_err(|e| e.to_string())?; // A65
    worksheet.write_string_with_format(64, 1, "GHC", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B65
    worksheet.write_string_with_format(64, 3, "Geografía, Historia y\nCiudadanía", &formato_datos).map_err(|e| e.to_string())?; // D65
    worksheet.write_string_with_format(64, 59, "SECCIÓN", &formato_negrita).map_err(|e| e.to_string())?; // BH65

    worksheet.write_string_with_format(65, 0, "8", &formato_centrado).map_err(|e| e.to_string())?; // A66
    worksheet.write_string_with_format(65, 1, "OC", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B66
    worksheet.write_string_with_format(65, 3, "Orientación y Convivencia", &formato_datos).map_err(|e| e.to_string())?; // D66
    worksheet.write_string_with_format(65, 59, "A", &formato_datos).map_err(|e| e.to_string())?; // BH66

    worksheet.write_string_with_format(66, 0, "9", &formato_centrado).map_err(|e| e.to_string())?; // A67
    worksheet.write_string_with_format(66, 1, "PGCRP", &formato_centrado_negrita).map_err(|e| e.to_string())?; // B67
    worksheet.write_string_with_format(66, 3, "Participación en Grupos de\nCreación, Recreación y\nProducción", &formato_datos).map_err(|e| e.to_string())?; // D67
    
    // Texto de estudiantes en las filas 65-66 en las combinaciones correctas
    worksheet.write_string_with_format(65, 59, "N° DE ESTUDIANTES\nPOR SECCIÓN", &formato_centrado_negrita).map_err(|e| e.to_string())?; // BH66:BM66
    worksheet.write_string_with_format(65, 65, "N° DE ESTUDIANTES\nEN ESTA PÁGINA", &formato_centrado_negrita).map_err(|e| e.to_string())?; // BN66:BP66
    
    // Eliminar el texto duplicado de la fila 67
    // worksheet.write_string_with_format(66, 59, "N° DE ESTUDIANTES\nEN ESTA ACTA", &formato_centrado_negrita).map_err(|e| e.to_string())?; // BH67

    // === COMBINAR FILA 69 COMPLETA A69:BP69 ===
    worksheet.merge_range(68, 0, 68, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // A69:BP69

    // === COMBINACIONES ESPECÍFICAS FILAS 70-77 ===
    // A70:V70 hasta A77:V77 (filas 69-76, columnas 0-21)
    for fila in 69..=76 {
        worksheet.merge_range(fila, 0, fila, 21, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // A:V
    }
    
    // W70:AM77 (filas 69-76, columnas 22-38) - UNA SOLA COMBINACIÓN VERTICAL
    worksheet.merge_range(69, 22, 76, 38, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // W70:AM77
    
    // AN70:BE70 hasta AN77:BE77 (filas 69-76, columnas 39-56)
    for fila in 69..=76 {
        worksheet.merge_range(fila, 39, fila, 56, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // AN:BE
    }
    
    // BF70:BP77 (filas 69-76, columnas 57-67) - UNA SOLA COMBINACIÓN VERTICAL
    worksheet.merge_range(69, 57, 76, 67, "", &formato_con_todos_bordes).map_err(|e| e.to_string())?; // BF70:BP77

    // === SECCIÓN FINAL: OBSERVACIONES, REMISIÓN Y RECEPCIÓN ===
    // Observaciones
    worksheet.merge_range(67, 0, 67, 67, "VII. Observaciones:", &formato_bordes_completos).map_err(|e| e.to_string())?;

    Ok(())
}

fn ocultar_areas_extra(worksheet: &mut Worksheet) -> Result<(), String> {
    // Ocultar columnas después de BP (68 en adelante)
    for i in 68..256 {
        worksheet.set_column_width(i, 0.0).map_err(|e| e.to_string())?;
    }
    
    // Ocultar filas después de la 77 (77 en adelante)
    for i in 77..200 {
        worksheet.set_row_height(i, 0.0).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn convertir_plantilla_mppe_a_pdf() -> Result<String, String> {
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;
    
    let timestamp = chrono::Utc::now().timestamp();
    let ruta_pdf = format!("plantilla_mppe_{}.pdf", timestamp);
    
    // Crear documento PDF con tamaño oficio vertical
    let (doc, page1, layer1) = PdfDocument::new(
        "Plantilla MPPE - Resumen Final del Rendimiento Estudiantil", 
        Mm(215.9), // Ancho oficio
        Mm(355.6), // Alto oficio largo  
        "Plantilla MPPE"
    );
    
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let current_layer = doc.get_page(page1).get_layer(layer1);
    
    // === ENCABEZADO OFICIAL ===
    let alto_pagina = Mm(355.6);
    let mut y_actual = alto_pagina - Mm(20.0); // Empezar desde arriba con margen
    
    // Logo y título principal (simulado - en Excel tendríamos imagen)
    current_layer.use_text("REPÚBLICA BOLIVARIANA DE VENEZUELA", 8.0, Mm(70.0), y_actual, &font);
    y_actual = y_actual - Mm(5.0);
    current_layer.use_text("MINISTERIO DEL PODER POPULAR PARA LA EDUCACIÓN", 10.0, Mm(55.0), y_actual, &font_bold);
    y_actual = y_actual - Mm(8.0);
    
    current_layer.use_text("RESUMEN FINAL DEL RENDIMIENTO ESTUDIANTIL", 12.0, Mm(45.0), y_actual, &font_bold);
    y_actual = y_actual - Mm(5.0);
    current_layer.use_text("Código del Formato: EMG", 8.0, Mm(80.0), y_actual, &font);
    y_actual = y_actual - Mm(10.0);
    
    // === SECCIÓN I: AÑO ESCOLAR Y TIPO DE EVALUACIÓN ===
    current_layer.use_text("I. Año Escolar:", 10.0, Mm(20.0), y_actual, &font_bold);
    current_layer.use_text("_______________________", 10.0, Mm(50.0), y_actual, &font);
    current_layer.use_text("Tipo de Evaluación:", 10.0, Mm(120.0), y_actual, &font_bold);
    current_layer.use_text("Final", 10.0, Mm(160.0), y_actual, &font);
    y_actual = y_actual - Mm(10.0);
    
    // === SECCIÓN II: DATOS DE LA INSTITUCIÓN ===
    current_layer.use_text("II. Datos de la Institución Educativa:", 10.0, Mm(20.0), y_actual, &font_bold);
    y_actual = y_actual - Mm(6.0);
    
    current_layer.use_text("Código Estadístico:", 9.0, Mm(20.0), y_actual, &font);
    current_layer.use_text("_________________", 9.0, Mm(65.0), y_actual, &font);
    current_layer.use_text("Código Dependencia:", 9.0, Mm(120.0), y_actual, &font);
    current_layer.use_text("_____________", 9.0, Mm(170.0), y_actual, &font);
    y_actual = y_actual - Mm(5.0);
    
    current_layer.use_text("Nombre de la Institución:", 9.0, Mm(20.0), y_actual, &font);
    current_layer.use_text("_________________________________________________", 9.0, Mm(80.0), y_actual, &font);
    y_actual = y_actual - Mm(5.0);
    
    current_layer.use_text("Dirección:", 9.0, Mm(20.0), y_actual, &font);
    current_layer.use_text("____________________________________________________________________________", 9.0, Mm(50.0), y_actual, &font);
    y_actual = y_actual - Mm(10.0);
    
    // === SECCIÓN III: IDENTIFICACIÓN DEL ESTUDIANTE ===
    current_layer.use_text("III. Identificación del Estudiante", 10.0, Mm(20.0), y_actual, &font_bold);
    y_actual = y_actual - Mm(8.0);
    
    // === TABLA DE ESTUDIANTES (35 filas) ===
    current_layer.use_text("N°", 8.0, Mm(15.0), y_actual, &font_bold);
    current_layer.use_text("Cédula", 8.0, Mm(25.0), y_actual, &font_bold);
    current_layer.use_text("Apellidos", 8.0, Mm(50.0), y_actual, &font_bold);
    current_layer.use_text("Nombres", 8.0, Mm(85.0), y_actual, &font_bold);
    current_layer.use_text("Lugar Nac.", 8.0, Mm(115.0), y_actual, &font_bold);
    current_layer.use_text("Estado", 8.0, Mm(140.0), y_actual, &font_bold);
    current_layer.use_text("M/F", 8.0, Mm(160.0), y_actual, &font_bold);
    current_layer.use_text("Día", 8.0, Mm(170.0), y_actual, &font_bold);
    current_layer.use_text("Mes", 8.0, Mm(180.0), y_actual, &font_bold);
    current_layer.use_text("Año", 8.0, Mm(190.0), y_actual, &font_bold);
    y_actual = y_actual - Mm(5.0);
    
    // Generar 35 filas para estudiantes
    for i in 1..=35 {
        current_layer.use_text(&format!("{:02}", i), 8.0, Mm(15.0), y_actual, &font);
        current_layer.use_text("_________", 8.0, Mm(25.0), y_actual, &font);
        current_layer.use_text("____________________", 8.0, Mm(50.0), y_actual, &font);
        current_layer.use_text("____________________", 8.0, Mm(85.0), y_actual, &font);
        current_layer.use_text("____________", 8.0, Mm(115.0), y_actual, &font);
        current_layer.use_text("_______", 8.0, Mm(140.0), y_actual, &font);
        current_layer.use_text("___", 8.0, Mm(160.0), y_actual, &font);
        current_layer.use_text("___", 8.0, Mm(170.0), y_actual, &font);
        current_layer.use_text("___", 8.0, Mm(180.0), y_actual, &font);
        current_layer.use_text("____", 8.0, Mm(190.0), y_actual, &font);
        y_actual = y_actual - Mm(4.0);
        
        // Ajustar posición si estamos llegando al final de la página
        if y_actual < Mm(100.0) {
            break; // Parar si llegamos muy abajo para dejar espacio para el resto
        }
    }
    
    // === SECCIÓN V: PROFESORES POR ÁREAS ===
    y_actual = y_actual - Mm(10.0);
    current_layer.use_text("V. Profesores por Áreas:", 10.0, Mm(20.0), y_actual, &font_bold);
    y_actual = y_actual - Mm(6.0);
    
    current_layer.use_text("N°", 8.0, Mm(15.0), y_actual, &font_bold);
    current_layer.use_text("Área", 8.0, Mm(25.0), y_actual, &font_bold);
    current_layer.use_text("Áreas de Formación", 8.0, Mm(35.0), y_actual, &font_bold);
    current_layer.use_text("Apellidos y Nombres del Profesor", 8.0, Mm(90.0), y_actual, &font_bold);
    current_layer.use_text("Cédula", 8.0, Mm(140.0), y_actual, &font_bold);
    current_layer.use_text("Firma", 8.0, Mm(170.0), y_actual, &font_bold);
    y_actual = y_actual - Mm(5.0);
    
    // Áreas de formación con sus iniciales
    let areas = [
        ("1", "CA", "Castellano"),
        ("2", "ILE", "Inglés y otras Lenguas Extranjeras"),
        ("3", "MA", "Matemáticas"),
        ("4", "EF", "Educación Física"),
        ("5", "AP", "Arte y Patrimonio"),
        ("6", "CN", "Ciencias Naturales"),
        ("7", "GHC", "Geografía, Historia y Ciudadanía"),
        ("8", "OC", "Orientación y Convivencia"),
        ("9", "PGCRP", "Participación en Grupos de Creación, Recreación y Producción"),
    ];
    
    for (num, inicial, nombre) in areas.iter() {
        current_layer.use_text(*num, 8.0, Mm(15.0), y_actual, &font);
        current_layer.use_text(*inicial, 8.0, Mm(25.0), y_actual, &font_bold);
        current_layer.use_text(*nombre, 8.0, Mm(35.0), y_actual, &font);
        current_layer.use_text("_________________________", 8.0, Mm(90.0), y_actual, &font);
        current_layer.use_text("____________", 8.0, Mm(140.0), y_actual, &font);
        current_layer.use_text("____________", 8.0, Mm(170.0), y_actual, &font);
        y_actual = y_actual - Mm(4.0);
    }
    
    // === SECCIÓN VI: IDENTIFICACIÓN DEL CURSO ===
    y_actual = y_actual - Mm(8.0);
    current_layer.use_text("VI. Identificación del Curso:", 10.0, Mm(130.0), y_actual, &font_bold);
    y_actual = y_actual - Mm(6.0);
    
    current_layer.use_text("PLAN DE ESTUDIO: EDUCACIÓN MEDIA GENERAL", 9.0, Mm(130.0), y_actual, &font);
    y_actual = y_actual - Mm(4.0);
    current_layer.use_text("CÓDIGO: 31059", 9.0, Mm(130.0), y_actual, &font);
    y_actual = y_actual - Mm(4.0);
    current_layer.use_text("AÑO CURSADO: PRIMERO", 9.0, Mm(130.0), y_actual, &font);
    y_actual = y_actual - Mm(4.0);
    current_layer.use_text("SECCIÓN: A", 9.0, Mm(130.0), y_actual, &font);
    y_actual = y_actual - Mm(4.0);
    current_layer.use_text("N° DE ESTUDIANTES POR SECCIÓN: ____", 9.0, Mm(130.0), y_actual, &font);
    y_actual = y_actual - Mm(4.0);
    current_layer.use_text("N° DE ESTUDIANTES EN ESTA PÁGINA: ____", 9.0, Mm(130.0), y_actual, &font);
    
    // === PIE DE PÁGINA ===
    y_actual = Mm(40.0); // Posición fija en la parte inferior
    current_layer.use_text("VII. Observaciones:", 9.0, Mm(20.0), y_actual, &font_bold);
    current_layer.use_text("_________________________________________________________________________", 8.0, Mm(60.0), y_actual, &font);
    
    y_actual = y_actual - Mm(15.0);
    current_layer.use_text("VIII. Fecha de Remisión:", 9.0, Mm(20.0), y_actual, &font_bold);
    current_layer.use_text("____/____/____", 9.0, Mm(70.0), y_actual, &font);
    current_layer.use_text("Sello de la Institución", 9.0, Mm(120.0), y_actual, &font);
    
    y_actual = y_actual - Mm(8.0);
    current_layer.use_text("IX. Fecha de Recepción:", 9.0, Mm(20.0), y_actual, &font_bold);
    current_layer.use_text("____/____/____", 9.0, Mm(70.0), y_actual, &font);
    current_layer.use_text("Sello del Plantel", 9.0, Mm(120.0), y_actual, &font);
    
    // Guardar PDF en la carpeta temporal del sistema
    let temp_dir = std::env::temp_dir();
    let ruta_pdf_completa = temp_dir.join(&ruta_pdf);
    
    let file = File::create(&ruta_pdf_completa).map_err(|e| format!("Error creando archivo PDF: {}", e))?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).map_err(|e| format!("Error guardando PDF: {}", e))?;
    
    // Retornar ruta absoluta del PDF
    let ruta_absoluta = ruta_pdf_completa
        .to_string_lossy()
        .to_string();
    
    Ok(ruta_absoluta)
} 