import React from "react";

export function ModalMensaje({
  mensaje,
  tipo,
  onClose,
}: {
  mensaje: string;
  tipo: "exito" | "error" | "info";
  onClose: () => void;
}) {
  const colores = {
    exito: "bg-green-100 border-green-400 text-green-700",
    error: "bg-red-100 border-red-400 text-red-700",
    info: "bg-blue-100 border-blue-400 text-blue-700",
  };
  return (
    <div className="fixed inset-0 bg-black bg-opacity-40 flex items-center justify-center z-50">
      <div className={`p-6 rounded-xl shadow-lg bg-white dark:bg-dark-800 max-w-md w-full border ${colores[tipo]}`}>
        <h2 className={`text-lg font-bold mb-2 capitalize`}>
          {tipo === "exito" ? "¡Éxito!" : tipo === "error" ? "Error" : "Información"}
        </h2>
        <p className="mb-4">{mensaje}</p>
        <button
          onClick={onClose}
          className={`px-4 py-2 rounded-lg text-white ${
            tipo === "exito"
              ? "bg-green-600"
              : tipo === "error"
              ? "bg-red-600"
              : "bg-blue-600"
          } hover:opacity-80`}
        >
          Cerrar
        </button>
      </div>
    </div>
  );
} 