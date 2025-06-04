import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface SelectorLapsoProps {
  value: string;
  onChange: (value: string) => void;
  className?: string;
}

export function SelectorLapso({ value, onChange, className = '' }: SelectorLapsoProps) {
  const [lapsos, setLapsos] = useState<string[]>([]);
  const [cargando, setCargando] = useState(true);

  useEffect(() => {
    cargarLapsos();
  }, []);

  const cargarLapsos = async () => {
    try {
      setCargando(true);
      const resultado = await invoke<string[]>('obtener_lapsos');
      setLapsos(resultado);
    } catch (error) {
      console.error('Error al cargar lapsos:', error);
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
      <option value="">Seleccione un lapso</option>
      {lapsos.map((lapso) => (
        <option key={lapso} value={lapso}>
          {lapso}° Lapso
        </option>
      ))}
    </select>
  );
} 