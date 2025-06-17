import React, { useState, useEffect, useCallback, useMemo } from 'react';
import { 
  GraduationCap, 
  Users, 
  User, 
  Search, 
  BookOpen, 
  Award, 
  Feather, 
  ScrollText, 
  UserPlus,
  X,
  Loader2,
  List
} from 'lucide-react';
import { invoke } from '@tauri-apps/api/tauri';
import { Paginacion } from '../../../componentes/Paginacion';
import { useGrados, type Grado } from '../hooks/useGrados';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { ModalAsignarDocentes } from '../componentes/ModalAsignarDocentes';
import { useAsignaturas } from '../../calificaciones/hooks/useAsignaturas';

// Types y constantes
interface FiltrosGrados {
  busqueda: string;
  grado: string;
  seccion: string;
  modalidad: string;
}

interface Docente {
  id_docente: number;
  nombres: string;
  apellidos: string;
}

interface TarjetaCursoProps {
  grado: Grado;
  onDocenteAsignado?: () => void;
}

type GradoIcon = 1 | 2 | 3 | 4 | 5;

// Constantes mejoradas con tipado estricto
const AÑOS = [
  { id: 1, label: "1er Año" },
  { id: 2, label: "2do Año" },
  { id: 3, label: "3er Año" },
  { id: 4, label: "4to Año" },
  { id: 5, label: "5to Año" }
] as const;

const MODALIDADES = [
  { value: 1, label: "Ciencias" },
  { value: 2, label: "Telemática" },
] as const;

const SECCIONES_ORDEN = ["A", "B", "C", "D", "E", "F", "G"] as const;

const ICONOS_GRADO: Record<GradoIcon, React.ComponentType<{ className?: string }>> = {
  1: GraduationCap,
  2: BookOpen,
  3: Award,
  4: Feather,
  5: ScrollText,
} as const;

// Utilities
const normalizar = (str: string): string => 
  str.normalize('NFD').replace(/[\u0300-\u036f]/g, '').toLowerCase();

const formatearNombreCompleto = (nombres: string, apellidos: string): string =>
  `${nombres.trim()} ${apellidos.trim()}`.trim();

// Componentes optimizados
const IconoGrado = React.memo<{ idGrado: number }>(({ idGrado }) => {
  const IconComponent = ICONOS_GRADO[idGrado as GradoIcon] || GraduationCap;
  return (
    <IconComponent 
      className="w-7 h-7 text-white" 
      aria-hidden="true"
    />
  );
});

IconoGrado.displayName = 'IconoGrado';

