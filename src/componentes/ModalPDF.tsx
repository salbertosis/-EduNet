import React from "react";

export const ModalPDF = ({
  open,
  onClose,
  pdfUrl,
  loading,
  error,
}: {
  open: boolean;
  onClose: () => void;
  pdfUrl: string | null;
  loading?: boolean;
  error?: string | null;
}) => {
  if (!open) return null;
  return (
    <div className="fixed inset-0 bg-black/40 flex items-center justify-center z-50">
      <div className="bg-white rounded-xl shadow-xl max-w-4xl w-full relative p-0">
        {/* Botón de cerrar en la esquina superior derecha */}
        <button
          onClick={onClose}
          className="absolute top-16 right-4 p-2 rounded-lg hover:bg-gray-200 text-gray-600 font-bold text-xl z-20"
          aria-label="Cerrar modal"
        >
          ×
        </button>
        {loading && <div className="p-8 text-center text-lg">Generando PDF...</div>}
        {error && <div className="p-8 text-center text-red-600">{error}</div>}
        {pdfUrl && !loading && !error && (
          <iframe
            src={pdfUrl}
            width="100%"
            height="700px"
            className="rounded-xl border-2 border-gray-200"
            title="PDF Estudiantes"
          />
        )}
      </div>
    </div>
  );
}; 