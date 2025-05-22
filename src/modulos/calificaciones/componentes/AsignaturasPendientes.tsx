import React, { useState } from 'react';
import { usePendientes } from '../hooks/usePendientes';
import { ModalPendiente } from './ModalPendiente';
import type { Pendiente } from './ModalPendiente';
import DeleteIcon from '@mui/icons-material/Delete';
import VisibilityIcon from '@mui/icons-material/Visibility';

interface AsignaturaPendiente {
  id_pendiente: number;
  id_estudiante: number;
  id_asignatura: number;
  id_periodo: number;
  grado: string;
  cal_momento1?: number;
  cal_momento2?: number;
  cal_momento3?: number;
  cal_momento4?: number;
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
  const { pendientes, loading, eliminarAsignaturaPendiente, recargar } = usePendientes(idEstudiante);
  const [mostrarModal, setMostrarModal] = useState(false);
  const [pendienteSeleccionada, setPendienteSeleccionada] = useState<Pendiente | null>(null);

  if (loading) return <div>Cargando asignaturas pendientes...</div>;

  const handleEliminarPendiente = async (id_pendiente: number) => {
    if (window.confirm('¿Estás seguro de que deseas eliminar esta asignatura pendiente?')) {
      await eliminarAsignaturaPendiente(id_pendiente);
      await recargar();
    }
  };

  const handleVerDetalle = (pendiente: AsignaturaPendiente) => {
    setPendienteSeleccionada({
      id_pendiente: pendiente.id_pendiente,
      nombre_asignatura: pendiente.nombre_asignatura ?? '',
      periodo_escolar: pendiente.periodo_escolar ?? '',
      cal_momento1: pendiente.cal_momento1,
      cal_momento2: pendiente.cal_momento2,
      cal_momento3: pendiente.cal_momento3,
      cal_momento4: pendiente.cal_momento4,
    });
    setMostrarModal(true);
  };

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
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">M1</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">M2</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">M3</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">M4</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ESTADO</th>
              <th className="px-2 py-2 text-center font-bold uppercase whitespace-nowrap">ACCIONES</th>
            </tr>
          </thead>
          <tbody>
            {pendientes.length === 0 ? (
              <tr>
                <td colSpan={9} className="text-center py-4 text-gray-400">No hay asignaturas pendientes.</td>
              </tr>
            ) : pendientes.map((p) => {
                const momentos = [p.cal_momento1, p.cal_momento2, p.cal_momento3, p.cal_momento4]
                  .map(m => (typeof m === 'number' && m > 0 ? m : null));
                // Estado profesional
                let estado = 'Pendiente';
                if (momentos.some(m => m !== null && m >= 10)) {
                  estado = 'Aprobado';
                } else if (
                  momentos.length === 4 &&
                  momentos.every(m => typeof m === 'number' && m !== null && m < 10)
                ) {
                  estado = 'Repite';
                }
                return (
                  <tr key={p.id_pendiente} className="hover:bg-emerald-50 dark:hover:bg-[#2563eb]/10 transition">
                    <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{p.periodo_escolar || ''}</td>
                    <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{p.grado}</td>
                    <td className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">{p.nombre_asignatura || p.id_asignatura}</td>
                    {[0,1,2,3].map(idx => (
                      <td key={idx} className="px-2 py-2 text-center text-xs md:text-sm text-gray-900 dark:text-cyan-200 whitespace-nowrap">
                        {typeof momentos[idx] === 'number' && momentos[idx] !== null ? (momentos[idx]! < 10 ? `0${momentos[idx]}`.slice(-2) : momentos[idx]) : '—'}
                      </td>
                    ))}
                    <td className="px-2 py-2 text-center whitespace-nowrap">
                      <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full 
                        ${estado === 'Aprobado' ? 'bg-green-100 text-green-800' : 
                          estado === 'Repite' ? 'bg-yellow-100 text-yellow-800' : 
                          'bg-red-100 text-red-800'}`}>
                        {estado.toUpperCase()}
                      </span>
                    </td>
                    <td className="px-2 py-2 text-center whitespace-nowrap flex gap-2 justify-center">
                      <DeleteIcon
                        fontSize="medium"
                        className="text-red-600 cursor-pointer hover:bg-red-100 rounded-full transition"
                        titleAccess="Eliminar"
                        onClick={() => handleEliminarPendiente(p.id_pendiente)}
                      />
                      <VisibilityIcon
                        fontSize="medium"
                        className="text-blue-600 cursor-pointer hover:bg-blue-100 rounded-full transition"
                        titleAccess="Editar"
                        onClick={() => handleVerDetalle(p)}
                      />
                    </td>
                  </tr>
                );
              })}
          </tbody>
        </table>
      </div>
      <ModalPendiente
        open={mostrarModal}
        onClose={() => setMostrarModal(false)}
        pendiente={pendienteSeleccionada}
        estudianteId={idEstudiante}
        onGuardar={(idPendiente: number, valores: { momento1: number, momento2: number, momento3: number, momento4: number }) => {
          // TODO: lógica para guardar los valores de los momentos en backend
          setMostrarModal(false);
        }}
      />
    </section>
  );
}; 