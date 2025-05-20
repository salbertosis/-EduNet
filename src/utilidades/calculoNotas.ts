// Utilidad para calcular la nota final de una calificación
import { CalificacionEstudiante, Asignatura } from '../modulos/calificaciones/types';

// Tipos de estado posibles
export type EstadoAsignatura = 'Aprobado' | 'Pendiente' | 'Repite' | 'Revisión';
export type EstadoGeneral = 'Aprobado' | 'Repite';

// Cálculo de nota final para una asignatura
export function calcularNotaFinal(calif: CalificacionEstudiante): number {
    const l1 = calif.lapso_1_ajustado ?? calif.lapso_1 ?? 0;
    const l2 = calif.lapso_2_ajustado ?? calif.lapso_2 ?? 0;
    const l3 = calif.lapso_3_ajustado ?? calif.lapso_3 ?? 0;
    if (l1 && l2 && l3) {
        return Math.round((l1 + l2 + l3) / 3);
    }
    return 0;
}

// Obtener la nota válida (revisión si existe, si no final calculada)
export function obtenerNotaValida(calif: CalificacionEstudiante): number {
    const revision = Number(calif.revision);
    if (!isNaN(revision) && revision > 0) return revision;
    return calcularNotaFinal(calif);
}

// Calcular promedio de un lapso específico
export function promedioLapso(
    asignaturas: Asignatura[],
    calificaciones: CalificacionEstudiante[],
    lapso: keyof CalificacionEstudiante,
    lapsoAjuste: keyof CalificacionEstudiante
): string {
    const notas = asignaturas
        .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
        .map(a => {
            const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
            if (!calif) return undefined;
            if (typeof calif[lapsoAjuste] === 'number') return calif[lapsoAjuste] as number;
            if (typeof calif[lapso] === 'number') return calif[lapso] as number;
            return undefined;
        })
        .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
    if (notas.length === 0) return '';
    return (notas.reduce((a, b) => a + b, 0) / notas.length).toFixed(2);
}

// Calcular promedio final
export function promedioFinal(
    asignaturas: Asignatura[],
    calificaciones: CalificacionEstudiante[]
): string {
    const notas = asignaturas
        .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
        .map(a => {
            const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
            return calif ? obtenerNotaValida(calif) : undefined;
        })
        .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
    if (notas.length === 0) return '';
    return (notas.reduce((a, b) => a + b, 0) / notas.length).toFixed(2);
}

// Calcular estado de una asignatura
export function calcularEstadoAsignatura(
    calif: CalificacionEstudiante,
    asignaturas: Asignatura[],
    calificaciones: CalificacionEstudiante[]
): EstadoAsignatura | 'Error' {
    const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : calcularNotaFinal(calif);
    const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
    const totalAplazadasRevision = calcularTotalAplazadasRevision(asignaturas, calificaciones);

    // 0. Si la nota final es >= 9.5 y existe nota de revisión: 'Error'
    if (notaFinal >= 9.5 && calif.revision !== undefined && calif.revision !== null && calif.revision !== '') return 'Error';

    // 1. Si la nota final es >= 9.5: 'Aprobado'
    if (notaFinal >= 9.5) return 'Aprobado';

    // 2. Si la nota final < 9.5 y NO hay revisión: 'Revisión' (prioridad absoluta)
    if (notaFinal < 9.5 && (calif.revision === undefined || calif.revision === null || calif.revision === '')) return 'Revisión';

    // 3. Si hay revisión:
    if (revision !== undefined && !isNaN(revision)) {
        if (revision >= 10) return 'Aprobado';
        if (revision < 10) {
            if (totalAplazadasRevision >= 3) return 'Repite';
            return 'Pendiente';
        }
    }

    // Por defecto
    return 'Revisión';
}

/**
 * Calcula el total de asignaturas pendientes de un estudiante.
 * Solo se consideran pendientes aquellas asignaturas cuya nota final es menor a 9.5
 * y cuya revisión (si existe) es menor a 10. Si la revisión es >= 10, se considera aprobada.
 * Se excluyen las asignaturas con id 9 y 11.
 */
