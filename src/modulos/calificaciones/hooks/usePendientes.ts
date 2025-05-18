import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export function usePendientes(estudianteId: number) {
  // Estado y lógica para cargar y guardar asignaturas pendientes
  const [pendientes, setPendientes] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const cargarPendientes = useCallback(async () => {
    if (!estudianteId) return;
    setLoading(true);
    setError(null);
    try {
      const params = { id_estudiante: Number(estudianteId) };
      const data = await invoke<any[]>('obtener_asignaturas_pendientes_estudiante', params);
      setPendientes(data);
    } catch (err: any) {
      setError(err?.message || 'Error al cargar asignaturas pendientes');
      setPendientes([]);
    } finally {
      setLoading(false);
    }
  }, [estudianteId]);

  useEffect(() => {
    cargarPendientes();
  }, [cargarPendientes]);

  return {
    pendientes,
    setPendientes,
    loading,
    error,
    recargar: cargarPendientes,
  };
} 