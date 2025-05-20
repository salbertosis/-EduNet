import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { HistorialAcademico } from '../componentes/HistorialAcademico';
import { AsignaturasPendientes } from '../componentes/AsignaturasPendientes';
import { useCalificaciones } from '../hooks/useCalificaciones';
import { useAsignaturas } from '../hooks/useAsignaturas';
import { useInputCalificaciones } from '../hooks/useInputCalificaciones';
import { useGuardarHistorial } from '../hooks/useGuardarHistorial';
import { useGuardarCalificaciones } from '../hooks/useGuardarCalificaciones';
import { DatosPersonalesEstudiante } from '../componentes/DetalleCalificaciones/DatosPersonalesEstudiante';
import { TabsCalificaciones } from '../componentes/DetalleCalificaciones/TabsCalificaciones';
import { CalificacionesActuales } from '../componentes/DetalleCalificaciones/CalificacionesActuales';
import { usePendientes } from '../hooks/usePendientes';

const TABS = [
  { key: 'datos', label: 'Datos Personales' },
  { key: 'actual', label: 'Calificaciones Año Actual' },
  { key: 'historial', label: 'Historial Académico' },
  { key: 'pendientes', label: 'Asignaturas Pendientes' },
];

interface EstudianteDetalle {
  id: number;
  cedula: string | undefined;
  apellidos: string | undefined;
  nombres: string | undefined;
  nombre_grado: string | undefined;
  nombre_seccion: string | undefined;
  nombre_modalidad: string | undefined;
  id_grado?: number;
  id_modalidad?: number;
  id_grado_secciones?: number;
}

interface DetalleCalificacionesProps {
  estudiante: EstudianteDetalle;
  onVolver: () => void;
}