// Modal mejorado con autocompletado de docentes
const ModalAsignarDocente = React.memo<{
  isOpen: boolean;
  onClose: () => void;
  onAsignar: (docenteId: number) => Promise<void>;
  cargandoDocentes: boolean;
  asignando: boolean;
  error?: boolean;
}>(({ isOpen, onClose, onAsignar, cargandoDocentes, asignando, error = false }) => {
  const [busqueda, setBusqueda] = useState('');
  const [docentes, setDocentes] = useState<Docente[]>([]);
  const [docenteSeleccionado, setDocenteSeleccionado] = useState<number | null>(null);
  const [cargandoBusqueda, setCargandoBusqueda] = useState(false);
  const [errorBusqueda, setErrorBusqueda] = useState(false);

  // Buscar docentes al escribir
  useEffect(() => {
    if (!isOpen || busqueda.length < 2) {
      setDocentes([]);
      return;
    }
    setCargandoBusqueda(true);
    setErrorBusqueda(false);
    invoke<any>('obtener_docentes', {
      filtro: { nombres: busqueda },
      paginacion: { pagina: 1, registros_por_pagina: 10 }
    })
      .then((res) => setDocentes(res.datos ?? []))
      .catch(() => {
        setDocentes([]);
        setErrorBusqueda(true);
      })
      .finally(() => setCargandoBusqueda(false));
  }, [busqueda, isOpen]);

  const handleAsignar = useCallback(async () => {
    if (!docenteSeleccionado) return;
    await onAsignar(docenteSeleccionado);
    setDocenteSeleccionado(null);
    setBusqueda('');
    setDocentes([]);
  }, [docenteSeleccionado, onAsignar]);

  const handleClose = useCallback(() => {
    if (!asignando) {
      setDocenteSeleccionado(null);
      setBusqueda('');
      setDocentes([]);
      onClose();
    }
  }, [asignando, onClose]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 flex items-center justify-center bg-black/50 backdrop-blur-sm z-50 p-4" role="dialog" aria-modal="true" aria-labelledby="modal-title" onClick={e => e.target === e.currentTarget && handleClose()}>
      <div className="bg-white dark:bg-slate-800 rounded-xl shadow-2xl w-full max-w-md max-h-[90vh] overflow-auto">
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-slate-700">
          <h2 id="modal-title" className="text-xl font-semibold text-gray-900 dark:text-white">Asignar Docente Guía</h2>
          <button type="button" onClick={handleClose} disabled={asignando} className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors disabled:opacity-50" aria-label="Cerrar modal">
            <X className="w-5 h-5" />
          </button>
        </div>
        <div className="p-6 space-y-4">
          <label htmlFor="busqueda-docente" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Buscar docente (nombre o apellido)</label>
          <input
            id="busqueda-docente"
            type="text"
            className="w-full p-3 rounded-lg border border-gray-300 dark:border-slate-600 bg-white dark:bg-slate-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
            placeholder="Escriba al menos 2 letras..."
            value={busqueda}
            onChange={e => setBusqueda(e.target.value)}
            disabled={asignando}
            autoFocus
          />
          {cargandoBusqueda ? (
            <div className="flex items-center justify-center py-4">
              <Loader2 className="w-5 h-5 animate-spin text-blue-600" />
              <span className="ml-2 text-gray-600 dark:text-gray-300">Buscando docentes...</span>
            </div>
          ) : errorBusqueda ? (
            <div className="text-center py-4 text-red-600 dark:text-red-400">Error al buscar docentes</div>
          ) : (
            docentes.length > 0 && (
              <ul className="border border-gray-200 dark:border-slate-700 rounded-lg max-h-48 overflow-auto divide-y divide-gray-100 dark:divide-slate-700">
                {docentes.map(docente => (
                  <li key={docente.id_docente} className={`p-2 cursor-pointer hover:bg-blue-50 dark:hover:bg-blue-900/30 ${docenteSeleccionado === docente.id_docente ? 'bg-blue-100 dark:bg-blue-800' : ''}`} onClick={() => setDocenteSeleccionado(docente.id_docente)}>
                    <span className="font-medium">{formatearNombreCompleto(docente.nombres, docente.apellidos)}</span>
                  </li>
                ))}
              </ul>
            )
          )}
        </div>
        <div className="flex justify-end gap-3 p-6 border-t border-gray-200 dark:border-slate-700">
          <button type="button" onClick={handleClose} disabled={asignando} className="px-4 py-2 text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-slate-700 rounded-lg hover:bg-gray-200 dark:hover:bg-slate-600 transition-colors disabled:opacity-50">Cerrar</button>
          <button type="button" onClick={handleAsignar} disabled={!docenteSeleccionado || asignando} className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center gap-2">{asignando ? <Loader2 className="w-4 h-4 animate-spin" /> : null}{asignando ? 'Asignando...' : 'Asignar'}</button>
        </div>
      </div>
    </div>
  );
});

ModalAsignarDocente.displayName = 'ModalAsignarDocente';

