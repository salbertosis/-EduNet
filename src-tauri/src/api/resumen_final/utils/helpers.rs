//! Funciones auxiliares para el resumen final

use crate::api::plantillas::Asignatura;

/// Calcula el número total de páginas necesarias
pub fn calcular_total_paginas(total_estudiantes: usize, estudiantes_por_pagina: usize) -> usize {
    (total_estudiantes + estudiantes_por_pagina - 1) / estudiantes_por_pagina
}

/// Calcula el rango de estudiantes para una página específica
pub fn calcular_rango_pagina(pagina: usize, estudiantes_por_pagina: usize, total_estudiantes: usize) -> (usize, usize) {
    let inicio = pagina * estudiantes_por_pagina;
    let fin = std::cmp::min(inicio + estudiantes_por_pagina, total_estudiantes);
    (inicio, fin)
}

/// Formatea el número de estudiante con ceros a la izquierda
pub fn formatear_numero_estudiante(numero: i32) -> String {
    format!("{:02}", numero)
}

/// Valida que las asignaturas no excedan el límite de columnas
pub fn validar_limite_asignaturas(asignaturas: &[Asignatura], max_columnas: usize) -> Result<(), String> {
    if asignaturas.len() > max_columnas {
        return Err(format!(
            "Demasiadas asignaturas ({}/{}). Máximo permitido: {}",
            asignaturas.len(),
            max_columnas,
            max_columnas
        ));
    }
    Ok(())
}

/// Genera un mensaje de error formateado
pub fn formatear_error(contexto: &str, error: &str) -> String {
    format!("[ERROR en {}]: {}", contexto, error)
}

/// Limpia caracteres especiales de strings para HTML
pub fn limpiar_html(texto: &str) -> String {
    texto
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

/// Trunca texto si excede la longitud máxima
pub fn truncar_texto(texto: &str, max_longitud: usize) -> String {
    if texto.len() > max_longitud {
        format!("{}...", &texto[..max_longitud.saturating_sub(3)])
    } else {
        texto.to_string()
    }
} 