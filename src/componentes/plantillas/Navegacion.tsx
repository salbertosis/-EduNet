import { Link, useLocation } from 'react-router-dom';
import { RUTAS } from '../../configuracion/rutas';
import { useStore } from '../../estado/useStore';
import { cn } from '../../utilidades/cn';
import { 
  LayoutDashboard, 
  Users, 
  GraduationCap, 
  BookOpen, 
  Award,
  Moon,
  Sun,
  LogOut
} from 'lucide-react';
import { Button } from '../ui/button';

export function Navegacion() {
  const { sesion, cerrarSesion, tema, establecerTema } = useStore();
  const location = useLocation();

  if (!sesion) return null;

  const enlaces = [
    {
      ruta: RUTAS.DASHBOARD.PRINCIPAL,
      icono: <LayoutDashboard className="w-5 h-5" />,
      texto: 'Dashboard',
      visible: true
    },
    {
      ruta: RUTAS.ESTUDIANTES.LISTA,
      icono: <Users className="w-5 h-5" />,
      texto: 'Estudiantes',
      visible: sesion.usuario.role === 'ADMIN'
    },
    {
      ruta: RUTAS.DOCENTES.LISTA,
      icono: <GraduationCap className="w-5 h-5" />,
      texto: 'Docentes',
      visible: sesion.usuario.role === 'ADMIN'
    },
    {
      ruta: RUTAS.CURSOS.LISTA,
      icono: <BookOpen className="w-5 h-5" />,
      texto: 'Cursos',
      visible: true
    },
    {
      ruta: RUTAS.CALIFICACIONES.LISTA,
      icono: <Award className="w-5 h-5" />,
      texto: 'Calificaciones',
      visible: true
    }
  ];

  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 space-y-1 p-2">
        {enlaces.filter(enlace => enlace.visible).map((enlace) => (
          <Link
            key={enlace.ruta}
            to={enlace.ruta}
            className={cn(
              "flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-all hover:bg-accent",
              location.pathname === enlace.ruta
                ? "bg-accent text-accent-foreground"
                : "text-muted-foreground"
            )}
          >
            {enlace.icono}
            {enlace.texto}
          </Link>
        ))}
      </div>

      <div className="mt-auto p-4 space-y-2">
        <Button
          variant="ghost"
          size="sm"
          className="w-full justify-start"
          onClick={() => establecerTema(tema === 'claro' ? 'oscuro' : 'claro')}
        >
          {tema === 'claro' ? (
            <Moon className="w-5 h-5 mr-2" />
          ) : (
            <Sun className="w-5 h-5 mr-2" />
          )}
          {tema === 'claro' ? 'Modo Oscuro' : 'Modo Claro'}
        </Button>

        <Button
          variant="ghost"
          size="sm"
          className="w-full justify-start text-destructive hover:text-destructive"
          onClick={cerrarSesion}
        >
          <LogOut className="w-5 h-5 mr-2" />
          Cerrar Sesión
        </Button>
      </div>
    </div>
  );
} 