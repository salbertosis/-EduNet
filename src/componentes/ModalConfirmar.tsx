import React from "react";

export function ModalConfirmar({
  abierto,
  mensaje,
  onConfirmar,
  onCancelar,
}: {
  abierto: boolean;
  mensaje: string;
  onConfirmar: () => void;
  onCancelar: () => void;
}) {
  if (!abierto) return null;
  return (
    <div className="fixed inset-0 bg-black bg-opacity-40 flex items-center justify-center z-50">
      <div className="p-6 rounded-xl shadow-lg bg-white dark:bg-dark-800 max-w-md w-full border border-red-400">
        <h2 className="text-lg font-bold mb-2 text-red-600">¿Confirmar eliminación?</h2>
        <p className="mb-4">{mensaje}</p>
        <div className="flex justify-end gap-2">
          <button
            onClick={onCancelar}
            className="px-4 py-2 rounded-lg bg-gray-300 text-gray-800 hover:bg-gray-400"
          >
            Cancelar
          </button>
          <button
            onClick={onConfirmar}
            className="px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700"
          >
            Eliminar
          </button>
        </div>
      </div>
    </div>
  );
} 