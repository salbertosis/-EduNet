import React, { useState, useEffect } from 'react';
import { useHistorial } from '../hooks/useHistorial';
import { EyeIcon } from '@heroicons/react/24/outline';
import { ModalCalificacionesHistorial } from './ModalCalificacionesHistorial';

interface HistorialAcademico {
  id_historial: number;
  id_estudiante: number;
  id_periodo: number;
  id_grado_secciones: number;
  promedio_anual: number;
  estatus: string;
  fecha_registro: string;
  periodo_escolar?: string;
  grado?: string;
  seccion?: string;
}

interface HistorialAcademicoProps {
  idEstudiante: number;
}

export const HistorialAcademico: React.FC<HistorialAcademicoProps> = ({ idEstudiante }) => {
  console.log('[DEBUG][COMPONENT] HistorialAcademico renderizado con idEstudiante:', idEstudiante);
  
  const { historial, loading, error, recargar } = useHistorial(idEstudiante);
  const [modalAbierto, setModalAbierto] = useState(false);
  const [periodoSeleccionado, setPeriodoSeleccionado] = useState<HistorialAcademico | null>(null);

  useEffect(() => {
    console.log('[DEBUG][COMPONENT] Historial actualizado:', historial);
  }, [historial]);

  if (loading) {
    console.log('[DEBUG][COMPONENT] Mostrando estado de carga');
    return (
      <div className="flex items-center justify-center p-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-500"></div>
        <span className="ml-3 text-gray-600">Cargando historial académico...</span>
      </div>
    );
  }

  if (error) {
    console.error('[DEBUG][COMPONENT] Error detectado:', error);
    return (
      <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
        <p className="text-red-600">Error: {error}</p>
        <button 
          onClick={recargar}
          className="mt-2 px-4 py-2 bg-red-100 text-red-700 rounded hover:bg-red-200"
        >
          Reintentar
        </button>
      </div>
    );
  }

  console.log('[DEBUG][COMPONENT] Renderizando tabla con historial:', historial);

  return (
    <section>
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-xl font-semibold text-emerald-600">Historial Académico</h3>
      </div>
      <div className="overflow-x-auto rounded-xl">
        <table className="min-w-full text-xs md:text-sm divide-y divide-emerald-400 dark:divide-cyan-800 rounded-xl overflow-hidden shadow-lg border border-emerald-200 dark:border-cyan-800">
          <thead className="sticky top-0 z-30 bg-gradient-to-r from-emerald-900 via-dark-800 to-dark-900 dark:from-[#059669] dark:via-[#2563eb] dark:to-[#181f2a] text-white">
            <tr>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">AÑO ESCOLAR</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">GRADO</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">SECCIÓN</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">PROMEDIO</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ESTADO</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ACCIONES</th>
            </tr>
          </thead>
          <tbody>
            {historial.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-6 py-4 text-center text-gray-500">
                  No hay historial registrado
                </td>
              </tr>
            ) : (
              historial.map((h) => (
                <tr key={h.id_historial} className="hover:bg-emerald-50 dark:hover:bg-[#2563eb]/10 transition">
                  <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{h.periodo_escolar}</td>
                  <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{h.grado}</td>
                  <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{h.seccion}</td>
                  <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{h.promedio_anual.toFixed(2)}</td>
                  <td className="px-2 py-2 text-center whitespace-nowrap">
                    <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full 
                      ${h.estatus === 'Aprobado' ? 'bg-green-100 text-green-800' : 
                        h.estatus === 'Repite' ? 'bg-yellow-100 text-yellow-800' : 
                        'bg-red-100 text-red-800'}`}>
                      {h.estatus?.toUpperCase()}
                    </span>
                  </td>
                  <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-500 whitespace-nowrap">
                    <button 
                      onClick={() => { 
                        setPeriodoSeleccionado(h); 
                        setModalAbierto(true); 
                      }}
                      className="text-emerald-600 hover:text-emerald-900"
                    >
                      <EyeIcon className="h-5 w-5" />
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
      {modalAbierto && periodoSeleccionado && (
        <ModalCalificacionesHistorial
          open={modalAbierto}
          onClose={() => {
            setModalAbierto(false);
            setPeriodoSeleccionado(null);
          }}
          idEstudiante={idEstudiante}
          idPeriodo={periodoSeleccionado.id_periodo}
          periodoEscolar={periodoSeleccionado.periodo_escolar || ''}
        />
      )}
    </section>
  );
}; 