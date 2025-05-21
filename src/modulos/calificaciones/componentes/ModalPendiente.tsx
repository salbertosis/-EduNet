import React, { useState, useEffect } from 'react';

interface ModalPendienteProps {
  open: boolean;
  onClose: () => void;
  pendiente: any;
  onGuardar: (valores: { momento1: number, momento2: number, momento3: number, momento4: number }) => void;
}

export const ModalPendiente: React.FC<ModalPendienteProps> = ({ open, onClose, pendiente, onGuardar }) => {
  const [momentos, setMomentos] = useState({ momento1: 0, momento2: 0, momento3: 0, momento4: 0 });
  useEffect(() => {
    if (pendiente) {
      setMomentos({
        momento1: pendiente.cal_momento1 ?? 0,
        momento2: pendiente.cal_momento2 ?? 0,
        momento3: pendiente.cal_momento3 ?? 0,
        momento4: pendiente.cal_momento4 ?? 0,
      });
    }
  }, [pendiente]);

  const calcularEstado = () => {
    if ([momentos.momento1, momentos.momento2, momentos.momento3, momentos.momento4].some(m => m >= 10)) return 'Aprobado';
    if (momentos.momento4 < 10) return 'Repite';
    if ([momentos.momento1, momentos.momento2, momentos.momento3].every(m => m < 10)) return 'Pendiente';
    return 'Pendiente';
  };

  if (!open || !pendiente) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-40">
      <div className="w-full max-w-lg bg-white dark:bg-[#1e293b] rounded-2xl shadow-2xl p-8 flex flex-col gap-4" style={{ maxHeight: '80vh', overflowY: 'auto' }}>
        <h2 className="text-2xl font-bold text-emerald-700 mb-2">Asignatura Pendiente</h2>
        <div className="mb-2">
          <div className="font-semibold text-lg text-cyan-700 dark:text-cyan-200">{pendiente.nombre_asignatura}</div>
          <div className="text-sm text-gray-500">Año escolar: {pendiente.periodo_escolar}</div>
        </div>
        <div className="grid grid-cols-2 gap-4 mb-4">
          {[1,2,3,4].map(i => (
            <div key={i} className="flex flex-col">
              <label className="text-xs font-semibold mb-1">Momento {i}</label>
              <input
                type="number"
                min={0}
                max={20}
                className="rounded-lg border px-3 py-2 text-base focus:outline-none focus:ring-2 focus:ring-emerald-400"
                value={momentos[`momento${i}` as keyof typeof momentos]}
                onChange={e => setMomentos(m => ({ ...m, [`momento${i}`]: Math.max(0, Math.min(20, Number(e.target.value))) }))}
              />
            </div>
          ))}
        </div>
        <div className="flex items-center gap-2 mb-4">
          <span className="font-bold text-base">Estado:</span>
          {(() => {
            const estado = calcularEstado();
            if (estado === 'Aprobado') return <span className="px-3 py-1 rounded-full text-xs font-bold bg-emerald-100 text-emerald-700 border border-emerald-400 shadow">Aprobado</span>;
            if (estado === 'Repite') return <span className="px-3 py-1 rounded-full text-xs font-bold bg-yellow-100 text-yellow-700 border border-yellow-400 shadow">Repite</span>;
            return <span className="px-3 py-1 rounded-full text-xs font-bold bg-cyan-100 text-cyan-700 border border-cyan-400 shadow">Pendiente</span>;
          })()}
        </div>
        <div className="flex justify-end gap-4 mt-2">
          <button className="px-6 py-2 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-bold shadow-lg hover:scale-105 transition-all" onClick={() => onGuardar(momentos)}>
            Guardar
          </button>
          <button className="px-6 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-200 rounded-xl font-bold shadow hover:bg-gray-300 dark:hover:bg-gray-600 transition-all" onClick={onClose}>
            Cerrar
          </button>
        </div>
      </div>
    </div>
  );
}; 