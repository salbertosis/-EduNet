import { Users, GraduationCap, BookOpen, Award, Cpu, Beaker, User, Plus, FileSpreadsheet, BarChart3, Calculator, LayoutDashboard } from 'lucide-react';
import { Tarjeta } from '../../../componentes/ui/Tarjeta';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { ActividadReciente } from '../componentes/ActividadReciente';
import { ProximosEventos } from '../componentes/ProximosEventos';
import { useActividadReciente } from '../../../hooks/useActividadReciente';
import { Link } from 'react-router-dom';

export function Dashboard() {
  const [totalEstudiantes, setTotalEstudiantes] = useState<number>(0);
  const [totalDocentes, setTotalDocentes] = useState<number>(0);
  const [totalCursos, setTotalCursos] = useState<number>(0);
  const [cursosTelematica, setCursosTelematica] = useState<number>(0);
  const [cursosCiencias, setCursosCiencias] = useState<number>(0);
  const [totalEstudiantesF, setTotalEstudiantesF] = useState<number>(0);
  const [totalEstudiantesM, setTotalEstudiantesM] = useState<number>(0);
  const [estadisticasTelematica, setEstadisticasTelematica] = useState<any>(null);
  const [estadisticasCiencias, setEstadisticasCiencias] = useState<any>(null);
  const [cursosTelematicaPorGrado, setCursosTelematicaPorGrado] = useState<any[]>([]);
  const [cursosCienciasPorGrado, setCursosCienciasPorGrado] = useState<any[]>([]);
  const [cargandoEstudiantes, setCargandoEstudiantes] = useState(true);
  const [cargandoDocentes, setCargandoDocentes] = useState(true);
  const [cargandoCursos, setCargandoCursos] = useState(true);
  const [cargandoEstadisticasModalidad, setCargandoEstadisticasModalidad] = useState(true);
  const [cargandoEstadisticasCursos, setCargandoEstadisticasCursos] = useState(true);
  const [estadisticasCursosCompacta, setEstadisticasCursosCompacta] = useState<any[]>([]);
  const [estadisticasCursosResumen, setEstadisticasCursosResumen] = useState<any>(null);
  const [estadisticasDocentes, setEstadisticasDocentes] = useState<any>(null);

  // Hook para actividad reciente
  const { actividades, cargando: cargandoActividades, obtenerActividades } = useActividadReciente();

  // Función auxiliar para obtener el nombre corto del grado
  const obtenerNombreCortoGrado = (nombreGrado: string) => {
    const match = nombreGrado.match(/(\d+)/);
    if (match) {
      const numero = parseInt(match[1]);
      const sufijo = numero === 1 ? 'er' : numero === 2 ? 'do' : numero === 3 ? 'er' : numero === 4 ? 'to' : numero === 5 ? 'to' : '';
      return `${numero}${sufijo} año`;
    }
    return nombreGrado;
  };

  // Datos de ejemplo para próximos eventos
  const proximosEventos = [
    {
      id: 1,
      titulo: 'Examen Final - Matemáticas',
      descripcion: '3er año, Sección A',
      tipo: 'examen' as const,
      fecha_evento: new Date(Date.now() + 1000 * 60 * 60 * 24).toISOString(), // mañana
      prioridad: 'alta' as const
    },
    {
      id: 2,
      titulo: 'Fecha límite calificaciones',
      descripcion: 'Cierre del 2do lapso',
      tipo: 'fecha_limite' as const,
      fecha_evento: new Date(Date.now() + 1000 * 60 * 60 * 24 * 3).toISOString(), // 3 días
      prioridad: 'media' as const
    },
    {
      id: 3,
      titulo: 'Reunión de padres',
      descripcion: 'Entrega de boletines',
      tipo: 'evento_escolar' as const,
      fecha_evento: new Date(Date.now() + 1000 * 60 * 60 * 24 * 7).toISOString(), // 7 días
      prioridad: 'baja' as const
    }
  ];

  useEffect(() => {
    // Cargar actividades recientes
    obtenerActividades(10);
    
    setCargandoEstudiantes(true);
    setCargandoDocentes(true);
    setCargandoCursos(true);
    setCargandoEstadisticasModalidad(true);
    setCargandoEstadisticasCursos(true);
    
    invoke<number>('contar_estudiantes')
      .then(setTotalEstudiantes)
      .catch(() => setTotalEstudiantes(0))
      .finally(() => setCargandoEstudiantes(false));
    invoke<number>('contar_docentes')
      .then(setTotalDocentes)
      .catch(() => setTotalDocentes(0));
    
    invoke<any>('obtener_estadisticas_docentes')
      .then(setEstadisticasDocentes)
      .catch(() => setEstadisticasDocentes(null))
      .finally(() => setCargandoDocentes(false));
    invoke<number>('contar_estudiantes_femeninos')
      .then(setTotalEstudiantesF)
      .catch(() => setTotalEstudiantesF(0));
    invoke<number>('contar_estudiantes_masculinos')
      .then(setTotalEstudiantesM)
      .catch(() => setTotalEstudiantesM(0));
    
    // Cargar estadísticas por modalidad
    invoke<any>('obtener_estadisticas_telematica')
      .then(setEstadisticasTelematica)
      .catch(() => setEstadisticasTelematica(null));
    invoke<any>('obtener_estadisticas_ciencias')
      .then(setEstadisticasCiencias)
      .catch(() => setEstadisticasCiencias(null))
      .finally(() => setCargandoEstadisticasModalidad(false));
    
    // Cargar estadísticas simples de cursos
    invoke<any>('obtener_estadisticas_cursos_simple')
      .then((datos) => {
        console.log('[DEBUG] Estadísticas simples recibidas:', datos);
        setEstadisticasCursosResumen(datos);
      })
      .catch((error) => {
        console.error('[DEBUG] Error cargando estadísticas simples:', error);
        setEstadisticasCursosResumen(null);
      })
      .finally(() => setCargandoEstadisticasCursos(false));
    
    invoke<any[]>('obtener_tarjetas_cursos')
      .then((cursos) => {
        if (Array.isArray(cursos)) {
          setTotalCursos(cursos.length);
          setCursosTelematica(cursos.filter(c => c.nombre_modalidad && c.nombre_modalidad.toLowerCase().includes('tele')).length);
          setCursosCiencias(cursos.filter(c => c.nombre_modalidad && c.nombre_modalidad.toLowerCase().includes('ciencia')).length);
        } else {
          setTotalCursos(0);
          setCursosTelematica(0);
          setCursosCiencias(0);
        }
      })
      .catch(() => {
        setTotalCursos(0);
        setCursosTelematica(0);
        setCursosCiencias(0);
      })
      .finally(() => setCargandoCursos(false));
  }, []);



  return (
    <div className="space-y-6">
      {/* Header elegante con gradiente */}
      <div className="bg-gradient-to-r from-emerald-600 to-teal-600 text-white p-6 rounded-2xl">
        <div className="max-w-7xl mx-auto">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-4">
              <div className="bg-white/20 p-3 rounded-full">
                <LayoutDashboard className="w-8 h-8" />
              </div>
              <div>
                <h1 className="text-3xl font-bold">Inicio</h1>
                <p className="text-emerald-100">Bienvenido al panel de control de EduNet</p>
              </div>
            </div>
            <div className="flex space-x-3">
              <Link
                to="/estudiantes"
                className="bg-white/20 hover:bg-white/30 text-white px-4 py-2 rounded-lg flex items-center space-x-2 transition-all duration-200"
              >
                <Plus className="w-4 h-4" />
                <span>Estudiante</span>
              </Link>
              <Link
                to="/docentes"
                className="bg-white/20 hover:bg-white/30 text-white px-4 py-2 rounded-lg flex items-center space-x-2 transition-all duration-200"
              >
                <GraduationCap className="w-4 h-4" />
                <span>Docente</span>
              </Link>
                             <Link
                 to="/calificaciones"
                 className="bg-white/20 hover:bg-white/30 text-white px-4 py-2 rounded-lg flex items-center space-x-2 transition-all duration-200"
               >
                 <Calculator className="w-4 h-4" />
                 <span>Calificaciones</span>
               </Link>
               <Link
                 to="/reportes"
                 className="bg-white/20 hover:bg-white/30 text-white px-4 py-2 rounded-lg flex items-center space-x-2 transition-all duration-200"
               >
                 <BarChart3 className="w-4 h-4" />
                 <span>Reporte</span>
               </Link>
            </div>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Tarjeta
          titulo="Total de Estudiantes"
          valor={cargandoEstudiantes ? <span className="animate-pulse text-gray-400">Cargando…</span> : totalEstudiantes.toLocaleString()}
          descripcion={cargandoEstudiantes ? null : (
            <div className="flex flex-col items-start gap-1 mt-2">
              {/* Total general por sexo */}
              <span className="flex items-center gap-2">
                <User className="w-5 h-5 text-pink-400 dark:text-pink-300" />
                <span className="font-bold text-lg text-pink-200 dark:text-pink-100">{totalEstudiantesF}</span>
                <span className="ml-1 text-base text-gray-500 dark:text-gray-300">Femeninas</span>
              </span>
              <span className="flex items-center gap-2">
                <User className="w-5 h-5 text-blue-400 dark:text-blue-300" />
                <span className="font-bold text-lg text-blue-200 dark:text-blue-100">{totalEstudiantesM}</span>
                <span className="ml-1 text-base text-gray-500 dark:text-gray-300">Masculinos</span>
              </span>
              
              {/* Separador */}
              <div className="w-full border-t border-gray-200 dark:border-gray-700 my-2"></div>
              
                             {/* Estadísticas por modalidad */}
               <div className="w-full">
                 <div className="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-0.5">
                   Por Modalidad:
                 </div>
                 
                 {/* Telemática */}
                 <div className="mb-1">
                   <div className="flex items-center gap-1 mb-0.5">
                     <Cpu className="w-3 h-3 text-cyan-400 dark:text-cyan-300" />
                     <span className="text-xs font-semibold text-cyan-600 dark:text-cyan-400">
                       Telemática: {cargandoEstadisticasModalidad ? '...' : (estadisticasTelematica?.total_estudiantes || 0)}
                     </span>
                   </div>
                   <div className="flex items-center gap-1 ml-3">
                     <User className="w-2.5 h-2.5 text-pink-400 dark:text-pink-300" />
                     <span className="text-xs text-pink-600 dark:text-pink-400">
                       {cargandoEstadisticasModalidad ? '...' : (estadisticasTelematica?.estudiantes_femeninos || 0)} F
                     </span>
                     <span className="text-xs text-gray-500 dark:text-gray-400">|</span>
                     <User className="w-2.5 h-2.5 text-blue-400 dark:text-blue-300" />
                     <span className="text-xs text-blue-600 dark:text-blue-400">
                       {cargandoEstadisticasModalidad ? '...' : (estadisticasTelematica?.estudiantes_masculinos || 0)} M
                     </span>
                   </div>
                 </div>
                 
                 {/* Ciencias */}
                 <div>
                   <div className="flex items-center gap-1 mb-0.5">
                     <Beaker className="w-3 h-3 text-emerald-400 dark:text-emerald-300" />
                     <span className="text-xs font-semibold text-emerald-600 dark:text-emerald-400">
                       Ciencias: {cargandoEstadisticasModalidad ? '...' : (estadisticasCiencias?.total_estudiantes || 0)}
                     </span>
                   </div>
                   <div className="flex items-center gap-1 ml-3">
                     <User className="w-2.5 h-2.5 text-pink-400 dark:text-pink-300" />
                     <span className="text-xs text-pink-600 dark:text-pink-400">
                       {cargandoEstadisticasModalidad ? '...' : (estadisticasCiencias?.estudiantes_femeninos || 0)} F
                     </span>
                     <span className="text-xs text-gray-500 dark:text-gray-400">|</span>
                     <User className="w-2.5 h-2.5 text-blue-400 dark:text-blue-300" />
                     <span className="text-xs text-blue-600 dark:text-blue-400">
                       {cargandoEstadisticasModalidad ? '...' : (estadisticasCiencias?.estudiantes_masculinos || 0)} M
                     </span>
                   </div>
                 </div>
               </div>
            </div>
          )}
          icono={<Users className="w-6 h-6" />}
          color="primary"
        />
        <Tarjeta
          titulo="Total Docentes"
          valor={cargandoDocentes ? <span className="animate-pulse text-gray-400">Cargando…</span> : totalDocentes.toLocaleString()}
          descripcion={cargandoDocentes ? null : (
            <div className="flex flex-col items-start gap-1 mt-2">
              <div className="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-1">
                Docentes Guías:
              </div>
              <div className="flex items-center gap-1.5">
                <GraduationCap className="w-3.5 h-3.5 text-green-400 dark:text-green-300" />
                <span className="font-semibold text-sm text-green-600 dark:text-green-400">
                  {estadisticasDocentes?.asignados || 0} asignados
                </span>
              </div>
              <div className="flex items-center gap-1.5">
                <GraduationCap className="w-3.5 h-3.5 text-orange-400 dark:text-orange-300" />
                <span className="font-semibold text-sm text-orange-600 dark:text-orange-400">
                  {estadisticasDocentes?.sin_asignar || 0} por asignar
                </span>
              </div>
              {estadisticasDocentes?.especialidades && estadisticasDocentes.especialidades.length > 0 && (
                <div className="mt-2 pt-1.5 border-t border-gray-200 dark:border-gray-700">
                  <div className="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-1">
                    Top especialidades:
                  </div>
                  <div className="space-y-0.5">
                    {estadisticasDocentes.especialidades.map((esp: any, index: number) => {
                      const colores = [
                        'text-blue-600 dark:text-blue-400',
                        'text-green-600 dark:text-green-400', 
                        'text-purple-600 dark:text-purple-400',
                        'text-orange-600 dark:text-orange-400',
                        'text-pink-600 dark:text-pink-400',
                        'text-indigo-600 dark:text-indigo-400'
                      ];
                      const color = colores[index % colores.length];
                      
                      return (
                        <div key={index} className="flex items-center justify-between text-xs">
                          <span className={`font-medium truncate pr-2 ${color}`}>
                            {esp.especialidad}
                          </span>
                          <span className={`font-semibold text-xs ${color}`}>
                            {esp.cantidad}
                          </span>
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}
            </div>
          )}
          icono={<GraduationCap className="w-6 h-6" />}
          color="success"
        />
        <Tarjeta
          titulo="Cursos Activos"
          valor={cargandoCursos ? <span className="animate-pulse text-gray-400">Cargando…</span> : totalCursos.toLocaleString()}
          descripcion={cargandoCursos ? null : (
            <div className="flex flex-col items-start gap-1 mt-2">
              {/* Total por modalidad */}
              <span className="flex items-center gap-2">
                <Cpu className="w-5 h-5 text-cyan-400 dark:text-cyan-300" />
                <span className="font-bold text-lg text-cyan-200 dark:text-cyan-100">{cursosTelematica}</span>
                <span className="ml-1 text-base text-gray-500 dark:text-gray-300">Telemática</span>
              </span>
              <span className="flex items-center gap-2">
                <Beaker className="w-5 h-5 text-emerald-400 dark:text-emerald-300" />
                <span className="font-bold text-lg text-emerald-200 dark:text-emerald-100">{cursosCiencias}</span>
                <span className="ml-1 text-base text-gray-500 dark:text-gray-300">Ciencias</span>
              </span>
              
              {/* Separador */}
              <div className="w-full border-t border-gray-200 dark:border-gray-700 my-2"></div>
              
                             {/* Estadísticas resumen */}
               <div className="w-full">
                 <div className="text-xs font-semibold text-gray-600 dark:text-gray-400 mb-0.5">
                   Estadísticas:
                 </div>
                 
                 {cargandoEstadisticasCursos ? (
                   <div className="text-xs text-gray-500 dark:text-gray-400">Cargando...</div>
                 ) : estadisticasCursosResumen ? (
                   <div className="space-y-0.5">
                     <div className="flex items-center justify-between text-xs">
                       <span className="text-gray-600 dark:text-gray-400">Mayor:</span>
                       <span className="font-semibold text-green-600 dark:text-green-400">
                         {estadisticasCursosResumen.seccion_mayor_matricula} ({estadisticasCursosResumen.estudiantes_mayor})
                       </span>
                     </div>
                     <div className="flex items-center justify-between text-xs">
                       <span className="text-gray-600 dark:text-gray-400">Menor:</span>
                       <span className="font-semibold text-orange-600 dark:text-orange-400">
                         {estadisticasCursosResumen.seccion_menor_matricula} ({estadisticasCursosResumen.estudiantes_menor})
                       </span>
                     </div>
                     <div className="flex items-center justify-between text-xs">
                       <span className="text-gray-600 dark:text-gray-400">Promedio:</span>
                       <span className="font-semibold text-blue-600 dark:text-blue-400">
                         {estadisticasCursosResumen.promedio_estudiantes_por_seccion.toFixed(1)}
                       </span>
                     </div>
                   </div>
                 ) : (
                   <div className="text-xs text-gray-500 dark:text-gray-400">Sin datos</div>
                 )}
               </div>
            </div>
          )}
          icono={<BookOpen className="w-6 h-6" />}
          color="warning"
        />
        <Tarjeta
          titulo="Promedio General"
          valor="8.5"
          descripcion="+0.3 desde el mes pasado"
          icono={<Award className="w-6 h-6" />}
          color="danger"
        />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Actividad Reciente */}
        <ActividadReciente 
          actividades={actividades.map(act => ({
            ...act,
            tipo: act.tipo_actividad || act.tipo || 'general'
          }))}
          cargando={cargandoActividades}
        />

        {/* Próximos Eventos */}
        <ProximosEventos 
          eventos={proximosEventos}
          cargando={false}
        />
      </div>
    </div>
  );
} 