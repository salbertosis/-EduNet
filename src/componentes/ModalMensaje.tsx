import React, { useEffect } from "react";

export function ModalMensaje({
  mensaje,
  tipo,
  onClose,
}: {
  mensaje: string;
  tipo: "exito" | "error" | "info";
  onClose: () => void;
}) {
  const estilos = {
    exito: {
      bg: "bg-green-50 dark:bg-green-900/80",
      border: "border-green-400",
      text: "text-green-700 dark:text-green-200",
      icon: "✅",
      btn: "bg-green-600 hover:bg-green-700",
    },
    error: {
      bg: "bg-red-50 dark:bg-red-900/80",
      border: "border-red-400",
      text: "text-red-700 dark:text-red-200",
      icon: "❌",
      btn: "bg-red-600 hover:bg-red-700",
    },
    info: {
      bg: "bg-blue-50 dark:bg-blue-900/80",
      border: "border-blue-400",
      text: "text-blue-700 dark:text-blue-200",
      icon: "ℹ️",
      btn: "bg-blue-600 hover:bg-blue-700",
    },
  }[tipo];

  useEffect(() => {
    if (tipo !== "error") {
      const timer = setTimeout(() => {
        onClose();
      }, 4000);
      return () => clearTimeout(timer);
    }
  }, [tipo, onClose]);

  return (
    <div className="fixed inset-0 bg-black bg-opacity-40 flex items-center justify-center z-50">
      <div
        className={`p-6 rounded-2xl shadow-2xl max-w-md w-full border-2 ${estilos.bg} ${estilos.border} ${estilos.text} animate-fadeIn`}
      >
        <div className="flex items-center gap-3 mb-2">
          <span className="text-3xl">{estilos.icon}</span>
          <h2 className="text-xl font-bold capitalize">
            {tipo === "exito"
              ? "¡Éxito!"
              : tipo === "error"
              ? "Error"
              : "Información"}
          </h2>
        </div>
        <p className="mb-6 text-base">{mensaje}</p>
        <div className="flex justify-end">
          <button
            onClick={onClose}
            className={`px-5 py-2 rounded-lg text-white font-semibold shadow ${estilos.btn} transition-all`}
          >
            Cerrar
          </button>
        </div>
        <style>{`
          @keyframes fadeIn { from { opacity: 0; transform: translateY(-20px);} to { opacity: 1; transform: none; } }
          .animate-fadeIn { animation: fadeIn 0.2s ease; }
        `}</style>
      </div>
    </div>
  );
} 