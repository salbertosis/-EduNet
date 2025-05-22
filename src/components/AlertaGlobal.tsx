import React from 'react';
import { useAlertas } from '../hooks/useAlertas';

export const AlertaGlobal: React.FC = () => {
  const { alertas } = useAlertas();

  return (
    <div className="fixed top-4 right-4 z-50 flex flex-col gap-2">
      {alertas.map(alerta => (
        <div
          key={alerta.id}
          className={`px-4 py-2 rounded-lg shadow-lg text-white transform transition-all duration-300 ${
            alerta.tipo === 'success' ? 'bg-emerald-500' :
            alerta.tipo === 'error' ? 'bg-red-500' :
            alerta.tipo === 'warning' ? 'bg-yellow-500' :
            'bg-blue-500'
          }`}
        >
          {alerta.mensaje}
        </div>
      ))}
    </div>
  );
}; 