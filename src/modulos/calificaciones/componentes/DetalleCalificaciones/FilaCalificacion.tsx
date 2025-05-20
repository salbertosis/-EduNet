import React from 'react';
import { CalificacionEstudiante } from '../../types';
import { Asignatura } from '../../types';

interface FilaCalificacionProps {
  asignatura: Asignatura;
  calificacion?: CalificacionEstudiante;
  errores: Record<string, string>;
  mostrarAjustes: boolean;
  onInputChange: (idAsignatura: number, campo: keyof CalificacionEstudiante, valor: string) => void;
  totalPendientes: number;
  asignaturas: Asignatura[];
  calificaciones: CalificacionEstudiante[];
  calcularNotaFinal: (c: CalificacionEstudiante) => number;
  calcularEstadoAsignatura: (c: CalificacionEstudiante, asignaturas: Asignatura[], calificaciones: CalificacionEstudiante[]) => string;
}

function padNota(nota: number | undefined): string {
  if (typeof nota === 'number') return nota.toString().padStart(2, '0');
  return '';
}

export const FilaCalificacion: React.FC<FilaCalificacionProps> = ({ 
  asignatura, 
  calificacion, 
  errores, 
  mostrarAjustes, 
  onInputChange,
  totalPendientes,
  asignaturas,
  calificaciones,
  calcularNotaFinal,
  calcularEstadoAsignatura
}) => {
  const c = calificacion || { id_asignatura: asignatura.id_asignatura, nombre_asignatura: asignatura.nombre_asignatura };
  const estado = calcularEstadoAsignatura(c, asignaturas, calificaciones);
  const notaFinal = calcularNotaFinal(c);
  const revisionHabilitada = notaFinal < 9.5;

  return (
    <tr>
      <td className="px-4 py-2 font-bold text-emerald-700 dark:text-cyan-200 whitespace-nowrap">
        <div className="flex items-center gap-3">
          <span className="flex items-center justify-center w-8 h-8 rounded-lg font-bold text-white text-base shadow" style={{ background: '#2563eb' }}>
            {asignatura.nombre_asignatura?.[0] ?? ''}
          </span>
          <span className="dark:text-cyan-200 text-emerald-700">{asignatura.nombre_asignatura}</span>
        </div>
      </td>
      {/* Lapso 1 */}
      <td className="px-2 py-2 align-middle text-center">
        <input
          type="number"
          min={0}
          max={20}
          value={padNota(c.lapso_1 as number)}
          onChange={e => onInputChange(asignatura.id_asignatura, 'lapso_1', e.target.value)}
          className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
        />
      </td>
      {mostrarAjustes && (
        <td className="px-2 py-2 align-middle text-center">
          <input
            type="number"
            min={0}
            max={20}
            value={padNota(c.lapso_1_ajustado as number)}
            onChange={e => onInputChange(asignatura.id_asignatura, 'lapso_1_ajustado', e.target.value)}
            className={`w-14 h-10 px-2 py-1 rounded-lg border-2 ${errores[`${asignatura.id_asignatura}_lapso_1_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
          />
          {errores[`${asignatura.id_asignatura}_lapso_1_ajustado`] && <div className="text-xs text-red-500">{errores[`${asignatura.id_asignatura}_lapso_1_ajustado`]}</div>}
        </td>
      )}
      {/* Lapso 2 */}
      <td className="px-2 py-2 align-middle text-center">
        <input
          type="number"
          min={0}
          max={20}
          value={padNota(c.lapso_2 as number)}
          onChange={e => onInputChange(asignatura.id_asignatura, 'lapso_2', e.target.value)}
          className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
        />
      </td>
      {mostrarAjustes && (
        <td className="px-2 py-2 align-middle text-center">
          <input
            type="number"
            min={0}
            max={20}
            value={padNota(c.lapso_2_ajustado as number)}
            onChange={e => onInputChange(asignatura.id_asignatura, 'lapso_2_ajustado', e.target.value)}
            className={`w-14 h-10 px-2 py-1 rounded-lg border-2 ${errores[`${asignatura.id_asignatura}_lapso_2_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
          />
          {errores[`${asignatura.id_asignatura}_lapso_2_ajustado`] && <div className="text-xs text-red-500">{errores[`${asignatura.id_asignatura}_lapso_2_ajustado`]}</div>}
        </td>
      )}
      {/* Lapso 3 */}
      <td className="px-2 py-2 align-middle text-center">
        <input
          type="number"
          min={0}
          max={20}
          value={padNota(c.lapso_3 as number)}
          onChange={e => onInputChange(asignatura.id_asignatura, 'lapso_3', e.target.value)}
          className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
        />
      </td>
      {mostrarAjustes && (
        <td className="px-2 py-2 align-middle text-center">
          <input
            type="number"
            min={0}
            max={20}
            value={padNota(c.lapso_3_ajustado as number)}
            onChange={e => onInputChange(asignatura.id_asignatura, 'lapso_3_ajustado', e.target.value)}
            className={`w-14 h-10 px-2 py-1 rounded-lg border-2 ${errores[`${asignatura.id_asignatura}_lapso_3_ajustado`] ? 'border-red-500' : 'border-cyan-400 dark:border-cyan-600'} bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal`}
          />
          {errores[`${asignatura.id_asignatura}_lapso_3_ajustado`] && <div className="text-xs text-red-500">{errores[`${asignatura.id_asignatura}_lapso_3_ajustado`]}</div>}
        </td>
      )}
      {/* Nota final */}
      <td className="px-1 py-2 w-20 align-middle text-center">
        <input
          type="text"
          value={Number.isFinite(notaFinal) ? notaFinal : ''}
          readOnly
          className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-emerald-50 dark:bg-[#232c3d] text-center text-emerald-700 dark:text-cyan-200 font-normal shadow text-sm"
        />
      </td>
      {/* Revisión */}
      <td className="px-1 py-2 w-20 align-middle text-center">
        <input
          type="text"
          value={c.revision ?? ''}
          onChange={e => onInputChange(asignatura.id_asignatura, 'revision', e.target.value)}
          className="w-14 h-10 px-2 py-1 rounded-lg border-2 border-emerald-400 dark:border-cyan-700 bg-white dark:bg-[#181f2a] text-center text-sm text-gray-900 dark:text-cyan-100 shadow focus:ring-2 focus:ring-cyan-400 focus:border-cyan-500 transition-all font-normal"
          disabled={!revisionHabilitada}
        />
      </td>
      {/* Estado */}
      <td className="px-1 py-2 w-24 align-middle text-center">
        {estado === 'Aprobado' && <span className="px-3 py-1 rounded-full text-xs font-bold bg-emerald-600/20 dark:bg-cyan-800/30 text-emerald-700 dark:text-cyan-200 border border-emerald-400 dark:border-cyan-400 shadow">Aprobado</span>}
        {estado === 'Pendiente' && <span className="px-3 py-1 rounded-full text-xs font-bold bg-yellow-600/20 dark:bg-yellow-800/30 text-yellow-700 dark:text-yellow-200 border border-yellow-400 dark:border-yellow-400 shadow">Pendiente</span>}
        {estado === 'Repite' && <span className="px-3 py-1 rounded-full text-xs font-bold bg-red-600/20 dark:bg-red-800/30 text-red-700 dark:text-red-200 border border-red-400 dark:border-red-400 shadow">Repite</span>}
        {estado === 'Revisión' && <span className="px-3 py-1 rounded-full text-xs font-bold bg-cyan-600/20 dark:bg-cyan-800/30 text-cyan-700 dark:text-cyan-200 border border-cyan-400 dark:border-cyan-400 shadow">Revisión</span>}
        {estado === 'Error' && <span className="px-3 py-1 rounded-full text-xs font-bold bg-pink-600/20 dark:bg-pink-800/30 text-pink-700 dark:text-pink-200 border border-pink-400 dark:border-pink-400 shadow">Error: Elimina la revisión</span>}
      </td>
    </tr>
  );
}; 