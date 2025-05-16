import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { HistorialAcademico } from '../componentes/HistorialAcademico';

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
  const [asignaturas, setAsignaturas] = useState<Asignatura[]>([]);
  const [calificaciones, setCalificaciones] = useState<CalificacionEstudiante[]>([]);
  const [mostrarAjustes, setMostrarAjustes] = useState(false);
  const [errores, setErrores] = useState<Record<string, string>>({});
  const [mensajeGuardado, setMensajeGuardado] = useState<string | null>(null);
  const [periodoActual, setPeriodoActual] = useState<number | null>(null);
  const { mostrarMensaje } = useMensajeGlobal();
  const [mostrarModalGuardarHistorialEstudiante, setMostrarModalGuardarHistorialEstudiante] = useState(false);

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

  useEffect(() => {
    if (!periodoActual) {
      console.warn('[DEBUG] No hay periodoActual:', periodoActual);
      return;
    }
    if (!estudiante.id_grado) {
      console.warn('[DEBUG] No hay estudiante.id_grado:', estudiante.id_grado);
      return;
    }
    if (!estudiante.id_modalidad) {
      console.warn('[DEBUG] No hay estudiante.id_modalidad:', estudiante.id_modalidad);
      return;
    }
    if (!estudiante.id) {
      console.warn('[DEBUG] No hay estudiante.id:', estudiante.id);
      return;
    }

    const cargarDatos = async () => {
      try {
        // 1. Obtener asignaturas
        console.log('[DEBUG] Invocando obtener_asignaturas_por_grado_modalidad', {
          idGrado: estudiante.id_grado,
          idModalidad: estudiante.id_modalidad
        });
        
        const asignaturasData = await invoke<Asignatura[]>('obtener_asignaturas_por_grado_modalidad', {
          idGrado: Number(estudiante.id_grado),
          idModalidad: Number(estudiante.id_modalidad),
        });
        
        console.log('[DEBUG] Asignaturas obtenidas:', asignaturasData);
        setAsignaturas(asignaturasData);

        // 2. Obtener calificaciones
        const params = {
          idEstudiante: Number(estudiante.id),
          idPeriodo: Number(periodoActual)
        };
        console.log('[DEBUG][FRONTEND] Params enviados a invoke:', params);
        try {
          const calificacionesData = await invoke<CalificacionEstudiante[]>('obtener_calificaciones_estudiante', params);
          console.log('[DEBUG][FRONTEND] Respuesta de calificaciones:', calificacionesData);
          setCalificaciones(calificacionesData);
        } catch (error) {
          console.error('[ERROR][FRONTEND] Error en invoke obtener_calificaciones_estudiante:', error);
          if (error && typeof error === 'object') {
            // @ts-ignore
            if (error.response) console.error('[ERROR][FRONTEND] error.response:', error.response);
          }
          mostrarMensaje(
            error instanceof Error
              ? error.message
              : 'Error al cargar asignaturas o calificaciones',
            'error'
          );
        }
      } catch (error) {
        console.error('[ERROR] Al cargar datos:', error);
        mostrarMensaje(
          error instanceof Error 
            ? error.message 
            : 'Error al cargar asignaturas o calificaciones', 
          'error'
        );
      }
    };

    cargarDatos();
  }, [estudiante.id_grado, estudiante.id_modalidad, estudiante.id, periodoActual]);

  // Mostrar advertencia si periodoActual es null, pero no bloquear la interfaz
  const advertenciaPeriodo = !periodoActual ? (
    <div className="mb-4 p-4 bg-yellow-100 text-yellow-800 rounded text-center">
      No hay un periodo escolar activo. No se pueden cargar calificaciones.
    </div>
  ) : null;

  const handleInputChange = (id_asignatura: number, campo: keyof CalificacionEstudiante, valor: string) => {
    let num: number | undefined = valor === '' ? undefined : Number(valor);
    let error = '';
    // Validar máximo 20
    if (['lapso_1', 'lapso_2', 'lapso_3', 'lapso_1_ajustado', 'lapso_2_ajustado', 'lapso_3_ajustado'].includes(campo) && num !== undefined) {
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
        nombre_asignatura: ASIGNATURAS.find(a => a.id === id_asignatura)?.nombre || '',
      } as CalificacionEstudiante : { ...updated[idx] };
      if (campo.includes('ajustado')) {
        // Determinar el lapso correspondiente
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
      // Asignar el valor solo si es number o undefined
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

  // Lógica para guardar o actualizar historial académico (igual que en el modal)
  const handleGuardarHistorial = async () => {
    console.log('[DEBUG][FUNCION] handleGuardarHistorial ejecutada');
    try {
      const notasValidas = asignaturas
        .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
        .map(a => {
          const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
          return calif ? obtenerNotaValida(calif) : undefined;
        })
        .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
      const promedio = notasValidas.length > 0 ? (notasValidas.reduce((a, b) => a + b, 0) / notasValidas.length) : 0;
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
      // Usar el id_grado_secciones real del estudiante
      const id_grado_secciones = estudiante.id_grado_secciones;
      if (!id_grado_secciones) {
        console.error('[ERROR][FUNCION] El estudiante no tiene id_grado_secciones. No se puede guardar el historial.');
        mostrarMensaje('No se puede guardar el historial: falta el id de grado/sección del estudiante.', 'error');
        return;
      }
      const params = {
        idEstudiante: estudiante.id,
        id_periodo: periodoActual,
        idPeriodo: periodoActual,
        id_grado_secciones,
        idGradoSecciones: id_grado_secciones,
        promedio_anual: promedio,
        promedioAnual: promedio,
        estatus,
      };
      console.log('[DEBUG][FUNCION] Params enviados a upsert_historial_academico:', params);
      await invoke('upsert_historial_academico', params);
      setMensajeGuardado('Historial guardado correctamente');
      mostrarMensaje('Historial guardado correctamente', 'exito');
    } catch (error) {
      console.error('[ERROR][FUNCION] Error en handleGuardarHistorial:', error);
      setMensajeGuardado('Error al guardar el historial');
      mostrarMensaje('Error al guardar el historial', 'error');
    }
  };

  // Log antes del renderizado de la tabla
  console.log('[DEBUG] Asignaturas en render:', asignaturas);

  // 1. Agregar función para ceros a la izquierda
  function padNota(nota: number | undefined): string {
    if (typeof nota === 'number') return nota.toString().padStart(2, '0');
    return '';
  }

  // Función para guardar historial individual y refrescar historial
  async function guardarHistorialEstudiante() {
    try {
      // Aquí deberías calcular promedio, grado, estatus, etc. según tu lógica
      // Por ahora, ejemplo con datos dummy:
      await invoke('upsert_historial_academico', {
        idEstudiante: estudiante.id,
        id_periodo: periodoActual,
        id_grado_secciones: estudiante.id_grado || 0,
        promedio_anual: 10.0, // Reemplaza por el promedio real
        estatus: 'Aprobado', // O 'Reprobado' según lógica
      });
      mostrarMensaje('Historial guardado correctamente', 'exito');
      // Refrescar historial
      if (estudiante.id) {
        const data = await invoke('obtener_historial_academico_estudiante', { id_estudiante: estudiante.id });
        setCalificaciones(data as CalificacionEstudiante[]);
      }
    } catch (error) {
      mostrarMensaje('Error al guardar el historial', 'error');
    } finally {
      setMostrarModalGuardarHistorialEstudiante(false);
    }
  }

  // Log global para saber si el componente se monta
  console.log('[DEBUG][GLOBAL] DetalleCalificaciones renderizado');

  return (
    <div className="max-w-5xl mx-auto p-4 md:p-8 bg-white dark:bg-dark-800 rounded-2xl shadow-lg">
      {advertenciaPeriodo}
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
            <div className="overflow-x-auto max-h-[60vh]">
              <table className="min-w-full divide-y divide-emerald-400 dark:divide-cyan-800 text-sm rounded-xl overflow-hidden shadow-lg">
                <thead className="sticky top-0 z-30 bg-white dark:bg-gradient-to-r dark:from-[#181f2a] dark:via-[#232c3d] dark:to-[#2563eb]">
                  <tr>
                    <th className="px-4 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">ASIGNATURA</th>
                    <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">1ER LAPSO</th>
                    {mostrarAjustes && <th className="px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">AJUSTE 1</th>}
                    <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">2DO LAPSO</th>
                    {mostrarAjustes && <th className="px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">AJUSTE 2</th>}
                    <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">3ER LAPSO</th>
                    {mostrarAjustes && <th className="px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">AJUSTE 3</th>}
                    <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">FINAL</th>
                    <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">REVISIÓN</th>
                    <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">ESTADO</th>
                  </tr>
                </thead>
                <tbody>
                  {asignaturas.length === 0 && (
                    <tr>
                      <td colSpan={8} className="text-center py-4 text-gray-400">No hay asignaturas registradas para este estudiante.</td>
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
                        <td className="px-4 py-2 font-bold text-emerald-700 dark:text-cyan-200 whitespace-nowrap">
                          <div className="flex items-center gap-3">
                            <span className="flex items-center justify-center w-8 h-8 rounded-lg font-bold text-white text-base shadow" style={{ background: '#2563eb' }}>
                              {asig.nombre_asignatura?.[0] ?? ''}
                            </span>
                            <span className="dark:text-cyan-200 text-emerald-700">{asig.nombre_asignatura}</span>
                          </div>
                        </td>
                        {/* Lapso 1 */}
                        <td className="px-2 py-2 align-middle text-center">
                          <input
                            type="number"
                            min={0}
                            max={20}
                            value={padNota((calif as Partial<CalificacionEstudiante>).lapso_1 as number)}
                            onChange={e => handleInputChange(asig.id_asignatura, 'lapso_1', e.target.value)}
                            className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
                          />
                        </td>
                        {mostrarAjustes && (
                          <td className="px-2 py-2 align-middle text-center">
                            <input
                              type="number"
                              min={0}
                              max={20}
                              value={padNota((calif as Partial<CalificacionEstudiante>).lapso_1_ajustado as number)}
                              onChange={e => handleInputChange(asig.id_asignatura, 'lapso_1_ajustado', e.target.value)}
                              className={`w-14 h-10 px-2 py-1 rounded-lg border-2 ${errores[`${asig.id_asignatura}_lapso_1_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
                            />
                            {errores[`${asig.id_asignatura}_lapso_1_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_1_ajustado`]}</div>}
                          </td>
                        )}
                        {/* Lapso 2 */}
                        <td className="px-2 py-2 align-middle text-center">
                          <input
                            type="number"
                            min={0}
                            max={20}
                            value={padNota((calif as Partial<CalificacionEstudiante>).lapso_2 as number)}
                            onChange={e => handleInputChange(asig.id_asignatura, 'lapso_2', e.target.value)}
                            className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
                          />
                        </td>
                        {mostrarAjustes && (
                          <td className="px-2 py-2 align-middle text-center">
                            <input
                              type="number"
                              min={0}
                              max={20}
                              value={padNota((calif as Partial<CalificacionEstudiante>).lapso_2_ajustado as number)}
                              onChange={e => handleInputChange(asig.id_asignatura, 'lapso_2_ajustado', e.target.value)}
                              className={`w-14 h-10 px-2 py-1 rounded-lg border-2 ${errores[`${asig.id_asignatura}_lapso_2_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
                            />
                            {errores[`${asig.id_asignatura}_lapso_2_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_2_ajustado`]}</div>}
                          </td>
                        )}
                        {/* Lapso 3 */}
                        <td className="px-2 py-2 align-middle text-center">
                          <input
                            type="number"
                            min={0}
                            max={20}
                            value={padNota((calif as Partial<CalificacionEstudiante>).lapso_3 as number)}
                            onChange={e => handleInputChange(asig.id_asignatura, 'lapso_3', e.target.value)}
                            className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
                          />
                        </td>
                        {mostrarAjustes && (
                          <td className="px-2 py-2 align-middle text-center">
                            <input
                              type="number"
                              min={0}
                              max={20}
                              value={padNota((calif as Partial<CalificacionEstudiante>).lapso_3_ajustado as number)}
                              onChange={e => handleInputChange(asig.id_asignatura, 'lapso_3_ajustado', e.target.value)}
                              className={`w-14 h-10 px-2 py-1 rounded-lg border-2 ${errores[`${asig.id_asignatura}_lapso_3_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
                            />
                            {errores[`${asig.id_asignatura}_lapso_3_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_3_ajustado`]}</div>}
                          </td>
                        )}
                        {/* Final */}
                        <td className="px-1 py-2 w-20 align-middle text-center">
                          <input
                            type="text"
                            value={padNota((calif as Partial<CalificacionEstudiante>).nota_final as number) || padNota(obtenerNotaValida(calif as CalificacionEstudiante))}
                            readOnly
                            className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-emerald-50 dark:bg-[#232c3d] text-center text-emerald-700 dark:text-cyan-200 font-normal shadow text-sm"
                          />
                        </td>
                        {/* Revisión */}
                        <td className="px-1 py-2 w-20 align-middle text-center">
                          <input
                            type="text"
                            value={(calif as Partial<CalificacionEstudiante>).revision ?? ''}
                            onChange={e => handleInputChange(asig.id_asignatura, 'revision', e.target.value)}
                            className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
                          />
                        </td>
                        {/* Estado */}
                        <td className="px-1 py-2 w-24 align-middle text-center">
                          <div className="h-10 flex items-center justify-center">
                            {(() => {
                              const estado = calcularEstadoAsignatura(calif as CalificacionEstudiante, totalRevisionMenor10);
                              if (estado === 'Aprobado')
                                return <span className="px-3 py-1 rounded-full text-xs font-bold bg-emerald-600/20 dark:bg-cyan-800/30 text-emerald-700 dark:text-cyan-200 border border-emerald-400 dark:border-cyan-400 shadow">Aprobado</span>;
                              if (estado === 'Pendiente')
                                return <span className="px-3 py-1 rounded-full text-xs font-bold bg-yellow-600/20 dark:bg-yellow-800/30 text-yellow-700 dark:text-yellow-200 border border-yellow-400 dark:border-yellow-400 shadow">Pendiente</span>;
                              if (estado === 'Repite')
                                return <span className="px-3 py-1 rounded-full text-xs font-bold bg-red-600/20 dark:bg-red-800/30 text-red-700 dark:text-red-200 border border-red-400 dark:border-red-400 shadow">Repite</span>;
                              if (estado === 'Revisión')
                                return <span className="px-3 py-1 rounded-full text-xs font-bold bg-cyan-600/20 dark:bg-cyan-800/30 text-cyan-700 dark:text-cyan-200 border border-cyan-400 dark:border-cyan-400 shadow">Revisión</span>;
                              return '';
                            })()}
                          </div>
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
                    {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
                    <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
                      <div className="flex items-center justify-center h-full">{padNota(Number(promedioLapso('lapso_2', 'lapso_2_ajustado')))}</div>
                    </td>
                    {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
                    <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
                      <div className="flex items-center justify-center h-full">{padNota(Number(promedioLapso('lapso_3', 'lapso_3_ajustado')))}</div>
                    </td>
                    {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
                    <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
                      <div className="flex items-center justify-center h-full">{padNota(Number(promedioFinal()))}</div>
                    </td>
                    <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>
                    <td className="px-1 py-2 w-24 text-center bg-white dark:bg-transparent align-middle"></td>
                  </tr>
                </tbody>
              </table>
            </div>
            <div className="mt-4 text-right flex flex-row items-center justify-end gap-4">
              {mensajeGuardado && <div className="mb-2 text-green-400 font-semibold">{mensajeGuardado}</div>}
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
                      onClick={async () => {
                        console.log('[DEBUG][MODAL] Click en Guardar del modal de confirmación');
                        try {
                          await handleGuardarHistorial();
                        } catch (e) {
                          console.error('[ERROR][MODAL] Error al ejecutar handleGuardarHistorial:', e);
                          mostrarMensaje('Error crítico al ejecutar handleGuardarHistorial', 'error');
                        }
                        setMostrarModalGuardarHistorialEstudiante(false);
                      }}
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
              <table className="min-w-full divide-y divide-gray-200 dark:divide-dark-700">
                <thead className="bg-gray-100 dark:bg-dark-900">
                  <tr>
                    <th className="px-4 py-2">Año Escolar</th>
                    <th className="px-4 py-2">Grado</th>
                    <th className="px-4 py-2">Asignatura</th>
                    <th className="px-4 py-2">Calificación</th>
                    <th className="px-4 py-2">Estado</th>
                  </tr>
                </thead>
                <tbody>
                  {PENDIENTES_FICTICIAS.map((p, idx) => (
                    <tr key={idx}>
                      <td className="px-4 py-2">{p.periodo}</td>
                      <td className="px-4 py-2">{p.grado}</td>
                      <td className="px-4 py-2">{p.asignatura}</td>
                      <td className="px-4 py-2 text-center">{p.calificacion}</td>
                      <td className="px-4 py-2">
                        <span className="px-3 py-1 rounded-full text-xs font-bold bg-red-200 text-red-800">{p.estado}</span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
        )}
      </div>
    </div>
  );
} 