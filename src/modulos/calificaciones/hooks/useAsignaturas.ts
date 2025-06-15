import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export function useAsignaturas(idGrado: number, idModalidad: number, idGradoSecciones?: number) {
  const [asignaturas, setAsignaturas] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const cargarAsignaturas = useCallback(async () => {
    let id_grado = idGrado;
    let id_modalidad = idModalidad;
    if (idGradoSecciones) {
      try {
        const res = await invoke<any>('obtener_grado_secciones_por_id', { id: idGradoSecciones });
        id_grado = res.id_grado;
        id_modalidad = res.id_modalidad;
      } catch (e) {
        setError('Error al obtener grado y modalidad desde grado_secciones');
        setAsignaturas([]);
        return;
      }
    }
    if (!id_grado || !id_modalidad) return;
    setLoading(true);
    setError(null);
    try {
      const params = { idGrado: Number(id_grado), idModalidad: Number(id_modalidad) };
      const data = await invoke<any[]>('obtener_asignaturas_por_grado_modalidad', params);
      setAsignaturas(data);
    } catch (err: any) {
      setError(err?.message || 'Error al cargar asignaturas');
      setAsignaturas([]);
    } finally {
      setLoading(false);
    }
  }, [idGrado, idModalidad, idGradoSecciones]);

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