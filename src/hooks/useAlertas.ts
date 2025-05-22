import { useState } from 'react';

type TipoAlerta = 'success' | 'error' | 'warning' | 'info';

interface Alerta {
  mensaje: string;
  tipo: TipoAlerta;
  id: number;
}

export const useAlertas = () => {
  const [alertas, setAlertas] = useState<Alerta[]>([]);

  const mostrarAlerta = (mensaje: string, tipo: TipoAlerta) => {
    const id = Date.now();
    setAlertas(prev => [...prev, { mensaje, tipo, id }]);
    setTimeout(() => {
      setAlertas(prev => prev.filter(alerta => alerta.id !== id));
    }, 5000);
  };

  return { alertas, mostrarAlerta };
}; 