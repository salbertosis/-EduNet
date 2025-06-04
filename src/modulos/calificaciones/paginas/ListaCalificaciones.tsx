import { useState, useEffect } from "react";
import { BuscadorEstudiante } from "../componentes/BuscadorEstudiante";
import { TabsCalificaciones } from "../componentes/DetalleCalificaciones/TabsCalificaciones";
import { DatosPersonalesEstudiante } from "../componentes/DetalleCalificaciones/DatosPersonalesEstudiante";
import { CalificacionesActuales } from "../componentes/DetalleCalificaciones/CalificacionesActuales";
import { HistorialAcademico } from "../componentes/HistorialAcademico";
import { AsignaturasPendientes } from "../componentes/AsignaturasPendientes";
import { useAsignaturas } from "../hooks/useAsignaturas";
import { useCalificaciones } from "../hooks/useCalificaciones";
import { useInputCalificaciones } from "../hooks/useInputCalificaciones";
import { useGuardarCalificaciones } from "../hooks/useGuardarCalificaciones";
import { useGuardarHistorial } from "../hooks/useGuardarHistorial";
import { usePendientes } from "../hooks/usePendientes";
import { CargaMasivaCalificaciones } from "../componentes/CargaMasivaCalificaciones";
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

const TABS = [
  { key: 'datos', label: 'Datos Personales' },
  { key: 'actual', label: 'Calificaciones Año Actual' },
  { key: 'historial', label: 'Historial Académico' },
  { key: 'pendientes', label: 'Asignaturas Pendientes' },
  { key: 'carga', label: 'Carga Masiva' },
];

