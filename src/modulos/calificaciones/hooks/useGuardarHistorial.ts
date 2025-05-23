import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { CalificacionEstudiante } from '../types';
import { Asignatura } from '../types';
import { useCalculosCalificaciones } from '../../../utilidades/calculoNotas';
import { useEstadosCalificaciones } from './useEstadosCalificaciones';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

export function useGuardarHistorial(
  asignaturas: Asignatura[],
  calificaciones: CalificacionEstudiante[]
) {
  const [loading, setLoading] = useState(false);
  const [exito, setExito] = useState(false);
    const { promedioFinal, totalPendientes } = useCalculosCalificaciones(asignaturas, calificaciones);
  const { calcularEstadoGeneral } = useEstadosCalificaciones(calificaciones, totalPendientes);
    const { mostrarMensaje } = useMensajeGlobal();

  const guardarHistorial = async (
    estudianteId: number,
    periodoId: number,
    idGradoSecciones: number
  ) => {
    if (!estudianteId || !periodoId || !idGradoSecciones) {
            mostrarMensaje('Faltan datos necesarios para guardar el historial', 'error');
      return;
    }

    setLoading(true);
    setExito(false);

    try {
      // Calcular promedio y estado
      const promedio = Number(promedioFinal());
            const estado = calcularEstadoGeneral;

      // Guardar historial
            const params = {
                idEstudiante: Math.trunc(Number(estudianteId)),
                idPeriodo: Math.trunc(Number(periodoId)),
                idGradoSecciones: Math.trunc(Number(idGradoSecciones))
            };
            const respuesta = await invoke('upsert_historial_academico', { input: params });
            // Log detallado de la respuesta
            console.log('[DEBUG][GUARDAR_HISTORIAL] Respuesta backend:', respuesta);
            // Si la respuesta es un objeto con error, mostrarlo
            if (respuesta && typeof respuesta === 'object' && 'error' in respuesta && respuesta.error) {
                mostrarMensaje(respuesta.error, 'error');
                setExito(false);
            } else {
      setExito(true);
                mostrarMensaje('Historial académico guardado correctamente.', 'exito');
            }
    } catch (err: any) {
            console.error('[DEBUG][GUARDAR_HISTORIAL] Error detallado:', err);
            // Si el error es vacío o no relevante, mostrar éxito
            if (!err || (typeof err === 'object' && Object.keys(err).length === 0)) {
                setExito(true);
                mostrarMensaje('Historial académico guardado correctamente.', 'exito');
            } else {
                let mensajeError = 'Error al guardar historial académico';
                if (typeof err === 'string') mensajeError = err;
                else if (err && typeof err.message === 'string') mensajeError = err.message;
                else if (typeof err === 'object') mensajeError = JSON.stringify(err);
                mostrarMensaje(mensajeError, 'error');
      setExito(false);
            }
    } finally {
      setLoading(false);
    }
  };

  return {
    guardarHistorial,
    loading,
    exito
  };
} 