import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Search, Plus, Pencil, Trash2, FileSpreadsheet, X } from 'lucide-react';
import { FormularioEstudiante } from '../componentes/FormularioEstudiante';
import * as XLSX from 'xlsx';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { ModalConfirmar } from '../../../componentes/ModalConfirmar';
import { Paginacion } from '../../../componentes/Paginacion';
import { OjoVerDetalles } from '../../calificaciones/componentes/OjoVerDetalles';
import { DetalleCalificaciones } from '../../calificaciones/paginas/DetalleCalificaciones';
import { useNavigate } from 'react-router-dom';

interface Estudiante {
  id: number;
  cedula: string;
  apellidos: string;
  nombres: string;
  nombre_grado?: string;
  nombre_seccion?: string;
  nombre_modalidad?: string;
}

interface FiltroEstudiantes {
  cedula?: string;
  apellidos?: string;
  grado?: number;
  modalidad?: number;
  estado?: string;
}

interface ResumenInsercion {
  total_registros: number;
  insertados: number;
  duplicados: number;
  errores: string[];
}

interface PaginacionInfo {
  paginaActual: number;
  totalPaginas: number;
  totalRegistros: number;
  registrosPorPagina: number;
}

interface Grado {
  id_grado: number;
  nombre_grado: string;
}

interface Modalidad {
  id_modalidad: number;
  nombre_modalidad: string;
}

interface ResultadoEstudiantes {
  datos: Estudiante[];
  paginacion: {
    pagina_actual: number;
    total_paginas: number;
    total_registros: number;
    registros_por_pagina: number;
  };
}

