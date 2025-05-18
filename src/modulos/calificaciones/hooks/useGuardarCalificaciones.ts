import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

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

export function useGuardarCalificaciones() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [exito, setExito] = useState(false);

  function calcularNotaFinal(calif: CalificacionEstudiante): number {
    const l1 = calif.lapso_1_ajustado ?? calif.lapso_1 ?? 0;
    const l2 = calif.lapso_2_ajustado ?? calif.lapso_2 ?? 0;
    const l3 = calif.lapso_3_ajustado ?? calif.lapso_3 ?? 0;
    if (l1 && l2 && l3) {
      return Math.round((l1 + l2 + l3) / 3);
    }
    return 0;
  }

  const guardarCalificaciones = async (calificaciones: CalificacionEstudiante[], estudianteId: number, periodoId: number) => {
    setLoading(true);
    setError(null);
    setExito(false);
    try {
      for (const calif of calificaciones) {
        const calificacionBackend = {
          ...calif,
          id_estudiante: estudianteId,
          id_periodo: periodoId,
          nota_final: calcularNotaFinal(calif),
        };
        await invoke('guardar_calificacion', { calificacion: calificacionBackend });
      }
      setExito(true);
    } catch (err: any) {
      setError(err?.message || 'Error al guardar calificaciones');
      setExito(false);
    } finally {
      setLoading(false);
    }
  };

  return { guardarCalificaciones, loading, error, exito };
} 