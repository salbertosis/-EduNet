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
      <header className="fixed top-0 left-0 right-0 h-16 bg-white dark:bg-dark-800 shadow-soft z-10">
        <div className="h-full px-4 flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <button
              onClick={() => setSidebarOpen(!sidebarOpen)}
              className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-dark-700 transition-colors"
            >
              <Menu className="w-6 h-6" />
            </button>
            <h1 className="text-xl font-bold text-gray-900 dark:text-gray-100">EduNet</h1>
          </div>
          <SelectorTema />
        </div>
      </header>

      <aside
        className={`fixed top-16 left-0 h-[calc(100vh-4rem)] w-64 bg-white dark:bg-dark-800 shadow-soft transition-transform duration-300 ease-in-out ${
          sidebarOpen ? 'translate-x-0' : '-translate-x-full'
        }`}
      >
        <nav className="p-4 space-y-2">
          {menuItems.map((item) => (
            <Link
              key={item.path}
              to={item.path}
              className={`flex items-center space-x-3 px-4 py-3 rounded-lg transition-colors ${
                location.pathname === item.path
                  ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-100'
                  : 'hover:bg-gray-100 dark:hover:bg-dark-700'
              }`}
            >
              {item.icon}
              <span>{item.label}</span>
            </Link>
          ))}
        </nav>
      </aside>

      <main className={`pt-16 transition-all duration-300 ease-in-out ${
        sidebarOpen ? 'ml-64' : 'ml-0'
      }`}>
        <div className="container mx-auto px-4 py-8">
          <Outlet />
        </div>
      </main>
    </div>
  );
} 