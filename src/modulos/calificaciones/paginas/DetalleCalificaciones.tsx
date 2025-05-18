import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { HistorialAcademico } from '../componentes/HistorialAcademico';
import { AsignaturasPendientes } from '../componentes/AsignaturasPendientes';
import { useCalificaciones } from '../hooks/useCalificaciones';
import { useAsignaturas } from '../hooks/useAsignaturas';
import { TablaCalificaciones } from '../componentes/DetalleCalificaciones/TablaCalificaciones';
import { useInputCalificaciones } from '../hooks/useInputCalificaciones';
import { useGuardarHistorial } from '../hooks/useGuardarHistorial';

const TABS = [
  { key: 'datos', label: 'Datos Personales' },
  { key: 'actual', label: 'Calificaciones Año Actual' },
  { key: 'historial', label: 'Historial Académico' },
  { key: 'pendientes', label: 'Asignaturas Pendientes' },
];

interface EstudianteDetalle {
  id: number;
  cedula: string | undefined;
  apellidos: string | undefined;
  nombres: string | undefined;
  nombre_grado: string | undefined;
  nombre_seccion: string | undefined;
  nombre_modalidad: string | undefined;
  id_grado?: number;
  id_modalidad?: number;
  id_grado_secciones?: number;
}

interface DetalleCalificacionesProps {
  estudiante: EstudianteDetalle;
  onVolver: () => void;
}

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

const ASIGNATURAS = [
  { id: 1, nombre: 'Lengua y Literatura' },
  { id: 2, nombre: 'Matemática' },
  { id: 3, nombre: 'Inglés' },
  { id: 4, nombre: 'Educación Física' },
  { id: 5, nombre: 'Física' },
  { id: 6, nombre: 'Química' },
];

const PENDIENTES_FICTICIAS = [
  { periodo: '2023-2024', grado: '2do Año', asignatura: 'Física', calificacion: 9, estado: 'Pendiente' },
  { periodo: '2022-2023', grado: '1er Año', asignatura: 'Matemática', calificacion: 10, estado: 'Pendiente' },
];

// Definir la interfaz para el historial académico
interface HistorialAcademico {
  id_historial: number;
  id_estudiante: number;
  id_periodo: number;
  id_grado_secciones: number;
  promedio_anual: number;
  estatus: string;
  fecha_registro: string;
  periodo_escolar: string | null;
  grado: string | null;
  seccion: string | null;
}

