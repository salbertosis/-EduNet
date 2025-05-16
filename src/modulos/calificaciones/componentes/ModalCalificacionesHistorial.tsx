import React from 'react';
import { DetalleCalificacionesHistorial } from './DetalleCalificacionesHistorial';

interface ModalCalificacionesHistorialProps {
  open: boolean;
  onClose: () => void;
  idEstudiante: number;
  idPeriodo: number;
  periodoEscolar: string;
}

export function ModalCalificacionesHistorial({ open, onClose, idEstudiante, idPeriodo, periodoEscolar }: ModalCalificacionesHistorialProps) {
  if (!open) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-40 transition-opacity duration-200">
      <div className="relative w-full max-w-4xl mx-4 my-12 bg-white dark:bg-dark-800 rounded-2xl shadow-2xl border border-blue-300 flex flex-col animate-fadeIn" style={{ maxHeight: '90vh' }}>
        <button className="absolute top-4 right-4 text-2xl text-gray-500 hover:text-blue-600 font-bold z-10" onClick={onClose} aria-label="Cerrar modal">
          &times;
        </button>
        <h2 className="text-2xl font-bold mb-4 text-blue-700 dark:text-cyan-300 px-8 pt-8 sticky top-0 bg-white dark:bg-dark-800 z-10 rounded-t-2xl">
          Calificaciones {periodoEscolar}
        </h2>
        <div className="overflow-y-auto px-6 pb-6" style={{ maxHeight: '70vh', minHeight: '300px' }}>
          <DetalleCalificacionesHistorial idEstudiante={idEstudiante} idPeriodo={idPeriodo} />
        </div>
        <div className="flex justify-end gap-4 px-8 pb-6 pt-2 bg-white dark:bg-dark-800 rounded-b-2xl sticky bottom-0 z-20">
          <button onClick={onClose} className="px-8 py-2 bg-gradient-to-r from-gray-400 to-gray-600 text-white rounded-xl font-bold shadow-lg hover:scale-105 hover:bg-gray-700 transition-all">
            Cerrar
          </button>
        </div>
      </div>
      <style>{`
        @keyframes fadeIn { from { opacity: 0; transform: translateY(-30px);} to { opacity: 1; transform: none; } }
        .animate-fadeIn { animation: fadeIn 0.25s ease; }
      `}</style>
    </div>
  );
} 