export function DetalleCalificaciones({ estudiante, onVolver }: DetalleCalificacionesProps) {
  const [tab, setTab] = useState('datos');
  const { asignaturas } = useAsignaturas(estudiante.id_grado ?? 0, estudiante.id_modalidad ?? 0);
  const [periodoActual, setPeriodoActual] = useState<number | null>(null);
  const { calificaciones, setCalificaciones } = useCalificaciones(estudiante.id, periodoActual ?? 0);
  const { mostrarMensaje } = useMensajeGlobal();
  const [mostrarModalGuardarHistorialEstudiante, setMostrarModalGuardarHistorialEstudiante] = useState(false);
  const { handleInputChange, errores, limpiarErrores } = useInputCalificaciones(asignaturas, calificaciones, setCalificaciones);
  const { guardarHistorial, loading: loadingGuardarHistorial, exito: exitoGuardarHistorial } = useGuardarHistorial(asignaturas, calificaciones);
  const {
    pendientes,
    loading: loadingPendientes,
    guardarPendientes,
    recargar: recargarPendientes,
  } = usePendientes(estudiante.id);
  const {
    guardarCalificaciones,
    loading: loadingGuardarCalificaciones,
  } = useGuardarCalificaciones();

  useEffect(() => {
    const cargarPeriodoActual = async () => {
      try {
        const periodos = await invoke<{ id_periodo: number, activo: boolean }[]>('listar_periodos_escolares');
        const periodoActivo = periodos.find(p => p.activo);
        if (periodoActivo) {
          setPeriodoActual(periodoActivo.id_periodo);
        } else {
          setPeriodoActual(null);
        }
      } catch (error) {
        mostrarMensaje('Error al cargar el periodo actual', 'error');
      }
    };
    cargarPeriodoActual();
  }, []);

  // Wrapper para onGuardarHistorial sin argumentos
  const handleGuardarHistorialSinArgs = () => {
    // Puedes ajustar los argumentos por defecto si es necesario
    if (estudiante && periodoActual && estudiante.id_grado_secciones) {
      guardarHistorial(estudiante.id, periodoActual, estudiante.id_grado_secciones);
    }
  };

  // Handler para guardar calificaciones (implementar lógica real según necesidad)
  const handleGuardarCalificaciones = async () => {
    try {
      await guardarCalificaciones(calificaciones, estudiante.id, periodoActual ?? 0);
      mostrarMensaje('Calificaciones guardadas correctamente', 'exito');
    } catch (err: any) {
      mostrarMensaje('Error al guardar calificaciones', 'error');
    }
  };

  // Handler para guardar pendientes (implementación real)
  const handleGuardarPendientes = async () => {
    // Construir el array de pendientes a guardar
    const pendientesAGuardar = asignaturas
      .filter(a => a.id_asignatura !== 9 && a.id_asignatura !== 11)
      .map(a => {
        const calif = calificaciones.find(c => c.id_asignatura === a.id_asignatura);
        if (!calif) return null;
        const notaFinal = typeof calif.nota_final === 'number' ? calif.nota_final : 0;
        const revision = calif.revision !== undefined && calif.revision !== '' ? Number(calif.revision) : undefined;
        if (notaFinal >= 9.5) return null;
        if (revision !== undefined && !isNaN(revision) && revision >= 10) return null;
        // Incluye nombre_asignatura para el mensaje de error personalizado
        return {
          id_asignatura: a.id_asignatura,
          id_periodo: periodoActual,
          revision: revision ?? null,
          nota_final: notaFinal, // para la validación en el hook
          nombre_asignatura: a.nombre_asignatura
        };
      })
      .filter(Boolean);

    // Validación de duplicados antes de guardar
    const duplicada = pendientesAGuardar.find(pend =>
      pend && pendientes.some(p => p.id_asignatura === pend.id_asignatura && p.id_periodo === pend.id_periodo)
    );
    if (duplicada) {
      mostrarMensaje(
        `La asignatura "${duplicada.nombre_asignatura}" ya está registrada como pendiente para este estudiante en el periodo y año escolar actual.`,
        'error'
      );
      return;
    }

    try {
      await guardarPendientes(pendientesAGuardar);
      recargarPendientes();
    } catch (err) {
      // El hook ya maneja los mensajes de error
    }
  };

  return (
    <div className="max-w-5xl mx-auto p-4 md:p-8 bg-white dark:bg-dark-800 rounded-2xl shadow-lg">
      <div className="flex items-center justify-between mb-6 gap-4">
        <div className="flex items-center gap-4">
          <div className="flex items-center justify-center w-14 h-14 rounded-full bg-gradient-to-br from-cyan-600 via-blue-700 to-emerald-700 shadow-lg text-3xl font-bold text-white select-none">
            {estudiante?.nombres?.[0] ?? ''}
          </div>
          <h2 className="text-3xl font-extrabold tracking-tight bg-gradient-to-r from-gray-900 via-gray-800 to-gray-700 bg-clip-text text-transparent drop-shadow-lg select-none dark:from-emerald-400 dark:via-cyan-400 dark:to-blue-400 dark:bg-gradient-to-r dark:bg-clip-text dark:text-transparent">
            {estudiante?.nombres} {estudiante?.apellidos}
          </h2>
        </div>
        <button onClick={onVolver} className="px-6 py-2 md:px-8 md:py-2 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-semibold shadow-lg hover:scale-105 hover:bg-emerald-700 transition-all">
          Volver
        </button>
      </div>
      <TabsCalificaciones tabs={TABS} activeTab={tab} onTabChange={setTab} />
      <div className="space-y-6">
        {tab === 'datos' && (
          <DatosPersonalesEstudiante estudiante={estudiante} />
        )}
        {tab === 'actual' && (
          <CalificacionesActuales
            asignaturas={asignaturas}
            calificaciones={calificaciones}
            errores={errores}
            onInputChange={handleInputChange}
            limpiarErrores={limpiarErrores}
            setCalificaciones={setCalificaciones}
            estudiante={estudiante}
            periodoActual={periodoActual}
            mostrarModalGuardarHistorialEstudiante={mostrarModalGuardarHistorialEstudiante}
            setMostrarModalGuardarHistorialEstudiante={setMostrarModalGuardarHistorialEstudiante}
            onGuardarHistorial={handleGuardarHistorialSinArgs}
            loadingGuardarHistorial={loadingGuardarHistorial}
            exitoGuardarHistorial={exitoGuardarHistorial}
            onGuardarCalificaciones={handleGuardarCalificaciones}
            onGuardarPendientes={handleGuardarPendientes}
            loadingGuardarCalificaciones={loadingGuardarCalificaciones}
            loadingGuardarPendientes={loadingPendientes}
          />
        )}
        {tab === 'historial' && (
          <HistorialAcademico idEstudiante={estudiante.id} />
        )}
        {tab === 'pendientes' && (
          <AsignaturasPendientes idEstudiante={estudiante.id} />
        )}
      </div>
    </div>
  );
} 