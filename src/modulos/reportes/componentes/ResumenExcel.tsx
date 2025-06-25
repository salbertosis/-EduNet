import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { save } from '@tauri-apps/api/dialog';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { SelectorPeriodo } from '../../../componentes/SelectorPeriodo';
import { SelectorModalidad } from '../../../componentes/SelectorModalidad';
import { SelectorGrado } from '../../../componentes/SelectorGrado';
import { SelectorSeccion } from '../../../componentes/SelectorSeccion';

interface SeccionCompleta {
  id_seccion: number;
  nombre_seccion: string;
  id_grado_secciones?: number;
  idGradoSecciones?: number;
}

interface FiltrosResumenExcel {
  id_periodo: number | null;
  id_modalidad: number | null;
  id_grado: number | null;
  id_seccion: number | 'todas' | null;
  cedula: string;
  nombreCompleto: string;
}

interface RespuestaResumenBasico {
  exito: boolean;
  mensaje: string;
  archivo_generado?: string;
  estudiantes_procesados: number;
}

export default function ResumenExcel() {
  const { mostrarMensaje } = useMensajeGlobal();
  const [filtros, setFiltros] = useState<FiltrosResumenExcel>({
    id_periodo: null,
    id_modalidad: null,
    id_grado: null,
    id_seccion: null,
    cedula: '',
    nombreCompleto: ''
  });
  const [cargando, setCargando] = useState(false);
  const [grados, setGrados] = useState<any[]>([]);
  const [secciones, setSecciones] = useState<SeccionCompleta[]>([]);

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
    console.log('[ResumenExcel] filtros.id_grado:', filtros.id_grado);
    console.log('[ResumenExcel] filtros.id_modalidad:', filtros.id_modalidad);
    console.log('[ResumenExcel] filtros.id_periodo:', filtros.id_periodo);
    if (filtros.id_grado && filtros.id_modalidad && filtros.id_periodo) {
      console.log('[ResumenExcel] Llamando a obtener_secciones_por_grado_modalidad_periodo con:', {
        id_grado: filtros.id_grado,
        id_modalidad: filtros.id_modalidad,
        id_periodo: filtros.id_periodo
      });
      invoke<SeccionCompleta[]>("obtener_secciones_por_grado_modalidad_periodo", {
        id_grado: Number(filtros.id_grado),
        id_modalidad: Number(filtros.id_modalidad),
        id_periodo: Number(filtros.id_periodo)
      })
        .then((res) => {
          setSecciones(res);
          console.log('[ResumenExcel] Secciones cargadas:', res);
          // Mostrar los id_grado_secciones disponibles
          res.forEach(seccion => {
            console.log(`[ResumenExcel] Sección ${seccion.nombre_seccion}: id_grado_secciones = ${seccion.id_grado_secciones}`);
          });
        })
        .catch(() => setSecciones([]));
    }
  }, [filtros.id_grado, filtros.id_modalidad, filtros.id_periodo]);

  const generarResumenBasico = async () => {
    console.log('[ResumenBasico] === INICIANDO FUNCIÓN ===');
    console.log('[ResumenBasico] Filtros actuales:', filtros);
    console.log('[ResumenBasico] Secciones disponibles:', secciones);
    
    if (!filtros.id_periodo || !filtros.id_modalidad || !filtros.id_grado || !filtros.id_seccion || filtros.id_seccion === 'todas') {
      console.log('[ResumenBasico] ❌ Validación fallida - filtros incompletos');
      mostrarMensaje('Para el resumen básico, debe seleccionar una sección específica.', 'error');
      return;
    }

    try {
      setCargando(true);
      console.log('[ResumenBasico] 📁 Abriendo diálogo de guardado...');
      
      const rutaGuardado = await save({
        title: 'Guardar Resumen Básico de Estudiantes',
        defaultPath: `resumen_basico_estudiantes_${filtros.id_grado}.xlsx`,
        filters: [{
          name: 'Excel',
          extensions: ['xlsx']
        }]
      });

      console.log('[ResumenBasico] 📁 Ruta seleccionada:', rutaGuardado);

      if (!rutaGuardado) {
        console.log('[ResumenBasico] ❌ Usuario canceló el diálogo de guardado');
        setCargando(false);
        return;
      }

      // Buscar el id_grado_secciones de la sección seleccionada
      console.log('[ResumenBasico] 🔍 Buscando sección con id_seccion:', filtros.id_seccion);
      console.log('[ResumenBasico] 🔍 Tipo de id_seccion:', typeof filtros.id_seccion);
      console.log('[ResumenBasico] 🔍 Secciones disponibles para búsqueda:', secciones);
      
      const seccionSeleccionada = secciones.find(s => {
        console.log(`[ResumenBasico] 🔍 Comparando: s.id_seccion=${s.id_seccion} (${typeof s.id_seccion}) vs filtros.id_seccion=${filtros.id_seccion} (${typeof filtros.id_seccion})`);
        return s.id_seccion === filtros.id_seccion;
      });
      
      console.log('[ResumenBasico] 🔍 Sección encontrada:', seccionSeleccionada);
      
      if (!seccionSeleccionada) {
        console.log('[ResumenBasico] ❌ No se encontró la sección seleccionada');
        mostrarMensaje('Error: No se encontró la sección seleccionada.', 'error');
        setCargando(false);
        return;
      }

      const idGS = seccionSeleccionada.id_grado_secciones ?? (seccionSeleccionada as any).idGradoSecciones;
      console.log('[ResumenBasico] 🔍 id_grado_secciones encontrado:', idGS);
      console.log('[ResumenBasico] 🔍 Tipo de id_grado_secciones:', typeof idGS);

      console.log('[ResumenBasico] 📤 Enviando parámetros individuales al backend:');
      console.log('[ResumenBasico] 📤 id_grado_secciones:', idGS, typeof idGS);
      console.log('[ResumenBasico] 📤 ruta_salida:', rutaGuardado, typeof rutaGuardado);

      const respuesta = await invoke<RespuestaResumenBasico>('generar_resumen_estudiantes_basico', {
        idGradoSecciones: idGS,
        rutaSalida: rutaGuardado
      });

      console.log('[ResumenBasico] 📥 Respuesta del backend:', respuesta);

      if (respuesta.exito) {
        console.log('[ResumenBasico] ✅ Éxito:', respuesta.mensaje);
        mostrarMensaje(`¡Resumen básico generado! ${respuesta.estudiantes_procesados} estudiantes procesados.`, 'exito');
      } else {
        console.log('[ResumenBasico] ❌ Error del backend:', respuesta.mensaje);
        mostrarMensaje(`Error: ${respuesta.mensaje}`, 'error');
      }

    } catch (error) {
      console.error('[ResumenBasico] ❌ Excepción capturada:', error);
      console.error('[ResumenBasico] ❌ Error completo:', JSON.stringify(error, null, 2));
      mostrarMensaje('Error al generar el resumen básico: ' + error, 'error');
    } finally {
      console.log('[ResumenBasico] 🏁 Finalizando función, cambiando cargando a false');
      setCargando(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="bg-white dark:bg-dark-700 p-6 rounded-xl shadow-sm">
        <h3 className="text-lg font-semibold mb-4 text-blue-700 dark:text-blue-300">
          Generar Resumen Básico de Estudiantes (Excel)
        </h3>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Año Escolar
            </label>
            <SelectorPeriodo
              value={filtros.id_periodo}
              onChange={valor => setFiltros(f => ({
                ...f,
                id_periodo: valor,
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
              onChange={valor => setFiltros(f => ({
                ...f,
                id_modalidad: valor,
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
              onChange={valor => setFiltros(f => ({
                ...f,
                id_grado: valor ? Number(valor) : null,
                id_seccion: null
              }))}
              className="w-full"
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Sección <span className="text-red-500">*</span>
            </label>
            <SelectorSeccion
              value={
                filtros.id_seccion === null ? '' :
                filtros.id_seccion === 'todas' ? 'todas' : filtros.id_seccion.toString()
              }
              onChange={valor => setFiltros(f => ({
                ...f,
                id_seccion: valor === 'todas' ? 'todas' : (valor ? Number(valor) : null)
              }))}
              className="w-full"
              secciones={secciones}
              grado={typeof filtros.id_grado === 'number' ? filtros.id_grado.toString() : ''}
              modalidad={typeof filtros.id_modalidad === 'number' ? filtros.id_modalidad.toString() : ''}
              periodo={typeof filtros.id_periodo === 'number' ? filtros.id_periodo.toString() : ''}
            />
          </div>
        </div>

        <div className="flex justify-center">
          <button
            onClick={generarResumenBasico}
            disabled={cargando || !filtros.id_periodo || !filtros.id_modalidad || !filtros.id_grado || !filtros.id_seccion || filtros.id_seccion === 'todas'}
            className="px-8 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2 text-lg font-medium"
          >
            {cargando ? (
              <>
                <div className="animate-spin rounded-full h-5 w-5 border-2 border-white border-t-transparent"></div>
                Generando...
              </>
            ) : (
              <>
                📊 Generar Resumen de Estudiantes
              </>
            )}
          </button>
        </div>

        {/* Información de debug cuando hay secciones cargadas */}
        {secciones.length > 0 && (
          <div className="mt-4 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
            <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              🔍 Secciones disponibles:
            </h4>
            <div className="text-xs text-gray-600 dark:text-gray-400 space-y-1">
              {secciones.map(seccion => (
                <div key={seccion.id_seccion}>
                  Sección <strong>{seccion.nombre_seccion}</strong>: id_grado_secciones = {seccion.id_grado_secciones}
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      <div className="bg-blue-50 dark:bg-blue-900/20 p-6 rounded-lg">
        <h4 className="font-semibold text-blue-800 dark:text-blue-300 mb-3">📋 Información del Resumen</h4>
        <ol className="text-sm text-blue-700 dark:text-blue-300 space-y-2">
          <li>1. ✅ Selecciona todos los campos obligatorios (año escolar, modalidad, grado y sección)</li>
          <li>2. 👥 Genera Excel con datos básicos de estudiantes según parámetros oficiales MPPE</li>
          <li>3. 📊 Aplica formato oficial: Arial 10pt, sin enmiendas ni tachaduras</li>
          <li>4. 📍 Coloca datos en coordenadas exactas: Cédula (B), Apellidos (O), Nombres (X), etc.</li>
          <li>5. 💾 Guarda archivo Excel compatible con plantillas oficiales del ministerio</li>
        </ol>
        <div className="mt-4 p-3 bg-blue-100 dark:bg-blue-800/30 rounded">
          <p className="text-sm text-blue-800 dark:text-blue-300">
            <strong>Incluye:</strong> Cédula, apellidos, nombres, lugar de nacimiento, entidad federal, género, día/mes/año de nacimiento según los parámetros oficiales del MPPE Venezuela.
          </p>
        </div>
      </div>
    </div>
  );
} 