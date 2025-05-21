import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { TablaCalificaciones } from './DetalleCalificaciones/TablaCalificaciones';
import { CalificacionEstudiante, Asignatura } from '../types';

interface DetalleCalificacionesHistorialProps {
  idEstudiante: number;
  idPeriodo: number;
}

export function DetalleCalificacionesHistorial({ idEstudiante, idPeriodo }: DetalleCalificacionesHistorialProps) {
  const [asignaturas, setAsignaturas] = useState<Asignatura[]>([]);
  const [calificaciones, setCalificaciones] = useState<CalificacionEstudiante[]>([]);
  const [cargando, setCargando] = useState(true);
  const [errores, setErrores] = useState<Record<string, string>>({});
  const [mensajeGuardado, setMensajeGuardado] = useState<string | null>(null);
  const { mostrarMensaje } = useMensajeGlobal();

  console.log('[DEBUG] Renderizando DetalleCalificacionesHistorial');

  useEffect(() => {
    const cargarDatos = async () => {
      setCargando(true);
      try {
        const estudiante = await invoke<any>('obtener_estudiante_por_id', { id: idEstudiante });
        const asignaturasData = await invoke<Asignatura[]>('obtener_asignaturas_por_grado_modalidad', {
          idGrado: estudiante.id_grado,
          idModalidad: estudiante.id_modalidad,
        });
        console.log('[DEBUG] AsignaturasData:', asignaturasData);
        console.log('[DEBUG][FRONTEND] Invocando obtener_calificaciones_estudiante con', { idEstudiante: idEstudiante, idPeriodo: idPeriodo });
        const calificacionesData = await invoke<CalificacionEstudiante[]>('obtener_calificaciones_estudiante', {
          idEstudiante: idEstudiante,
          idPeriodo: idPeriodo,
        });
        console.log('[DEBUG] CalificacionesData:', calificacionesData);
        // Construir array completo de calificaciones
        const calificacionesCompletas = asignaturasData.map(asig => {
          return (
            calificacionesData.find(c => c.id_asignatura === asig.id_asignatura) || {
              id_asignatura: asig.id_asignatura,
              nombre_asignatura: asig.nombre_asignatura,
              lapso_1: undefined,
              lapso_1_ajustado: undefined,
              lapso_2: undefined,
              lapso_2_ajustado: undefined,
              lapso_3: undefined,
              lapso_3_ajustado: undefined,
              nota_final: undefined,
              revision: ''
            }
          );
        });
        console.log('[DEBUG] CalificacionesCompletas:', calificacionesCompletas);
        setAsignaturas(asignaturasData);
        setCalificaciones(calificacionesCompletas);
      } catch (error) {
        // Manejo de errores
      } finally {
        setCargando(false);
      }
    };
    cargarDatos();
  }, [idEstudiante, idPeriodo]);

  function padNota(nota: number | undefined): string {
    if (typeof nota === 'number') return nota.toString().padStart(2, '0');
    return '';
  }

  const handleInputChange = (id_asignatura: number, campo: keyof CalificacionEstudiante, valor: string) => {
    let num: number | undefined = valor === '' ? undefined : Number(valor);
    let error = '';
    if ([
      'lapso_1', 'lapso_2', 'lapso_3',
      'lapso_1_ajustado', 'lapso_2_ajustado', 'lapso_3_ajustado',
    ].includes(campo) && num !== undefined) {
      if (num > 20) {
        num = 20;
        error = 'La calificación máxima es 20';
      }
      if (num < 0) {
        num = 0;
        error = 'La calificación mínima es 0';
      }
    }
    setCalificaciones(prev => {
      const idx = prev.findIndex(c => c.id_asignatura === id_asignatura);
      let updated = [...prev];
      let calif = idx === -1 ? {
        id_asignatura,
        nombre_asignatura: asignaturas.find(a => a.id_asignatura === id_asignatura)?.nombre_asignatura || '',
      } as CalificacionEstudiante : { ...updated[idx] };
      if (campo.includes('ajustado')) {
        const lapsoCampo = campo.replace('_ajustado', '') as keyof CalificacionEstudiante;
        const lapsoValorRaw = calif[lapsoCampo];
        const lapsoValor = typeof lapsoValorRaw === 'number' ? lapsoValorRaw : 0;
        if (num !== undefined) {
          if (num < lapsoValor) {
            error = 'El ajuste no puede ser menor que la nota original';
            num = lapsoValor;
          }
          if (num > lapsoValor + 2) {
            error = 'El ajuste no puede ser mayor que la nota original +2';
            num = lapsoValor + 2;
          }
        }
      }
      (calif as any)[campo] = num;
      if (idx === -1) {
        updated.push(calif);
      } else {
        updated[idx] = calif;
      }
      setErrores(prevErr => ({ ...prevErr, [`${id_asignatura}_${campo}`]: error }));
      return updated;
    });
  };

  const calcularNotaFinal = (calificacion: CalificacionEstudiante): number => {
    const l1 = calificacion.lapso_1_ajustado ?? calificacion.lapso_1 ?? 0;
    const l2 = calificacion.lapso_2_ajustado ?? calificacion.lapso_2 ?? 0;
    const l3 = calificacion.lapso_3_ajustado ?? calificacion.lapso_3 ?? 0;
    if (l1 && l2 && l3) {
      return Math.round((l1 + l2 + l3) / 3);
    }
    return 0;
  };

  const handleGuardar = async () => {
    setMensajeGuardado(null);
    setErrores({});
    let erroresNuevos: Record<string, string> = {};
    let exito = true;
    await Promise.all(calificaciones.map(async (calif) => {
      const calificacionBackend = {
        id_calificacion: calif.id_calificacion ?? null,
        id_estudiante: idEstudiante,
        id_asignatura: calif.id_asignatura,
        id_periodo: idPeriodo,
        lapso_1: calif.lapso_1 ?? null,
        lapso_1_ajustado: calif.lapso_1_ajustado ?? null,
        lapso_2: calif.lapso_2 ?? null,
        lapso_2_ajustado: calif.lapso_2_ajustado ?? null,
        lapso_3: calif.lapso_3 ?? null,
        lapso_3_ajustado: calif.lapso_3_ajustado ?? null,
        revision: calif.revision ?? null,
        nota_final: calcularNotaFinal(calif),
      };
      try {
        await invoke('guardar_calificacion', { calificacion: calificacionBackend });
      } catch (err: any) {
        exito = false;
        if (typeof err === 'string' && err.includes('ajuste')) {
          if (err.includes('1er lapso')) erroresNuevos[`${calif.id_asignatura}_lapso_1_ajustado`] = err;
          if (err.includes('2do lapso')) erroresNuevos[`${calif.id_asignatura}_lapso_2_ajustado`] = err;
          if (err.includes('3er lapso')) erroresNuevos[`${calif.id_asignatura}_lapso_3_ajustado`] = err;
        } else {
          erroresNuevos[`${calif.id_asignatura}_general`] = typeof err === 'string' ? err : 'Error al guardar';
        }
      }
    }));
    setErrores(erroresNuevos);
    if (exito && Object.keys(erroresNuevos).length === 0) {
      setMensajeGuardado('Cambios guardados exitosamente');
    } else {
      setMensajeGuardado(null);
    }
  };

  // Función para obtener la nota válida (revisión si existe, si no final calculada)
  const obtenerNotaValida = (calif: CalificacionEstudiante): number => {
    const revision = Number(calif.revision);
    if (!isNaN(revision) && revision > 0) return revision;
    return calcularNotaFinal(calif);
  };

  // Promedio de cada lapso (usando ajustes si existen, excluyendo ids 9 y 11)
  const promedioLapso = (lapso: keyof CalificacionEstudiante, lapsoAjuste: keyof CalificacionEstudiante) => {
    const notas = asignaturas
      .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
      .map(a => {
        const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
        if (!calif) return undefined;
        if (typeof calif[lapsoAjuste] === 'number') return calif[lapsoAjuste] as number;
        if (typeof calif[lapso] === 'number') return calif[lapso] as number;
        return undefined;
      })
      .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
    if (notas.length === 0) return '';
    return (notas.reduce((a, b) => a + b, 0) / notas.length).toFixed(2);
  };

  // Promedio final (usando revisión si existe, excluyendo ids 9 y 11)
  const promedioFinal = () => {
    const notas = asignaturas
      .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
      .map(a => {
        const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
        return calif ? obtenerNotaValida(calif) : undefined;
      })
      .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
    if (notas.length === 0) return '';
    return (notas.reduce((a, b) => a + b, 0) / notas.length).toFixed(2);
  };

  // Lógica para guardar o actualizar historial académico
  const handleGuardarHistorial = async () => {
    console.log('[DEBUG] handleGuardarHistorial ejecutado');
    console.log('[DEBUG] INICIO handleGuardarHistorial');
    console.log('[DEBUG] idPeriodo:', idPeriodo);
    console.log('[DEBUG] asignaturas:', asignaturas);
    console.log('[DEBUG] calificaciones:', calificaciones);

    if (!idPeriodo || idPeriodo === 0) {
      console.error('[ERROR] idPeriodo inválido:', idPeriodo);
      mostrarMensaje('No hay período escolar seleccionado', 'error');
      return;
    }
    if (!asignaturas.length) {
      console.error('[ERROR] No hay asignaturas');
      mostrarMensaje('No hay asignaturas para guardar historial', 'error');
      return;
    }
    if (!calificaciones.length) {
      console.error('[ERROR] No hay calificaciones');
      mostrarMensaje('No hay calificaciones para guardar historial', 'error');
      return;
    }

    try {
      // Obtener el estudiante para obtener su id_grado_secciones
      console.log('[DEBUG][FRONTEND] Obteniendo datos del estudiante...');
      const estudiante = await invoke<any>('obtener_estudiante_por_id', { id: idEstudiante });
      console.log('[DEBUG][FRONTEND] Datos del estudiante:', estudiante);
      
      const id_grado_secciones = estudiante.id_grado_secciones;
      if (!id_grado_secciones) {
        mostrarMensaje('El estudiante no tiene un grado y sección asignado', 'error');
        return;
      }

      // Calcular promedio anual
      const notasValidas = asignaturas
        .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
        .map(a => {
          const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
          return calif ? obtenerNotaValida(calif) : undefined;
        })
        .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
      
      const promedio = notasValidas.length > 0 ? (notasValidas.reduce((a, b) => a + b, 0) / notasValidas.length) : 0;

      // Calcular estatus
      const pendientes = asignaturas
        .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
        .map(a => {
          const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
          const nota = calif ? obtenerNotaValida(calif) : 0;
          return nota < 9.5;
        })
        .filter(Boolean).length;
      
      let estatus = 'Aprobado';
      if (pendientes >= 3) estatus = 'Repite';
      else if (pendientes === 2) estatus = 'Aprobado';
      else if (pendientes === 1) estatus = 'Aprobado';

      const params = {
        idEstudiante: estudiante.id,
        idPeriodo: Number(idPeriodo),
        idGradoSecciones: id_grado_secciones,
      };
      console.log('[DEBUG][HISTORIAL] Params enviados a upsert_historial_academico:', params);
      await invoke('upsert_historial_academico', params);
      console.log('[DEBUG][HISTORIAL] Llamada a upsert_historial_academico finalizada');
      setMensajeGuardado('Historial guardado correctamente');
    } catch (error) {
      console.error('[ERROR][FRONTEND] Error detallado:', error);
      const mensajeError = error instanceof Error ? error.message : 'Error desconocido al guardar el historial';
      setMensajeGuardado('Error al guardar el historial: ' + mensajeError);
      if (typeof mostrarMensaje === 'function') {
        mostrarMensaje('Error al guardar el historial: ' + mensajeError, 'error');
      }
    }
  };

  const periodoValido = !!idPeriodo && idPeriodo > 0;

  console.log("[DEBUG] Renderizando botón Guardar Historial");

  return (
    <div>
      {cargando ? (
        <div className="text-center py-8 text-gray-400">Cargando calificaciones...</div>
      ) : (
        <div className="overflow-x-auto rounded-xl">
          <TablaCalificaciones
            asignaturas={asignaturas}
            calificaciones={calificaciones}
            errores={errores}
            mostrarAjustes={true}
            onInputChange={handleInputChange}
          />
          <div className="mt-4 text-right flex flex-row items-center justify-end gap-4 sticky bottom-0 bg-white dark:bg-dark-800 pb-2 pt-2 z-10">
            {mensajeGuardado && <div className="mb-2 text-green-400 font-semibold">{mensajeGuardado}</div>}
            {!periodoValido && (
              <div className="mb-2 text-red-400 font-semibold">Debes seleccionar un período escolar válido para guardar el historial.</div>
            )}
            <button
              className="px-8 py-2 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-bold shadow-lg hover:scale-105 hover:bg-emerald-700 transition-all"
              onClick={handleGuardar}
              disabled={!periodoValido}
            >
              Guardar Cambios
            </button>
            <button
              className="px-8 py-2 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-bold shadow-lg hover:scale-105 hover:bg-emerald-700 transition-all"
              onClick={() => {
                if (!periodoValido) {
                  mostrarMensaje('Debes seleccionar un período escolar válido para guardar el historial', 'error');
                  return;
                }
                console.log("[DEBUG] Click en Guardar Historial");
                handleGuardarHistorial();
              }}
              disabled={!periodoValido}
            >
              Guardar Historial
            </button>
          </div>
        </div>
      )}
    </div>
  );
} 