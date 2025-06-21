import React, { useState, useCallback, useEffect } from 'react';
import { X, Search, Activity } from 'lucide-react';
import { ActividadPgcrp, AsignacionPgcrpInput } from '../tipos/pgcrp_simple';
import { obtenerActividadesPgcrpSimple, asignarPgcrpSeccionSimple } from '../servicios/pgcrpSimpleService';
import { useMensajeGlobal } from './MensajeGlobalContext';

interface ModalAsignarPgcrpProps {
  isOpen: boolean;
  onClose: () => void;
  grado: {
    id_grado_secciones: number;
    nombre_grado: string;
    nombre_seccion: string;
  };
  id_periodo: number;
  onAsignacionCompletada?: () => void;
}

export const ModalAsignarPgcrp: React.FC<ModalAsignarPgcrpProps> = ({
  isOpen,
  onClose,
  grado,
  id_periodo,
  onAsignacionCompletada
}) => {
  const { mostrarMensaje } = useMensajeGlobal();
  const [actividades, setActividades] = useState<ActividadPgcrp[]>([]);
  const [actividadesFiltradas, setActividadesFiltradas] = useState<ActividadPgcrp[]>([]);
  const [busqueda, setBusqueda] = useState('');
  const [actividadSeleccionada, setActividadSeleccionada] = useState<ActividadPgcrp | null>(null);
  const [cargandoActividades, setCargandoActividades] = useState(false);
  const [asignando, setAsignando] = useState(false);
  const [errorCargandoActividades, setErrorCargandoActividades] = useState(false);

  useEffect(() => {
    if (isOpen) {
      cargarActividades();
      setBusqueda('');
      setActividadSeleccionada(null);
    }
  }, [isOpen]);

  useEffect(() => {
    // Filtrar actividades por búsqueda
    const filtradas = actividades.filter(actividad =>
      actividad.nombre.toLowerCase().includes(busqueda.toLowerCase())
    );
    setActividadesFiltradas(filtradas);
  }, [busqueda, actividades]);

  const cargarActividades = async () => {
    setCargandoActividades(true);
    setErrorCargandoActividades(false);
    try {
      const listaActividades = await obtenerActividadesPgcrpSimple();
      setActividades(listaActividades);
      setActividadesFiltradas(listaActividades);
    } catch (error) {
      console.error('Error al cargar actividades PGCRP:', error);
      setErrorCargandoActividades(true);
      mostrarMensaje('Error al cargar las actividades PGCRP', 'error');
    } finally {
      setCargandoActividades(false);
    }
  };

  const handleAsignar = async () => {
    if (!actividadSeleccionada) {
      mostrarMensaje('Debe seleccionar una actividad PGCRP', 'error');
      return;
    }

    setAsignando(true);
    try {
      const input: AsignacionPgcrpInput = {
        id_grado_secciones: grado.id_grado_secciones,
        id_pgcrp: actividadSeleccionada.id_pgcrp,
        id_periodo: id_periodo
      };

      await asignarPgcrpSeccionSimple(input);
      mostrarMensaje('Actividad PGCRP asignada correctamente', 'exito');
      onAsignacionCompletada?.();
      onClose();
    } catch (error) {
      console.error('Error al asignar PGCRP:', error);
      mostrarMensaje('Error al asignar la actividad PGCRP', 'error');
    } finally {
      setAsignando(false);
    }
  };

  const handleClose = useCallback(() => {
    if (!asignando) {
      setActividadSeleccionada(null);
      setBusqueda('');
      setActividades([]);
      onClose();
    }
  }, [asignando, onClose]);

  if (!isOpen) return null;

  return (
    <div 
      className="fixed inset-0 flex items-center justify-center bg-black/50 backdrop-blur-sm z-50 p-4" 
      role="dialog" 
      aria-modal="true" 
      aria-labelledby="modal-title" 
      onClick={e => e.target === e.currentTarget && handleClose()}
    >
      <div className="bg-white dark:bg-slate-800 rounded-xl shadow-2xl w-full max-w-md max-h-[90vh] overflow-auto">
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-slate-700">
          <h2 id="modal-title" className="text-xl font-semibold text-gray-900 dark:text-white">
            Asignar Actividad PGCRP
          </h2>
          <button 
            type="button" 
            onClick={handleClose} 
            disabled={asignando} 
            className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors disabled:opacity-50" 
            aria-label="Cerrar modal"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="p-6 space-y-4">
          <div className="text-center">
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Sección: <span className="font-semibold text-blue-600 dark:text-blue-400">
                {grado.nombre_grado} Año {grado.nombre_seccion}
              </span>
            </p>
          </div>

          {/* Campo de búsqueda */}
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
            <input
              type="text"
              placeholder="Buscar actividad PGCRP..."
              value={busqueda}
              onChange={(e) => setBusqueda(e.target.value)}
              disabled={cargandoActividades || asignando}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-slate-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-slate-700 dark:text-white disabled:opacity-50"
              autoFocus
            />
          </div>

          {/* Lista de actividades */}
          <div className="space-y-2 max-h-60 overflow-y-auto">
            {cargandoActividades ? (
              <div className="flex items-center justify-center py-8">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                <span className="ml-2 text-gray-600 dark:text-gray-400">Cargando actividades...</span>
              </div>
            ) : errorCargandoActividades ? (
              <div className="text-center py-8">
                <p className="text-red-600 dark:text-red-400">Error al cargar las actividades</p>
                <button
                  onClick={cargarActividades}
                  className="mt-2 text-sm text-blue-600 dark:text-blue-400 hover:underline"
                >
                  Reintentar
                </button>
              </div>
            ) : actividadesFiltradas.length === 0 ? (
              <div className="text-center py-8">
                <Activity className="w-12 h-12 text-gray-400 mx-auto mb-2" />
                <p className="text-gray-500 dark:text-gray-400">
                  {busqueda ? 'No se encontraron actividades' : 'No hay actividades disponibles'}
                </p>
              </div>
            ) : (
                             actividadesFiltradas.map((actividad) => (
                 <button
                   key={actividad.id_pgcrp}
                   onClick={() => setActividadSeleccionada(actividad)}
                   disabled={asignando}
                   className={`w-full text-left p-3 rounded-lg border transition-colors disabled:opacity-50 ${
                     actividadSeleccionada?.id_pgcrp === actividad.id_pgcrp
                       ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300'
                       : 'border-gray-200 dark:border-slate-600 hover:bg-gray-50 dark:hover:bg-slate-700'
                   }`}
                 >
                  <div className="flex items-center space-x-3">
                    <Activity className="w-5 h-5 text-gray-400" />
                    <span className="font-medium">{actividad.nombre}</span>
                  </div>
                </button>
              ))
            )}
          </div>

          {/* Botones de acción */}
          <div className="flex justify-end space-x-3 pt-4 border-t border-gray-200 dark:border-slate-700">
            <button
              type="button"
              onClick={handleClose}
              disabled={asignando}
              className="px-4 py-2 text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-slate-600 rounded-lg hover:bg-gray-50 dark:hover:bg-slate-700 transition-colors disabled:opacity-50"
            >
              Cancelar
            </button>
            <button
              type="button"
              onClick={handleAsignar}
              disabled={!actividadSeleccionada || asignando}
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center space-x-2"
            >
              {asignando ? (
                <>
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                  <span>Asignando...</span>
                </>
              ) : (
                <span>Asignar</span>
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}; 