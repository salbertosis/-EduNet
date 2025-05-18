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
  // Estado y lógica para cargar y guardar historial académico
  const [historial, setHistorial] = useState<HistorialAcademico[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Log de entrada al hook
  console.log('[DEBUG][HOOK] useHistorial ejecutado con estudianteId:', estudianteId, 'tipo:', typeof estudianteId);

  const cargarHistorial = useCallback(async () => {
    console.log('[DEBUG][HOOK] cargarHistorial llamado con estudianteId:', estudianteId, 'tipo:', typeof estudianteId);
    if (typeof estudianteId !== 'number' || isNaN(estudianteId) || estudianteId <= 0) {
      setError('ID de estudiante no válido (' + estudianteId + ')');
      console.warn('[DEBUG][HOOK] ID de estudiante no válido:', estudianteId, 'tipo:', typeof estudianteId);
      return;
    }

    setLoading(true);
    setError(null);

    try {
      console.log('[DEBUG][HOOK] Llamando a invoke obtener_historial_academico_estudiante con id:', estudianteId);
      const data = await invoke<HistorialAcademico[]>('obtener_historial_academico_estudiante', {
        id_estudiante: estudianteId
      });
      console.log('[DEBUG][HOOK] Respuesta de invoke:', data);

      // Ordenar por periodo escolar (más reciente primero)
      const historialOrdenado = data.sort((a, b) => {
        if (!a.periodo_escolar || !b.periodo_escolar) return 0;
        return b.periodo_escolar.localeCompare(a.periodo_escolar);
      });

      setHistorial(historialOrdenado);
    } catch (err: any) {
      setError(err?.message || 'Error al cargar historial académico');
      setHistorial([]);
      console.error('[DEBUG][HOOK] Error en invoke:', err);
    } finally {
      setLoading(false);
    }
  }, [estudianteId]);

  useEffect(() => {
    console.log('[DEBUG][HOOK] useEffect disparado con estudianteId:', estudianteId, 'tipo:', typeof estudianteId);
    cargarHistorial();
  }, [cargarHistorial, estudianteId]);

  return {
    historial,
    setHistorial,
    loading,
    error,
    recargar: cargarHistorial,
    // Métodos para cargar, guardar, etc.
  };
} 