import { useParams, useNavigate } from 'react-router-dom';
import { useEffect, useState } from 'react';
import { DetalleCalificaciones } from './DetalleCalificaciones';
import { invoke } from '@tauri-apps/api/tauri';
import { CargaMasivaCalificaciones } from '../componentes/CargaMasivaCalificaciones';

export function DetalleCalificacionesPage() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [estudiante, setEstudiante] = useState<any>(null);

  useEffect(() => {
    console.log('[DEBUG] id recibido en DetalleCalificacionesPage:', id);
    if (id) {
      invoke('obtener_estudiante_por_id', { id: Number(id) })
        .then((data: any) => {
          console.log('[DEBUG] Datos recibidos del backend:', data);
          setEstudiante(data);
        })
        .catch((err) => {
          console.error('[ERROR] Error al obtener estudiante:', err);
          setEstudiante(null);
        });
    }
  }, [id]);

  if (!estudiante) return <div className="text-center py-12 text-gray-500 dark:text-gray-400">Cargando datos del estudiante...</div>;

  return (
    <div>
      <CargaMasivaCalificaciones />
      <DetalleCalificaciones estudiante={estudiante} onVolver={() => navigate(-1)} />
    </div>
  );
} 