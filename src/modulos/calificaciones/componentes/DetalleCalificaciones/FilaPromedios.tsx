import React from 'react';
import { useCalculosCalificaciones } from '../../../../utilidades/calculoNotas';
import { CalificacionEstudiante } from '../../types';
import { Asignatura } from '../../types';

interface FilaPromediosProps {
  asignaturas: Asignatura[];
  calificaciones: CalificacionEstudiante[];
  mostrarAjustes: boolean;
}

export const FilaPromedios: React.FC<FilaPromediosProps> = ({ asignaturas, calificaciones, mostrarAjustes }) => {
  const { promedioLapso, promedioFinal } = useCalculosCalificaciones(asignaturas, calificaciones);

  return (
    <tr className="bg-gradient-to-r from-emerald-900 via-dark-800 to-dark-900 dark:from-[#059669] dark:via-[#2563eb] dark:to-[#181f2a] font-bold">
      <td className="px-4 py-2 w-48 text-right text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent"></td>
      <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
        <div className="flex items-center justify-center h-full">{promedioLapso('lapso_1', 'lapso_1_ajustado')}</div>
      </td>
      {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
      <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
        <div className="flex items-center justify-center h-full">{promedioLapso('lapso_2', 'lapso_2_ajustado')}</div>
      </td>
      {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
      <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
        <div className="flex items-center justify-center h-full">{promedioLapso('lapso_3', 'lapso_3_ajustado')}</div>
      </td>
      {mostrarAjustes && <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>}
      <td className="px-1 py-2 w-20 text-center text-emerald-700 dark:text-cyan-200 bg-white dark:bg-transparent align-middle">
        <div className="flex items-center justify-center h-full">{promedioFinal()}</div>
      </td>
      <td className="px-1 py-2 w-20 text-center bg-white dark:bg-transparent align-middle"></td>
      <td className="px-1 py-2 w-24 text-center bg-white dark:bg-transparent align-middle"></td>
    </tr>
  );
}; 