export function ListaCalificaciones() {
  const [estudiante, setEstudiante] = useState<any>(null);
  const [tab, setTab] = useState('datos');
  const [periodoActual, setPeriodoActual] = useState<number | null>(null);
  const { mostrarMensaje } = useMensajeGlobal();

  // Cargar periodo escolar actual cuando hay estudiante
  useEffect(() => {
    if (!estudiante) {
      setPeriodoActual(null);
      return;
    }
    const cargarPeriodo = async () => {
      try {
        const periodos = await invoke<{ id_periodo: number, activo: boolean }[]>('listar_periodos_escolares');
        const periodoActivo = periodos.find(p => p.activo);
        if (periodoActivo) {
          setPeriodoActual(periodoActivo.id_periodo);
        } else {
          setPeriodoActual(null);
          mostrarMensaje('No hay periodo escolar activo.', 'error');
        }
      } catch (error) {
        setPeriodoActual(null);
        mostrarMensaje('Error al cargar el periodo escolar actual.', 'error');
      }
    };
    cargarPeriodo();
  }, [estudiante, mostrarMensaje]);

  // Hooks para asignaturas y calificaciones SOLO si hay estudiante y periodo actual
  const { asignaturas, loading: loadingAsignaturas, error: errorAsignaturas } = useAsignaturas(estudiante?.id_grado ?? 0, estudiante?.id_modalidad ?? 0);
  const { calificaciones, setCalificaciones, loading: loadingCalificaciones, error: errorCalificaciones } = useCalificaciones(estudiante?.id ?? 0, periodoActual ?? 0);
  const { handleInputChange, errores, limpiarErrores } = useInputCalificaciones(asignaturas, calificaciones, setCalificaciones);
  const { guardarCalificaciones, loading: loadingGuardarCalificaciones, exito: exitoGuardarCalificaciones } = useGuardarCalificaciones();
  const { guardarHistorial, loading: loadingGuardarHistorial, exito: exitoGuardarHistorial } = useGuardarHistorial(asignaturas, calificaciones);
  const {
    pendientes,
    guardarPendientes,
    loading: loadingGuardarPendientes,
    recargar: recargarPendientes
  } = usePendientes(estudiante?.id ?? 0);

  // Loader general para el tab de calificaciones
  const loadingTabActual = (estudiante && (loadingAsignaturas || loadingCalificaciones || loadingGuardarCalificaciones || loadingGuardarHistorial || loadingGuardarPendientes));

  return (
    <div className="max-w-5xl mx-auto p-4 md:p-8 bg-white dark:bg-dark-800 rounded-2xl shadow-lg mt-8">
      <h2 className="text-2xl font-bold mb-6 text-emerald-700 dark:text-cyan-200">Gestión de Calificaciones</h2>
      <BuscadorEstudiante onSeleccionar={setEstudiante} />
      <div className="mt-8">
        <TabsCalificaciones tabs={TABS} activeTab={tab} onTabChange={setTab} />
        <div className="mt-6">
          {tab === 'datos' && (
            estudiante ? (
              <DatosPersonalesEstudiante estudiante={estudiante} />
            ) : (
              <div className="text-gray-500 dark:text-gray-400 text-lg text-center py-12">
                Selecciona o busca un estudiante para ver sus datos personales.
              </div>
            )
          )}
          {tab === 'actual' && (
            estudiante ? (
              loadingTabActual ? (
                <div className="flex flex-col items-center justify-center py-12 text-lg text-gray-500 dark:text-gray-400">
                  <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-emerald-500 mb-4"></div>
                  Cargando calificaciones y asignaturas...
                </div>
              ) : errorAsignaturas ? (
                <div className="text-red-500 text-center py-8">{errorAsignaturas}</div>
              ) : errorCalificaciones ? (
                <div className="text-red-500 text-center py-8">{errorCalificaciones}</div>
              ) : (
                <>
                  <div className="flex gap-4 mb-6 justify-end">
                    <button
                      className="px-5 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 font-semibold shadow flex items-center gap-2"
                      onClick={() => {
                        if (estudiante && periodoActual && estudiante.id_grado_secciones) {
                          guardarHistorial(estudiante.id, periodoActual, estudiante.id_grado_secciones);
                        } else {
                          mostrarMensaje('Faltan datos para guardar el historial.', 'error');
                        }
                      }}
                      disabled={loadingGuardarHistorial}
                    >
                      {loadingGuardarHistorial ? 'Guardando…' : 'Guardar Historial'}
                    </button>
                  </div>
                  <CalificacionesActuales
                    asignaturas={asignaturas}
                    calificaciones={calificaciones}
                    errores={errores}
                    onInputChange={handleInputChange}
                    limpiarErrores={limpiarErrores}
                    setCalificaciones={setCalificaciones}
                    estudiante={estudiante}
                    periodoActual={periodoActual}
                    mostrarModalGuardarHistorialEstudiante={false}
                    setMostrarModalGuardarHistorialEstudiante={() => {}}
                    onGuardarHistorial={() => guardarHistorial(estudiante.id, periodoActual!, estudiante.id_grado_secciones)}
                    loadingGuardarHistorial={loadingGuardarHistorial}
                    exitoGuardarHistorial={exitoGuardarHistorial}
                    onGuardarCalificaciones={() => guardarCalificaciones(calificaciones, estudiante.id, periodoActual!)}
                    onGuardarPendientes={() => guardarPendientes(pendientes)}
                    loadingGuardarCalificaciones={loadingGuardarCalificaciones}
                    loadingGuardarPendientes={loadingGuardarPendientes}
                  />
                </>
              )
            ) : (
              <div className="text-gray-500 dark:text-gray-400 text-lg text-center py-12">
                Selecciona o busca un estudiante para ver sus calificaciones.
              </div>
            )
          )}
          {tab === 'historial' && (
            estudiante ? (
              <HistorialAcademico idEstudiante={estudiante.id} />
            ) : (
              <div className="text-gray-500 dark:text-gray-400 text-lg text-center py-12">
                Selecciona o busca un estudiante para ver su historial académico.
              </div>
            )
          )}
          {tab === 'pendientes' && (
            estudiante ? (
              <>
                <div className="flex gap-4 mb-6 justify-end">
                  <button
                    className="px-5 py-2 rounded-lg bg-emerald-600 text-white hover:bg-emerald-700 font-semibold shadow flex items-center gap-2"
                    onClick={() => guardarPendientes(pendientes)}
                    disabled={loadingGuardarPendientes}
                  >
                    {loadingGuardarPendientes ? 'Guardando…' : 'Guardar Asignaturas Pendientes'}
                  </button>
                </div>
                <AsignaturasPendientes idEstudiante={estudiante.id} />
              </>
            ) : (
              <div className="text-gray-500 dark:text-gray-400 text-lg text-center py-12">
                Selecciona o busca un estudiante para ver sus asignaturas pendientes.
              </div>
            )
          )}
          {tab === 'carga' && (
            <CargaMasivaCalificaciones />
          )}
        </div>
      </div>
    </div>
  );
} 