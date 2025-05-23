import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Search, Plus, Pencil, Trash2, FileSpreadsheet, X } from 'lucide-react';
import { FormularioDocente } from '../componentes/FormularioDocente';
import * as XLSX from 'xlsx';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { ModalConfirmar } from '../../../componentes/ModalConfirmar';
import { Paginacion } from '../../../componentes/Paginacion';

interface Docente {
  id_docente: number;
  cedula: string;
  apellidos: string;
  nombres: string;
  especialidad: string;
}

interface FiltroDocentes {
  cedula?: string;
  apellidos?: string;
  especialidad?: string;
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

export function ListaDocentes() {
  const [docentes, setDocentes] = useState<Docente[]>([]);
  const [filtros, setFiltros] = useState<FiltroDocentes>({});
  const [docenteSeleccionado, setDocenteSeleccionado] = useState<Docente | null>(null);
  const [mostrarFormulario, setMostrarFormulario] = useState(false);
  const [resumenInsercion, setResumenInsercion] = useState<ResumenInsercion | null>(null);
  const [modalBorrar, setModalBorrar] = useState<{ abierto: boolean, docente?: Docente }>({ abierto: false });
  const [paginacion, setPaginacion] = useState<PaginacionInfo>({
    paginaActual: 1,
    totalPaginas: 1,
    totalRegistros: 0,
    registrosPorPagina: 10
  });
  const { mostrarMensaje } = useMensajeGlobal();
  const [cargando, setCargando] = useState(false);
  const [modalConfirmarExcel, setModalConfirmarExcel] = useState<{ abierto: boolean, cantidad: number, nombre: string, docentes: any[] } | null>(null);
  const inputExcelRef = useRef<HTMLInputElement>(null);

  const cargarDocentes = async () => {
    try {
      setCargando(true);
      const resultado = await invoke<{ datos: Docente[], paginacion: PaginacionInfo }>('obtener_docentes', {
        filtro: Object.keys(filtros).length > 0 ? filtros : null,
        paginacion: {
          pagina: paginacion.paginaActual || 1,
          registros_por_pagina: paginacion.registrosPorPagina || 10
        }
      });
      setDocentes(resultado.datos);
      const paginacionBackend = resultado.paginacion;
      const paginacionFrontend = {
        paginaActual: paginacionBackend.paginaActual,
        totalPaginas: paginacionBackend.totalPaginas,
        totalRegistros: paginacionBackend.totalRegistros,
        registrosPorPagina: paginacionBackend.registrosPorPagina
      };
      setPaginacion(paginacionFrontend);
    } catch (error) {
      console.error('Error al cargar docentes:', error);
      mostrarMensaje('Error al cargar docentes', 'error');
    } finally {
      setCargando(false);
    }
  };

  useEffect(() => {
    const delayDebounce = setTimeout(() => {
      cargarDocentes();
    }, 200); // Búsqueda en tiempo real con pequeño debounce
    return () => clearTimeout(delayDebounce);
  }, [filtros, paginacion.paginaActual]);

  const handleFiltroChange = (campo: keyof FiltroDocentes, valor: string) => {
    setFiltros(prev => {
      if (valor === '') {
        return { ...prev, [campo]: undefined };
      }
      if (campo === 'cedula') {
        return { ...prev, [campo]: valor.replace(/[^0-9]/g, '') };
      }
      return { ...prev, [campo]: valor };
    });
    setPaginacion(prev => ({ ...prev, paginaActual: 1 }));
  };

  const handleCambiarPagina = (nuevaPagina: number) => {
    setPaginacion(prev => ({ ...prev, paginaActual: nuevaPagina }));
  };

  const handleConfirmarBorrar = async () => {
    if (!modalBorrar.docente) return;
    try {
      console.log('Eliminando docente con idDocente:', modalBorrar.docente.id_docente);
      const resp = await invoke('eliminar_docente', { idDocente: modalBorrar.docente.id_docente });
      console.log('Respuesta eliminar_docente:', resp);
      mostrarMensaje(`Docente "${modalBorrar.docente.nombres} ${modalBorrar.docente.apellidos}" eliminado correctamente`, "exito");
      setModalBorrar({ abierto: false });
      cargarDocentes();
    } catch (err) {
      console.error('Error al eliminar docente:', err);
      mostrarMensaje("Error al eliminar docente: " + err, "error");
      setModalBorrar({ abierto: false });
    }
  };

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
        let docentes = XLSX.utils.sheet_to_json(sheet);
        setModalConfirmarExcel({ abierto: true, cantidad: docentes.length, nombre: file.name, docentes });
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
      const docentesCamelCase = modalConfirmarExcel.docentes
        .filter(d => d.cedula && !isNaN(Number(d.cedula)))
        .map((d) => ({
          cedula: Number(d.cedula),
          apellidos: d.apellidos || '',
          nombres: d.nombres || '',
          especialidad: d.especialidad || '',
          telefono: d.telefono || '',
          correoelectronico: d.correoelectronico || '',
        }));
      console.log('Insertando docentes masivo:', docentesCamelCase);
      const resumen = await invoke<ResumenInsercion>('insertar_docentes_masivo', { docentes: docentesCamelCase });
      console.log('Respuesta insertar_docentes_masivo:', resumen);
      setResumenInsercion(resumen);
      cargarDocentes();
    } catch (err) {
      console.error('Error al procesar o insertar docentes:', err);
      mostrarMensaje('Error al procesar o insertar docentes: ' + err, 'error');
    } finally {
      setModalConfirmarExcel(null);
      if (inputExcelRef.current) inputExcelRef.current.value = "";
    }
  };

  const handleCambiarRegistrosPorPagina = (cantidad: number) => {
    setPaginacion(prev => ({ ...prev, registrosPorPagina: cantidad, paginaActual: 1 }));
  };

  const limpiarFiltros = () => setFiltros({});
  const filtrosActivos = Object.entries(filtros).filter(([_, v]) => v !== undefined && v !== '');

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-extrabold tracking-tight bg-gradient-to-r from-emerald-400 via-cyan-400 to-blue-400 bg-clip-text text-transparent drop-shadow-lg select-none">
          Docentes
        </h1>
        <div className="flex items-center space-x-3">
        <button
          onClick={() => {
            setDocenteSeleccionado(null);
            setMostrarFormulario(true);
          }}
            className="flex items-center space-x-2 px-5 py-2.5 bg-gradient-to-r from-emerald-600 to-cyan-600 text-white rounded-xl font-semibold shadow-lg hover:scale-105 hover:bg-emerald-700 transition-all"
        >
          <Plus className="w-5 h-5" />
          <span>Nuevo Docente</span>
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
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 p-4 bg-white dark:bg-dark-800 rounded-xl shadow-soft">
        <div className="space-y-2">
          <label className="text-sm font-semibold text-gray-700 dark:text-emerald-300">Cédula</label>
          <div className="relative">
            <input
              type="text"
              value={filtros.cedula || ''}
              onChange={(e) => handleFiltroChange('cedula', e.target.value)}
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
          <label className="text-sm font-semibold text-gray-700 dark:text-emerald-300">Especialidad</label>
          <div className="relative">
            <input
              type="text"
              value={filtros.especialidad || ''}
              onChange={(e) => handleFiltroChange('especialidad', e.target.value)}
              className="w-full pl-10 pr-4 py-2 rounded-xl border-2 border-emerald-400 dark:border-emerald-700 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 shadow focus:ring-2 focus:ring-emerald-400 focus:border-emerald-500 transition-all"
              placeholder="Buscar por especialidad..."
            />
            <Search className="absolute left-3 top-2.5 w-5 h-5 text-emerald-400" />
          </div>
        </div>
      </div>

      {/* Tabla de docentes */}
      <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft overflow-hidden">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-dark-700">
            <thead className="sticky top-0 z-10 bg-gray-100 dark:bg-dark-900">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-extrabold text-gray-700 dark:text-green-300 tracking-widest uppercase border-b border-gray-200 dark:border-dark-600">CÉDULA</th>
                <th className="px-6 py-3 text-left text-xs font-extrabold text-gray-700 dark:text-green-300 tracking-widest uppercase border-b border-gray-200 dark:border-dark-600">APELLIDOS</th>
                <th className="px-6 py-3 text-left text-xs font-extrabold text-gray-700 dark:text-green-300 tracking-widest uppercase border-b border-gray-200 dark:border-dark-600">NOMBRES</th>
                <th className="px-6 py-3 text-left text-xs font-extrabold text-gray-700 dark:text-green-300 tracking-widest uppercase border-b border-gray-200 dark:border-dark-600">ESPECIALIDAD</th>
                <th className="px-6 py-3 text-right text-xs font-extrabold text-gray-700 dark:text-green-300 tracking-widest uppercase border-b border-gray-200 dark:border-dark-600">ACCIONES</th>
              </tr>
            </thead>
            <tbody>
              {cargando ? (
                [...Array(paginacion.registrosPorPagina)].map((_, idx) => (
                  <tr key={idx} className={idx % 2 === 0 ? 'bg-white dark:bg-dark-800 border-b border-gray-200 dark:border-dark-600' : 'bg-gray-50 dark:bg-dark-700 border-b border-gray-200 dark:border-dark-600'}>
                    <td colSpan={5} className="py-6 text-center animate-pulse text-gray-500 dark:text-gray-600">Cargando...</td>
                  </tr>
                ))
              ) : docentes.length === 0 ? (
                <tr>
                  <td colSpan={5} className="py-8 text-center text-gray-400 bg-white dark:bg-dark-800">No se encontraron docentes.</td>
                </tr>
              ) : (
                docentes.map((docente, idx) => (
                  <tr
                    key={docente.id_docente}
                    className={
                      (idx % 2 === 0
                        ? 'bg-white dark:bg-dark-800 border-b border-gray-200 dark:border-dark-600'
                        : 'bg-gray-50 dark:bg-dark-700 border-b border-gray-200 dark:border-dark-600') +
                      ' transition-all duration-200 hover:shadow-lg hover:shadow-emerald-500/10'
                    }
                  >
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-200">{docente.cedula}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-200">{docente.apellidos ? docente.apellidos.split(' ')[0] : ''}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-200">{docente.nombres}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-200">{docente.especialidad}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium flex gap-2 justify-end">
                    <button
                      onClick={() => {
                        setDocenteSeleccionado(docente);
                        setMostrarFormulario(true);
                      }}
                        className="rounded-full p-2 text-emerald-400 hover:text-white hover:bg-emerald-900/40 hover:shadow-md hover:shadow-emerald-400/30 transition-all"
                        title="Editar"
                    >
                        <Pencil className="w-5 h-5" />
                    </button>
                    <button
                        onClick={() => setModalBorrar({ abierto: true, docente })}
                        className="rounded-full p-2 text-red-400 hover:text-white hover:bg-red-900/40 hover:shadow-md hover:shadow-red-400/30 transition-all"
                        title="Eliminar"
                    >
                        <Trash2 className="w-5 h-5" />
                    </button>
                  </td>
                </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
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
              {docenteSeleccionado ? 'Editar Docente' : 'Nuevo Docente'}
            </h2>
            <div className="flex-1 overflow-y-auto">
              <FormularioDocente
                docente={docenteSeleccionado || undefined}
                onGuardar={() => {
                  setMostrarFormulario(false);
                  cargarDocentes();
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
        mensaje={modalBorrar.docente ? `¿Estás seguro de que deseas eliminar a "${modalBorrar.docente.nombres} ${modalBorrar.docente.apellidos}"?` : ''}
        onConfirmar={handleConfirmarBorrar}
        onCancelar={() => setModalBorrar({ abierto: false })}
      />

      <ModalConfirmar
        abierto={!!modalConfirmarExcel}
        titulo="Confirmar carga de docentes"
        mensaje={modalConfirmarExcel ? `¿Estás seguro de insertar ${modalConfirmarExcel.cantidad} docentes desde el archivo "${modalConfirmarExcel.nombre}"?` : ''}
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