use crate::models::calificacion::CalificacionEstudiante;

pub fn calcular_nota_final(calificacion: &CalificacionEstudiante) -> i32 {
    let l1 = calificacion.lapso_1_ajustado.or(calificacion.lapso_1).unwrap_or(0);
    let l2 = calificacion.lapso_2_ajustado.or(calificacion.lapso_2).unwrap_or(0);
    let l3 = calificacion.lapso_3_ajustado.or(calificacion.lapso_3).unwrap_or(0);
    
    ((l1 + l2 + l3) as f64 / 3.0).round() as i32
}

pub fn calcular_promedio_anual(calificaciones: &[CalificacionEstudiante]) -> f64 {
    // Excluir asignaturas administrativas (id 9 y 11)
    let califs_validas: Vec<&CalificacionEstudiante> = calificaciones
        .iter()
        .filter(|c| c.id_asignatura != 9 && c.id_asignatura != 11)
        .collect();
    if califs_validas.is_empty() {
        return 0.0;
    }
    // Usar revisión si existe y es > 0, si no, nota final
    let suma_notas: f64 = califs_validas
        .iter()
        .map(|cal| {
            if let Some(rev) = cal.revision {
                if rev > 0 {
                    return rev as f64;
                }
            }
            calcular_nota_final(cal) as f64
        })
        .sum();
    let promedio = suma_notas / califs_validas.len() as f64;
    (promedio * 100.0).round() / 100.0
}

/// Calcula el estatus académico según el número de asignaturas aplazadas.
/// Una asignatura está aplazada si nota final < 9.5 y revisión < 10.
pub fn calcular_estatus_academico(calificaciones: &[CalificacionEstudiante]) -> String {
    let mut aplazadas = 0;
    for cal in calificaciones {
        let nota_final = calcular_nota_final(cal);
        let revision = cal.revision;
        // Si la nota final es >= 10, está aprobada
        if nota_final >= 10 {
            continue;
        }
        // Si la revisión existe y es >= 10, está aprobada
        if let Some(rev) = revision {
            if rev >= 10 {
                continue;
            }
        }
        // Si la revisión no existe o es < 10, es aplazada
        aplazadas += 1;
    }
    if aplazadas >= 3 {
        "REPITE".to_string()
    } else {
        "APROBADO".to_string()
    }
} 