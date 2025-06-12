import { Outlet, Link, useLocation } from 'react-router-dom';
import { SelectorTema } from './SelectorTema';
import { 
  LayoutDashboard, 
  Users, 
  GraduationCap, 
  BookOpen, 
  Award,
  Menu,
  Settings,
  BarChart3
} from 'lucide-react';
import { useState } from 'react';
import { ModalMensaje } from '../ModalMensaje';

export function Layout() {
  const location = useLocation();
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [mostrarConfig, setMostrarConfig] = useState(false);
  const [mostrarModalGuardarHistorial, setMostrarModalGuardarHistorial] = useState(false);

  // Definir menuItems dentro del componente para acceder a los estados
const menuItems = [
  { 
    icon: <LayoutDashboard className="w-5 h-5" />, 
    label: 'Dashboard', 
    path: '/dashboard'
  },
  { 
    icon: <Users className="w-5 h-5" />, 
    label: 'Estudiantes', 
    path: '/estudiantes'
  },
  { 
    icon: <GraduationCap className="w-5 h-5" />, 
    label: 'Docentes', 
    path: '/docentes'
  },
  { 
    icon: <Award className="w-5 h-5" />, 
    label: 'Calificaciones', 
    path: '/calificaciones'
  },
  { 
    icon: <BarChart3 className="w-5 h-5" />, 
    label: 'Reportes', 
    path: '/reportes'
  },
  { 
    icon: <BookOpen className="w-5 h-5" />, 
    label: 'Cursos', 
    path: '/cursos'
  },
  {
    icon: <Settings className="w-5 h-5" />, 
    label: 'Configuración',
    children: [
      {
        label: 'Nuevo año escolar',
        path: '/nuevo-periodo'
      },
      {
        label: 'Migrar Estudiantes',
        path: '/migrar-estudiantes'
      },
      {
        label: 'Guardar Historial',
        onClick: () => setMostrarModalGuardarHistorial(true)
      }
    ]
  },
];

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-dark-900">
      <header className="fixed top-0 left-0 right-0 h-20 bg-white dark:bg-gradient-to-r dark:from-dark-900 dark:via-dark-800 dark:to-emerald-900 dark:shadow-emerald-800/40 shadow-lg z-20 border-b-4 border-emerald-500">
        <div className="h-full px-8 flex items-center justify-between">
          <div className="flex items-center space-x-6">
            <button
              onClick={() => setSidebarOpen(!sidebarOpen)}
              className="p-3 rounded-xl bg-gray-100 dark:bg-dark-700 hover:bg-emerald-800/40 transition-colors border border-gray-300 dark:border-dark-600 shadow-md shadow-emerald-900/10"
            >
              <Menu className="w-7 h-7 text-emerald-600 dark:text-emerald-400" />
            </button>
            <h1 className="text-3xl font-extrabold tracking-tight bg-gradient-to-r from-emerald-500 via-cyan-500 to-blue-500 bg-clip-text text-transparent drop-shadow-lg select-none dark:from-emerald-400 dark:via-cyan-400 dark:to-blue-400">
              EduNet
            </h1>
          </div>
          <SelectorTema />
        </div>
      </header>

      <aside
        className={`fixed top-20 left-0 h-[calc(100vh-5rem)] w-64 bg-white dark:bg-gradient-to-b dark:from-dark-900 dark:via-dark-800 dark:to-dark-700 dark:shadow-emerald-900/20 shadow-lg transition-transform duration-300 ease-in-out ${
          sidebarOpen ? 'translate-x-0' : '-translate-x-full'
        }`}
      >
        <nav className="p-6 space-y-3">
          {menuItems.map((item) => (
            item.children ? (
              <div key={item.label}>
                <button
                  className={`flex items-center space-x-4 px-5 py-3 rounded-xl transition-all font-semibold text-lg tracking-wide shadow-sm hover:bg-emerald-100 dark:hover:bg-emerald-900/30 hover:text-emerald-700 dark:hover:text-emerald-300 focus:bg-emerald-100 dark:focus:bg-emerald-900/40 focus:text-emerald-800 dark:focus:text-emerald-200 outline-none w-full ${mostrarConfig ? 'bg-emerald-100 dark:bg-emerald-900/60 text-emerald-700 dark:text-emerald-300' : 'text-gray-700 dark:text-gray-300'}`}
                  onClick={() => setMostrarConfig((v) => !v)}
                >
                  {item.icon}
                  <span>{item.label}</span>
                  <span className="ml-auto">{mostrarConfig ? '▲' : '▼'}</span>
                </button>
                {mostrarConfig && (
                  <div className="ml-8 mt-2 space-y-2">
                    {item.children.map((child) => (
                      child.path ? (
                        <Link
                          key={child.label}
                          to={child.path}
                          className="flex items-center space-x-2 px-3 py-2 rounded-lg text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-emerald-50 dark:hover:bg-emerald-900/30 w-full text-left"
                        >
                          <span>{child.label}</span>
                        </Link>
                      ) : (
                        <button
                          key={child.label}
                          className="flex items-center space-x-2 px-3 py-2 rounded-lg text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-emerald-50 dark:hover:bg-emerald-900/30 w-full text-left"
                          onClick={child.onClick}
                        >
                          <span>{child.label}</span>
                        </button>
                      )
                    ))}
                  </div>
                )}
              </div>
            ) : (
            <Link
              key={item.path}
              to={item.path}
                className={`flex items-center space-x-4 px-5 py-3 rounded-xl transition-all font-semibold text-lg tracking-wide shadow-sm hover:bg-emerald-100 dark:hover:bg-emerald-900/30 hover:text-emerald-700 dark:hover:text-emerald-300 focus:bg-emerald-100 dark:focus:bg-emerald-900/40 focus:text-emerald-800 dark:focus:text-emerald-200 outline-none ${
                location.pathname === item.path
                    ? 'bg-emerald-100 dark:bg-emerald-900/60 text-emerald-700 dark:text-emerald-300 shadow-emerald-800/30'
                    : 'text-gray-700 dark:text-gray-300'
              }`}
            >
              {item.icon}
              <span>{item.label}</span>
            </Link>
            )
          ))}
        </nav>
      </aside>

      <main className={`pt-20 transition-all duration-300 ease-in-out ${
        sidebarOpen ? 'ml-64' : 'ml-0'
      }`}>
        <div className="container mx-auto px-4 py-8">
          <Outlet />
        </div>
      </main>

      {/* Modal de confirmación para Guardar Historial */}
      {mostrarModalGuardarHistorial && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-40">
          <div className="bg-white dark:bg-dark-800 p-8 rounded-xl shadow-lg max-w-md w-full border border-emerald-300">
            <h2 className="text-xl font-bold mb-4 text-emerald-700 dark:text-emerald-300">Guardar Historial Académico</h2>
            <p className="mb-6 text-gray-700 dark:text-gray-200">¿Está seguro que desea guardar el historial académico de todos los estudiantes para el periodo actual? <b>Esta acción no se puede deshacer.</b></p>
            <div className="flex justify-end gap-4">
              <button
                className="px-5 py-2 rounded-lg bg-gray-200 dark:bg-dark-700 text-gray-700 dark:text-gray-200 hover:bg-gray-300 dark:hover:bg-dark-600 font-medium"
                onClick={() => setMostrarModalGuardarHistorial(false)}
              >
                Cancelar
              </button>
              <button
                className="px-5 py-2 rounded-lg bg-emerald-600 text-white hover:bg-emerald-700 font-semibold shadow"
                onClick={() => setMostrarModalGuardarHistorial(false)}
              >
                Guardar
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
} 