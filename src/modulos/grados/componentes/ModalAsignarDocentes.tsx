import React, { useState, useEffect } from 'react';
import { X, Loader2, CheckCircle2 } from 'lucide-react';
import { invoke } from '@tauri-apps/api/tauri';

import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

interface Docente {
  id_docente: number;
  nombres: string;
  apellidos: string;
}

interface Asignatura {
  id_asignatura: number;
  nombre_asignatura: string;
  codigo_asignatura: string;
  id_docente?: number;
}

interface ModalAsignarDocentesProps {
  isOpen: boolean;
  onClose: () => void;
  grado: {
    id_grado_secciones: number;
    nombre_grado: string;
    nombre_seccion: string;
  };
  asignaturas: any[];
  cargandoAsignaturas: boolean;
  errorAsignaturas: string | null;
  onAsignacionCompletada?: () => void;
}

export const ModalAsignarDocentes: React.FC<ModalAsignarDocentesProps> = ({
  isOpen,
  onClose,
  grado,
  asignaturas,
  cargandoAsignaturas,
  errorAsignaturas,
  onAsignacionCompletada
}) => {
  const { mostrarMensaje } = useMensajeGlobal();
  const [docentes, setDocentes] = useState<Docente[]>([]);
  const [cargandoDocentes, setCargandoDocentes] = useState(false);
  const [asignando, setAsignando] = useState(false);
  const [errorCargandoDocentes, setErrorCargandoDocentes] = useState(false);
  const [asignaturasLocales, setAsignaturasLocales] = useState<Asignatura[]>(asignaturas || []);
  const [asignaturasEditadas, setAsignaturasEditadas] = useState<{[id:number]: number}>({});
  const [enviado, setEnviado] = useState(false);

  useEffect(() => {
    if (isOpen) {
      cargarDocentes();
      setEnviado(false);
    }
  }, [isOpen]);

  useEffect(() => {
    setAsignaturasLocales(asignaturas || []);
    setAsignaturasEditadas({});
  }, [asignaturas]);

  const cargarDocentes = async () => {
    setCargandoDocentes(true);
    setErrorCargandoDocentes(false);
    try {
      const listaDocentes = await invoke<any>('obtener_docentes', {
        filtro: null,
        paginacion: { pagina: 1, registros_por_pagina: 1000 }
      });
      setDocentes(listaDocentes.datos ?? []);
    } catch (error) {
      setErrorCargandoDocentes(true);
      mostrarMensaje('Error al cargar la lista de docentes', 'error');
    } finally {
      setCargandoDocentes(false);
    }
  };

  const handleSelectDocente = (asignaturaId: number, docenteId: number) => {
    setAsignaturasEditadas(prev => ({ ...prev, [asignaturaId]: docenteId }));
    setAsignaturasLocales(prev => prev.map(asig =>
      asig.id_asignatura === asignaturaId ? { ...asig, id_docente: docenteId } : asig
    ));
  };

  const handleEnviar = async () => {
    setAsignando(true);
    let exito = true;
    for (const [id_asignatura, id_docente] of Object.entries(asignaturasEditadas)) {
      try {
        await invoke('asignar_docente_asignatura', {
          id_asignatura: Number(id_asignatura),
          id_docente: id_docente,
          id_grado_secciones: grado.id_grado_secciones
        });
      } catch (error) {
        exito = false;
        mostrarMensaje('Error al asignar docente', 'error');
      }
    }
    setAsignando(false);
    setEnviado(true);
    if (exito) {
      mostrarMensaje('Asignaciones guardadas correctamente', 'exito');
      setAsignaturasEditadas({});
      onAsignacionCompletada?.();
      // Opcional: cerrar modal tras éxito
      // onClose();
    }
  };

  if (!isOpen) return null;

  return (
    <div 
      className="fixed inset-0 flex items-center justify-center bg-black/50 backdrop-blur-sm z-50 p-4" 
      role="dialog" 
      aria-modal="true" 
      aria-labelledby="modal-asignaturas-title" 
      onClick={(e) => { if (e.target === e.currentTarget) onClose(); }}
    >
      <div className="bg-white dark:bg-slate-800 rounded-xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-auto focus:outline-none" tabIndex={-1}>
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-slate-700">
          <h2 id="modal-asignaturas-title" className="text-xl font-semibold text-gray-900 dark:text-white">
            Asignar Docentes - {grado.nombre_grado} {grado.nombre_seccion}
          </h2>
          <button 
            type="button" 
            onClick={onClose}
            className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors" 
            aria-label="Cerrar modal"
            autoFocus
          >
            <X className="w-5 h-5" />
          </button>
        </div>
        <form className="p-6 space-y-2" onSubmit={e => { e.preventDefault(); handleEnviar(); }}>
          {cargandoAsignaturas ? (
            <div className="flex items-center justify-center py-4">
              <Loader2 className="w-5 h-5 animate-spin text-blue-600" />
              <span className="ml-2 text-gray-600 dark:text-gray-300">Cargando asignaturas...</span>
            </div>
          ) : errorAsignaturas ? (
            <div className="text-center py-4 text-red-600 dark:text-red-400">{errorAsignaturas}</div>
          ) : cargandoDocentes ? (
            <div className="flex items-center justify-center py-4">
              <Loader2 className="w-5 h-5 animate-spin text-blue-600" />
              <span className="ml-2 text-gray-600 dark:text-gray-300">Cargando docentes...</span>
            </div>
          ) : errorCargandoDocentes ? (
            <div className="text-center py-4 text-red-600 dark:text-red-400">Error al cargar la lista de docentes</div>
          ) : asignaturasLocales.length === 0 ? (
            <div className="text-center py-4 text-gray-600 dark:text-gray-400">No hay asignaturas disponibles</div>
          ) : (
            <div className="flex flex-col gap-2">
              {asignaturasLocales.map((asignatura) => (
                <div key={asignatura.id_asignatura} className="flex flex-col sm:flex-row sm:items-center justify-between bg-white dark:bg-slate-800 rounded-lg shadow-sm px-4 py-2 border border-slate-100 dark:border-slate-700">
                  <label htmlFor={`docente-select-${asignatura.id_asignatura}`} className="font-medium text-gray-900 dark:text-white mb-1 sm:mb-0 sm:w-1/2">
                    {asignatura.nombre_asignatura}
                  </label>
                  <select
                    id={`docente-select-${asignatura.id_asignatura}`}
                    value={asignatura.id_docente?.toString() || ''}
                    onChange={e => {
                      const docenteId = e.target.value ? parseInt(e.target.value) : null;
                      if (docenteId) handleSelectDocente(asignatura.id_asignatura, docenteId);
                    }}
                    disabled={asignando}
                    className="w-full sm:w-64 px-3 py-2 bg-white dark:bg-slate-700 border border-gray-300 dark:border-slate-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:text-white disabled:opacity-50"
                    aria-label={`Seleccionar docente para ${asignatura.nombre_asignatura}`}
                  >
                    <option value="">Seleccionar docente</option>
                    {docentes.map((docente) => (
                      <option key={docente.id_docente} value={docente.id_docente.toString()}>
                        {docente.nombres} {docente.apellidos}
                      </option>
                    ))}
                  </select>
                </div>
              ))}
            </div>
          )}
          <div className="flex justify-end mt-6">
            <button
              type="submit"
              className="inline-flex items-center gap-2 px-6 py-2 rounded-lg bg-blue-600 text-white font-semibold shadow hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
              disabled={asignando || Object.keys(asignaturasEditadas).length === 0}
              aria-disabled={asignando || Object.keys(asignaturasEditadas).length === 0}
            >
              {asignando ? <Loader2 className="w-5 h-5 animate-spin" /> : <CheckCircle2 className="w-5 h-5" />}
              {asignando ? 'Enviando...' : enviado ? 'Enviado' : 'Enviar'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}; 