import { useState, useEffect, useCallback } from 'react';
import { guardarPendientesAPI, cargarPendientesAPI } from '../services/pendientesService';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { invoke } from '@tauri-apps/api/tauri';

function contarAplazadas(pendientesAGuardar: any[]) {
  // Considera aplazada si nota_final < 9.5 o revisión < 10
  return pendientesAGuardar.filter(p => {
    const notaFinal = typeof p.nota_final === 'number' ? p.nota_final : 0;
    const revision = p.revision !== undefined && p.revision !== null ? Number(p.revision) : undefined;
    if (revision !== undefined && !isNaN(revision)) {
      return revision < 10;
    }
    return notaFinal < 9.5;
  }).length;
}

function validarPendientes(pendientesAGuardar: any[]) {
  if (pendientesAGuardar.length === 0) {
    throw new Error('No hay asignaturas pendientes para guardar. Por favor, selecciona al menos una asignatura.');
  }
  const aplazadas = contarAplazadas(pendientesAGuardar);
  if (aplazadas > 2) {
    throw new Error(`El estudiante debe de repetir el año escolar. Asignaturas aplazadas: ${aplazadas}`);
  }
}

export interface CalificacionesPendiente {
  id_pendiente: number;
  cal_momento1: number;
  cal_momento2: number;
  cal_momento3: number;
  cal_momento4: number;
}

export function usePendientes(estudianteId: number) {
  // Estado y lógica para cargar y guardar asignaturas pendientes
  const [pendientes, setPendientes] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [exito, setExito] = useState(false);
  const { mostrarMensaje } = useMensajeGlobal();
  const [error, setError] = useState<string | null>(null);

  const cargarPendientes = useCallback(async () => {
    if (!estudianteId) return;
    setLoading(true);
    try {
      const data = await cargarPendientesAPI(Number(estudianteId));
      setPendientes(Array.isArray(data) ? data : []);
    } catch (err: any) {
      mostrarMensaje(err?.message || 'No se pudieron cargar las asignaturas pendientes. Por favor, intenta nuevamente o contacta al soporte.', 'error');
      setPendientes([]);
    } finally {
      setLoading(false);
    }
  }, [estudianteId, mostrarMensaje]);

  const guardarPendientes = async (pendientesAGuardar: any[]) => {
    setLoading(true);
    setExito(false);
    try {
      validarPendientes(pendientesAGuardar);
      await guardarPendientesAPI(Number(estudianteId), pendientesAGuardar);
      setExito(true);
      mostrarMensaje('Asignaturas pendientes guardadas correctamente.', 'exito');
    } catch (err: any) {
      mostrarMensaje(err?.message || 'Ocurrió un error al guardar las asignaturas pendientes. Por favor, intenta nuevamente.', 'error');
      setExito(false);
    } finally {
      setLoading(false);
    }
  };

  const obtenerCalificacionesPendiente = async (idPendiente: number): Promise<CalificacionesPendiente | null> => {
    setError(null);
    try {
      console.log('Invocando obtener_calificaciones_pendiente con:', idPendiente);
      const data = await invoke<CalificacionesPendiente>('obtener_calificaciones_pendiente', { idPendiente });
      console.log('Datos recibidos:', data);
      return data;
    } catch (e: any) {
      console.error('Error al obtener calificaciones:', e);
      let msg = 'Error al obtener calificaciones';
      if (e && typeof e === 'object' && e.message) {
        msg = e.message;
      } else if (typeof e === 'string') {
        msg = e;
      }
      if (msg.includes('ERR_DB')) {
        if (process.env.NODE_ENV === 'development') {
          console.error('[ERROR][obtenerCalificacionesPendiente]', msg);
        }
        setError('No se pudieron obtener las calificaciones. Por favor, intente más tarde o contacte al soporte.');
      } else {
        setError(msg);
      }
      return null;
    }
  };

  const upsertCalificacionesPendiente = async (input: CalificacionesPendiente): Promise<boolean> => {
    setError(null);
    try {
      const inputSnake = {
        id_pendiente: input.id_pendiente,
        cal_momento1: input.cal_momento1,
        cal_momento2: input.cal_momento2,
        cal_momento3: input.cal_momento3,
        cal_momento4: input.cal_momento4,
      };
      await invoke<string>('upsert_calificaciones_pendiente', { input: inputSnake });
      return true;
    } catch (e: any) {
      let msg = 'Error al guardar calificaciones';
      if (e && typeof e === 'object' && e.message) {
        msg = e.message;
      } else if (typeof e === 'string') {
        msg = e;
      }
      if (msg.includes('ERR_DB')) {
        if (process.env.NODE_ENV === 'development') {
          console.error('[ERROR][upsertCalificacionesPendiente]', msg);
        }
        setError('No se pudieron guardar las calificaciones. Por favor, intente más tarde o contacte al soporte.');
      } else {
        setError(msg);
      }
      return false;
    }
  };

  const eliminarAsignaturaPendiente = async (idPendiente: number): Promise<boolean> => {
    setLoading(true);
    setError(null);
    try {
      await invoke<string>('eliminar_asignatura_pendiente', { idPendiente });
      mostrarMensaje('Asignatura pendiente eliminada correctamente', 'exito');
      await cargarPendientes();
      return true;
    } catch (e: any) {
      let msg = 'Error al eliminar asignatura pendiente';
      if (e && typeof e === 'object' && e.message) {
        msg = e.message;
      } else if (typeof e === 'string') {
        msg = e;
      }
      setError(msg);
      mostrarMensaje(msg, 'error');
      return false;
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    cargarPendientes();
  }, [cargarPendientes]);

  return {
    pendientes,
    setPendientes,
    loading,
    exito,
    recargar: cargarPendientes,
    guardarPendientes,
    error,
    obtenerCalificacionesPendiente,
    upsertCalificacionesPendiente,
    eliminarAsignaturaPendiente,
  };
} 