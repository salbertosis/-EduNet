import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export function useCalificaciones(estudianteId: number, periodoId: number) {
  // Estado y lógica para cargar, actualizar y guardar calificaciones
  const [calificaciones, setCalificaciones] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const cargarCalificaciones = useCallback(async () => {
    if (!estudianteId || !periodoId) return;
    setLoading(true);
    setError(null);
    try {
      const params = { idEstudiante: Number(estudianteId), idPeriodo: Number(periodoId) };
      const data = await invoke<any[]>('obtener_calificaciones_estudiante', params);
      setCalificaciones(data);
    } catch (err: any) {
      setError(err?.message || 'Error al cargar calificaciones');
      setCalificaciones([]);
    } finally {
      setLoading(false);
    }
  }, [estudianteId, periodoId]);

  useEffect(() => {
    cargarCalificaciones();
  }, [cargarCalificaciones]);

  return {
    calificaciones,
    setCalificaciones,
    loading,
    error,
    recargar: cargarCalificaciones,
    // Métodos para cargar, guardar, etc.
  };
} 