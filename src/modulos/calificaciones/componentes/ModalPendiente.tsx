import React, { useState, useEffect } from 'react';
import { usePendientes, CalificacionesPendiente } from '../hooks/usePendientes';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

export interface Pendiente {
  id_pendiente: number;
  nombre_asignatura: string;
  periodo_escolar: string;
}

interface ModalPendienteProps {
  open: boolean;
  onClose: () => void;
  pendiente: Pendiente | null;
  onGuardar: (idPendiente: number, valores: { momento1: number, momento2: number, momento3: number, momento4: number }) => void;
  onEliminar?: (idPendiente: number) => void;
  estudianteId: number;
}

export const ModalPendiente: React.FC<ModalPendienteProps> = ({ open, onClose, pendiente, onGuardar, onEliminar, estudianteId }) => {
  const [momentos, setMomentos] = useState({ momento1: 0, momento2: 0, momento3: 0, momento4: 0 });
  const { obtenerCalificacionesPendiente, upsertCalificacionesPendiente } = usePendientes(estudianteId);
  const { mostrarMensaje } = useMensajeGlobal();
  const [feedback, setFeedback] = useState<string | null>(null);
  const [loadingCalificacion, setLoadingCalificacion] = useState(false);
  const [loadingGuardar, setLoadingGuardar] = useState(false);
  const [focusInput, setFocusInput] = useState<number | null>(null);

  useEffect(() => {
    if (pendiente && open) {
      setLoadingCalificacion(true);
      console.log('Solicitando calificaciones para:', pendiente.id_pendiente);
      obtenerCalificacionesPendiente(pendiente.id_pendiente).then(data => {
        console.log('Respuesta de calificaciones:', data);
        if (data) {
          setMomentos({
            momento1: data.cal_momento1 ?? 0,
            momento2: data.cal_momento2 ?? 0,
            momento3: data.cal_momento3 ?? 0,
            momento4: data.cal_momento4 ?? 0,
          });
        } else {
          setMomentos({ momento1: 0, momento2: 0, momento3: 0, momento4: 0 });
        }
      }).finally(() => setLoadingCalificacion(false));
    }
    if (!open) setFeedback(null);
  }, [pendiente, open]);

  const calcularEstado = () => {
    if ([momentos.momento1, momentos.momento2, momentos.momento3, momentos.momento4].some(m => m >= 10)) return 'Aprobado';
    if ([momentos.momento1, momentos.momento2, momentos.momento3, momentos.momento4].every(m => m < 10)) return 'Repite';
    return 'Pendiente';
  };

  const handleGuardar = async () => {
    if (!pendiente) return;
    setFeedback(null);
    setLoadingGuardar(true);
    const input: CalificacionesPendiente = {
      id_pendiente: pendiente.id_pendiente,
      cal_momento1: momentos.momento1 ?? 0,
      cal_momento2: momentos.momento2 ?? 0,
      cal_momento3: momentos.momento3 ?? 0,
      cal_momento4: momentos.momento4 ?? 0,
    };
    const ok = await upsertCalificacionesPendiente(input);
    setLoadingGuardar(false);
    if (ok) {
      mostrarMensaje('Calificaciones guardadas correctamente', 'exito');
      onGuardar(pendiente.id_pendiente, momentos);
      onClose();
    } else {
      setFeedback('Error al guardar calificaciones');
    }
  };

  const handleEliminar = () => {
    if (pendiente && typeof onEliminar === 'function') {
      if (window.confirm('¿Estás seguro de que deseas eliminar esta asignatura pendiente?')) {
        onEliminar(pendiente.id_pendiente);
      }
    }
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
        {loadingCalificacion ? (
          <div className="text-center text-gray-500">Cargando calificaciones...</div>
        ) : (
          <>
            <div className="flex gap-2 mb-4">
              {[1,2,3,4].map(i => {
                const value = momentos[`momento${i}` as keyof typeof momentos] ?? 0;
                const isFocused = focusInput === i;
                const displayValue = (!isFocused && value === 0) ? '' : value < 10 ? `0${value}` : String(value);
                let disabled = false;
                if (i === 1) {
                  disabled = false;
                } else if (i === 2) {
                  disabled = !(momentos.momento1 < 10 && momentos.momento1 !== 0 && momentos.momento1 !== undefined);
                } else if (i === 3) {
                  disabled = !(momentos.momento2 < 10 && momentos.momento2 !== 0 && momentos.momento2 !== undefined);
                } else if (i === 4) {
                  disabled = !(momentos.momento3 < 10 && momentos.momento3 !== 0 && momentos.momento3 !== undefined);
                }
                if ((i > 1 && momentos[`momento${i-1}` as keyof typeof momentos] >= 10) || [momentos.momento1, momentos.momento2, momentos.momento3, momentos.momento4].some((m, idx) => idx < i-1 && m >= 10)) {
                  disabled = true;
                }
                return (
                  <div key={i} className="flex flex-col items-center">
                    <label className="text-xs font-semibold mb-1">Momento {i}</label>
                    <input
                      type="number"
                      min={0}
                      max={20}
                      className="w-14 px-1 py-1 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-emerald-500 dark:focus:ring-cyan-500 focus:border-transparent dark:bg-gray-700 dark:text-white text-center text-sm"
                      value={displayValue}
                      onFocus={() => setFocusInput(i)}
                      onBlur={() => setFocusInput(null)}
                      onChange={e => {
                        let val = e.target.value;
                        if (val.length > 1 && val.startsWith('0')) val = val.replace(/^0+/, '');
                        let num = Number(val);
                        if (isNaN(num)) num = 0;
                        num = Math.max(0, Math.min(20, num));
                        setMomentos(m => ({ ...m, [`momento${i}`]: num }));
                      }}
                      disabled={disabled}
                    />
                  </div>
                );
              })}
            </div>
            <div className="flex items-center gap-2 mb-4">
              <span className="font-bold text-base">Estado:</span>
              {(() => {
                const estado = calcularEstado();
                if (estado === 'Aprobado') return <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">Aprobado</span>;
                if (estado === 'Repite') return <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-yellow-100 text-yellow-800">Repite</span>;
                return <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800">Pendiente</span>;
              })()}
            </div>
            {feedback && <div className={`text-sm font-semibold text-red-500`}>{feedback}</div>}
            <div className="flex justify-end gap-4 mt-2">
              <button 
                className="px-6 py-2 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-bold shadow-lg hover:scale-105 transition-all" 
                onClick={handleGuardar}
                disabled={loadingGuardar}
              >
                {loadingGuardar ? 'Guardando...' : 'Guardar'}
              </button>
              <button 
                className="px-6 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-200 rounded-xl font-bold shadow hover:bg-gray-300 dark:hover:bg-gray-600 transition-all" 
                onClick={onClose}
                disabled={loadingGuardar}
              >
                Cerrar
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
}; 