import React from 'react';

interface PaginacionProps {
  paginaActual: number;
  totalPaginas: number;
  totalRegistros: number;
  registrosPorPagina: number;
  onCambiarPagina: (pagina: number) => void;
  onCambiarRegistrosPorPagina: (cantidad: number) => void;
  mostrarPrimeraUltima?: boolean;
  mostrarFlechas?: boolean;
  maximoBotones?: number;
}

export function Paginacion({
  paginaActual,
  totalPaginas,
  totalRegistros,
  registrosPorPagina,
  onCambiarPagina,
  onCambiarRegistrosPorPagina,
  mostrarPrimeraUltima = true,
  mostrarFlechas = true,
  maximoBotones = 5
}: PaginacionProps) {
  const generarNumerosPagina = () => {
    const numeros = [];
    const mitad = Math.floor(maximoBotones / 2);
    let inicio = Math.max(1, paginaActual - mitad);
    let fin = Math.min(totalPaginas, inicio + maximoBotones - 1);
    if (fin - inicio + 1 < maximoBotones) {
      inicio = Math.max(1, fin - maximoBotones + 1);
    }
    for (let i = inicio; i <= fin; i++) {
      numeros.push(i);
    }
    return numeros;
  };
  const botones = generarNumerosPagina();
  const desde = totalRegistros === 0 ? 0 : (paginaActual - 1) * registrosPorPagina + 1;
  const hasta = Math.min(paginaActual * registrosPorPagina, totalRegistros);

  return (
    <div className="flex flex-col md:flex-row md:items-center md:justify-between w-full gap-2">
      <div className="text-xs text-gray-600 dark:text-gray-300 mb-2 md:mb-0">
        Mostrando <b>{desde}</b> - <b>{hasta}</b> de <b>{totalRegistros}</b> registros
      </div>
      <div className="flex items-center gap-2">
        <label className="text-xs text-gray-600 dark:text-gray-300 mr-2" htmlFor="registrosPorPagina">Filas por página:</label>
        <select
          id="registrosPorPagina"
          value={registrosPorPagina}
          onChange={e => onCambiarRegistrosPorPagina(Number(e.target.value))}
          className="px-2 py-1 rounded border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-primary-500 focus:border-primary-500"
        >
          {[10, 25, 50, 100].map(num => (
            <option key={num} value={num}>{num}</option>
          ))}
        </select>
        {mostrarPrimeraUltima && (
          <button
            onClick={() => onCambiarPagina(1)}
            disabled={paginaActual === 1}
            className="px-3 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-dark-700 border border-gray-300 dark:border-dark-600 hover:bg-gray-50 dark:hover:bg-dark-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200"
            aria-label="Primera página"
          >
            ««
          </button>
        )}
        {mostrarFlechas && (
          <button
            onClick={() => onCambiarPagina(paginaActual - 1)}
            disabled={paginaActual === 1}
            className="px-3 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-dark-700 border border-gray-300 dark:border-dark-600 hover:bg-gray-50 dark:hover:bg-dark-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200"
            aria-label="Página anterior"
          >
            «
          </button>
        )}
        {botones.map((numero) => (
          <button
            key={numero}
            onClick={() => onCambiarPagina(numero)}
            className={`px-3 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
              numero === paginaActual
                ? 'bg-primary-500 text-white hover:bg-primary-600'
                : 'text-gray-700 dark:text-gray-300 bg-white dark:bg-dark-700 border border-gray-300 dark:border-dark-600 hover:bg-gray-50 dark:hover:bg-dark-600'
            }`}
            aria-current={numero === paginaActual ? 'page' : undefined}
          >
            {numero}
          </button>
        ))}
        {mostrarFlechas && (
          <button
            onClick={() => onCambiarPagina(paginaActual + 1)}
            disabled={paginaActual === totalPaginas}
            className="px-3 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-dark-700 border border-gray-300 dark:border-dark-600 hover:bg-gray-50 dark:hover:bg-dark-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200"
            aria-label="Página siguiente"
          >
            »
          </button>
        )}
        {mostrarPrimeraUltima && (
          <button
            onClick={() => onCambiarPagina(totalPaginas)}
            disabled={paginaActual === totalPaginas}
            className="px-3 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-dark-700 border border-gray-300 dark:border-dark-600 hover:bg-gray-50 dark:hover:bg-dark-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200"
            aria-label="Última página"
          >
            »»
          </button>
        )}
      </div>
    </div>
  );
} 