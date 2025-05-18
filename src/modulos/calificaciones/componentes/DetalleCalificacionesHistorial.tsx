import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

interface CalificacionEstudiante {
  id_calificacion?: number;
  id_asignatura: number;
  nombre_asignatura: string;
  lapso_1?: number;
  lapso_1_ajustado?: number;
  lapso_2?: number;
  lapso_2_ajustado?: number;
  lapso_3?: number;
  lapso_3_ajustado?: number;
  nota_final?: number;
  revision?: string;
}

interface Asignatura {
  id_asignatura: number;
  nombre_asignatura: string;
  id_grado: number;
  id_modalidad: number;
}

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
        setAsignaturas(asignaturasData);
        const calificacionesData = await invoke<CalificacionEstudiante[]>('obtener_calificaciones_estudiante', {
          idEstudiante,
          idPeriodo,
        });
        setCalificaciones(calificacionesData);
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
          <table className="min-w-full text-xs md:text-sm divide-y divide-emerald-400 dark:divide-cyan-800 rounded-xl overflow-hidden shadow-lg border border-emerald-200 dark:border-cyan-800">
            <thead className="sticky top-0 z-30 bg-gradient-to-r from-emerald-900 via-dark-800 to-dark-900 dark:from-[#059669] dark:via-[#2563eb] dark:to-[#181f2a] text-white">
              <tr>
                <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ASIGNATURA</th>
                <th className="px-1 py-2 text-center font-bold uppercase whitespace-nowrap">1ER LAPSO</th>
                <th className="px-1 py-2 text-center font-bold uppercase whitespace-nowrap">AJUSTE 1</th>
                <th className="px-1 py-2 text-center font-bold uppercase whitespace-nowrap">2DO LAPSO</th>
                <th className="px-1 py-2 text-center font-bold uppercase whitespace-nowrap">AJUSTE 2</th>
                <th className="px-1 py-2 text-center font-bold uppercase whitespace-nowrap">3ER LAPSO</th>
                <th className="px-1 py-2 text-center font-bold uppercase whitespace-nowrap">AJUSTE 3</th>
                <th className="px-1 py-2 text-center font-bold uppercase whitespace-nowrap">FINAL</th>
                <th className="px-1 py-2 text-center font-bold uppercase whitespace-nowrap">REVISIÓN</th>
              </tr>
            </thead>
            <tbody>
              {asignaturas.length === 0 && (
                <tr>
                  <td colSpan={9} className="text-center py-4 text-gray-400">No hay asignaturas registradas para este estudiante.</td>
                </tr>
              )}
              {asignaturas.map((asig, idx) => {
                const calif = calificaciones.find(c => c.id_asignatura === asig.id_asignatura) || {
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
                };
                return (
                  <tr key={asig.id_asignatura} className={
                    (idx % 2 === 0 ? 'bg-white dark:bg-[#232c3d]' : 'bg-gray-100 dark:bg-[#222b3a]') +
                    ' transition hover:dark:bg-[#2563eb]/10 hover:bg-emerald-100'
                  }>
                    <td className="px-2 py-2 font-bold text-emerald-700 dark:text-cyan-200 whitespace-nowrap text-xs md:text-sm">
                      <div className="flex items-center gap-2 md:gap-3">
                        <span className="flex items-center justify-center w-7 h-7 md:w-8 md:h-8 rounded-lg font-bold text-white text-xs md:text-base shadow" style={{ background: '#2563eb' }}>
                          {asig.nombre_asignatura?.[0] ?? ''}
                        </span>
                        <span className="dark:text-cyan-200 text-emerald-700">{asig.nombre_asignatura}</span>
                      </div>
                    </td>
                    {/* Lapso 1 */}
                    <td className="px-1 py-1 align-middle text-center">
                      <input
                        type="number"
                        min={0}
                        max={20}
                        value={padNota((calif as Partial<CalificacionEstudiante>).lapso_1 as number)}
                        onChange={e => handleInputChange(asig.id_asignatura, 'lapso_1', e.target.value)}
                        className="w-10 md:w-14 h-8 md:h-10 px-1 md:px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-xs md:text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
                      />
                    </td>
                    {/* Ajuste 1 */}
                    <td className="px-1 py-1 align-middle text-center">
                      <input
                        type="number"
                        min={0}
                        max={20}
                        value={padNota((calif as Partial<CalificacionEstudiante>).lapso_1_ajustado as number)}
                        onChange={e => handleInputChange(asig.id_asignatura, 'lapso_1_ajustado', e.target.value)}
                        className={`w-10 md:w-14 h-8 md:h-10 px-1 md:px-2 py-1 rounded-lg border-2 ${errores[`${asig.id_asignatura}_lapso_1_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-xs md:text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
                      />
                      {errores[`${asig.id_asignatura}_lapso_1_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_1_ajustado`]}</div>}
                    </td>
                    {/* Lapso 2 */}
                    <td className="px-1 py-1 align-middle text-center">
                      <input
                        type="number"
                        min={0}
                        max={20}
                        value={padNota((calif as Partial<CalificacionEstudiante>).lapso_2 as number)}
                        onChange={e => handleInputChange(asig.id_asignatura, 'lapso_2', e.target.value)}
                        className="w-10 md:w-14 h-8 md:h-10 px-1 md:px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-xs md:text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
                      />
                    </td>
                    {/* Ajuste 2 */}
                    <td className="px-1 py-1 align-middle text-center">
                      <input
                        type="number"
                        min={0}
                        max={20}
                        value={padNota((calif as Partial<CalificacionEstudiante>).lapso_2_ajustado as number)}
                        onChange={e => handleInputChange(asig.id_asignatura, 'lapso_2_ajustado', e.target.value)}
                        className={`w-10 md:w-14 h-8 md:h-10 px-1 md:px-2 py-1 rounded-lg border-2 ${errores[`${asig.id_asignatura}_lapso_2_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-xs md:text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
                      />
                      {errores[`${asig.id_asignatura}_lapso_2_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_2_ajustado`]}</div>}
                    </td>
                    {/* Lapso 3 */}
                    <td className="px-1 py-1 align-middle text-center">
                      <input
                        type="number"
                        min={0}
                        max={20}
                        value={padNota((calif as Partial<CalificacionEstudiante>).lapso_3 as number)}
                        onChange={e => handleInputChange(asig.id_asignatura, 'lapso_3', e.target.value)}
                        className="w-10 md:w-14 h-8 md:h-10 px-1 md:px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-xs md:text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
                      />
                    </td>
                    {/* Ajuste 3 */}
                    <td className="px-1 py-1 align-middle text-center">
                      <input
                        type="number"
                        min={0}
                        max={20}
                        value={padNota((calif as Partial<CalificacionEstudiante>).lapso_3_ajustado as number)}
                        onChange={e => handleInputChange(asig.id_asignatura, 'lapso_3_ajustado', e.target.value)}
                        className={`w-10 md:w-14 h-8 md:h-10 px-1 md:px-2 py-1 rounded-lg border-2 ${errores[`${asig.id_asignatura}_lapso_3_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-xs md:text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
                      />
                      {errores[`${asig.id_asignatura}_lapso_3_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_3_ajustado`]}</div>}
                    </td>
                    {/* Final */}
                    <td className="px-1 py-1 w-16 align-middle text-center">
                      <div className="flex flex-col items-center gap-1">
                        <input
                          type="text"
                          value={padNota((calif as Partial<CalificacionEstudiante>).nota_final as number) || padNota(calcularNotaFinal(calif as CalificacionEstudiante))}
                          readOnly
                          className="w-10 md:w-14 h-8 md:h-10 px-1 md:px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-emerald-50 dark:bg-[#232c3d] text-center text-emerald-700 dark:text-cyan-200 font-normal shadow text-xs md:text-sm"
                        />
                        {calif.revision && !isNaN(Number(calif.revision)) && Number(calif.revision) > 0 && (
                          <span className="text-xs text-blue-500 font-bold">Rev: {padNota(Number(calif.revision))}</span>
                        )}
                      </div>
                    </td>
                    {/* Revisión */}
                    <td className="px-1 py-1 w-16 align-middle text-center">
                      <input
                        type="text"
                        value={(calif as Partial<CalificacionEstudiante>).revision ?? ''}
                        onChange={e => handleInputChange(asig.id_asignatura, 'revision', e.target.value)}
                        className="w-10 md:w-14 h-8 md:h-10 px-1 md:px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-xs md:text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
                      />
                    </td>
                  </tr>
                );
              })}
              {/* Fila de promedios */}
              <tr className="bg-gradient-to-r from-emerald-900 via-dark-800 to-dark-900 dark:from-[#059669] dark:via-[#2563eb] dark:to-[#181f2a] font-bold">
                <td className="px-4 py-2 w-48 text-right text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent"></td>
                <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
                  <div className="flex items-center justify-center h-full">{padNota(Number(promedioLapso('lapso_1', 'lapso_1_ajustado')))}</div>
                </td>
                <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>
                <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
                  <div className="flex items-center justify-center h-full">{padNota(Number(promedioLapso('lapso_2', 'lapso_2_ajustado')))}</div>
                </td>
                <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>
                <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
                  <div className="flex items-center justify-center h-full">{padNota(Number(promedioLapso('lapso_3', 'lapso_3_ajustado')))}</div>
                </td>
                <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>
                <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
                  <div className="flex items-center justify-center h-full">{padNota(Number(promedioFinal()))}</div>
                </td>
                <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>
              </tr>
            </tbody>
          </table>
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