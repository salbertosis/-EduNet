import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

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

const HISTORIAL_FICTICIO = [
  { periodo: '2023-2024', grado: '2do Año', promedio: 15.2, estado: 'Aprobado' },
  { periodo: '2022-2023', grado: '1er Año', promedio: 14.7, estado: 'Aprobado' },
];

const PENDIENTES_FICTICIAS = [
  { periodo: '2023-2024', grado: '2do Año', asignatura: 'Física', calificacion: 9, estado: 'Pendiente' },
  { periodo: '2022-2023', grado: '1er Año', asignatura: 'Matemática', calificacion: 10, estado: 'Pendiente' },
];

export function DetalleCalificaciones({ estudiante, onVolver }: DetalleCalificacionesProps) {
  const [tab, setTab] = useState('datos');
  const [asignaturas, setAsignaturas] = useState<Asignatura[]>([]);
  const [calificaciones, setCalificaciones] = useState<CalificacionEstudiante[]>([]);
  const [mostrarAjustes, setMostrarAjustes] = useState(false);
  const [errores, setErrores] = useState<Record<string, string>>({});
  const [mensajeGuardado, setMensajeGuardado] = useState<string | null>(null);
  const [periodoActual, setPeriodoActual] = useState<number | null>(null);
  const { mostrarMensaje } = useMensajeGlobal();

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
  const promedioFinal = () => {
    const notas = calificaciones.map(c => c.nota_final).filter(n => typeof n === 'number');
    if (notas.length === 0) return '';
    return (notas.reduce((a, b) => (a as number) + (b as number), 0) / notas.length).toFixed(2);
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

  // Nueva función para calcular el promedio de cada lapso usando ajustes si existen
  const promedioLapsoConAjuste = (lapso: keyof CalificacionEstudiante, lapsoAjuste: keyof CalificacionEstudiante) => {
    const notas = calificaciones.map(c => {
      if (typeof c[lapsoAjuste] === 'number') return c[lapsoAjuste] as number;
      if (typeof c[lapso] === 'number') return c[lapso] as number;
      return undefined;
    }).filter(n => typeof n === 'number') as number[];
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

  // Log antes del renderizado de la tabla
  console.log('[DEBUG] Asignaturas en render:', asignaturas);

  return (
    <div className="max-w-5xl mx-auto p-6 bg-white dark:bg-dark-800 rounded-2xl shadow-lg">
      {advertenciaPeriodo}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold text-primary-600">Calificaciones de {estudiante?.nombres} {estudiante?.apellidos}</h2>
        <button onClick={onVolver} className="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-all">Volver</button>
      </div>
      <div className="flex gap-2 border-b border-gray-200 dark:border-dark-700 mb-8">
        {TABS.map(t => (
          <button
            key={t.key}
            onClick={() => setTab(t.key)}
            className={`px-6 py-3 font-medium rounded-t-lg transition-all duration-200 focus:outline-none ${tab === t.key ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-100' : 'bg-gray-100 dark:bg-dark-700 text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-dark-600'}`}
          >
            {t.label}
          </button>
        ))}
      </div>
      <div>
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
            <h3 className="text-xl font-semibold mb-4">Calificaciones del Año Actual</h3>
            <div className="mb-4 flex justify-end">
              <button
                className="px-4 py-1 text-sm rounded-lg bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-100 hover:bg-primary-200 dark:hover:bg-primary-800 font-semibold transition-all"
                onClick={() => setMostrarAjustes(v => !v)}
              >
                {mostrarAjustes ? 'Ocultar ajustes' : 'Mostrar ajustes'}
              </button>
            </div>
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200 dark:divide-dark-700 text-xs">
                <thead className="bg-gray-100 dark:bg-dark-900">
                  <tr>
                    <th className="px-2 py-2 text-left">Asignatura</th>
                    <th className="px-1 py-2 text-left">1er Lapso</th>
                    {mostrarAjustes && <th className="px-1 py-2 text-left">Ajuste 1</th>}
                    <th className="px-1 py-2 text-left">2do Lapso</th>
                    {mostrarAjustes && <th className="px-1 py-2 text-left">Ajuste 2</th>}
                    <th className="px-1 py-2 text-left">3er Lapso</th>
                    {mostrarAjustes && <th className="px-1 py-2 text-left">Ajuste 3</th>}
                    <th className="px-1 py-2 text-left">Final</th>
                    <th className="px-1 py-2 text-left">Revisión</th>
                    <th className="px-1 py-2 text-left">Estado</th>
                  </tr>
                </thead>
                <tbody>
                  {asignaturas.length === 0 && (
                    <tr>
                      <td colSpan={8} className="text-center py-4 text-gray-400">No hay asignaturas registradas para este estudiante.</td>
                    </tr>
                  )}
                  {asignaturas.map(asig => {
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
                      <tr key={asig.id_asignatura}>
                        <td className="px-2 py-2 font-semibold text-green-400 whitespace-nowrap">{asig.nombre_asignatura}</td>
                        {/* Lapso 1 */}
                        <td className="px-1 py-2">
                          <input
                            type="number"
                            min={0}
                            max={20}
                            value={(calif as Partial<CalificacionEstudiante>).lapso_1 ?? ''}
                            onChange={e => handleInputChange(asig.id_asignatura, 'lapso_1', e.target.value)}
                            className="w-10 px-1 py-1 rounded border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-center"
                          />
                        </td>
                        {mostrarAjustes && (
                          <td className="px-1 py-2">
                            <input
                              type="number"
                              min={0}
                              max={20}
                              value={(calif as Partial<CalificacionEstudiante>).lapso_1_ajustado ?? ''}
                              onChange={e => handleInputChange(asig.id_asignatura, 'lapso_1_ajustado', e.target.value)}
                              className={`w-10 px-1 py-1 rounded border ${errores[`${asig.id_asignatura}_lapso_1_ajustado`] ? 'border-red-500' : 'border-yellow-400 dark:border-yellow-600'} bg-white dark:bg-dark-700 text-center`}
                            />
                            {errores[`${asig.id_asignatura}_lapso_1_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_1_ajustado`]}</div>}
                          </td>
                        )}
                        {/* Lapso 2 */}
                        <td className="px-1 py-2">
                          <input
                            type="number"
                            min={0}
                            max={20}
                            value={(calif as Partial<CalificacionEstudiante>).lapso_2 ?? ''}
                            onChange={e => handleInputChange(asig.id_asignatura, 'lapso_2', e.target.value)}
                            className="w-10 px-1 py-1 rounded border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-center"
                          />
                        </td>
                        {mostrarAjustes && (
                          <td className="px-1 py-2">
                            <input
                              type="number"
                              min={0}
                              max={20}
                              value={(calif as Partial<CalificacionEstudiante>).lapso_2_ajustado ?? ''}
                              onChange={e => handleInputChange(asig.id_asignatura, 'lapso_2_ajustado', e.target.value)}
                              className={`w-10 px-1 py-1 rounded border ${errores[`${asig.id_asignatura}_lapso_2_ajustado`] ? 'border-red-500' : 'border-yellow-400 dark:border-yellow-600'} bg-white dark:bg-dark-700 text-center`}
                            />
                            {errores[`${asig.id_asignatura}_lapso_2_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_2_ajustado`]}</div>}
                          </td>
                        )}
                        {/* Lapso 3 */}
                        <td className="px-1 py-2">
                          <input
                            type="number"
                            min={0}
                            max={20}
                            value={(calif as Partial<CalificacionEstudiante>).lapso_3 ?? ''}
                            onChange={e => handleInputChange(asig.id_asignatura, 'lapso_3', e.target.value)}
                            className="w-10 px-1 py-1 rounded border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-center"
                          />
                        </td>
                        {mostrarAjustes && (
                          <td className="px-1 py-2">
                            <input
                              type="number"
                              min={0}
                              max={20}
                              value={(calif as Partial<CalificacionEstudiante>).lapso_3_ajustado ?? ''}
                              onChange={e => handleInputChange(asig.id_asignatura, 'lapso_3_ajustado', e.target.value)}
                              className={`w-10 px-1 py-1 rounded border ${errores[`${asig.id_asignatura}_lapso_3_ajustado`] ? 'border-red-500' : 'border-yellow-400 dark:border-yellow-600'} bg-white dark:bg-dark-700 text-center`}
                            />
                            {errores[`${asig.id_asignatura}_lapso_3_ajustado`] && <div className="text-xs text-red-500">{errores[`${asig.id_asignatura}_lapso_3_ajustado`]}</div>}
                          </td>
                        )}
                        {/* Final */}
                        <td className="px-1 py-2">
                          <input
                            type="number"
                            value={(calif as Partial<CalificacionEstudiante>).nota_final ?? calcularNotaFinal(calif as CalificacionEstudiante)}
                            readOnly
                            className="w-10 px-1 py-1 rounded border border-gray-300 dark:border-dark-600 bg-gray-100 dark:bg-dark-700 text-center text-gray-500"
                          />
                        </td>
                        {/* Revisión */}
                        <td className="px-1 py-2">
                          <input
                            type="text"
                            value={(calif as Partial<CalificacionEstudiante>).revision ?? ''}
                            onChange={e => handleInputChange(asig.id_asignatura, 'revision', e.target.value)}
                            className="w-10 px-1 py-1 rounded border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-center"
                          />
                        </td>
                        {/* Estado */}
                        <td className="px-1 py-2 text-center">
                          {calcularEstadoAsignatura(calif as CalificacionEstudiante, totalRevisionMenor10)}
                        </td>
                      </tr>
                    );
                  })}
                  {/* Fila de promedios */}
                  <tr className="bg-primary-100 dark:bg-primary-900 font-bold">
                    <td className="px-2 py-2 text-right">Promedio</td>
                    <td className="px-1 py-2 text-center">{promedioLapsoConAjuste('lapso_1', 'lapso_1_ajustado')}</td>
                    {mostrarAjustes && <td className="px-1 py-2 text-center"></td>}
                    <td className="px-1 py-2 text-center">{promedioLapsoConAjuste('lapso_2', 'lapso_2_ajustado')}</td>
                    {mostrarAjustes && <td className="px-1 py-2 text-center"></td>}
                    <td className="px-1 py-2 text-center">{promedioLapsoConAjuste('lapso_3', 'lapso_3_ajustado')}</td>
                    {mostrarAjustes && <td className="px-1 py-2 text-center"></td>}
                    <td className="px-1 py-2 text-center">{promedioFinal()}</td>
                    <td className="px-1 py-2 text-center"></td>
                    <td className="px-1 py-2 text-center"></td>
                  </tr>
                </tbody>
              </table>
            </div>
            <div className="mt-4 text-right">
              {mensajeGuardado && <div className="mb-2 text-green-600 font-semibold">{mensajeGuardado}</div>}
              <button className="px-6 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-all font-semibold" onClick={handleGuardar}>Guardar Cambios</button>
            </div>
          </section>
        )}
        {tab === 'historial' && (
          <section>
            <h3 className="text-xl font-semibold mb-4">Historial Académico</h3>
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200 dark:divide-dark-700">
                <thead className="bg-gray-100 dark:bg-dark-900">
                  <tr>
                    <th className="px-4 py-2">Año Escolar</th>
                    <th className="px-4 py-2">Grado</th>
                    <th className="px-4 py-2">Promedio General</th>
                    <th className="px-4 py-2">Estado</th>
                  </tr>
                </thead>
                <tbody>
                  {HISTORIAL_FICTICIO.map((h, idx) => (
                    <tr key={idx}>
                      <td className="px-4 py-2">{h.periodo}</td>
                      <td className="px-4 py-2">{h.grado}</td>
                      <td className="px-4 py-2 text-center">{h.promedio}</td>
                      <td className="px-4 py-2">
                        <span className={`px-3 py-1 rounded-full text-xs font-bold ${h.estado === 'Aprobado' ? 'bg-green-200 text-green-800' : 'bg-red-200 text-red-800'}`}>{h.estado}</span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
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