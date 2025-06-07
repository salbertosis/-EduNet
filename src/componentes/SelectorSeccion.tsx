import { Seccion } from '../modulos/reportes/types';

interface SelectorSeccionProps {
  value: string;
  onChange: (value: string) => void;
  className?: string;
  secciones: Seccion[];
  grado: string;
  modalidad: string;
  periodo: string;
}

export function SelectorSeccion({ value, onChange, className = '', secciones, grado, modalidad, periodo }: SelectorSeccionProps) {
  const cargando = !grado || !modalidad || !periodo;

  return (
    <select
      value={value}
      onChange={(e) => onChange(e.target.value === 'todas' ? 'todas' : e.target.value)}
      className={`w-full px-3 py-2 bg-white dark:bg-dark-600 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent ${className}`}
      disabled={cargando}
    >
      <option value="">Seleccione una sección</option>
      <option value="todas">Todas las secciones</option>
      {(secciones ?? []).map((seccion) => (
        <option key={seccion.id_seccion} value={seccion.id_seccion}>
          {seccion.nombre_seccion}
        </option>
      ))}
    </select>
  );
} 