export function ListaEstudiantes() {
  const [estudiantes, setEstudiantes] = useState<Estudiante[]>([]);
  const [filtros, setFiltros] = useState<FiltroEstudiantes>({});
  const [estudianteSeleccionado, setEstudianteSeleccionado] = useState<Estudiante | null>(null);
  const [mostrarFormulario, setMostrarFormulario] = useState(false);
  const [grados, setGrados] = useState<Grado[]>([]);
  const [modalidades, setModalidades] = useState<Modalidad[]>([]);
  const [resumenInsercion, setResumenInsercion] = useState<ResumenInsercion | null>(null);
  const [modalBorrar, setModalBorrar] = useState<{ abierto: boolean, estudiante?: Estudiante }>({ abierto: false });
  const [paginacion, setPaginacion] = useState<PaginacionInfo>({
    paginaActual: 1,
    totalPaginas: 1,
    totalRegistros: 0,
    registrosPorPagina: 10
  });
  const { mostrarMensaje } = useMensajeGlobal();
  const [cargando, setCargando] = useState(false);
  const [mostrarDetalleCalificaciones, setMostrarDetalleCalificaciones] = useState(false);
  const [estudianteDetalle, setEstudianteDetalle] = useState<Estudiante | null>(null);
  const navigate = useNavigate();
  const [modalConfirmarExcel, setModalConfirmarExcel] = useState<{ abierto: boolean, cantidad: number, nombre: string, estudiantes: any[] } | null>(null);
  const inputExcelRef = useRef<HTMLInputElement>(null);

  const cargarEstudiantes = async () => {
    try {
      setCargando(true);
      const resultado = await invoke<{ datos: Estudiante[], paginacion: any }>('obtener_estudiantes', {
        filtro: Object.keys(filtros).length > 0 ? filtros : null,
        paginacion: {
          pagina: paginacion.paginaActual || 1,
          registros_por_pagina: paginacion.registrosPorPagina || 10
        }
      });
      setEstudiantes(resultado.datos);
      const paginacionBackend = resultado.paginacion;
      const paginacionFrontend = {
        paginaActual: paginacionBackend.pagina_actual,
        totalPaginas: paginacionBackend.total_paginas,
        totalRegistros: paginacionBackend.total_registros,
        registrosPorPagina: paginacionBackend.registros_por_pagina
      };
      setPaginacion(paginacionFrontend);
    } catch (error) {
      const mensajeError = error instanceof Error ? error.message : 'Error desconocido';
      mostrarMensaje(`Error: ${mensajeError}`, 'error');
    } finally {
      setCargando(false);
    }
  };

  useEffect(() => {
    cargarEstudiantes();
  }, [filtros, paginacion.paginaActual, paginacion.registrosPorPagina]);

  useEffect(() => {
    // Cargar grados y modalidades al montar
    invoke('listar_grados').then((data: any) => setGrados(data)).catch(console.error);
    invoke('listar_modalidades').then((data: any) => setModalidades(data)).catch(console.error);
  }, []);

  const handleFiltroChange = (campo: keyof FiltroEstudiantes, valor: string) => {
    setFiltros(prev => {
      if (valor === '') {
        return { ...prev, [campo]: undefined };
      }
      if (campo === 'grado' || campo === 'modalidad') {
        const num = Number(valor);
        return { ...prev, [campo]: isNaN(num) ? undefined : num };
      }
      if (campo === 'cedula') {
        return { ...prev, [campo]: valor };
      }
      return { ...prev, [campo]: valor };
    });
    // Resetear a la primera página cuando cambian los filtros
    setPaginacion(prev => ({ ...prev, paginaActual: 1 }));
  };

  const handleCambiarPagina = (nuevaPagina: number) => {
    setPaginacion(prev => ({ ...prev, paginaActual: nuevaPagina }));
  };

  const handleEliminar = async (id: number) => {
    // Ya no se usa confirm, ahora se usa el modal
    // Esta función se mantiene para compatibilidad, pero no se llama directamente
  };

  const handleConfirmarBorrar = async () => {
    if (!modalBorrar.estudiante) return;
    try {
      await invoke('eliminar_estudiante', { id: modalBorrar.estudiante.id });
      mostrarMensaje(`Estudiante "${modalBorrar.estudiante.nombres} ${modalBorrar.estudiante.apellidos}" eliminado correctamente`, "exito");
      setModalBorrar({ abierto: false });
      cargarEstudiantes();
    } catch (err) {
      mostrarMensaje("Error al eliminar estudiante: " + err, "error");
      setModalBorrar({ abierto: false });
    }
  };

  function excelDateToISO(serial: number) {
    const utc_days = Math.floor(serial - 25569);
    const utc_value = utc_days * 86400;
    const date_info = new Date(utc_value * 1000);
    return date_info.toISOString().split('T')[0];
  }

  const handleExcelUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) {
      mostrarMensaje('No se seleccionó ningún archivo', 'error');
      if (inputExcelRef.current) inputExcelRef.current.value = "";
      return;
    }
    const reader = new FileReader();
    reader.onload = async (evt) => {
      const data = evt.target?.result;
      try {
        const workbook = XLSX.read(data, { type: 'binary' });
        const sheetName = workbook.SheetNames[0];
        const sheet = workbook.Sheets[sheetName];
        const estudiantesRaw = XLSX.utils.sheet_to_json<Estudiante>(sheet);
        const estudiantes = (estudiantesRaw as Record<string, any>[]).map(est => ({
          ...est,
          fecha_nacimiento: typeof est.fecha_nacimiento === 'number' ? excelDateToISO(est.fecha_nacimiento) : est.fecha_nacimiento,
          fecha_ingreso: typeof est.fecha_ingreso === 'number' ? excelDateToISO(est.fecha_ingreso) : est.fecha_ingreso,
          fecha_retiro: typeof est.fecha_retiro === 'number' ? excelDateToISO(est.fecha_retiro) : est.fecha_retiro,
        }));
        setModalConfirmarExcel({ abierto: true, cantidad: estudiantes.length, nombre: file.name, estudiantes });
      } catch (err) {
        mostrarMensaje('Error al procesar el archivo Excel: ' + err, 'error');
      } finally {
        if (inputExcelRef.current) inputExcelRef.current.value = "";
      }
    };
    reader.readAsBinaryString(file);
  };

  const handleConfirmarInsertarExcel = async () => {
    if (!modalConfirmarExcel) return;
    try {
      const resumen = await invoke<ResumenInsercion>('insertar_estudiantes_masivo', { estudiantes: modalConfirmarExcel.estudiantes });
      setResumenInsercion(resumen);
      cargarEstudiantes();
    } catch (err) {
      mostrarMensaje('Error al procesar o insertar estudiantes: ' + err, 'error');
    } finally {
      setModalConfirmarExcel(null);
      if (inputExcelRef.current) inputExcelRef.current.value = "";
    }
  };

  const handleCambiarRegistrosPorPagina = (cantidad: number) => {
    setPaginacion(prev => ({ ...prev, registrosPorPagina: cantidad, paginaActual: 1 }));
  };

  // Chips de filtros activos
  const filtrosActivos = Object.entries(filtros).filter(([_, v]) => v !== undefined && v !== '');

  const limpiarFiltros = () => setFiltros({});

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-extrabold tracking-tight bg-gradient-to-r from-emerald-400 via-cyan-400 to-blue-400 bg-clip-text text-transparent drop-shadow-lg select-none">
          Estudiantes
        </h1>
        <div className="flex items-center space-x-3">
        <button
          onClick={() => {
            setEstudianteSeleccionado(null);
            setMostrarFormulario(true);
          }}
            className="flex items-center space-x-2 px-5 py-2.5 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-semibold shadow-lg hover:scale-105 hover:bg-emerald-700 transition-all"
        >
          <Plus className="w-5 h-5" />
          <span>Nuevo Estudiante</span>
        </button>
          <label className="flex items-center space-x-2 px-5 py-2.5 bg-gradient-to-r from-green-600 to-emerald-600 text-white rounded-xl font-semibold shadow-lg hover:scale-105 hover:bg-green-700 transition-all cursor-pointer">
            <FileSpreadsheet className="w-5 h-5" />
            <span>Cargar Excel</span>
            <input
              ref={inputExcelRef}
              type="file"
              accept=".xlsx,.xls"
              style={{ display: 'none' }}
              onChange={handleExcelUpload}
            />
          </label>
        </div>
      </div>

      {/* Filtros activos como chips y botón limpiar */}
      {filtrosActivos.length > 0 && (
        <div className="flex flex-wrap items-center gap-2 mb-2">
          {filtrosActivos.map(([k, v]) => (
            <span key={k} className="flex items-center bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-200 px-3 py-1 rounded-full text-xs font-medium">
              {k}: {v}
              <button onClick={() => setFiltros(prev => ({ ...prev, [k]: undefined }))} className="ml-1 text-primary-500 hover:text-primary-800">
                <X className="w-3 h-3" />
              </button>
            </span>
          ))}
          <button onClick={limpiarFiltros} className="ml-2 px-3 py-1 bg-gray-200 dark:bg-dark-700 text-gray-700 dark:text-gray-200 rounded-full text-xs font-medium hover:bg-gray-300 dark:hover:bg-dark-600 transition-all">Limpiar filtros</button>
        </div>
      )}

      {/* Filtros */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-4 p-4 bg-white dark:bg-dark-800 rounded-xl shadow-soft">
        <div className="space-y-2">
          <label className="text-sm font-semibold text-gray-700 dark:text-emerald-300">Cédula</label>
          <div className="relative">
            <input
              type="text"
              value={filtros.cedula || ''}
              onChange={(e) => handleFiltroChange('cedula', e.target.value.replace(/[^0-9]/g, ''))}
              className="w-full pl-10 pr-4 py-2 rounded-xl border-2 border-emerald-400 dark:border-emerald-700 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 shadow focus:ring-2 focus:ring-emerald-400 focus:border-emerald-500 transition-all"
              placeholder="Buscar por cédula..."
            />
            <Search className="absolute left-3 top-2.5 w-5 h-5 text-emerald-400" />
          </div>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-semibold text-gray-700 dark:text-emerald-300">Apellidos</label>
          <div className="relative">
            <input
              type="text"
              value={filtros.apellidos || ''}
              onChange={(e) => handleFiltroChange('apellidos', e.target.value)}
              className="w-full pl-10 pr-4 py-2 rounded-xl border-2 border-emerald-400 dark:border-emerald-700 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 shadow focus:ring-2 focus:ring-emerald-400 focus:border-emerald-500 transition-all"
              placeholder="Buscar por apellidos..."
            />
            <Search className="absolute left-3 top-2.5 w-5 h-5 text-emerald-400" />
          </div>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-semibold text-gray-700 dark:text-emerald-300">Grado</label>
          <select
            value={filtros.grado !== undefined ? filtros.grado.toString() : ''}
            onChange={(e) => {
              const val = e.target.value;
              handleFiltroChange('grado', val);
            }}
            className="w-full px-4 py-2 rounded-xl border-2 border-emerald-400 dark:border-emerald-700 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 shadow focus:ring-2 focus:ring-emerald-400 focus:border-emerald-500 transition-all"
          >
            <option value="">Todos</option>
            {grados.map((grado) => (
              <option key={grado.id_grado} value={grado.id_grado}>{grado.nombre_grado}</option>
            ))}
          </select>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-semibold text-gray-700 dark:text-emerald-300">Modalidad</label>
          <select
            value={filtros.modalidad !== undefined ? filtros.modalidad.toString() : ''}
            onChange={(e) => {
              const val = e.target.value;
              handleFiltroChange('modalidad', val);
            }}
            className="w-full px-4 py-2 rounded-xl border-2 border-emerald-400 dark:border-emerald-700 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 shadow focus:ring-2 focus:ring-emerald-400 focus:border-emerald-500 transition-all"
          >
            <option value="">Todas</option>
            {modalidades.map((mod) => (
              <option key={mod.id_modalidad} value={mod.id_modalidad}>{mod.nombre_modalidad}</option>
            ))}
          </select>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-semibold text-gray-700 dark:text-emerald-300">Estado</label>
          <select
            value={filtros.estado || ''}
            onChange={(e) => handleFiltroChange('estado', e.target.value)}
            className="w-full px-4 py-2 rounded-xl border-2 border-emerald-400 dark:border-emerald-700 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 shadow focus:ring-2 focus:ring-emerald-400 focus:border-emerald-500 transition-all"
          >
            <option value="">Todos</option>
            <option value="Activo">Activo</option>
            <option value="Retirado">Retirado</option>
          </select>
        </div>
      </div>

      {/* Tabla de estudiantes */}
      <div className="bg-white dark:bg-dark-800 rounded-2xl shadow-lg overflow-hidden">
        <table className="min-w-full">
          <thead className="sticky top-0 z-10 bg-gradient-to-r from-emerald-600 via-emerald-500 to-emerald-600 dark:from-emerald-900 dark:via-dark-800 dark:to-dark-900">
            <tr>
              <th className="px-6 py-4 text-left text-xs font-extrabold text-white dark:text-emerald-300 tracking-widest uppercase">CÉDULA</th>
              <th className="px-6 py-4 text-left text-xs font-extrabold text-white dark:text-emerald-300 tracking-widest uppercase">APELLIDOS</th>
              <th className="px-6 py-4 text-left text-xs font-extrabold text-white dark:text-emerald-300 tracking-widest uppercase">NOMBRES</th>
              <th className="px-6 py-4 text-left text-xs font-extrabold text-white dark:text-emerald-300 tracking-widest uppercase">GRADO</th>
              <th className="px-6 py-4 text-xs font-extrabold text-white dark:text-emerald-300 tracking-widest uppercase text-center">SECCIÓN</th>
              <th className="px-6 py-4 text-xs font-extrabold text-white dark:text-emerald-300 tracking-widest uppercase text-center">MODALIDAD</th>
              <th className="px-6 py-4 text-right text-xs font-extrabold text-white dark:text-emerald-300 tracking-widest uppercase">ACCIONES</th>
            </tr>
          </thead>
          <tbody>
            {cargando ? (
              [...Array(paginacion.registrosPorPagina)].map((_, idx) => (
                <tr key={idx} className={idx % 2 === 0 ? 'bg-white dark:bg-dark-800/80' : 'bg-gray-50 dark:bg-dark-700/80'}>
                  <td colSpan={7} className="py-6 text-center animate-pulse text-gray-500 dark:text-gray-600">Cargando...</td>
                </tr>
              ))
            ) : estudiantes.length === 0 ? (
              <tr>
                <td colSpan={7} className="py-8 text-center text-gray-400 bg-white dark:bg-dark-800">No se encontraron estudiantes.</td>
              </tr>
            ) : (
              estudiantes.map((estudiante, idx) => (
                <tr
                  key={estudiante.id}
                  className={`transition-all duration-200 ${idx % 2 === 0 ? 'bg-white dark:bg-dark-800/80' : 'bg-gray-50 dark:bg-dark-700/80'} hover:bg-emerald-50 dark:hover:bg-emerald-900/40`}
                >
                  <td className="px-6 py-4 whitespace-nowrap text-base text-gray-900 dark:text-gray-100 font-medium align-middle">{estudiante.cedula}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-base text-gray-900 dark:text-gray-100 font-medium align-middle">{estudiante.apellidos}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-base text-gray-900 dark:text-gray-100 font-medium align-middle">{estudiante.nombres}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-base text-gray-900 dark:text-gray-100 font-medium align-middle">{estudiante.nombre_grado}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-base text-gray-900 dark:text-gray-100 font-medium align-middle text-center">{estudiante.nombre_seccion}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-base text-gray-900 dark:text-gray-100 font-medium align-middle text-center">{estudiante.nombre_modalidad}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-right text-base font-medium align-middle flex gap-2 justify-end">
                    <button
                      onClick={() => {
                        setEstudianteSeleccionado(estudiante);
                        setMostrarFormulario(true);
                      }}
                      className="rounded-full p-2 text-emerald-600 dark:text-emerald-400 hover:text-white hover:bg-emerald-600 dark:hover:bg-emerald-700/40 hover:scale-110 transition-all"
                      title="Editar"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" className="w-5 h-5" viewBox="0 0 20 20"><path d="M17.414 2.586a2 2 0 0 0-2.828 0l-9.9 9.9A2 2 0 0 0 4 14.414V17a1 1 0 0 0 1 1h2.586a2 2 0 0 0 1.414-.586l9.9-9.9a2 2 0 0 0 0-2.828l-2.414-2.414z"/></svg>
                    </button>
                    <button
                      onClick={() => setModalBorrar({ abierto: true, estudiante })}
                      className="rounded-full p-2 text-red-600 dark:text-red-400 hover:text-white hover:bg-red-600 dark:hover:bg-red-700/40 hover:scale-110 transition-all"
                      title="Eliminar"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" className="w-5 h-5" viewBox="0 0 20 20"><path d="M6 8a1 1 0 0 1 2 0v6a1 1 0 1 1-2 0V8zm4 0a1 1 0 0 1 2 0v6a1 1 0 1 1-2 0V8zm4 0a1 1 0 0 1 2 0v6a1 1 0 1 1-2 0V8z"/><path fillRule="evenodd" d="M4 5a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v1H4V5zm2-3a4 4 0 0 0-4 4v1a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1V6a4 4 0 0 0-4-4H6z" clipRule="evenodd"/></svg>
                    </button>
                    <OjoVerDetalles
                      onClick={() => navigate(`/estudiantes/${estudiante.id}/calificaciones`)}
                    />
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Paginación avanzada */}
      <div className="flex justify-between items-center bg-white dark:bg-dark-800 p-4 rounded-xl shadow-soft">
        <Paginacion
          paginaActual={paginacion.paginaActual}
          totalPaginas={paginacion.totalPaginas}
          totalRegistros={paginacion.totalRegistros}
          registrosPorPagina={paginacion.registrosPorPagina}
          onCambiarPagina={handleCambiarPagina}
          onCambiarRegistrosPorPagina={handleCambiarRegistrosPorPagina}
        />
      </div>

      {/* Modal de formulario */}
      {mostrarFormulario && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6 w-full max-w-2xl max-h-[90vh] flex flex-col">
            <h2 className="text-xl font-bold mb-4">
              {estudianteSeleccionado ? 'Editar Estudiante' : 'Nuevo Estudiante'}
            </h2>
            <div className="flex-1 overflow-y-auto">
              <FormularioEstudiante
                estudiante={estudianteSeleccionado || undefined}
                onGuardar={() => {
                  setMostrarFormulario(false);
                  cargarEstudiantes();
                }}
                onCancelar={() => setMostrarFormulario(false)}
              />
            </div>
          </div>
        </div>
      )}

      {/* Modal de resumen de inserción */}
      {resumenInsercion && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6 w-full max-w-2xl">
            <h2 className="text-xl font-bold mb-4">Resumen de la Importación</h2>
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div className="bg-gray-50 dark:bg-dark-700 p-4 rounded-lg">
                  <p className="text-sm text-gray-600 dark:text-gray-400">Total de registros</p>
                  <p className="text-2xl font-semibold">{resumenInsercion.total_registros}</p>
                </div>
                <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
                  <p className="text-sm text-green-600 dark:text-green-400">Registros insertados</p>
                  <p className="text-2xl font-semibold text-green-600 dark:text-green-400">{resumenInsercion.insertados}</p>
                </div>
                <div className="bg-yellow-50 dark:bg-yellow-900/20 p-4 rounded-lg">
                  <p className="text-sm text-yellow-600 dark:text-yellow-400">Cédulas duplicadas</p>
                  <p className="text-2xl font-semibold text-yellow-600 dark:text-yellow-400">{resumenInsercion.duplicados}</p>
                </div>
                <div className="bg-red-50 dark:bg-red-900/20 p-4 rounded-lg">
                  <p className="text-sm text-red-600 dark:text-red-400">Errores</p>
                  <p className="text-2xl font-semibold text-red-600 dark:text-red-400">{resumenInsercion.errores.length}</p>
                </div>
              </div>
              
              {resumenInsercion.errores.length > 0 && (
                <div className="mt-4">
                  <h3 className="text-lg font-semibold mb-2">Detalles de errores:</h3>
                  <div className="max-h-60 overflow-y-auto bg-gray-50 dark:bg-dark-700 rounded-lg p-4">
                    {resumenInsercion.errores.map((error, index) => (
                      <p key={index} className="text-sm text-gray-600 dark:text-gray-400 mb-1">{error}</p>
                    ))}
                  </div>
                </div>
              )}
              
              <div className="mt-6 flex justify-end">
                <button
                  onClick={() => setResumenInsercion(null)}
                  className="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
                >
                  Cerrar
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Modal de confirmación para borrar */}
      <ModalConfirmar
        abierto={modalBorrar.abierto}
        mensaje={modalBorrar.estudiante ? `¿Estás seguro de que deseas eliminar a "${modalBorrar.estudiante.nombres} ${modalBorrar.estudiante.apellidos}"?` : ''}
        onConfirmar={handleConfirmarBorrar}
        onCancelar={() => setModalBorrar({ abierto: false })}
      />

      <ModalConfirmar
        abierto={!!modalConfirmarExcel}
        titulo="Confirmar carga de estudiantes"
        mensaje={modalConfirmarExcel ? `¿Estás seguro de insertar ${modalConfirmarExcel.cantidad} estudiantes desde el archivo "${modalConfirmarExcel.nombre}"?` : ''}
        textoConfirmar="Insertar"
        onConfirmar={handleConfirmarInsertarExcel}
        onCancelar={() => {
          setModalConfirmarExcel(null);
          if (inputExcelRef.current) inputExcelRef.current.value = "";
        }}
      />
    </div>
  );
}