// Modal de solo lectura para ver docentes por asignatura
const ModalVerDocentesPorAsignatura: React.FC<{
  isOpen: boolean;
  onClose: () => void;
  id_grado_secciones: number;
}> = ({ isOpen, onClose, id_grado_secciones }) => {
  const [asignaturas, setAsignaturas] = React.useState<any[]>([]);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);

  useEffect(() => {
    if (!isOpen) return;
    setLoading(true);
    setError(null);
    console.log('[DEBUG][ModalVerDocentesPorAsignatura] Llamando a obtener_docentes_por_asignatura_seccion con id_grado_secciones:', id_grado_secciones);
    invoke('obtener_docentes_por_asignatura_seccion', { id_grado_secciones })
      .then((data) => {
        const asignaturas = data as any[];
        console.log('[DEBUG][ModalVerDocentesPorAsignatura] Respuesta recibida:', asignaturas);
        setAsignaturas(asignaturas);
      })
      .catch((err: any) => {
        console.error('[DEBUG][ModalVerDocentesPorAsignatura] Error recibido:', err);
        setError(err?.message || 'Error al cargar docentes');
      })
      .finally(() => setLoading(false));
  }, [isOpen, id_grado_secciones]);

  const getNombreDocente = (asig: any) => {
    if (asig.nombres_docente && asig.apellidos_docente) {
      return `${asig.nombres_docente} ${asig.apellidos_docente}`;
    }
    return <span className="text-gray-400 italic">Sin asignar</span>;
  };

  if (!isOpen) return null;
  return (
    <div className="fixed inset-0 flex items-center justify-center bg-black/50 backdrop-blur-sm z-50 p-4" role="dialog" aria-modal="true" aria-labelledby="modal-docentes-title" onClick={e => { if (e.target === e.currentTarget) onClose(); }}>
      <div className="bg-white dark:bg-slate-800 rounded-xl shadow-2xl w-full max-w-lg max-h-[90vh] overflow-auto focus:outline-none border border-gray-200 dark:border-slate-700" tabIndex={-1}>
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-slate-700 bg-gradient-to-r from-blue-50 to-blue-100 dark:from-slate-700 dark:to-slate-800 sticky top-0 z-20">
          <h2 id="modal-docentes-title" className="text-xl font-bold text-blue-900 dark:text-blue-100 tracking-wide">Docentes por asignatura</h2>
          <button type="button" onClick={onClose} className="p-2 rounded-lg hover:bg-blue-100 dark:hover:bg-slate-700 transition-colors" aria-label="Cerrar modal" autoFocus>
            <X className="w-5 h-5" />
          </button>
        </div>
        <div className="p-6">
          {loading ? (
            <div className="flex items-center justify-center py-4">Cargando...</div>
          ) : error ? (
            <div className="text-center py-4 text-red-600 dark:text-red-400">{error}</div>
          ) : (
            <table className="min-w-full divide-y divide-gray-200 dark:divide-slate-700 text-base">
              <thead className="bg-blue-50 dark:bg-slate-700 sticky top-0 z-10">
                <tr>
                  <th className="px-4 py-2 text-left font-semibold text-blue-800 dark:text-blue-200 tracking-wide w-1/3">Asignatura</th>
                  <th className="px-4 py-2 text-left font-semibold text-blue-800 dark:text-blue-200 tracking-wide">Docente asignado</th>
                </tr>
              </thead>
              <tbody>
                {asignaturas.map((asig: any, idx: number) => (
                  <tr
                    key={asig.id_asignatura}
                    className={`transition-colors ${idx % 2 === 0 ? 'bg-white dark:bg-slate-800' : 'bg-gray-50 dark:bg-slate-700'} hover:bg-blue-50 dark:hover:bg-blue-900/30`}
                  >
                    <td className="px-4 py-2 font-medium text-gray-900 dark:text-white whitespace-nowrap border-b border-gray-100 dark:border-slate-700 align-top">{asig.nombre_asignatura}</td>
                    <td className="px-4 py-2 border-b border-gray-100 dark:border-slate-700 align-top">
                      <span className="text-gray-800 dark:text-gray-100 font-semibold tracking-wide break-words whitespace-pre-line block">
                        {getNombreDocente(asig)}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
          <div className="flex justify-end mt-6">
            <button type="button" onClick={onClose} className="px-6 py-2 rounded-lg bg-blue-700 text-white font-semibold shadow hover:bg-blue-800 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all">Cerrar</button>
          </div>
        </div>
      </div>
    </div>
  );
};

// Componente principal de tarjeta optimizado
const TarjetaCurso = React.memo<TarjetaCursoProps>(({ grado, onDocenteAsignado }) => {
  const { mostrarMensaje } = useMensajeGlobal();
  const [modalAbierto, setModalAbierto] = useState(false);
  const [modalAsignaturasAbierto, setModalAsignaturasAbierto] = useState(false);
  const [docentes, setDocentes] = useState<Docente[]>([]);
  const [cargandoDocentes, setCargandoDocentes] = useState(false);
  const [asignando, setAsignando] = useState(false);
  const [errorCargandoDocentes, setErrorCargandoDocentes] = useState(false);
  const [modalVerDocentesAbierto, setModalVerDocentesAbierto] = useState(false);

  // Usar el hook useAsignaturas para cargar las asignaturas
  const { asignaturas, loading: cargandoAsignaturas, error: errorAsignaturas } = useAsignaturas(
    grado.id_grado,
    grado.id_modalidad,
    grado.id_grado_secciones
  );

  const abrirModal = useCallback(async () => {
    setModalAbierto(true);
    setCargandoDocentes(true);
    setErrorCargandoDocentes(false);

    try {
      const listaDocentes = await invoke<any>('obtener_docentes', {
        filtro: null,
        paginacion: { pagina: 1, registros_por_pagina: 1000 }
      });
      setDocentes(listaDocentes.datos ?? []);
    } catch (error) {
      console.error('Error al obtener docentes:', error);
      setErrorCargandoDocentes(true);
      mostrarMensaje('Error al cargar la lista de docentes', 'error');
      setDocentes([]);
    } finally {
      setCargandoDocentes(false);
    }
  }, [mostrarMensaje]);

  const asignarDocenteGuia = useCallback(async (docenteId: number) => {
    setAsignando(true);
    
    try {
      await invoke('asignar_docente_guia', {
        id_grado_secciones: grado.id_grado_secciones,
        id_docente_guia: docenteId
      });
      
      mostrarMensaje('Docente guía asignado correctamente', 'exito');
      onDocenteAsignado?.();
      setModalAbierto(false);
    } catch (error) {
      console.error('Error al asignar docente:', error);
      mostrarMensaje('Error al asignar docente guía', 'error');
    } finally {
      setAsignando(false);
    }
  }, [grado.id_grado_secciones, mostrarMensaje, onDocenteAsignado]);

  const cerrarModal = useCallback(() => {
    setModalAbierto(false);
    setErrorCargandoDocentes(false);
  }, []);

  return (
    <>
      <article className="group relative rounded-2xl border border-slate-200 dark:border-slate-700 shadow-lg bg-white dark:bg-slate-800 p-6 transition-all duration-300 hover:scale-[1.02] hover:shadow-xl hover:shadow-blue-500/10 min-w-[300px]">
        {/* Gradiente superior */}
        <div className="absolute top-0 left-0 right-0 h-1 rounded-t-2xl bg-gradient-to-r from-emerald-400 via-cyan-400 to-blue-400" />
        
        {/* Botón de asignar docentes (esquina superior izquierda) */}
        <button
          type="button"
          onClick={() => setModalAsignaturasAbierto(true)}
          className="absolute top-4 left-4 p-2 bg-blue-50 dark:bg-blue-900/30 rounded-full hover:bg-blue-100 dark:hover:bg-blue-900/50 transition-colors group/btn"
          title="Asignar docentes a asignaturas"
          aria-label={`Asignar docentes a asignaturas para ${grado.nombre_grado} Año ${grado.nombre_seccion}`}
        >
          <BookOpen className="w-5 h-5 text-blue-600 dark:text-blue-400 group-hover/btn:scale-110 transition-transform" />
        </button>

        {/* Botón de asignar docente guía (esquina superior derecha) */}
        <button
          type="button"
          onClick={abrirModal}
          className="absolute top-4 right-4 p-2 bg-emerald-50 dark:bg-emerald-900/30 rounded-full hover:bg-emerald-100 dark:hover:bg-emerald-900/50 transition-colors group/btn"
          title="Asignar docente guía"
          aria-label={`Asignar docente guía para ${grado.nombre_grado} Año ${grado.nombre_seccion}`}
        >
          <UserPlus className="w-5 h-5 text-emerald-600 dark:text-emerald-400 group-hover/btn:scale-110 transition-transform" />
        </button>

        {/* Icono del grado */}
        <div className="flex justify-center mb-4">
          <div className="flex items-center justify-center w-16 h-16 rounded-full bg-gradient-to-br from-cyan-500 to-blue-600 shadow-lg">
            <IconoGrado idGrado={grado.id_grado} />
          </div>
        </div>

        {/* Información principal */}
        <div className="text-center space-y-2 mb-4">
          <h3 className="font-bold text-xl text-gray-900 dark:text-white">
            {grado.nombre_grado} Año {grado.nombre_seccion}
          </h3>
          <p className="text-sm text-cyan-600 dark:text-cyan-400 font-medium">
            {grado.nombre_modalidad}
          </p>
          <p className="text-xs text-gray-500 dark:text-gray-400">
            Docente Guía: 
            <span className="font-semibold text-cyan-600 dark:text-cyan-400 ml-1">
              {grado.docente_guia || 'Sin asignar'}
            </span>
          </p>
        </div>

        {/* Estadísticas - Ajustado para una sola línea */}
        <div className="flex justify-center gap-2 mb-4">
          <div className="flex items-center gap-1 text-blue-600 dark:text-blue-400 text-sm bg-blue-50 dark:bg-blue-900/30 px-2 py-1 rounded-full font-medium">
            <Users className="w-3 h-3" aria-hidden="true" />
            <span aria-label={`${grado.total_estudiantes} estudiantes en total`}>
              {grado.total_estudiantes}
            </span>
          </div>
          <div className="flex items-center gap-1 text-pink-500 dark:text-pink-400 text-sm bg-pink-50 dark:bg-pink-900/30 px-2 py-1 rounded-full font-medium">
            <User className="w-3 h-3" aria-hidden="true" />
            <span aria-label={`${grado.estudiantes_femeninos} estudiantes femeninas`}>
              {grado.estudiantes_femeninos}F
            </span>
          </div>
          <div className="flex items-center gap-1 text-indigo-600 dark:text-indigo-400 text-sm bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-full font-medium">
            <User className="w-3 h-3" aria-hidden="true" />
            <span aria-label={`${grado.estudiantes_masculinos} estudiantes masculinos`}>
              {grado.estudiantes_masculinos}M
            </span>
          </div>
        </div>

        {/* Botones de acción: Docentes y Estudiantes */}
        <div className="flex justify-center gap-2 mt-4">
          <button
            type="button"
            onClick={() => setModalVerDocentesAbierto(true)}
            className="flex items-center gap-2 px-4 py-2 rounded-lg bg-gray-100 dark:bg-slate-700 text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-slate-700 hover:bg-gray-200 dark:hover:bg-slate-600 transition-colors font-medium"
            aria-label="Ver docentes por asignatura"
          >
            <List className="w-5 h-5" /> Docentes
          </button>
          <button
            type="button"
            className="flex items-center gap-2 px-4 py-2 rounded-lg bg-gray-100 dark:bg-slate-700 text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-slate-700 hover:bg-gray-200 dark:hover:bg-slate-600 transition-colors font-medium"
            aria-label="Ver estudiantes de la sección"
            // onClick={...}  // Aquí puedes agregar la lógica para el modal de estudiantes
          >
            <Users className="w-5 h-5" /> Estudiantes
          </button>
        </div>
      </article>

      {/* Modal de Asignar Docentes */}
      <ModalAsignarDocentes
        isOpen={modalAsignaturasAbierto}
        onClose={() => setModalAsignaturasAbierto(false)}
        grado={grado}
        asignaturas={asignaturas}
        cargandoAsignaturas={cargandoAsignaturas}
        errorAsignaturas={errorAsignaturas}
        onAsignacionCompletada={onDocenteAsignado}
      />

      {/* Modal de Asignar Docente Guía */}
      <ModalAsignarDocente
        isOpen={modalAbierto}
        onClose={cerrarModal}
        onAsignar={asignarDocenteGuia}
        cargandoDocentes={cargandoDocentes}
        asignando={asignando}
        error={errorCargandoDocentes}
      />

      <ModalVerDocentesPorAsignatura
        isOpen={modalVerDocentesAbierto}
        onClose={() => setModalVerDocentesAbierto(false)}
        id_grado_secciones={grado.id_grado_secciones}
      />
    </>
  );
});

TarjetaCurso.displayName = 'TarjetaCurso';

// Componente principal
export const ListaGrados: React.FC = () => {
  const [añoActivo, setAñoActivo] = useState(1);
  const [modalidad, setModalidad] = useState(1);

  const { grados, cargando, error, obtenerGrados } = useGrados();

  useEffect(() => {
    obtenerGrados();
  }, [obtenerGrados]);

  // Memoizar grados filtrados para optimizar rendimiento
  const gradosFiltrados = useMemo(() => {
    return grados
      .filter((grado) => {
        const cumpleAño = grado.id_grado === añoActivo;
        const cumpleModalidad = grado.id_modalidad === modalidad;
        const cumpleSeccion = añoActivo !== 1 || SECCIONES_ORDEN.includes(grado.nombre_seccion as any);
        
        return cumpleAño && cumpleModalidad && cumpleSeccion;
      })
      .sort((a, b) => {
        if (añoActivo === 1) {
          const ordenA = SECCIONES_ORDEN.indexOf(a.nombre_seccion as any);
          const ordenB = SECCIONES_ORDEN.indexOf(b.nombre_seccion as any);
          return ordenA - ordenB;
        }
        return 0;
      });
  }, [grados, añoActivo, modalidad]);

  const handleRefresh = useCallback(() => {
    obtenerGrados();
  }, [obtenerGrados]);

  if (error) {
    return (
      <div className="p-6">
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 text-center">
          <p className="text-red-700 dark:text-red-400 font-medium">Error al cargar los grados</p>
          <button
            onClick={handleRefresh}
            className="mt-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
          >
            Reintentar
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 space-y-6">
      {/* Header con controles */}
      <header>
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
          {/* Tabs de años */}
          <nav role="tablist" aria-label="Seleccionar año académico">
            <div className="flex gap-1 p-1 bg-gray-100 dark:bg-slate-800 rounded-lg">
              {AÑOS.map((año) => (
                <button
                  key={año.id}
                  role="tab"
                  aria-selected={añoActivo === año.id}
                  className={`px-4 py-2 rounded-md font-medium transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-100 dark:focus:ring-offset-slate-800 ${
                    añoActivo === año.id
                      ? "bg-gradient-to-r from-blue-500 to-cyan-500 text-white shadow-md"
                      : "text-gray-600 dark:text-gray-300 hover:bg-white dark:hover:bg-slate-700 hover:text-gray-900 dark:hover:text-white"
                  }`}
                  onClick={() => setAñoActivo(año.id)}
                >
                  {año.label}
                </button>
              ))}
            </div>
          </nav>

          {/* Selector de modalidad */}
          <div className="flex items-center gap-2">
            <label htmlFor="modalidad-select" className="text-sm font-medium text-gray-700 dark:text-gray-300">
              Modalidad:
            </label>
            <select
              id="modalidad-select"
              className="px-3 py-2 rounded-lg bg-white dark:bg-slate-700 text-gray-900 dark:text-white border border-gray-300 dark:border-slate-600 shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
              value={modalidad}
              onChange={(e) => setModalidad(Number(e.target.value))}
            >
              {MODALIDADES.map((mod) => (
                <option key={mod.value} value={mod.value}>
                  {mod.label}
                </option>
              ))}
            </select>
          </div>
        </div>
      </header>

      {/* Contenido principal */}
      <main>
        {cargando ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-blue-600" />
            <span className="ml-3 text-gray-600 dark:text-gray-300">Cargando grados...</span>
          </div>
        ) : gradosFiltrados.length > 0 ? (
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-6">
            {gradosFiltrados.map((grado) => (
              <TarjetaCurso 
                key={grado.id_grado_secciones} 
                grado={grado} 
                onDocenteAsignado={handleRefresh}
              />
            ))}
          </div>
        ) : (
          <div className="text-center py-12">
            <div className="max-w-md mx-auto">
              <GraduationCap className="w-16 h-16 text-gray-300 dark:text-gray-600 mx-auto mb-4" />
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-2">
                No hay cursos disponibles
              </h3>
              <p className="text-gray-500 dark:text-gray-400">
                No se encontraron cursos para el año y modalidad seleccionados.
              </p>
            </div>
          </div>
        )}
      </main>
    </div>
  );
};