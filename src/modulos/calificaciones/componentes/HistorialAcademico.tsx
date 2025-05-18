import React, { useState } from 'react';
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
  const { historial, loading, error, recargar } = useHistorial(idEstudiante);
  const [modalAbierto, setModalAbierto] = useState(false);
  const [periodoSeleccionado, setPeriodoSeleccionado] = useState<HistorialAcademico | null>(null);

  if (loading) return (
    <div className="flex items-center justify-center p-8">
      <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-500"></div>
      <span className="ml-3 text-gray-600">Cargando historial académico...</span>
    </div>
  );

  if (error) return (
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

  return (
    <section>
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-xl font-semibold text-emerald-600">Historial Académico</h3>
        <button 
          onClick={recargar}
          className="px-4 py-2 bg-emerald-100 text-emerald-700 rounded-lg hover:bg-emerald-200 transition-colors"
        >
          Actualizar
        </button>
      </div>

      <div className="overflow-x-auto rounded-lg border border-gray-200">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Año Escolar</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Grado</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Sección</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Promedio</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Estado</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Acciones</th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {historial.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-6 py-4 text-center text-gray-500">
                  No hay historial registrado
                </td>
              </tr>
            ) : (
              historial.map((h) => (
                <tr key={h.id_historial} className="hover:bg-gray-50">
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{h.periodo_escolar}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{h.grado}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{h.seccion}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{h.promedio_anual.toFixed(2)}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full 
                      ${h.estatus === 'Aprobado' ? 'bg-green-100 text-green-800' : 
                        h.estatus === 'Repite' ? 'bg-red-100 text-red-800' : 
                        'bg-yellow-100 text-yellow-800'}`}>
                      {h.estatus}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
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
          onClose={() => setModalAbierto(false)}
          idEstudiante={idEstudiante}
          idPeriodo={periodoSeleccionado.id_periodo}
          periodoEscolar={periodoSeleccionado.periodo_escolar || ''}
        />
      )}
    </section>
  );
}; 