import { useMemo } from 'react';

interface CalificacionEstudiante {
  id_calificacion?: number;
  id_asignatura: number;
  nombre_asignatura: string;
  lapso_1?: number;
  lapso_1_ajustado?: number;
  lapso_2?: number;
  lapso_2_ajustado?: number;
  lapso_3?: number;
  lapso_3_ajustado?: number;
  nota_final?: number;
  revision?: string;
}

interface Asignatura {
  id_asignatura: number;
  nombre_asignatura: string;
  id_grado: number;
  id_modalidad: number;
}

export function useCalculosCalificaciones(
  asignaturas: Asignatura[],
  calificaciones: CalificacionEstudiante[]
) {
  // Calcular nota final para una calificación específica
  const calcularNotaFinal = (calif: CalificacionEstudiante): number => {
    const l1 = calif.lapso_1_ajustado ?? calif.lapso_1 ?? 0;
    const l2 = calif.lapso_2_ajustado ?? calif.lapso_2 ?? 0;
    const l3 = calif.lapso_3_ajustado ?? calif.lapso_3 ?? 0;
    if (l1 && l2 && l3) {
      return Math.round((l1 + l2 + l3) / 3);
    }
    return 0;
  };

  // Obtener la nota válida (revisión si existe, si no final calculada)
  const obtenerNotaValida = (calif: CalificacionEstudiante): number => {
    const revision = Number(calif.revision);
    if (!isNaN(revision) && revision > 0) return revision;
    return calcularNotaFinal(calif);
  };

  // Calcular promedio de un lapso específico
  const promedioLapso = (lapso: keyof CalificacionEstudiante, lapsoAjuste: keyof CalificacionEstudiante) => {
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
  };

  // Calcular promedio final
  const promedioFinal = useMemo(() => {
    const notas = asignaturas
      .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
      .map(a => {
        const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
        return calif ? obtenerNotaValida(calif) : undefined;
      })
      .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
    if (notas.length === 0) return '';
    return (notas.reduce((a, b) => a + b, 0) / notas.length).toFixed(2);
  }, [asignaturas, calificaciones]);

  // Calcular total de asignaturas pendientes
  const totalPendientes = useMemo(() => {
    return asignaturas
      .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
      .map(a => {
        const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
        const nota = calif ? obtenerNotaValida(calif) : 0;
        return nota < 9.5;
      })
      .filter(Boolean).length;
  }, [asignaturas, calificaciones]);

  return {
    calcularNotaFinal,
    obtenerNotaValida,
    promedioLapso,
    promedioFinal,
    totalPendientes
  };
} 