export function DetalleCalificaciones({ estudiante, onVolver }: DetalleCalificacionesProps) {
  const [tab, setTab] = useState('datos');
  const { asignaturas, loading: loadingAsignaturas, error: errorAsignaturas } = useAsignaturas(estudiante.id_grado ?? 0, estudiante.id_modalidad ?? 0);
  const [periodoActual, setPeriodoActual] = useState<number | null>(null);
  const { calificaciones, setCalificaciones, loading: loadingCalificaciones, error: errorCalificaciones } = useCalificaciones(estudiante.id, periodoActual ?? 0);
  const [mostrarAjustes, setMostrarAjustes] = useState(false);
  const [errores, setErrores] = useState<Record<string, string>>({});
  const [mensajeGuardado, setMensajeGuardado] = useState<string | null>(null);
  const { mostrarMensaje } = useMensajeGlobal();
  const [mostrarModalGuardarHistorialEstudiante, setMostrarModalGuardarHistorialEstudiante] = useState(false);

  // Hooks especializados
  const { handleInputChange, errores: erroresInput, limpiarErrores } = useInputCalificaciones(asignaturas, calificaciones, setCalificaciones);
  const { guardarHistorial, loading: loadingGuardarHistorial, error: errorGuardarHistorial, exito: exitoGuardarHistorial } = useGuardarHistorial(asignaturas, calificaciones);

  useEffect(() => {
    const cargarPeriodoActual = async () => {
      try {
        const periodos = await invoke<{ id_periodo: number, activo: boolean }[]>('listar_periodos_escolares');
        console.log('[DEBUG] Periodos escolares recibidos:', periodos);
        const periodoActivo = periodos.find(p => p.activo);
        if (periodoActivo) {
          setPeriodoActual(periodoActivo.id_periodo);
        } else {
          setPeriodoActual(null);
        }
      } catch (error) {
        console.error('Error al cargar periodo actual:', error);
        mostrarMensaje('Error al cargar el periodo actual', 'error');
      }
    };
    cargarPeriodoActual();
  }, []);

  // Calcular promedios
  const calcularNotaFinal = (calificacion: CalificacionEstudiante): number => {
    const l1 = calificacion.lapso_1_ajustado ?? calificacion.lapso_1 ?? 0;
    const l2 = calificacion.lapso_2_ajustado ?? calificacion.lapso_2 ?? 0;
    const l3 = calificacion.lapso_3_ajustado ?? calificacion.lapso_3 ?? 0;
    if (l1 && l2 && l3) {
      return Math.round((l1 + l2 + l3) / 3);
    }
    return 0;
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

  // Nueva función para calcular el estado de cada asignatura
  const calcularEstadoAsignatura = (calif: CalificacionEstudiante, totalRevisionMenor10: number) => {
    // Solo mostrar estado si tiene los 3 lapsos
    const lapsosCompletos = [calif.lapso_1_ajustado ?? calif.lapso_1, calif.lapso_2_ajustado ?? calif.lapso_2, calif.lapso_3_ajustado ?? calif.lapso_3].every(n => typeof n === 'number');
    if (!lapsosCompletos) return '';
    const notaFinal = calcularNotaFinal(calif);
    const revision = typeof calif.revision === 'number' ? calif.revision : undefined;
    if (revision !== undefined && revision >= 10) return 'Aprobado';
    if (revision !== undefined && revision < 10) {
      if (totalRevisionMenor10 >= 3) return 'Repite';
      if (totalRevisionMenor10 <= 2) return 'Pendiente';
      return 'Revisión';
    }
    if (notaFinal >= 9.5) return 'Aprobado';
    if (notaFinal < 9.5) return 'Revisión';
    return '';
  };

  // Calcular totales para estados
  const totalRevisionMenor10 = calificaciones.filter(c => typeof c.revision === 'number' && c.revision < 10).length;

  const handleGuardar = async () => {
    console.log('[DEBUG][FRONTEND] Iniciando guardado de calificaciones');
    console.log('[DEBUG][FRONTEND] Estado actual de calificaciones:', calificaciones);
    
    if (!estudiante.id || !periodoActual) {
      console.error('[ERROR][FRONTEND] No se puede guardar: estudiante.id o periodoActual es null', {
        estudianteId: estudiante.id,
        periodoActual
      });
      return;
    }

    setMensajeGuardado(null);
    setErrores({});
    let erroresNuevos: Record<string, string> = {};
    let exito = true;

    console.log('[DEBUG][FRONTEND] Preparando para guardar', {
      totalCalificaciones: calificaciones.length,
      periodoActual,
      estudianteId: estudiante.id
    });

    await Promise.all(calificaciones.map(async (calif) => {
      const calificacionBackend = {
        id_calificacion: calif.id_calificacion ?? null,
        id_estudiante: estudiante.id,
        id_asignatura: calif.id_asignatura,
        id_periodo: periodoActual,
        lapso_1: calif.lapso_1 ?? null,
        lapso_1_ajustado: calif.lapso_1_ajustado ?? null,
        lapso_2: calif.lapso_2 ?? null,
        lapso_2_ajustado: calif.lapso_2_ajustado ?? null,
        lapso_3: calif.lapso_3 ?? null,
        lapso_3_ajustado: calif.lapso_3_ajustado ?? null,
        revision: calif.revision ?? null,
        nota_final: calcularNotaFinal(calif),
      };

      console.log('[DEBUG][FRONTEND] Enviando calificación al backend:', {
        asignatura: calif.nombre_asignatura,
        datos: calificacionBackend
      });

      try {
        const resultado = await invoke('guardar_calificacion', { calificacion: calificacionBackend });
        console.log('[DEBUG][FRONTEND] Respuesta del backend para', calif.nombre_asignatura, ':', resultado);
      } catch (err: any) {
        console.error('[ERROR][FRONTEND] Error al guardar calificación:', {
          asignatura: calif.nombre_asignatura,
          error: err
        });
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

    console.log('[DEBUG][FRONTEND] Proceso de guardado completado', {
      exito,
      errores: erroresNuevos
    });

    setErrores(erroresNuevos);
    if (exito && Object.keys(erroresNuevos).length === 0) {
      setMensajeGuardado('Cambios guardados exitosamente');
      mostrarMensaje('Calificaciones guardadas correctamente', 'exito');
    } else {
      setMensajeGuardado(null);
    }
  };

  // Función para guardar historial
  const handleGuardarHistorial = async () => {
    if (!periodoActual || !estudiante.id_grado_secciones) {
      mostrarMensaje('No se puede guardar el historial: faltan datos necesarios', 'error');
      return;
    }

    try {
      console.log('[DEBUG][FRONTEND] Llamando a obtener_historial_academico_estudiante con id:', estudiante.id);
      const data = await guardarHistorial(estudiante.id, periodoActual, estudiante.id_grado_secciones);
      console.log('[DEBUG][FRONTEND] Respuesta de historial:', data);
      if (exitoGuardarHistorial) {
        setMensajeGuardado('Historial guardado correctamente');
        mostrarMensaje('Historial guardado correctamente', 'exito');
      } else if (errorGuardarHistorial) {
        mostrarMensaje(errorGuardarHistorial, 'error');
      }
    } catch (error) {
      console.error('[ERROR] Error al guardar historial:', error);
      mostrarMensaje('Error al guardar el historial', 'error');
    } finally {
      setMostrarModalGuardarHistorialEstudiante(false);
    }
  };

  // Log antes del renderizado de la tabla
  console.log('[DEBUG] Asignaturas en render:', asignaturas);

  // 1. Agregar función para ceros a la izquierda
  function padNota(nota: number | undefined): string {
    if (typeof nota === 'number') return nota.toString().padStart(2, '0');
    return '';
  }

  // Log global para saber si el componente se monta
  console.log('[DEBUG][GLOBAL] DetalleCalificaciones renderizado');

  // Agregar la función handleGuardarPendientes dentro del componente
  const handleGuardarPendientes = async () => {
    const pendientes = calificaciones.filter(c => typeof c.revision === 'number' && c.revision < 10);
    if (pendientes.length === 0) {
      mostrarMensaje('El estudiante no tiene asignaturas pendientes.', 'advertencia');
      return;
    }
    if (pendientes.length > 2) {
      mostrarMensaje('El estudiante tiene más de 2 asignaturas aplazadas. No se pueden guardar como pendientes.', 'error');
      return;
    }
    try {
      await invoke('guardar_asignaturas_pendientes', {
        idEstudiante: estudiante.id,
        pendientes: pendientes.map(p => ({
          id_asignatura: p.id_asignatura,
          id_periodo: periodoActual,
          revision: p.revision,
        })),
      });
      mostrarMensaje('Asignaturas pendientes guardadas correctamente.', 'exito');
    } catch (error) {
      mostrarMensaje('Error al guardar las asignaturas pendientes.', 'error');
      console.error(error);
    }
  };

  console.log('[DEBUG][COMPONENTE] idEstudiante recibido:', estudiante.id);

  console.log('[DEBUG][DETALLE] Renderizando <HistorialAcademico> con id:', estudiante.id);

  return (
    <div className="max-w-5xl mx-auto p-4 md:p-8 bg-white dark:bg-dark-800 rounded-2xl shadow-lg">
      {/* Advertencia si periodoActual es null, pero no bloquear la interfaz */}
      {periodoActual === null && (
        <div className="mb-4 p-4 bg-yellow-100 text-yellow-800 rounded text-center">
          No hay un periodo escolar activo. No se pueden cargar calificaciones.
        </div>
      )}
      <div className="flex items-center justify-between mb-6 gap-4">
        {/* Avatar y nombre del estudiante */}
        <div className="flex items-center gap-4">
          <div className="flex items-center justify-center w-14 h-14 rounded-full bg-gradient-to-br from-cyan-600 via-blue-700 to-emerald-700 shadow-lg text-3xl font-bold text-white select-none">
            {estudiante?.nombres?.[0] ?? ''}
          </div>
          <h2 className="text-3xl font-extrabold tracking-tight bg-gradient-to-r from-gray-900 via-gray-800 to-gray-700 bg-clip-text text-transparent drop-shadow-lg select-none dark:from-emerald-400 dark:via-cyan-400 dark:to-blue-400 dark:bg-gradient-to-r dark:bg-clip-text dark:text-transparent">
            {estudiante?.nombres} {estudiante?.apellidos}
          </h2>
        </div>
        <button onClick={onVolver} className="px-6 py-2 md:px-8 md:py-2 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-semibold shadow-lg hover:scale-105 hover:bg-emerald-700 transition-all">
          Volver
        </button>
      </div>
      <div className="flex gap-2 border-b-2 border-emerald-400 dark:border-emerald-700 mb-8">
        {TABS.map(t => (
          <button
            key={t.key}
            onClick={() => setTab(t.key)}
            className={`px-4 py-2 font-semibold rounded-t-lg transition-all duration-200 focus:outline-none text-base tracking-wide shadow-sm ${tab === t.key ? 'bg-gradient-to-r from-emerald-400 via-cyan-400 to-blue-400 text-white shadow-emerald-800/30' : 'bg-gray-100 dark:bg-dark-700 text-gray-600 dark:text-gray-300 hover:bg-emerald-900/30 hover:text-emerald-300'}`}
            style={{marginBottom: '-2px'}}
          >
            {t.label}
          </button>
        ))}
      </div>
      <div className="space-y-6">
        {tab === 'datos' && (
          <section className="space-y-4">
            <h3 className="text-xl font-semibold mb-4">Datos Personales</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              <div><span className="font-bold">Cédula:</span> {estudiante?.cedula}</div>
              <div><span className="font-bold">Apellidos:</span> {estudiante?.apellidos}</div>
              <div><span className="font-bold">Nombres:</span> {estudiante?.nombres}</div>
              <div><span className="font-bold">Grado:</span> {estudiante?.nombre_grado}</div>
              <div><span className="font-bold">Sección:</span> {estudiante?.nombre_seccion}</div>
              <div><span className="font-bold">Modalidad:</span> {estudiante?.nombre_modalidad}</div>
            </div>
          </section>
        )}
        {tab === 'actual' && (
          <section>
            <h3 className="text-xl font-semibold mb-4 text-emerald-400">Calificaciones del Año Actual</h3>
            <div className="mb-4 flex justify-end">
              <button
                className="px-4 py-2 text-sm rounded-lg bg-gradient-to-r from-emerald-600 to-cyan-600 text-white font-semibold shadow hover:scale-105 transition-all"
                onClick={() => setMostrarAjustes(v => !v)}
              >
                {mostrarAjustes ? 'Ocultar ajustes' : 'Mostrar ajustes'}
              </button>
            </div>
            <TablaCalificaciones
              asignaturas={asignaturas}
              calificaciones={calificaciones}
              errores={errores}
              mostrarAjustes={mostrarAjustes}
              onInputChange={handleInputChange}
            />
            <div className="mt-4 text-right flex flex-row items-center justify-end gap-4">
              {mensajeGuardado && <div className="mb-2 text-green-400 font-semibold">{mensajeGuardado}</div>}
              <button
                className="px-8 py-2 bg-gradient-to-r from-yellow-500 to-red-500 text-white rounded-xl font-bold shadow-lg hover:scale-105 hover:bg-yellow-600 transition-all"
                onClick={handleGuardarPendientes}
              >
                Guardar Pendientes
              </button>
              <button className="px-8 py-2 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-bold shadow-lg hover:scale-105 hover:bg-emerald-700 transition-all" onClick={handleGuardar}>Guardar Cambios</button>
              <button className="px-8 py-2 bg-gradient-to-r from-blue-600 to-cyan-600 text-white rounded-xl font-bold shadow-lg hover:scale-105 hover:bg-blue-700 transition-all" onClick={() => setMostrarModalGuardarHistorialEstudiante(true)}>Guardar Historial</button>
            </div>
            {/* Modal de confirmación para guardar historial individual */}
            {mostrarModalGuardarHistorialEstudiante && (
              (() => { console.log('[DEBUG][MODAL] Modal de guardar historial renderizado'); return null; })(),
              <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-40">
                <div className="bg-white dark:bg-dark-800 p-8 rounded-xl shadow-lg max-w-md w-full border border-blue-300">
                  <h2 className="text-xl font-bold mb-4 text-blue-700 dark:text-cyan-300">Guardar Historial</h2>
                  <p className="mb-6 text-gray-700 dark:text-gray-200">
                    ¿Está seguro que desea guardar el historial académico de este estudiante para el periodo actual?
                  </p>
                  <div className="flex justify-end gap-4">
                    <button
                      className="px-5 py-2 rounded-lg bg-gray-200 dark:bg-dark-700 text-gray-700 dark:text-gray-200 hover:bg-gray-300 dark:hover:bg-dark-600 font-medium"
                      onClick={() => {
                        console.log('[DEBUG][MODAL] Click en Cancelar del modal');
                        setMostrarModalGuardarHistorialEstudiante(false);
                      }}
                    >
                      Cancelar
                    </button>
                    <button
                      className="px-5 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 font-semibold shadow"
                      onClick={handleGuardarHistorial}
                    >
                      Guardar
                    </button>
                  </div>
                </div>
              </div>
            )}
          </section>
        )}
        {tab === 'historial' && (
          <HistorialAcademico idEstudiante={estudiante.id} />
        )}
        {tab === 'pendientes' && (
          <section>
            <h3 className="text-xl font-semibold mb-4">Asignaturas Pendientes</h3>
            <div className="overflow-x-auto">
              <AsignaturasPendientes idEstudiante={estudiante.id} />
            </div>
          </section>
        )}
      </div>
    </div>
  );
} 