import React from 'react';
import { FilaCalificacion } from './FilaCalificacion';
import { FilaPromedios } from './FilaPromedios';
import { calcularTotalPendientes, useCalculosCalificaciones } from '../../../../utilidades/calculoNotas';

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
  const { calcularNotaFinal, calcularEstadoAsignatura } = useCalculosCalificaciones(asignaturas, calificaciones);
  const totalPendientes = calcularTotalPendientes(asignaturas, calificaciones);
  return (
    <div className="w-full">
      <table className="min-w-full divide-y divide-emerald-400 dark:divide-cyan-800 text-sm rounded-xl overflow-hidden shadow-lg">
        <thead>
          <tr>
            <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-4 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase">ASIGNATURA</th>
            <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase">1ER LAPSO</th>
            {mostrarAjustes && <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase">AJUSTE 1</th>}
            <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase">2DO LAPSO</th>
            {mostrarAjustes && <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase">AJUSTE 2</th>}
            <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase">3ER LAPSO</th>
            {mostrarAjustes && <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-cyan-700 dark:text-cyan-200 font-bold uppercase">AJUSTE 3</th>}
            <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase">FINAL</th>
            <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase">REVISIÓN</th>
            <th className="sticky top-0 z-20 bg-white dark:bg-slate-800 border-b-2 border-emerald-400 dark:border-cyan-800 px-2 py-3 text-center text-emerald-700 dark:text-cyan-200 font-bold uppercase">ESTADO</th>
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
              calcularNotaFinal={calcularNotaFinal}
              calcularEstadoAsignatura={calcularEstadoAsignatura}
              totalPendientes={totalPendientes}
              asignaturas={asignaturas}
              calificaciones={calificaciones}
            />
          ))}
          <FilaPromedios asignaturas={asignaturas} calificaciones={calificaciones} mostrarAjustes={mostrarAjustes} />
        </tbody>
      </table>
    </div>
  );
}; 