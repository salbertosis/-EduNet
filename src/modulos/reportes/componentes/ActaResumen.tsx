import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { SelectorPeriodo } from '../../../componentes/SelectorPeriodo';
import { SelectorModalidad } from '../../../componentes/SelectorModalidad';
import { SelectorGrado } from '../../../componentes/SelectorGrado';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

interface FiltrosResumen {
  id_periodo: number | null;
  id_modalidad: number | null;
  id_grado: number | null;
  id_seccion: number | null;
}

interface GradoSeccion {
  id_grado_secciones: number;
  nombre_grado: string;
  nombre_seccion: string;
}

export default function ActaResumen() {
  const [filtros, setFiltros] = useState<FiltrosResumen>({
    id_periodo: null,
    id_modalidad: null,
    id_grado: null,
    id_seccion: null,
  });
  const [gradoSecciones, setGradoSecciones] = useState<GradoSeccion[]>([]);
  const [cargando, setCargando] = useState(false);
  const { mostrarMensaje } = useMensajeGlobal();

  // Cargar grado-secciones cuando cambian los filtros
  useEffect(() => {
    if (filtros.id_periodo && filtros.id_modalidad && filtros.id_grado) {
      cargarGradoSecciones();
    } else {
      setGradoSecciones([]);
    }
  }, [filtros.id_periodo, filtros.id_modalidad, filtros.id_grado]);

  const cargarGradoSecciones = async () => {
    try {
      const resultado = await invoke<GradoSeccion[]>('obtener_grado_secciones_por_filtros', {
        idPeriodo: filtros.id_periodo,
        idModalidad: filtros.id_modalidad,
        idGrado: filtros.id_grado,
        idSeccion: filtros.id_seccion
      });
      setGradoSecciones(resultado);
    } catch (error) {
      console.error('Error cargando grado-secciones:', error);
      setGradoSecciones([]);
    }
  };

  const generarActaResumen = async (idGradoSecciones: number) => {
    try {
      setCargando(true);
      const pdfBase64 = await invoke<string>('generar_acta_resumen', {
        idGradoSecciones: idGradoSecciones
      });
      
      // Convertir base64 a blob y descargar
      const byteCharacters = atob(pdfBase64);
      const byteNumbers = new Array(byteCharacters.length);
      for (let i = 0; i < byteCharacters.length; i++) {
        byteNumbers[i] = byteCharacters.charCodeAt(i);
      }
      const byteArray = new Uint8Array(byteNumbers);
      const blob = new Blob([byteArray], { type: 'application/pdf' });
      
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `acta_resumen_${idGradoSecciones}.pdf`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);
      
      mostrarMensaje('Acta de resumen generada exitosamente', 'exito');
    } catch (error) {
      console.error('Error generando acta de resumen:', error);
      mostrarMensaje('Error al generar el acta de resumen', 'error');
    } finally {
      setCargando(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="bg-white dark:bg-dark-700 p-6 rounded-xl shadow-sm">
        <h3 className="text-lg font-semibold mb-4 text-emerald-700 dark:text-emerald-300">
          Generar Acta de Resumen de Calificaciones
        </h3>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Año Escolar
            </label>
            <SelectorPeriodo
              value={filtros.id_periodo}
              onChange={(id_periodo: number) => setFiltros(f => ({
                ...f,
                id_periodo,
                id_grado: null,
                id_seccion: null
              }))}
              className="w-full"
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Modalidad
            </label>
            <SelectorModalidad
              value={filtros.id_modalidad}
              onChange={(id_modalidad: number) => setFiltros(f => ({
                ...f,
                id_modalidad,
                id_grado: null,
                id_seccion: null
              }))}
              className="w-full"
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Grado
            </label>
            <SelectorGrado
              value={filtros.id_grado !== null ? filtros.id_grado.toString() : ''}
              onChange={(valor: string) => setFiltros(f => ({
                ...f,
                id_grado: valor ? Number(valor) : null,
                id_seccion: null
              }))}
              className="w-full"
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Sección (Opcional)
            </label>
            <select
              value={filtros.id_seccion !== null ? filtros.id_seccion.toString() : ''}
              onChange={(e) => setFiltros(f => ({
                ...f,
                id_seccion: e.target.value ? Number(e.target.value) : null
              }))}
              className="w-full px-3 py-2 bg-white dark:bg-dark-600 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
            >
              <option value="">Todas las secciones</option>
              {gradoSecciones.map((gs) => (
                <option key={gs.id_grado_secciones} value={gs.id_grado_secciones}>
                  {gs.nombre_seccion}
                </option>
              ))}
            </select>
          </div>
        </div>

        {gradoSecciones.length > 0 && (
          <div className="mt-6">
            <h4 className="text-md font-semibold mb-3 text-gray-700 dark:text-gray-300">
              Grado-Secciones Disponibles
            </h4>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
              {gradoSecciones.map((gs) => (
                <div
                  key={gs.id_grado_secciones}
                  className="bg-gray-50 dark:bg-dark-600 p-4 rounded-lg border border-gray-200 dark:border-dark-500"
                >
                  <div className="flex justify-between items-center">
                    <div>
                      <p className="font-medium text-gray-900 dark:text-gray-100">
                        {gs.nombre_grado} - {gs.nombre_seccion}
                      </p>
                    </div>
                    <button
                      onClick={() => generarActaResumen(gs.id_grado_secciones)}
                      disabled={cargando}
                      className="px-3 py-1 bg-gradient-to-r from-emerald-500 to-cyan-500 text-white text-sm rounded-md hover:from-emerald-600 hover:to-cyan-600 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 flex items-center gap-1"
                    >
                      {cargando ? (
                        <>
                          <svg className="animate-spin h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                          </svg>
                          Generando...
                        </>
                      ) : (
                        <>
                          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                          </svg>
                          Generar PDF
                        </>
                      )}
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
      
      <div className="bg-white dark:bg-dark-700 p-6 rounded-xl shadow-sm">
        <h3 className="text-lg font-semibold mb-4 text-emerald-700 dark:text-emerald-300">
          Instrucciones
        </h3>
        <div className="prose dark:prose-invert max-w-none">
          <ol className="list-decimal list-inside space-y-2">
            <li>Seleccione el año escolar, modalidad y grado para filtrar las secciones disponibles.</li>
            <li>Opcionalmente, seleccione una sección específica para filtrar aún más los resultados.</li>
            <li>Haga clic en "Generar PDF" para la sección deseada.</li>
            <li>El sistema generará un PDF con el resumen de calificaciones de todos los estudiantes de esa sección.</li>
          </ol>
          <div className="mt-4 p-4 bg-blue-50 dark:bg-blue-900/30 rounded-lg">
            <p className="text-sm text-blue-800 dark:text-blue-200">
              <strong>Nota:</strong> El acta de resumen incluye las calificaciones de todos los lapsos y la definitiva para cada asignatura.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
} 