import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export interface Modalidad {
  id_modalidad: number;
  nombre_modalidad: string;
}

interface SelectorModalidadProps {
  value: number | null;
  onChange: (id_modalidad: number) => void;
  className?: string;
}

export function SelectorModalidad({ value, onChange, className }: SelectorModalidadProps) {
  const [modalidades, setModalidades] = useState<Modalidad[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    invoke<Modalidad[]>("obtener_modalidades")
      .then((data) => {
        setModalidades(data);
        setLoading(false);
        // Seleccionar por defecto la primera modalidad si no hay valor
        if (!value && data.length > 0) {
          onChange(data[0].id_modalidad);
        }
      })
      .catch(() => setLoading(false));
  }, []);

  return (
    <select
      className={className}
      value={value ?? ''}
      onChange={e => onChange(Number(e.target.value))}
      disabled={loading}
    >
      <option value="">Seleccione una modalidad</option>
      {modalidades.map(modalidad => (
        <option key={modalidad.id_modalidad} value={modalidad.id_modalidad}>
          {modalidad.nombre_modalidad}
        </option>
      ))}
    </select>
  );
} 