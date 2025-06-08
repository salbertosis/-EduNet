import { Users, GraduationCap, BookOpen, Award, Cpu, Beaker, User } from 'lucide-react';
import { Tarjeta } from '../../../componentes/ui/Tarjeta';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export function Dashboard() {
  const [totalEstudiantes, setTotalEstudiantes] = useState<number>(0);
  const [totalDocentes, setTotalDocentes] = useState<number>(0);
  const [totalCursos, setTotalCursos] = useState<number>(0);
  const [cursosTelematica, setCursosTelematica] = useState<number>(0);
  const [cursosCiencias, setCursosCiencias] = useState<number>(0);
  const [totalEstudiantesF, setTotalEstudiantesF] = useState<number>(0);
  const [totalEstudiantesM, setTotalEstudiantesM] = useState<number>(0);
  const [cargandoEstudiantes, setCargandoEstudiantes] = useState(true);
  const [cargandoDocentes, setCargandoDocentes] = useState(true);
  const [cargandoCursos, setCargandoCursos] = useState(true);

  useEffect(() => {
    setCargandoEstudiantes(true);
    setCargandoDocentes(true);
    setCargandoCursos(true);
    invoke<number>('contar_estudiantes')
      .then(setTotalEstudiantes)
      .catch(() => setTotalEstudiantes(0))
      .finally(() => setCargandoEstudiantes(false));
    invoke<number>('contar_docentes')
      .then(setTotalDocentes)
      .catch(() => setTotalDocentes(0))
      .finally(() => setCargandoDocentes(false));
    invoke<number>('contar_estudiantes_femeninos')
      .then(setTotalEstudiantesF)
      .catch(() => setTotalEstudiantesF(0));
    invoke<number>('contar_estudiantes_masculinos')
      .then(setTotalEstudiantesM)
      .catch(() => setTotalEstudiantesM(0));
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
      <div>
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <p className="text-gray-600 dark:text-gray-400">Bienvenido al panel de control de EduNet</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Tarjeta
          titulo="Total de Estudiantes"
          valor={cargandoEstudiantes ? <span className="animate-pulse text-gray-400">Cargando…</span> : totalEstudiantes.toLocaleString()}
          descripcion={cargandoEstudiantes ? null : (
            <div className="flex flex-col items-start gap-1 mt-2">
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
            </div>
          )}
          icono={<Users className="w-6 h-6" />}
          color="primary"
        />
        <Tarjeta
          titulo="Total Docentes"
          valor={cargandoDocentes ? <span className="animate-pulse text-gray-400">Cargando…</span> : totalDocentes.toLocaleString()}
          descripcion="+5% desde el mes pasado"
          icono={<GraduationCap className="w-6 h-6" />}
          color="success"
        />
        <Tarjeta
          titulo="Cursos Activos"
          valor={cargandoCursos ? <span className="animate-pulse text-gray-400">Cargando…</span> : totalCursos.toLocaleString()}
          descripcion={cargandoCursos ? null : (
            <div className="flex flex-col items-start gap-1 mt-2">
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
        <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6">
          <h2 className="text-lg font-semibold mb-4">Actividad Reciente</h2>
          <div className="space-y-4">
            {[1, 2, 3].map((i) => (
              <div key={i} className="flex items-center space-x-4">
                <div className="w-2 h-2 rounded-full bg-primary-500" />
                <div>
                  <p className="font-medium">Nueva calificación registrada</p>
                  <p className="text-sm text-gray-500 dark:text-gray-400">Hace {i} horas</p>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6">
          <h2 className="text-lg font-semibold mb-4">Próximos Eventos</h2>
          <div className="space-y-4">
            {[1, 2, 3].map((i) => (
              <div key={i} className="flex items-center space-x-4">
                <div className="w-2 h-2 rounded-full bg-yellow-500" />
                <div>
                  <p className="font-medium">Examen Final - Matemáticas</p>
                  <p className="text-sm text-gray-500 dark:text-gray-400">En {i} días</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
} 