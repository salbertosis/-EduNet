import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export function useAsignaturas(idGrado: number, idModalidad: number) {
  const [asignaturas, setAsignaturas] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const cargarAsignaturas = useCallback(async () => {
    if (!idGrado || !idModalidad) return;
    setLoading(true);
    setError(null);
    try {
      const params = { idGrado: Number(idGrado), idModalidad: Number(idModalidad) };
      const data = await invoke<any[]>('obtener_asignaturas_por_grado_modalidad', params);
      setAsignaturas(data);
    } catch (err: any) {
      setError(err?.message || 'Error al cargar asignaturas');
      setAsignaturas([]);
    } finally {
      setLoading(false);
    }
  }, [idGrado, idModalidad]);

  useEffect(() => {
    cargarAsignaturas();
  }, [cargarAsignaturas]);

  return {
    asignaturas,
    setAsignaturas,
    loading,
    error,
    recargar: cargarAsignaturas,
  };
} 