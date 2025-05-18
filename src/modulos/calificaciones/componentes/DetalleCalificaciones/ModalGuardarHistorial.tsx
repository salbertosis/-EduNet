import React from 'react';

interface ModalGuardarHistorialProps {
  abierto: boolean;
  onCancelar: () => void;
  onConfirmar: () => void;
}

export const ModalGuardarHistorial: React.FC<ModalGuardarHistorialProps> = ({ abierto, onCancelar, onConfirmar }) => {
  if (!abierto) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-40">
      <div className="bg-white dark:bg-dark-800 p-8 rounded-xl shadow-lg max-w-md w-full border border-blue-300">
        <h2 className="text-xl font-bold mb-4 text-blue-700 dark:text-cyan-300">Guardar Historial</h2>
        <p className="mb-6 text-gray-700 dark:text-gray-200">
          ¿Está seguro que desea guardar el historial académico de este estudiante para el periodo actual?
        </p>
        <div className="flex justify-end gap-4">
          <button
            className="px-5 py-2 rounded-lg bg-gray-200 dark:bg-dark-700 text-gray-700 dark:text-gray-200 hover:bg-gray-300 dark:hover:bg-dark-600 font-medium"
            onClick={onCancelar}
          >
            Cancelar
          </button>
          <button
            className="px-5 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 font-semibold shadow"
            onClick={onConfirmar}
          >
            Guardar
          </button>
        </div>
      </div>
    </div>
  );
}; 