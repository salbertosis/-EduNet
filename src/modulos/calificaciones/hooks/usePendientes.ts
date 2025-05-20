import { useState, useEffect, useCallback } from 'react';
import { guardarPendientesAPI, cargarPendientesAPI } from '../services/pendientesService';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

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

export function usePendientes(estudianteId: number) {
  // Estado y lógica para cargar y guardar asignaturas pendientes
  const [pendientes, setPendientes] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [exito, setExito] = useState(false);
  const { mostrarMensaje } = useMensajeGlobal();

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
  };
} 