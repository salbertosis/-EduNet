import { ReactNode } from 'react';

interface TarjetaProps {
  titulo: string;
  icono: ReactNode;
  valor: string | number | ReactNode;
  descripcion?: string | ReactNode;
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
    <div
      className={
        `group bg-white dark:bg-dark-800 rounded-2xl shadow-soft p-7 transition-all duration-200 hover:shadow-xl hover:-translate-y-1 flex flex-col justify-between min-h-[180px]`
      }
    >
      <div className="flex items-start justify-between mb-2">
        <div>
          <p className="text-base font-semibold text-gray-700 dark:text-gray-200 tracking-tight mb-1">{titulo}</p>
          <p className="text-4xl font-extrabold text-gray-900 dark:text-white leading-tight mb-2">{valor}</p>
        </div>
        <div className={`flex items-center justify-center w-14 h-14 rounded-full shadow-inner ${colores[color]} transition-all duration-200 group-hover:scale-105`}>
          {icono}
        </div>
      </div>
      {descripcion && (
        typeof descripcion === 'string' ? (
          <p className="text-sm text-gray-500 dark:text-gray-400 mt-2">{descripcion}</p>
        ) : (
          <div className="mt-2">{descripcion}</div>
        )
      )}
    </div>
  );
}