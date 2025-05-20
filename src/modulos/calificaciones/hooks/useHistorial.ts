import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface HistorialAcademico {
  id_historial: number;
  id_estudiante: number;
  id_periodo: number;
  id_grado_secciones: number;
  promedio_anual: number;
  estatus: string;
  fecha_registro: string;
  periodo_escolar?: string;
  grado?: string;
  seccion?: string;
}

export function useHistorial(estudianteId: number) {
  const [historial, setHistorial] = useState<HistorialAcademico[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  console.log('[DEBUG][HOOK] useHistorial ejecutado con estudianteId:', estudianteId);

  const cargarHistorial = useCallback(async () => {
    console.log('[DEBUG][HOOK] cargarHistorial iniciado con estudianteId:', estudianteId);
    
    // Validación estricta del ID
    if (typeof estudianteId !== 'number' || isNaN(estudianteId) || estudianteId <= 0) {
      setError('ID de estudiante no válido (' + estudianteId + ')');
      console.warn('[DEBUG][HOOK] ID de estudiante no válido:', estudianteId);
      return;
    }

    setLoading(true);
    setError(null);

    try {
      console.log('[DEBUG][HOOK] Llamando a obtener_historial_academico_estudiante con id_estudiante:', estudianteId);
      const data = await invoke<HistorialAcademico[]>('obtener_historial_academico_estudiante', {
        idEstudiante: estudianteId
      });
      console.log('[DEBUG][HOOK] Datos recibidos:', data);

      if (!Array.isArray(data)) {
        throw new Error('Formato de respuesta inválido');
      }

      // Ordenar por periodo escolar (más reciente primero)
      const historialOrdenado = data.sort((a, b) => {
        if (!a.periodo_escolar || !b.periodo_escolar) return 0;
        return b.periodo_escolar.localeCompare(a.periodo_escolar);
      });

      console.log('[DEBUG][HOOK] Historial ordenado:', historialOrdenado);
      setHistorial(historialOrdenado);
    } catch (err: any) {
      console.error('[DEBUG][HOOK] Error al cargar historial:', err);
      setError(err?.message || 'Error al cargar historial académico');
      setHistorial([]);
    } finally {
      setLoading(false);
    }
  }, [estudianteId]);

  useEffect(() => {
    console.log('[DEBUG][HOOK] useEffect disparado');
    cargarHistorial();
  }, [cargarHistorial]);

  return {
    historial,
    setHistorial,
    loading,
    error,
    recargar: cargarHistorial
  };
} 