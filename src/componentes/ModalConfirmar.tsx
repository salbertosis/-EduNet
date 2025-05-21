import React from "react";

export function ModalConfirmar({
  abierto,
  mensaje,
  onConfirmar,
  onCancelar,
  titulo = "¿Confirmar eliminación?",
  textoConfirmar = "Eliminar",
}: {
  abierto: boolean;
  mensaje: string;
  onConfirmar: () => void;
  onCancelar: () => void;
  titulo?: string;
  textoConfirmar?: string;
}) {
  if (!abierto) return null;
  // Detectar si es acción positiva (insertar, aceptar, confirmar, etc.)
  const esPositiva = textoConfirmar?.toLowerCase().includes('insertar') || textoConfirmar?.toLowerCase().includes('aceptar') || textoConfirmar?.toLowerCase().includes('confirmar');
  const colorBorde = esPositiva ? 'border-emerald-400' : 'border-red-400';
  const colorTitulo = esPositiva ? 'text-emerald-600' : 'text-red-600';
  const colorBoton = esPositiva ? 'bg-emerald-600 hover:bg-emerald-700' : 'bg-red-600 hover:bg-red-700';
  return (
    <div className="fixed inset-0 bg-black bg-opacity-40 flex items-center justify-center z-50">
      <div className={`p-6 rounded-xl shadow-lg bg-white dark:bg-dark-800 max-w-md w-full border ${colorBorde}`}>
        <h2 className={`text-lg font-bold mb-2 ${colorTitulo}`}>{titulo}</h2>
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
            className={`px-4 py-2 rounded-lg text-white ${colorBoton}`}
          >
            {textoConfirmar}
          </button>
        </div>
      </div>
    </div>
  );
} 