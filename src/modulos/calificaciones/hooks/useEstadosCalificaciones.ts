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

export function useEstadosCalificaciones(
  calificaciones: CalificacionEstudiante[],
  totalPendientes: number
) {
  // Calcular estado de una asignatura específica
  const calcularEstadoAsignatura = (calif: CalificacionEstudiante): string => {
    const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : 0;
    const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
    const revisionHabilitada = notaFinal < 9.5;

    if (revisionHabilitada) {
      if (revision !== undefined && !isNaN(revision)) {
        if (revision >= 10) return 'Aprobado';
        if (revision < 10) {
          if (totalPendientes >= 3) return 'Repite';
          return 'Pendiente';
        }
      }
      return 'Revisión';
    }

    // Si el input de revisión está deshabilitado, solo depende de la nota final
    if (notaFinal >= 9.5) return 'Aprobado';
    return 'Revisión';
  };

  // Calcular estado general del estudiante
  const calcularEstadoGeneral = useMemo(() => {
    if (totalPendientes >= 3) return 'Repite';
    if (totalPendientes === 2) return 'Aprobado';
    if (totalPendientes === 1) return 'Aprobado';
    return 'Aprobado';
  }, [totalPendientes]);

  // Verificar si una asignatura está en revisión
  const estaEnRevision = (calif: CalificacionEstudiante): boolean => {
    const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : 0;
    return notaFinal < 9.5;
  };

  // Verificar si una asignatura está pendiente
  const estaPendiente = (calif: CalificacionEstudiante): boolean => {
    const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
    return revision !== undefined && revision < 10;
  };

  return {
    calcularEstadoAsignatura,
    calcularEstadoGeneral,
    estaEnRevision,
    estaPendiente
  };
} 