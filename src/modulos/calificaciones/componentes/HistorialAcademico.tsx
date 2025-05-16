import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { EyeIcon } from '@heroicons/react/24/outline';
import { ModalCalificacionesHistorial } from './ModalCalificacionesHistorial';

interface HistorialAcademico {
  id_historial: number;
  id_estudiante: number;
  id_periodo: number;
  id_grado_secciones: number;
  promedio_anual: number;
  estatus: string;
  fecha_registro: string;
  periodo_escolar?: string;
  grado?: string;
  seccion?: string;
}

interface HistorialAcademicoProps {
  idEstudiante: number;
}

export function HistorialAcademico({ idEstudiante }: HistorialAcademicoProps) {
  const [historial, setHistorial] = useState<HistorialAcademico[]>([]);
  const [cargando, setCargando] = useState(false);
  const { mostrarMensaje } = useMensajeGlobal();
  const [modalAbierto, setModalAbierto] = useState(false);
  const [periodoSeleccionado, setPeriodoSeleccionado] = useState<HistorialAcademico | null>(null);

  useEffect(() => {
    if (!idEstudiante || isNaN(Number(idEstudiante))) {
      console.warn('[WARN][FRONTEND] idEstudiante no es válido:', idEstudiante);
      return;
    }
    setCargando(true);
    console.log('[DEBUG][FRONTEND] Solicitando historial académico para idEstudiante:', idEstudiante);
    invoke('obtener_historial_academico_estudiante', { idEstudiante })
      .then((data) => {
        console.log('[DEBUG][FRONTEND] Historial académico recibido:', data);
        setHistorial(data as HistorialAcademico[]);
      })
      .catch((err) => {
        console.error('[ERROR][FRONTEND] Error al cargar historial académico:', err);
        setHistorial([]);
        mostrarMensaje('Error al cargar el historial académico', 'error');
      })
      .finally(() => {
        setCargando(false);
        console.log('[DEBUG][FRONTEND] Finalizó carga de historial académico');
      });
  }, [idEstudiante]);

  return (
    <div>
      <h3 className="text-xl font-semibold mb-4">Historial Académico</h3>
      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200 dark:divide-dark-700">
          <thead className="bg-gray-800 text-white">
            <tr>
              <th className="px-4 py-2">Año Escolar</th>
              <th className="px-4 py-2">Grado</th>
              <th className="px-4 py-2">Sección</th>
              <th className="px-4 py-2">Promedio</th>
              <th className="px-4 py-2">Estado</th>
              <th className="px-4 py-2">Acciones</th>
            </tr>
          </thead>
          <tbody>
            {historial.length === 0 ? (
              <tr><td colSpan={5} className="text-center py-4 text-gray-400">No hay historial registrado.</td></tr>
            ) : (
              historial.map((h) => (
                <tr key={h.id_historial}>
                  <td className="px-4 py-2 text-center">{h.periodo_escolar}</td>
                  <td className="px-4 py-2 text-center">{h.grado}</td>
                  <td className="px-4 py-2 text-center">{h.seccion}</td>
                  <td className="px-4 py-2 text-center">{h.promedio_anual.toFixed(2)}</td>
                  <td className="px-4 py-2 text-center">
                    <span className="px-3 py-1 rounded-full text-xs font-bold bg-green-200 text-green-800">
                      {h.estatus}
                    </span>
                  </td>
                  <td className="px-4 py-2 text-center">
                    <button onClick={() => { setPeriodoSeleccionado(h); setModalAbierto(true); }} className="p-2 rounded hover:bg-emerald-100">
                      <EyeIcon className="w-5 h-5 text-emerald-600" />
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
      {modalAbierto && periodoSeleccionado && (
        <ModalCalificacionesHistorial
          open={modalAbierto}
          onClose={() => setModalAbierto(false)}
          idEstudiante={idEstudiante}
          idPeriodo={periodoSeleccionado?.id_periodo ?? 0}
          periodoEscolar={periodoSeleccionado?.periodo_escolar || ''}
        />
      )}
    </div>
  );
} 