import { Clock, User, BookOpen, Award, GraduationCap, Settings } from 'lucide-react';

interface Actividad {
  id: number;
  tipo: string;
  descripcion: string;
  timestamp: string;
  usuario?: string;
  metadata?: any;
}

interface ActividadRecienteProps {
  actividades: Actividad[];
  cargando?: boolean;
}

export function ActividadReciente({ actividades, cargando = false }: ActividadRecienteProps) {
  const getIcono = (tipo: string) => {
    switch (tipo) {
      case 'calificacion': return <Award className="w-4 h-4" />;
      case 'estudiante': return <User className="w-4 h-4" />;
      case 'docente': return <GraduationCap className="w-4 h-4" />;
      case 'periodo': return <BookOpen className="w-4 h-4" />;
      case 'configuracion': return <Settings className="w-4 h-4" />;
      default: return <Clock className="w-4 h-4" />;
    }
  };

  const getColor = (tipo: string) => {
    switch (tipo) {
      case 'calificacion': return 'bg-green-500';
      case 'estudiante': return 'bg-blue-500';
      case 'docente': return 'bg-purple-500';
      case 'periodo': return 'bg-orange-500';
      case 'configuracion': return 'bg-gray-500';
      default: return 'bg-primary-500';
    }
  };

  const formatearTiempo = (timestamp: string) => {
    const ahora = new Date();
    const fecha = new Date(timestamp);
    const diffMs = ahora.getTime() - fecha.getTime();
    const diffMin = Math.floor(diffMs / (1000 * 60));
    const diffHoras = Math.floor(diffMs / (1000 * 60 * 60));
    const diffDias = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffMin < 1) return 'Hace un momento';
    if (diffMin < 60) return `Hace ${diffMin} min`;
    if (diffHoras < 24) return `Hace ${diffHoras} horas`;
    if (diffDias < 7) return `Hace ${diffDias} días`;
    return fecha.toLocaleDateString();
  };

  if (cargando) {
    return (
      <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6">
        <h2 className="text-lg font-semibold mb-4 flex items-center gap-2">
          <Clock className="w-5 h-5" />
          Actividad Reciente
        </h2>
        <div className="space-y-4">
          {[1, 2, 3].map((i) => (
            <div key={i} className="animate-pulse">
              <div className="flex items-center space-x-3">
                <div className="w-8 h-8 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
                <div className="flex-1">
                  <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4 mb-2"></div>
                  <div className="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6">
      <h2 className="text-lg font-semibold mb-4 flex items-center gap-2">
        <Clock className="w-5 h-5" />
        Actividad Reciente
        {actividades.length > 0 && (
          <span className="ml-auto text-sm text-gray-500 dark:text-gray-400">
            {actividades.length} actividades
          </span>
        )}
      </h2>
      
      <div className="space-y-4">
        {actividades.length === 0 ? (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            <Clock className="w-12 h-12 mx-auto mb-3 opacity-50" />
            <p>No hay actividad reciente</p>
          </div>
        ) : (
          actividades.slice(0, 3).map((actividad) => (
            <div 
              key={actividad.id} 
              className="flex items-start space-x-3 p-3 rounded-lg hover:bg-gray-50 dark:hover:bg-dark-700 transition-colors group"
            >
              <div className={`flex-shrink-0 w-8 h-8 rounded-full ${getColor(actividad.tipo)} flex items-center justify-center text-white`}>
                {getIcono(actividad.tipo)}
              </div>
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-gray-900 dark:text-gray-100 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors">
                  {actividad.descripcion}
                </p>
                <div className="flex items-center gap-2 mt-1">
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    {formatearTiempo(actividad.timestamp)}
                  </p>
                  {actividad.usuario && (
                    <>
                      <span className="text-xs text-gray-300 dark:text-gray-600">•</span>
                      <p className="text-xs text-gray-500 dark:text-gray-400">
                        por {actividad.usuario}
                      </p>
                    </>
                  )}
                </div>
              </div>
            </div>
          ))
        )}
      </div>
      
      {actividades.length > 3 && (
        <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
          <button className="text-sm text-primary-600 dark:text-primary-400 hover:text-primary-700 dark:hover:text-primary-300 transition-colors">
            Ver todas las actividades ({actividades.length - 3} más) →
          </button>
        </div>
      )}
    </div>
  );
} 