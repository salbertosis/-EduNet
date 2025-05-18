// Utilidad para calcular la nota final de una calificación
import type { CalificacionEstudiante } from '../modulos/calificaciones/paginas/DetalleCalificaciones';

export function calcularNotaFinal(calificacion: CalificacionEstudiante): number {
    const l1 = calificacion.lapso_1_ajustado ?? calificacion.lapso_1 ?? 0;
    const l2 = calificacion.lapso_2_ajustado ?? calificacion.lapso_2 ?? 0;
    const l3 = calificacion.lapso_3_ajustado ?? calificacion.lapso_3 ?? 0;
    const resultado = Math.round((l1 + l2 + l3) / 3);
    console.log('[LOG][calcularNotaFinal] l1:', l1, 'l2:', l2, 'l3:', l3, '=> resultado:', resultado, 'calif:', calificacion);
    return resultado;
} 