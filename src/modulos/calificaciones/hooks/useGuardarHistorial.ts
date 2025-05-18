import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useCalculosCalificaciones } from './useCalculosCalificaciones';
import { useEstadosCalificaciones } from './useEstadosCalificaciones';

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

export function useGuardarHistorial(
  asignaturas: Asignatura[],
  calificaciones: CalificacionEstudiante[]
) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [exito, setExito] = useState(false);

  const { obtenerNotaValida, promedioFinal, totalPendientes } = useCalculosCalificaciones(asignaturas, calificaciones);
  const { calcularEstadoGeneral } = useEstadosCalificaciones(calificaciones, totalPendientes);

  const guardarHistorial = async (
    estudianteId: number,
    periodoId: number,
    idGradoSecciones: number
  ) => {
    if (!estudianteId || !periodoId || !idGradoSecciones) {
      setError('Faltan datos necesarios para guardar el historial');
      return;
    }

    setLoading(true);
    setError(null);
    setExito(false);

    try {
      // Calcular promedio y estado
      const promedio = Number(promedioFinal());
      const estatus = calcularEstadoGeneral;

      // Guardar historial
      await invoke('upsert_historial_academico', {
        id_estudiante: estudianteId,
        id_periodo: periodoId,
        id_grado_secciones: idGradoSecciones,
        promedio_anual: promedio,
        estatus
      });

      setExito(true);
    } catch (err: any) {
      setError(err?.message || 'Error al guardar historial académico');
      setExito(false);
    } finally {
      setLoading(false);
    }
  };

  return {
    guardarHistorial,
    loading,
    error,
    exito
  };
} 