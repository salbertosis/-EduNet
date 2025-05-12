import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Search, Plus, Pencil, Trash2, FileSpreadsheet } from 'lucide-react';
import { FormularioEstudiante } from '../componentes/FormularioEstudiante';
import * as XLSX from 'xlsx';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { ModalConfirmar } from '../../../componentes/ModalConfirmar';
import { Paginacion } from '../../../componentes/Paginacion';

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

export function ListaEstudiantes() {
  const [estudiantes, setEstudiantes] = useState<Estudiante[]>([]);
  const [filtros, setFiltros] = useState<FiltroEstudiantes>({});
  const [estudianteSeleccionado, setEstudianteSeleccionado] = useState<Estudiante | null>(null);
  const [mostrarFormulario, setMostrarFormulario] = useState(false);
  const [grados, setGrados] = useState<{ id_grado: number; nombre_grado: string }[]>([]);
  const [modalidades, setModalidades] = useState<{ id_modalidad: number; nombre_modalidad: string }[]>([]);
  const [resumenInsercion, setResumenInsercion] = useState<ResumenInsercion | null>(null);
  const [modalBorrar, setModalBorrar] = useState<{ abierto: boolean, estudiante?: Estudiante }>({ abierto: false });
  const [paginacion, setPaginacion] = useState<PaginacionInfo>({
    paginaActual: 1,
    totalPaginas: 1,
    totalRegistros: 0,
    registrosPorPagina: 10
  });
  const { mostrarMensaje } = useMensajeGlobal();

  const cargarEstudiantes = async () => {
    try {
      const resultado = await invoke<{ datos: Estudiante[], paginacion: PaginacionInfo }>('obtener_estudiantes', {
        filtro: Object.keys(filtros).length > 0 ? filtros : null,
        paginacion: {
          pagina: paginacion.paginaActual || 1,
          registros_por_pagina: paginacion.registrosPorPagina || 10
        }
      });
      setEstudiantes(resultado.datos);
      const paginacionBackend = resultado.paginacion;
      const paginacionFrontend = {
        paginaActual: paginacionBackend['pagina_actual'],
        totalPaginas: paginacionBackend['total_paginas'],
        totalRegistros: paginacionBackend['total_registros'],
        registrosPorPagina: paginacionBackend['registros_por_pagina']
      };
      setPaginacion(paginacionFrontend);
    } catch (error) {
      console.error('Error al cargar estudiantes:', error);
      mostrarMensaje('Error al cargar estudiantes', 'error');
    }
  };

  useEffect(() => {
    cargarEstudiantes();
  }, [filtros, paginacion.paginaActual]);

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
      return;
    }
    const reader = new FileReader();
    reader.onload = async (evt) => {
      const data = evt.target?.result;
      try {
        const workbook = XLSX.read(data, { type: 'binary' });
        const sheetName = workbook.SheetNames[0];
        const sheet = workbook.Sheets[sheetName];
        let estudiantes = XLSX.utils.sheet_to_json(sheet);
        estudiantes = (estudiantes as Record<string, any>[]).map(est => ({
          ...est,
          fecha_nacimiento: typeof est.fecha_nacimiento === 'number' ? excelDateToISO(est.fecha_nacimiento) : est.fecha_nacimiento,
          fecha_ingreso: typeof est.fecha_ingreso === 'number' ? excelDateToISO(est.fecha_ingreso) : est.fecha_ingreso,
          fecha_retiro: typeof est.fecha_retiro === 'number' ? excelDateToISO(est.fecha_retiro) : est.fecha_retiro,
        }));
        const resumen = await invoke<ResumenInsercion>('insertar_estudiantes_masivo', { estudiantes });
        setResumenInsercion(resumen);
        mostrarMensaje(`Importación finalizada: ${resumen.insertados} insertados, ${resumen.duplicados} duplicados.`, 'exito');
        cargarEstudiantes();
      } catch (err) {
        mostrarMensaje('Error al procesar o insertar estudiantes: ' + err, 'error');
      }
    };
    reader.readAsBinaryString(file);
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Estudiantes</h1>
        <div className="flex items-center space-x-2">
          <button
            onClick={() => {
              setEstudianteSeleccionado(null);
              setMostrarFormulario(true);
            }}
            className="flex items-center space-x-2 px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          >
            <Plus className="w-5 h-5" />
            <span>Nuevo Estudiante</span>
          </button>
          <label className="flex items-center space-x-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors cursor-pointer">
            <FileSpreadsheet className="w-5 h-5" />
            <span>Cargar Excel</span>
            <input
              type="file"
              accept=".xlsx,.xls"
              style={{ display: 'none' }}
              onChange={handleExcelUpload}
            />
          </label>
        </div>
      </div>

      {/* Filtros */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-4 p-4 bg-white dark:bg-dark-800 rounded-xl shadow-soft">
        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Cédula</label>
          <div className="relative">
            <input
              type="text"
              value={filtros.cedula || ''}
              onChange={(e) => handleFiltroChange('cedula', e.target.value.replace(/[^0-9]/g, ''))}
              className="w-full pl-10 pr-4 py-2 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100"
              placeholder="Buscar por cédula..."
            />
            <Search className="absolute left-3 top-2.5 w-5 h-5 text-gray-400" />
          </div>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Apellidos</label>
          <div className="relative">
            <input
              type="text"
              value={filtros.apellidos || ''}
              onChange={(e) => handleFiltroChange('apellidos', e.target.value)}
              className="w-full pl-10 pr-4 py-2 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100"
              placeholder="Buscar por apellidos..."
            />
            <Search className="absolute left-3 top-2.5 w-5 h-5 text-gray-400" />
          </div>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Grado</label>
          <select
            value={filtros.grado !== undefined ? filtros.grado.toString() : ''}
            onChange={(e) => {
              const val = e.target.value;
              handleFiltroChange('grado', val);
            }}
            className="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100"
          >
            <option value="">Todos</option>
            {grados.map((grado) => (
              <option key={grado.id_grado} value={grado.id_grado}>{grado.nombre_grado}</option>
            ))}
          </select>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Modalidad</label>
          <select
            value={filtros.modalidad !== undefined ? filtros.modalidad.toString() : ''}
            onChange={(e) => {
              const val = e.target.value;
              handleFiltroChange('modalidad', val);
            }}
            className="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100"
          >
            <option value="">Todas</option>
            {modalidades.map((mod) => (
              <option key={mod.id_modalidad} value={mod.id_modalidad}>{mod.nombre_modalidad}</option>
            ))}
          </select>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium text-gray-700 dark:text-gray-300">Estado</label>
          <select
            value={filtros.estado || ''}
            onChange={(e) => handleFiltroChange('estado', e.target.value)}
            className="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100"
          >
            <option value="">Todos</option>
            <option value="Activo">Activo</option>
            <option value="Retirado">Retirado</option>
          </select>
        </div>
      </div>

      {/* Tabla de estudiantes */}
      <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft overflow-hidden">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-dark-700">
            <thead className="bg-gray-50 dark:bg-dark-700">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Cédula</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Nombres</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Apellidos</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Grado</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Sección</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Modalidad</th>
                <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Acciones</th>
              </tr>
            </thead>
            <tbody className="bg-white dark:bg-dark-800 divide-y divide-gray-200 dark:divide-dark-700">
              {Array.isArray(estudiantes) && estudiantes.map((estudiante) => (
                <tr key={estudiante.id} className="hover:bg-gray-50 dark:hover:bg-dark-700">
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">{estudiante.cedula}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">{estudiante.nombres}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">{estudiante.apellidos}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">{estudiante.nombre_grado}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">{estudiante.nombre_seccion}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">{estudiante.nombre_modalidad}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <button
                      onClick={() => {
                        setEstudianteSeleccionado(estudiante);
                        setMostrarFormulario(true);
                      }}
                      className="text-primary-600 hover:text-primary-900 dark:text-primary-400 dark:hover:text-primary-300 mr-4"
                    >
                      <Pencil className="w-5 h-5" />
                    </button>
                    <button
                      onClick={() => setModalBorrar({ abierto: true, estudiante })}
                      className="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300"
                    >
                      <Trash2 className="w-5 h-5" />
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Paginación */}
      <div className="flex justify-between items-center bg-white dark:bg-dark-800 p-4 rounded-xl shadow-soft">
        <div className="text-sm text-gray-700 dark:text-gray-300">
          Mostrando {estudiantes.length} de {paginacion.totalRegistros} registros
        </div>
        <Paginacion
          paginaActual={paginacion.paginaActual}
          totalPaginas={paginacion.totalPaginas}
          onCambiarPagina={handleCambiarPagina}
        />
      </div>

      {/* Modal de formulario */}
      {mostrarFormulario && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
          <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6 w-full max-w-2xl">
            <h2 className="text-xl font-bold mb-4">
              {estudianteSeleccionado ? 'Editar Estudiante' : 'Nuevo Estudiante'}
            </h2>
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
    </div>
  );
}