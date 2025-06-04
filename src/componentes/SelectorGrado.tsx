import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface SelectorGradoProps {
  value: string;
  onChange: (value: string) => void;
  className?: string;
}

interface Grado {
  id_grado: number;
  nombre_grado: string;
}

export function SelectorGrado({ value, onChange, className = '' }: SelectorGradoProps) {
  const [grados, setGrados] = useState<Grado[]>([]);
  const [cargando, setCargando] = useState(true);

  useEffect(() => {
    cargarGrados();
  }, []);

  const cargarGrados = async () => {
    try {
      setCargando(true);
      const resultado = await invoke<Grado[]>('obtener_grados');
      setGrados(resultado);
    } catch (error) {
      console.error('Error al cargar grados:', error);
    } finally {
      setCargando(false);
    }
  };

  return (
    <select
      value={value}
      onChange={(e) => onChange(e.target.value)}
      className={`w-full px-3 py-2 bg-white dark:bg-dark-600 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent ${className}`}
      disabled={cargando}
    >
      <option value="">Seleccione un grado</option>
      {grados.map((grado) => (
        <option key={grado.id_grado} value={grado.id_grado}>
          {grado.nombre_grado}
        </option>
      ))}
    </select>
  );
} 