import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface SelectorAsignaturaProps {
  value: string;
  onChange: (value: string) => void;
  className?: string;
  grado: string;
  modalidad: string;
}

interface Asignatura {
  id_asignatura: number;
  nombre_asignatura: string;
  id_grado: number;
  id_modalidad: number;
}

export function SelectorAsignatura({ value, onChange, className = '', grado, modalidad }: SelectorAsignaturaProps) {
  const [asignaturas, setAsignaturas] = useState<Asignatura[]>([]);
  const [cargando, setCargando] = useState(false);

  useEffect(() => {
    if (grado && modalidad) {
      console.log('[SelectorAsignatura] grado recibido:', grado, 'modalidad recibida:', modalidad);
      cargarAsignaturas();
    } else {
      setAsignaturas([]);
    }
  }, [grado, modalidad]);

  const cargarAsignaturas = async () => {
    try {
      setCargando(true);
      const id_grado = parseInt(grado, 10);
      const id_modalidad = parseInt(modalidad, 10);
      console.log('[SelectorAsignatura] id_grado convertido:', id_grado, 'id_modalidad convertido:', id_modalidad);
      if (!isNaN(id_grado) && !isNaN(id_modalidad)) {
        const resultado = await invoke<Asignatura[]>('obtener_asignaturas', { idGrado: id_grado, idModalidad: id_modalidad });
        console.log('[SelectorAsignatura] resultado backend:', resultado);
        setAsignaturas(resultado);
      } else {
        console.warn('[SelectorAsignatura] id_grado o id_modalidad no son números válidos:', grado, modalidad);
        setAsignaturas([]);
      }
    } catch (error) {
      console.error('[SelectorAsignatura] Error al cargar asignaturas:', error);
    } finally {
      setCargando(false);
    }
  };

  return (
    <select
      value={value}
      onChange={(e) => onChange(e.target.value === 'todas' ? 'todas' : e.target.value)}
      className={`w-full px-3 py-2 bg-white dark:bg-dark-600 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent ${className}`}
      disabled={cargando || !grado || !modalidad}
    >
      <option value="">Seleccione una asignatura</option>
      <option value="todas">Todas las asignaturas</option>
      {asignaturas.map((asignatura) => (
        <option key={asignatura.id_asignatura} value={asignatura.id_asignatura}>
          {asignatura.nombre_asignatura}
        </option>
      ))}
    </select>
  );
} 