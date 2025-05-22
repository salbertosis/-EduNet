import React, { useState } from 'react';
import { TablaCalificaciones } from './TablaCalificaciones';
import { ModalGuardarHistorial } from './ModalGuardarHistorial';
import { CalificacionEstudiante, Asignatura } from '../../types';

interface CalificacionesActualesProps {
  asignaturas: Asignatura[];
  calificaciones: CalificacionEstudiante[];
  errores: Record<string, string>;
  onInputChange: (id_asignatura: number, campo: keyof CalificacionEstudiante, valor: string) => void;
  limpiarErrores: () => void;
  setCalificaciones: (calificaciones: CalificacionEstudiante[]) => void;
  estudiante: any;
  periodoActual: number | null;
  mostrarModalGuardarHistorialEstudiante: boolean;
  setMostrarModalGuardarHistorialEstudiante: (v: boolean) => void;
  onGuardarHistorial: () => void;
  loadingGuardarHistorial: boolean;
  exitoGuardarHistorial: boolean;
  onGuardarCalificaciones: () => void;
  onGuardarPendientes: () => void;
  loadingGuardarCalificaciones: boolean;
  loadingGuardarPendientes: boolean;
}

export const CalificacionesActuales: React.FC<CalificacionesActualesProps> = ({
  asignaturas,
  calificaciones,
  errores,
  onInputChange,
  limpiarErrores,
  setCalificaciones,
  estudiante,
  periodoActual,
  mostrarModalGuardarHistorialEstudiante,
  setMostrarModalGuardarHistorialEstudiante,
  onGuardarHistorial,
  loadingGuardarHistorial,
  exitoGuardarHistorial,
  onGuardarCalificaciones,
  onGuardarPendientes,
  loadingGuardarCalificaciones,
  loadingGuardarPendientes,
}) => {
  const [mostrarAjustes, setMostrarAjustes] = useState(false);

  return (
    <section>
      <h3 className="text-xl font-semibold mb-4 text-emerald-400">Calificaciones del Año Actual</h3>
      {/* Botón Mostrar/Ocultar Ajustes */}
      <div className="flex justify-end mb-4">
        <button
          className="px-4 py-2 rounded-lg bg-cyan-600 text-white hover:bg-cyan-700 font-semibold shadow"
          onClick={() => setMostrarAjustes(!mostrarAjustes)}
        >
          {mostrarAjustes ? 'Ocultar Ajustes' : 'Mostrar Ajustes'}
        </button>
      </div>
      <TablaCalificaciones
        asignaturas={asignaturas}
        calificaciones={calificaciones}
        errores={errores}
        mostrarAjustes={mostrarAjustes}
        onInputChange={onInputChange}
      />
      {/* Botones de acción */}
      <div className="flex flex-col items-end gap-2 mt-6">
        <div className="flex gap-4">
          <button
            className="px-5 py-2 rounded-lg bg-gray-200 text-gray-700 hover:bg-gray-300 font-medium"
            onClick={limpiarErrores}
            disabled={loadingGuardarCalificaciones || loadingGuardarPendientes || loadingGuardarHistorial}
          >
            Limpiar
          </button>
          <button
            className="px-5 py-2 rounded-lg bg-emerald-600 text-white hover:bg-emerald-700 font-semibold shadow flex items-center gap-2"
            onClick={onGuardarCalificaciones}
            disabled={loadingGuardarCalificaciones}
          >
            {loadingGuardarCalificaciones && (
              <svg className="animate-spin h-5 w-5 text-white" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z" />
              </svg>
            )}
            {loadingGuardarCalificaciones ? 'Guardando…' : 'Guardar Calificaciones'}
          </button>
          <button
            className="px-5 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 font-semibold shadow flex items-center gap-2"
            onClick={() => setMostrarModalGuardarHistorialEstudiante(true)}
            disabled={loadingGuardarHistorial}
          >
            {loadingGuardarHistorial && (
              <svg className="animate-spin h-5 w-5 text-white" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z" />
              </svg>
            )}
            {loadingGuardarHistorial ? 'Guardando…' : 'Guardar Historial'}
          </button>
          <button
            className="px-5 py-2 rounded-lg bg-yellow-600 text-white hover:bg-yellow-700 font-semibold shadow flex items-center gap-2"
            onClick={onGuardarPendientes}
            disabled={loadingGuardarPendientes}
          >
            {loadingGuardarPendientes && (
              <svg className="animate-spin h-5 w-5 text-white" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z" />
              </svg>
            )}
            {loadingGuardarPendientes ? 'Guardando…' : 'Guardar Pendientes'}
          </button>
        </div>
      </div>
      <ModalGuardarHistorial
        open={mostrarModalGuardarHistorialEstudiante}
        onClose={() => setMostrarModalGuardarHistorialEstudiante(false)}
        onConfirmar={() => {
          onGuardarHistorial();
          setMostrarModalGuardarHistorialEstudiante(false);
        }}
        loading={loadingGuardarHistorial}
      />
    </section>
  );
}; 