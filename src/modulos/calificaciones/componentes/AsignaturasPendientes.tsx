import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

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

interface Props {
  idEstudiante: number;
}

export function AsignaturasPendientes({ idEstudiante }: Props) {
  const [pendientes, setPendientes] = useState<AsignaturaPendiente[]>([]);
  const [cargando, setCargando] = useState(true);

  useEffect(() => {
    setCargando(true);
    console.log('[DEBUG][FRONTEND] Solicitando asignaturas pendientes para:', idEstudiante);
    invoke<AsignaturaPendiente[]>('obtener_asignaturas_pendientes_estudiante', { idEstudiante })
      .then(data => {
        console.log('[DEBUG][FRONTEND] Asignaturas pendientes recibidas:', data);
        setPendientes(data);
      })
      .catch((err) => {
        console.error('[DEBUG][FRONTEND] Error al obtener asignaturas pendientes:', err);
        setPendientes([]);
      })
      .finally(() => setCargando(false));
  }, [idEstudiante]);

  return (
    <section>
      <h3 className="text-xl font-semibold mb-4">Asignaturas Pendientes</h3>
      {cargando ? (
        <div className="text-center py-8 text-gray-400">Cargando asignaturas pendientes...</div>
      ) : (
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
              {pendientes.length === 0 ? (
                <tr>
                  <td colSpan={5} className="text-center py-4 text-gray-400">No hay asignaturas pendientes.</td>
                </tr>
              ) : pendientes.map((p) => (
                <tr key={p.id_pendiente}>
                  <td className="px-4 py-2">{p.periodo_escolar || ''}</td>
                  <td className="px-4 py-2">{p.grado}</td>
                  <td className="px-4 py-2">{p.nombre_asignatura || p.id_asignatura}</td>
                  <td className="px-4 py-2 text-center">{p.cal_momento1 ?? ''}</td>
                  <td className="px-4 py-2">
                    <span className="px-3 py-1 rounded-full text-xs font-bold bg-red-200 text-red-800">{p.estado}</span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </section>
  );
} 