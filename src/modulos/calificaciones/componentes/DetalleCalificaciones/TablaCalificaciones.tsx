import React from 'react';
import { FilaCalificacion } from './FilaCalificacion';
import { FilaPromedios } from './FilaPromedios';

interface Asignatura {
  id_asignatura: number;
  nombre_asignatura: string;
  id_grado: number;
  id_modalidad: number;
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

interface TablaCalificacionesProps {
  asignaturas: Asignatura[];
  calificaciones: CalificacionEstudiante[];
  errores: Record<string, string>;
  mostrarAjustes: boolean;
  onInputChange: (id_asignatura: number, campo: keyof CalificacionEstudiante, valor: string) => void;
}

export const TablaCalificaciones: React.FC<TablaCalificacionesProps> = ({ asignaturas, calificaciones, errores, mostrarAjustes, onInputChange }) => {
  const totalRevisionMenor10 = calificaciones.filter(c => typeof c.revision === 'number' && Number(c.revision) < 10).length;
  // Calcula el total de inputs de revisión activos (nota_final < 9.5) con revision < 10
  const totalPendientesRevisionActivos = calificaciones.filter(c => {
    const notaFinal = typeof c.nota_final === 'number' ? c.nota_final : 0;
    const revision = c.revision !== undefined && c.revision !== '' ? Number(c.revision) : undefined;
    return notaFinal < 9.5 && revision !== undefined && !isNaN(revision) && revision < 10;
  }).length;
  return (
    <div className="overflow-x-auto max-h-[60vh]">
      <table className="min-w-full divide-y divide-emerald-400 dark:divide-cyan-800 text-sm rounded-xl overflow-hidden shadow-lg">
        <thead>
          <tr>
            <th className="px-4 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">ASIGNATURA</th>
            <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">1ER LAPSO</th>
            {mostrarAjustes && <th className="px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">AJUSTE 1</th>}
            <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">2DO LAPSO</th>
            {mostrarAjustes && <th className="px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">AJUSTE 2</th>}
            <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">3ER LAPSO</th>
            {mostrarAjustes && <th className="px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">AJUSTE 3</th>}
            <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">FINAL</th>
            <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">REVISIÓN</th>
            <th className="px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase bg-white dark:bg-transparent">ESTADO</th>
          </tr>
        </thead>
        <tbody>
          {asignaturas.map((asig) => (
            <FilaCalificacion
              key={asig.id_asignatura}
              asignatura={asig}
              calificacion={calificaciones.find(c => c.id_asignatura === asig.id_asignatura)}
              errores={errores}
              mostrarAjustes={mostrarAjustes}
              onInputChange={onInputChange}
              totalPendientesRevisionActivos={totalPendientesRevisionActivos}
            />
          ))}
          <FilaPromedios asignaturas={asignaturas} calificaciones={calificaciones} mostrarAjustes={mostrarAjustes} />
        </tbody>
      </table>
    </div>
  );
}; 