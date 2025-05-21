import React from 'react';
import { usePendientes } from '../hooks/usePendientes';

interface AsignaturaPendiente {
  id_pendiente: number;
  id_estudiante: number;
  id_asignatura: number;
  id_periodo: number;
  grado: string;
  cal_momento1?: number;
  estado: string;
  fecha_registro: string;
  id_grado_secciones: number;
  nombre_asignatura?: string;
  periodo_escolar?: string;
}

interface AsignaturasPendientesProps {
  idEstudiante: number;
}

export const AsignaturasPendientes: React.FC<AsignaturasPendientesProps> = ({ idEstudiante }) => {
  const { pendientes, loading, recargar } = usePendientes(idEstudiante);

  if (loading) return <div>Cargando asignaturas pendientes...</div>;

  return (
    <section>
      <h3 className="text-xl font-semibold mb-4 text-emerald-600">Asignaturas Pendientes</h3>
      <div className="overflow-x-auto rounded-xl">
        <table className="min-w-full text-xs md:text-sm divide-y divide-emerald-400 dark:divide-cyan-800 rounded-xl overflow-hidden shadow-lg border border-emerald-200 dark:border-cyan-800">
          <thead className="sticky top-0 z-30 bg-gradient-to-r from-emerald-900 via-dark-800 to-dark-900 dark:from-[#059669] dark:via-[#2563eb] dark:to-[#181f2a] text-white">
            <tr>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">AÑO ESCOLAR</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">GRADO</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ASIGNATURA</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">CALIFICACIÓN</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ESTADO</th>
            </tr>
          </thead>
          <tbody>
            {pendientes.length === 0 ? (
              <tr>
                <td colSpan={5} className="text-center py-4 text-gray-400">No hay asignaturas pendientes.</td>
              </tr>
            ) : pendientes.map((p) => (
              <tr key={p.id_pendiente} className="hover:bg-emerald-50 dark:hover:bg-[#2563eb]/10 transition">
                <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{p.periodo_escolar || ''}</td>
                <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{p.grado}</td>
                <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{p.nombre_asignatura || p.id_asignatura}</td>
                <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{p.cal_momento1 ?? ''}</td>
                <td className="px-2 py-2 text-center whitespace-nowrap">
                  <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full 
                    ${p.estado === 'Aprobado' ? 'bg-green-100 text-green-800' : 
                      p.estado === 'Repite' ? 'bg-yellow-100 text-yellow-800' : 
                      'bg-red-100 text-red-800'}`}>
                    {p.estado?.toUpperCase()}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}; 