import React from 'react';

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

interface FilaPromediosProps {
  asignaturas: Asignatura[];
  calificaciones: CalificacionEstudiante[];
  mostrarAjustes: boolean;
}

function promedioLapso(asignaturas: Asignatura[], calificaciones: CalificacionEstudiante[], lapso: keyof CalificacionEstudiante, lapsoAjuste: keyof CalificacionEstudiante) {
  const notas = asignaturas
    .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
    .map(a => {
      const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
      if (!calif) return undefined;
      if (typeof calif[lapsoAjuste] === 'number') return calif[lapsoAjuste] as number;
      if (typeof calif[lapso] === 'number') return calif[lapso] as number;
      return undefined;
    })
    .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
  if (notas.length === 0) return '';
  return (notas.reduce((a, b) => a + b, 0) / notas.length).toFixed(2);
}

function promedioFinal(asignaturas: Asignatura[], calificaciones: CalificacionEstudiante[]) {
  const notas = asignaturas
    .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
    .map(a => {
      const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
      if (!calif) return undefined;
      // Aquí podrías usar la lógica de obtenerNotaValida si la tienes en un util
      if (typeof calif.revision === 'number' && calif.revision > 0) return Number(calif.revision);
      if (typeof calif.nota_final === 'number') return calif.nota_final;
      return undefined;
    })
    .filter(n => typeof n === 'number' && !isNaN(n)) as number[];
  if (notas.length === 0) return '';
  return (notas.reduce((a, b) => a + b, 0) / notas.length).toFixed(2);
}

export const FilaPromedios: React.FC<FilaPromediosProps> = ({ asignaturas, calificaciones, mostrarAjustes }) => {
  return (
    <tr className="bg-gradient-to-r from-emerald-900 via-dark-800 to-dark-900 dark:from-[#059669] dark:via-[#2563eb] dark:to-[#181f2a] font-bold">
      <td className="px-4 py-2 w-48 text-right text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent"></td>
      <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
        <div className="flex items-center justify-center h-full">{promedioLapso(asignaturas, calificaciones, 'lapso_1', 'lapso_1_ajustado')}</div>
      </td>
      {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
      <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
        <div className="flex items-center justify-center h-full">{promedioLapso(asignaturas, calificaciones, 'lapso_2', 'lapso_2_ajustado')}</div>
      </td>
      {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
      <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
        <div className="flex items-center justify-center h-full">{promedioLapso(asignaturas, calificaciones, 'lapso_3', 'lapso_3_ajustado')}</div>
      </td>
      {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
      <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
        <div className="flex items-center justify-center h-full">{promedioFinal(asignaturas, calificaciones)}</div>
      </td>
      <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>
      <td className="px-1 py-2 w-24 text-center bg-white dark:bg-transparent align-middle"></td>
    </tr>
  );
}; 