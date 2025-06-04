import { ReactNode } from 'react';

interface TarjetaProps {
  titulo: string;
  icono: ReactNode;
  valor: string | number | ReactNode;
  descripcion?: string;
  color?: 'primary' | 'success' | 'warning' | 'danger';
}

const colores = {
  primary: 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-100',
  success: 'bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-100',
  warning: 'bg-yellow-100 dark:bg-yellow-900 text-yellow-700 dark:text-yellow-100',
  danger: 'bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-100',
};

export function Tarjeta({ titulo, icono, valor, descripcion, color = 'primary' }: TarjetaProps) {
  return (
    <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm font-medium text-gray-600 dark:text-gray-400">{titulo}</p>
          <p className="text-2xl font-semibold mt-2">{valor}</p>
          {descripcion && (
            <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">{descripcion}</p>
          )}
        </div>
        <div className={`p-3 rounded-lg ${colores[color]}`}>
          {icono}
        </div>
      </div>
    </div>
  );
} 