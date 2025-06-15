import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { CalificacionEstudiante, Asignatura } from '../types';

interface DetalleCalificacionesHistorialProps {
  idEstudiante: number;
  idPeriodo: number;
}

export function DetalleCalificacionesHistorial({ idEstudiante, idPeriodo }: DetalleCalificacionesHistorialProps) {
  const [asignaturas, setAsignaturas] = useState<Asignatura[]>([]);
  const [calificaciones, setCalificaciones] = useState<CalificacionEstudiante[]>([]);
  const [cargando, setCargando] = useState(true);
  const [mensajeGuardado, setMensajeGuardado] = useState<string | null>(null);
  const { mostrarMensaje } = useMensajeGlobal();

  useEffect(() => {
    const cargarDatos = async () => {
      setCargando(true);
      try {
        const estudiante = await invoke<any>('obtener_estudiante_por_id', { id: idEstudiante });
        let id_grado = estudiante.id_grado;
        let id_modalidad = estudiante.id_modalidad;
        if (estudiante.id_grado_secciones) {
          const gradoSeccion = await invoke<any>('obtener_grado_secciones_por_id', { id: estudiante.id_grado_secciones });
          id_grado = gradoSeccion.id_grado;
          id_modalidad = gradoSeccion.id_modalidad;
        }
        const asignaturasData = await invoke<Asignatura[]>('obtener_asignaturas_por_grado_modalidad', {
          idGrado: id_grado,
          idModalidad: id_modalidad,
        });
        const calificacionesData = await invoke<CalificacionEstudiante[]>('obtener_calificaciones_estudiante', {
          idEstudiante: idEstudiante,
          idPeriodo: idPeriodo,
        });
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

  // Función para obtener la nota válida (revisión si existe, si no final calculada)
  const obtenerNotaValida = (calif: CalificacionEstudiante): number => {
    const revision = Number(calif.revision);
    if (!isNaN(revision) && revision > 0) return revision;
    // Calcula nota final
    const l1 = calif.lapso_1_ajustado ?? calif.lapso_1 ?? 0;
    const l2 = calif.lapso_2_ajustado ?? calif.lapso_2 ?? 0;
    const l3 = calif.lapso_3_ajustado ?? calif.lapso_3 ?? 0;
    if (l1 && l2 && l3) {
      return Math.round((l1 + l2 + l3) / 3);
    }
    return 0;
  };

  const handleGuardarHistorial = async () => {
    if (!idPeriodo || idPeriodo === 0) {
      mostrarMensaje('No hay período escolar seleccionado', 'error');
      return;
    }
    if (!asignaturas.length) {
      mostrarMensaje('No hay asignaturas para guardar historial', 'error');
      return;
    }
    if (!calificaciones.length) {
      mostrarMensaje('No hay calificaciones para guardar historial', 'error');
      return;
    }
    try {
      const estudiante = await invoke<any>('obtener_estudiante_por_id', { id: idEstudiante });
      const id_grado_secciones = estudiante.id_grado_secciones;
      if (!id_grado_secciones) {
        mostrarMensaje('El estudiante no tiene un grado y sección asignado', 'error');
        return;
      }
      await invoke('upsert_historial_academico', {
        idEstudiante: estudiante.id,
        idPeriodo: Number(idPeriodo),
        idGradoSecciones: id_grado_secciones,
      });
      setMensajeGuardado('Historial guardado correctamente');
    } catch (error) {
      const mensajeError = error instanceof Error ? error.message : 'Error desconocido al guardar el historial';
      setMensajeGuardado('Error al guardar el historial: ' + mensajeError);
      if (typeof mostrarMensaje === 'function') {
        mostrarMensaje('Error al guardar el historial: ' + mensajeError, 'error');
      }
    }
  };

  const periodoValido = !!idPeriodo && idPeriodo > 0;

  // Preprocesar calificaciones para mostrar la suma de lapso y ajuste en cada lapso
  const calificacionesSumadas = calificaciones.map(c => ({
    ...c,
    lapso_1: (c.lapso_1 ?? 0) + (c.lapso_1_ajustado ?? 0),
    lapso_2: (c.lapso_2 ?? 0) + (c.lapso_2_ajustado ?? 0),
    lapso_3: (c.lapso_3 ?? 0) + (c.lapso_3_ajustado ?? 0),
    lapso_1_ajustado: undefined,
    lapso_2_ajustado: undefined,
    lapso_3_ajustado: undefined,
  }));

  return (
    <div className="w-full flex flex-col items-center">
      {cargando ? (
        <div className="text-center py-8 text-gray-400">Cargando calificaciones...</div>
      ) : (
        <div className="w-full max-w-5xl bg-white dark:bg-[#1e293b] rounded-2xl shadow-2xl p-6 mb-4 flex flex-col" style={{ maxHeight: '70vh', overflowY: 'auto' }}>
          <table className="w-full rounded-xl overflow-hidden shadow-lg">
            <thead className="sticky top-0 z-30 bg-gradient-to-r from-emerald-700 via-cyan-700 to-blue-700 dark:from-[#059669] dark:via-[#2563eb] dark:to-[#181f2a] text-white shadow-md">
              <tr>
                <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ASIGNATURA</th>
                <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">LAPSO 1</th>
                <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">LAPSO 2</th>
                <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">LAPSO 3</th>
                <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">FINAL</th>
                <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">REVISIÓN</th>
                <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ESTADO</th>
              </tr>
            </thead>
            <tbody>
              {asignaturas.map((asig, idx) => {
                const calif: Partial<CalificacionEstudiante> = calificacionesSumadas.find(c => c.id_asignatura === asig.id_asignatura) || {};
                const notaColor = (nota: number | undefined) => typeof nota === 'number' ? (nota >= 10 ? 'text-emerald-600 dark:text-emerald-300 font-bold' : 'text-red-500 font-bold') : 'text-gray-700 dark:text-gray-200';
                const badgeEstado = () => {
                  const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : 0;
                  const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
                  const revisionHabilitada = notaFinal < 9.5;
                  if (revisionHabilitada) {
                    if (revision !== undefined && !isNaN(revision)) {
                      if (revision >= 10) return <span className="px-3 py-1 rounded-full text-xs font-bold bg-emerald-100 dark:bg-cyan-800/30 text-emerald-700 dark:text-cyan-200 border border-emerald-400 dark:border-cyan-400 shadow">Aprobado</span>;
                      if (revision < 10) return <span className="px-3 py-1 rounded-full text-xs font-bold bg-yellow-100 dark:bg-yellow-800/30 text-yellow-700 dark:text-yellow-200 border border-yellow-400 dark:border-yellow-400 shadow">Pendiente</span>;
                    }
                    return <span className="px-3 py-1 rounded-full text-xs font-bold bg-cyan-100 dark:bg-cyan-800/30 text-cyan-700 dark:text-cyan-200 border border-cyan-400 dark:border-cyan-400 shadow">Revisión</span>;
                  }
                  if (notaFinal >= 9.5) return <span className="px-3 py-1 rounded-full text-xs font-bold bg-emerald-100 dark:bg-cyan-800/30 text-emerald-700 dark:text-cyan-200 border border-emerald-400 dark:border-cyan-400 shadow">Aprobado</span>;
                  return <span className="px-3 py-1 rounded-full text-xs font-bold bg-cyan-100 dark:bg-cyan-800/30 text-cyan-700 dark:text-cyan-200 border border-cyan-400 dark:border-cyan-400 shadow">Revisión</span>;
                };
                return (
                  <tr key={asig.id_asignatura} className={
                    (idx % 2 === 0 ? 'bg-white dark:bg-[#232c3d]' : 'bg-gray-50 dark:bg-[#222b3a]') +
                    ' transition hover:dark:bg-[#2563eb]/10 hover:bg-emerald-50 dark:hover:bg-cyan-900/20'
                  }>
                    <td className="px-2 py-2 font-bold text-emerald-700 dark:text-cyan-200 whitespace-nowrap text-xs md:text-sm max-w-[180px] truncate overflow-hidden text-ellipsis">
                      <div className="flex items-center gap-2 md:gap-3">
                        <span className="flex items-center justify-center w-7 h-7 md:w-8 md:h-8 rounded-lg font-bold text-white text-xs md:text-base shadow bg-gradient-to-br from-blue-600 to-cyan-500">
                          {asig.nombre_asignatura?.[0] ?? ''}
                        </span>
                        <span className="dark:text-cyan-200 text-emerald-700 font-semibold tracking-wide truncate max-w-[120px]">{asig.nombre_asignatura}</span>
                      </div>
                    </td>
                    <td className={`px-2 py-2 align-middle text-center ${notaColor(calif.lapso_1)}`}>{typeof calif.lapso_1 === 'number' && calif.lapso_1 > 0 ? padNota(calif.lapso_1) : ''}</td>
                    <td className={`px-2 py-2 align-middle text-center ${notaColor(calif.lapso_2)}`}>{typeof calif.lapso_2 === 'number' && calif.lapso_2 > 0 ? padNota(calif.lapso_2) : ''}</td>
                    <td className={`px-2 py-2 align-middle text-center ${notaColor(calif.lapso_3)}`}>{typeof calif.lapso_3 === 'number' && calif.lapso_3 > 0 ? padNota(calif.lapso_3) : ''}</td>
                    <td className={`px-2 py-2 align-middle text-center ${notaColor(calif.nota_final)}`}>{typeof calif.nota_final === 'number' ? padNota(calif.nota_final) : ''}</td>
                    <td className="px-2 py-2 align-middle text-center font-bold">{calif.revision !== undefined && calif.revision !== '' ? padNota(Number(calif.revision)) : <span className="text-gray-400">—</span>}</td>
                    <td className="px-2 py-2 align-middle text-center">{badgeEstado()}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
          <div className="mt-4 text-right flex flex-row items-center justify-end gap-4 w-full max-w-5xl bg-white dark:bg-dark-800 pb-2 pt-2 z-10 rounded-b-2xl">
            {mensajeGuardado && <div className="mb-2 text-green-400 font-semibold">{mensajeGuardado}</div>}
            {!periodoValido && (
              <div className="mb-2 text-red-400 font-semibold">Debes seleccionar un período escolar válido para guardar el historial.</div>
            )}
            <button
              className="px-8 py-2 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-bold shadow-lg hover:scale-105 hover:bg-emerald-700 transition-all"
              onClick={handleGuardarHistorial}
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