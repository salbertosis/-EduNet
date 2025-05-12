import { Users, GraduationCap, BookOpen, Award } from 'lucide-react';
import { Tarjeta } from '../../../componentes/ui/Tarjeta';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export function Dashboard() {
  const [totalEstudiantes, setTotalEstudiantes] = useState<number>(0);

  useEffect(() => {
    invoke<number>('contar_estudiantes')
      .then(setTotalEstudiantes)
      .catch(() => setTotalEstudiantes(0));
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
          valor={totalEstudiantes.toLocaleString()}
          descripcion="Estudiantes registrados en el sistema"
          icono={<Users className="w-6 h-6" />}
          color="primary"
        />
        <Tarjeta
          titulo="Total Profesores"
          valor="89"
          descripcion="+5% desde el mes pasado"
          icono={<GraduationCap className="w-6 h-6" />}
          color="success"
        />
        <Tarjeta
          titulo="Cursos Activos"
          valor="45"
          descripcion="+8% desde el mes pasado"
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