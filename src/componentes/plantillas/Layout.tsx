import { Outlet, Link, useLocation } from 'react-router-dom';
import { SelectorTema } from './SelectorTema';
import { 
  LayoutDashboard, 
  Users, 
  GraduationCap, 
  BookOpen, 
  Award,
  Menu
} from 'lucide-react';
import { useState } from 'react';

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
    label: 'Profesores', 
    path: '/profesores'
  },
  { 
    icon: <BookOpen className="w-5 h-5" />, 
    label: 'Cursos', 
    path: '/cursos'
  },
  { 
    icon: <Award className="w-5 h-5" />, 
    label: 'Calificaciones', 
    path: '/calificaciones'
  },
];

export function Layout() {
  const location = useLocation();
  const [sidebarOpen, setSidebarOpen] = useState(true);

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
    </div>
  );
} 