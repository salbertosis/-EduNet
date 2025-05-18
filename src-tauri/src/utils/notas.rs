use crate::models::calificacion::CalificacionEstudiante;

pub fn calcular_nota_final(calificacion: &CalificacionEstudiante) -> i32 {
    let l1 = calificacion.lapso_1_ajustado.or(calificacion.lapso_1).unwrap_or(0);
    let l2 = calificacion.lapso_2_ajustado.or(calificacion.lapso_2).unwrap_or(0);
    let l3 = calificacion.lapso_3_ajustado.or(calificacion.lapso_3).unwrap_or(0);
    
    ((l1 + l2 + l3) as f64 / 3.0).round() as i32
}

pub fn calcular_promedio_anual(calificaciones: &[CalificacionEstudiante]) -> f64 {
    if calificaciones.is_empty() {
        return 0.0;
    }

    let suma_notas: f64 = calificaciones.iter()
        .map(|cal| calcular_nota_final(cal) as f64)
        .sum();

    let promedio = suma_notas / calificaciones.len() as f64;
    (promedio * 100.0).round() / 100.0
} 