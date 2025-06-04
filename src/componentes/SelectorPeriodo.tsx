import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export interface PeriodoEscolar {
  id_periodo: number;
  periodo_escolar: string;
  activo: boolean;
}

interface SelectorPeriodoProps {
  value: number | null;
  onChange: (id_periodo: number) => void;
  className?: string;
}

export function SelectorPeriodo({ value, onChange, className }: SelectorPeriodoProps) {
  const [periodos, setPeriodos] = useState<PeriodoEscolar[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    invoke<PeriodoEscolar[]>("obtener_periodos_escolares")
      .then((data) => {
        setPeriodos(data);
        setLoading(false);
        // Seleccionar por defecto el periodo activo si no hay valor
        if (!value) {
          const activo = data.find(p => p.activo);
          if (activo) onChange(activo.id_periodo);
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
      <option value="">Seleccione un año escolar</option>
      {periodos.map(periodo => (
        <option key={periodo.id_periodo} value={periodo.id_periodo}>
          {periodo.periodo_escolar} {periodo.activo ? '(Activo)' : ''}
        </option>
      ))}
    </select>
  );
} 