export function calcularTotalPendientes(
    asignaturas: Asignatura[],
    calificaciones: CalificacionEstudiante[]
): number {
    const pendientes = asignaturas
        .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
        .map(a => {
            const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
            if (!calif) {
                return false;
            }
            const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : calcularNotaFinal(calif);
            const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
            // Si la nota final es >= 9.5, está aprobada
            if (notaFinal >= 9.5) {
                return false;
            }
            // Si la revisión existe y es >= 10, está aprobada
            if (revision !== undefined && !isNaN(revision) && revision >= 10) {
                return false;
            }
            // Si la revisión existe y es < 10, es pendiente
            if (revision !== undefined && !isNaN(revision) && revision < 10) {
                return true;
            }
            // Si no hay revisión y la nota final es < 9.5, es pendiente
            if (notaFinal < 9.5) {
                return true;
            }
            return false;
        })
        .filter(Boolean);
    return pendientes.length;
}

/**
 * Calcula el total de asignaturas reprobadas (nota final < 9.5 y revisión < 9.5)
 */
export function calcularTotalReprobadas(
    asignaturas: Asignatura[],
    calificaciones: CalificacionEstudiante[]
): number {
    return asignaturas
        .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
        .map(a => {
            const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
            if (!calif) return false;
            const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : calcularNotaFinal(calif);
            const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
            // Reprobada solo si nota final < 9.5 y (sin revisión o revisión < 9.5)
            if (notaFinal < 9.5 && (revision === undefined || isNaN(revision) || revision < 9.5)) return true;
            return false;
        })
        .filter(Boolean).length;
}

/**
 * Calcula el total de asignaturas aplazadas en revisión (nota final < 9.5 y revisión existe y < 10)
 */
export function calcularTotalAplazadasRevision(
    asignaturas: Asignatura[],
    calificaciones: CalificacionEstudiante[]
): number {
    return asignaturas
        .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
        .map(a => {
            const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
            if (!calif) return false;
            const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : calcularNotaFinal(calif);
            const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
            if (notaFinal < 9.5 && revision !== undefined && !isNaN(revision) && revision < 10) return true;
            return false;
        })
        .filter(Boolean).length;
}

// Calcular estado general del estudiante
export function calcularEstadoGeneral(
    asignaturas: Asignatura[],
    calificaciones: CalificacionEstudiante[]
): EstadoGeneral {
    const totalAplazadasRevision = calcularTotalAplazadasRevision(asignaturas, calificaciones);
    return totalAplazadasRevision >= 3 ? 'Repite' : 'Aprobado';
}

// Hook personalizado para todos los cálculos
export function useCalculosCalificaciones(
    asignaturas: Asignatura[],
    calificaciones: CalificacionEstudiante[]
) {
    const totalPendientes = calcularTotalPendientes(asignaturas, calificaciones);
    const totalReprobadas = calcularTotalReprobadas(asignaturas, calificaciones);
    const totalAplazadasRevision = calcularTotalAplazadasRevision(asignaturas, calificaciones);
    const estadoGeneral = totalAplazadasRevision >= 3 ? 'Repite' : 'Aprobado';

    return {
        calcularNotaFinal,
        obtenerNotaValida,
        promedioLapso: (lapso: keyof CalificacionEstudiante, lapsoAjuste: keyof CalificacionEstudiante) => 
            promedioLapso(asignaturas, calificaciones, lapso, lapsoAjuste),
        promedioFinal: () => promedioFinal(asignaturas, calificaciones),
        calcularEstadoAsignatura: (calif: CalificacionEstudiante) => 
            calcularEstadoAsignatura(calif, asignaturas, calificaciones),
        totalPendientes,
        totalReprobadas,
        totalAplazadasRevision,
        estadoGeneral
    };
} 