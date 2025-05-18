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

interface FilaCalificacionProps {
  asignatura: Asignatura;
  calificacion?: CalificacionEstudiante;
  errores: Record<string, string>;
  mostrarAjustes: boolean;
  onInputChange: (id_asignatura: number, campo: keyof CalificacionEstudiante, valor: string) => void;
  totalPendientesRevisionActivos: number;
}

function padNota(nota: number | undefined): string {
  if (typeof nota === 'number') return nota.toString().padStart(2, '0');
  return '';
}

function calcularEstadoAsignatura(calif: CalificacionEstudiante, totalPendientesRevisionActivos: number): string {
  const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : calcularNotaFinal(calif);
  const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
  const revisionHabilitada = notaFinal < 9.5;
  if (revisionHabilitada) {
    if (revision !== undefined && !isNaN(revision)) {
      if (revision >= 10) return 'Aprobado';
      if (revision < 10) {
        if (totalPendientesRevisionActivos >= 3) return 'Repite';
        return 'Pendiente';
      }
    }
    return 'Revisión';
  }
  // Si el input de revisión está deshabilitado, solo depende de la nota final
  if (notaFinal >= 9.5) return 'Aprobado';
  return 'Revisión';
}

function calcularNotaFinal(calif: CalificacionEstudiante): number {
  const l1 = calif.lapso_1_ajustado ?? calif.lapso_1 ?? 0;
  const l2 = calif.lapso_2_ajustado ?? calif.lapso_2 ?? 0;
  const l3 = calif.lapso_3_ajustado ?? calif.lapso_3 ?? 0;
  if (l1 && l2 && l3) {
    return Math.round((l1 + l2 + l3) / 3);
  }
  return 0;
}

export const FilaCalificacion: React.FC<FilaCalificacionProps> = ({ asignatura, calificacion, errores, mostrarAjustes, onInputChange, totalPendientesRevisionActivos }) => {
  const c = calificacion || { id_asignatura: asignatura.id_asignatura, nombre_asignatura: asignatura.nombre_asignatura };
  const estado = calcularEstadoAsignatura(c, totalPendientesRevisionActivos);
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
      </td>
    </tr>
  );
}; 