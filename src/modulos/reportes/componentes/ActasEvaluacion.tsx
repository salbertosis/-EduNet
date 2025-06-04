import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { SelectorPeriodo } from '../../../componentes/SelectorPeriodo';
import { SelectorModalidad } from '../../../componentes/SelectorModalidad';
import { SelectorGrado } from '../../../componentes/SelectorGrado';
import { SelectorSeccion } from '../../../componentes/SelectorSeccion';
import { SelectorAsignatura } from '../../../componentes/SelectorAsignatura';
import { SelectorLapso } from '../../../componentes/SelectorLapso';

import type { PeriodoEscolar } from '../../../componentes/SelectorPeriodo';
import type { Modalidad } from '../../../componentes/SelectorModalidad';

interface FiltrosPlantilla {
  id_periodo: number | null;
  id_modalidad: number | null;
  id_grado: number | null;
  id_seccion: number | null;
  id_asignatura: number | null;
  lapso: string;
}

export default function ActasEvaluacion() {
  const { mostrarMensaje } = useMensajeGlobal();
  const [filtros, setFiltros] = useState<FiltrosPlantilla>({
    id_periodo: null,
    id_modalidad: null,
    id_grado: null,
    id_seccion: null,
    id_asignatura: null,
    lapso: ''
  });
  const [cargando, setCargando] = useState(false);
  const [grados, setGrados] = useState<any[]>([]);
  const [secciones, setSecciones] = useState<any[]>([]);
  const [asignaturas, setAsignaturas] = useState<any[]>([]);

  // Cargar grados cuando cambian periodo o modalidad
  useEffect(() => {
    if (filtros.id_modalidad) {
      invoke<any[]>("obtener_grados_por_modalidad", { id_modalidad: filtros.id_modalidad })
        .then(setGrados)
        .catch(() => setGrados([]));
    }
  }, [filtros.id_modalidad]);

  // Cargar secciones cuando cambian grado, modalidad o periodo
  useEffect(() => {
    console.log('[ActasEvaluacion] filtros.id_grado:', filtros.id_grado);
    console.log('[ActasEvaluacion] filtros.id_modalidad:', filtros.id_modalidad);
    console.log('[ActasEvaluacion] filtros.id_periodo:', filtros.id_periodo);
    if (filtros.id_grado && filtros.id_modalidad && filtros.id_periodo) {
      console.log('[ActasEvaluacion] Llamando a obtener_secciones_por_grado_modalidad_periodo con:', {
        idGrado: filtros.id_grado,
        idModalidad: filtros.id_modalidad,
        idPeriodo: filtros.id_periodo
      });
      invoke<any[]>("obtener_secciones_por_grado_modalidad_periodo", {
        idGrado: filtros.id_grado,
        idModalidad: filtros.id_modalidad,
        idPeriodo: filtros.id_periodo
      })
        .then((res) => {
          setSecciones(res);
          console.log('[ActasEvaluacion] Secciones cargadas:', res);
        })
        .catch(() => setSecciones([]));
    }
  }, [filtros.id_grado, filtros.id_modalidad, filtros.id_periodo]);

  // Cargar asignaturas cuando cambian grado o modalidad
  useEffect(() => {
    if (filtros.id_grado && filtros.id_modalidad) {
      invoke<any[]>("obtener_asignaturas_por_grado_modalidad", {
        id_grado: filtros.id_grado,
        id_modalidad: filtros.id_modalidad
      })
        .then(setAsignaturas)
        .catch(() => setAsignaturas([]));
    }
  }, [filtros.id_grado, filtros.id_modalidad]);

  const handleFiltroChange = (campo: keyof FiltrosPlantilla, valor: any) => {
    setFiltros(prev => ({ ...prev, [campo]: valor }));
  };

  const generarPlantilla = async () => {
    console.log('=== INICIO GENERACIÓN PLANTILLA ===');
    console.log('Filtros actuales:', JSON.stringify(filtros, null, 2));

    if (!filtros.id_periodo || !filtros.id_modalidad || !filtros.id_grado || !filtros.id_seccion || !filtros.id_asignatura || !filtros.lapso) {
      console.log('Faltan campos requeridos:', {
        id_periodo: filtros.id_periodo,
        id_modalidad: filtros.id_modalidad,
        id_grado: filtros.id_grado,
        id_seccion: filtros.id_seccion,
        id_asignatura: filtros.id_asignatura,
        lapso: filtros.lapso
      });
      mostrarMensaje('Por favor, complete todos los campos', 'error');
      return;
    }

    try {
      setCargando(true);
      console.log('=== BUSCANDO DATOS DE REFERENCIA ===');

      // Buscar los nombres a partir de los ids seleccionados
      const gradoObj = grados.find(g => g.id_grado === Number(filtros.id_grado));
      console.log('Grado encontrado:', gradoObj);

      const asignaturaObj = asignaturas.find(a => a.id_asignatura === Number(filtros.id_asignatura));
      console.log('Asignatura encontrada:', asignaturaObj);

      const grado = gradoObj ? gradoObj.nombre_grado : '';
      const seccion = '';
      const asignatura = asignaturaObj ? asignaturaObj.nombre_asignatura : '';

      console.log('=== BUSCANDO SECCIÓN ===');
      console.log('Secciones disponibles:', JSON.stringify(secciones, null, 2));
      console.log('ID de sección seleccionada:', filtros.id_seccion);
      
      const seccionObj = secciones.find(s => s.id_seccion === Number(filtros.id_seccion));
      console.log('Sección encontrada:', JSON.stringify(seccionObj, null, 2));

      // Verificar la estructura de la sección
      if (seccionObj) {
        console.log('Propiedades de la sección:', Object.keys(seccionObj));
        console.log('id_grado_secciones:', seccionObj.id_grado_secciones);
        console.log('idGradoSecciones:', seccionObj.idGradoSecciones);
      }

      const idGradoSecciones = seccionObj
        ? seccionObj.id_grado_secciones ?? seccionObj.idGradoSecciones ?? null
        : null;

      console.log('idGradoSecciones final:', idGradoSecciones);

      if (!idGradoSecciones) {
        console.log('ERROR: No se pudo obtener idGradoSecciones');
        mostrarMensaje('Debes seleccionar una sección válida.', 'error');
        setCargando(false);
        return;
      }

      console.log('=== GENERANDO PLANTILLA ===');
      console.log('Parámetros para generar_plantilla_acta:', {
        idGradoSecciones: Number(idGradoSecciones),
        idPeriodo: Number(filtros.id_periodo),
        idAsignatura: Number(filtros.id_asignatura),
        lapso: filtros.lapso
      });

      const resultado = await invoke('generar_plantilla_acta', {
        idGradoSecciones: Number(idGradoSecciones),
        idPeriodo: Number(filtros.id_periodo),
        idAsignatura: Number(filtros.id_asignatura),
        lapso: filtros.lapso
      });

      console.log('Resultado de generar_plantilla_acta:', resultado);

      if (resultado) {
        console.log('Plantilla generada exitosamente');
        mostrarMensaje('Plantilla generada exitosamente', 'exito');
      }
    } catch (error) {
      console.error('Error al generar la plantilla:', error);
      mostrarMensaje('Error al generar la plantilla: ' + error, 'error');
    } finally {
      setCargando(false);
      console.log('=== FIN GENERACIÓN PLANTILLA ===');
    }
  };

  return (
    <div className="space-y-6">
      <div className="bg-white dark:bg-dark-700 p-6 rounded-xl shadow-sm">
        <h3 className="text-lg font-semibold mb-4 text-emerald-700 dark:text-emerald-300">
          Generar Plantilla de Acta de Evaluación
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Año Escolar
            </label>
            <SelectorPeriodo
              value={filtros.id_periodo !== null ? filtros.id_periodo.toString() : ''}
              onChange={valor => setFiltros(f => ({
                ...f,
                id_periodo: valor ? Number(valor) : null,
                id_grado: null,
                id_seccion: null,
                id_asignatura: null
              }))}
              className="w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Modalidad
            </label>
            <SelectorModalidad
              value={filtros.id_modalidad !== null ? filtros.id_modalidad.toString() : ''}
              onChange={valor => setFiltros(f => ({
                ...f,
                id_modalidad: valor ? Number(valor) : null,
                id_grado: null,
                id_seccion: null,
                id_asignatura: null
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
              onChange={valor => setFiltros(f => ({
                ...f,
                id_grado: valor ? Number(valor) : null,
                id_seccion: null,
                id_asignatura: null
              }))}
              className="w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Sección
            </label>
            <SelectorSeccion
              value={filtros.id_seccion !== null ? filtros.id_seccion.toString() : ''}
              onChange={valor => setFiltros(f => ({
                ...f,
                id_seccion: valor ? Number(valor) : null
              }))}
              className="w-full"
              secciones={secciones}
              grado={filtros.id_grado ? filtros.id_grado.toString() : ''}
              modalidad={filtros.id_modalidad ? filtros.id_modalidad.toString() : ''}
              periodo={filtros.id_periodo ? filtros.id_periodo.toString() : ''}
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Asignatura
            </label>
            <SelectorAsignatura
              value={filtros.id_asignatura !== null ? filtros.id_asignatura.toString() : ''}
              onChange={valor => setFiltros(f => ({
                ...f,
                id_asignatura: valor ? Number(valor) : null
              }))}
              className="w-full"
              grado={filtros.id_grado ? filtros.id_grado.toString() : ''}
              modalidad={filtros.id_modalidad ? filtros.id_modalidad.toString() : ''}
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Lapso
            </label>
            <SelectorLapso
              value={filtros.lapso}
              onChange={valor => handleFiltroChange('lapso', valor)}
              className="w-full"
            />
          </div>
        </div>
        <div className="flex justify-end">
          <button
            onClick={generarPlantilla}
            disabled={cargando}
            className="px-4 py-2 bg-gradient-to-r from-emerald-500 to-cyan-500 text-white rounded-lg hover:from-emerald-600 hover:to-cyan-600 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 flex items-center gap-2"
          >
            {cargando ? (
              <>
                <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Generando...
              </>
            ) : (
              <>
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                </svg>
                Descargar Plantilla
              </>
            )}
          </button>
        </div>
      </div>
      <div className="bg-white dark:bg-dark-700 p-6 rounded-xl shadow-sm">
        <h3 className="text-lg font-semibold mb-4 text-emerald-700 dark:text-emerald-300">
          Instrucciones
        </h3>
        <div className="prose dark:prose-invert max-w-none">
          <ol className="list-decimal list-inside space-y-2">
            <li>Seleccione el año escolar, modalidad, grado, sección, asignatura y lapso para el cual desea generar la plantilla.</li>
            <li>Haga clic en "Descargar Plantilla" para obtener el archivo Excel.</li>
            <li>Complete los datos de calificaciones en la plantilla descargada.</li>
            <li>Utilice la plantilla para cargar las calificaciones de manera masiva en el sistema.</li>
          </ol>
          <div className="mt-4 p-4 bg-yellow-50 dark:bg-yellow-900/30 rounded-lg">
            <p className="text-sm text-yellow-800 dark:text-yellow-200">
              <strong>Nota:</strong> La plantilla incluye validaciones automáticas para asegurar la integridad de los datos.
              Por favor, no modifique la estructura del archivo.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
} 