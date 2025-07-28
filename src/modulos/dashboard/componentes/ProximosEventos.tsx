import { Calendar, AlertTriangle, CheckCircle, Clock, ExternalLink } from 'lucide-react';

interface Evento {
  id: number;
  titulo: string;
  descripcion?: string;
  tipo: 'examen' | 'fecha_limite' | 'evento_escolar' | 'recordatorio';
  fecha_evento: string;
  prioridad: 'alta' | 'media' | 'baja';
  completado?: boolean;
}

interface ProximosEventosProps {
  eventos: Evento[];
  cargando?: boolean;
}

export function ProximosEventos({ eventos, cargando = false }: ProximosEventosProps) {
  const getIcono = (tipo: string) => {
    switch (tipo) {
      case 'examen': return <AlertTriangle className="w-4 h-4" />;
      case 'fecha_limite': return <Clock className="w-4 h-4" />;
      case 'evento_escolar': return <Calendar className="w-4 h-4" />;
      case 'recordatorio': return <CheckCircle className="w-4 h-4" />;
      default: return <Calendar className="w-4 h-4" />;
    }
  };

  const getColor = (tipo: string, prioridad: string) => {
    if (tipo === 'examen') return 'bg-red-500';
    if (tipo === 'fecha_limite') return 'bg-orange-500';
    if (prioridad === 'alta') return 'bg-red-500';
    if (prioridad === 'media') return 'bg-yellow-500';
    return 'bg-green-500';
  };

  const formatearFecha = (fecha: string) => {
    const fechaEvento = new Date(fecha);
    const ahora = new Date();
    const diffMs = fechaEvento.getTime() - ahora.getTime();
    const diffDias = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    const diffHoras = Math.floor(diffMs / (1000 * 60 * 60));

    if (diffMs < 0) return 'Pasado';
    if (diffDias === 0) {
      if (diffHoras === 0) return 'Hoy';
      return `En ${diffHoras} horas`;
    }
    if (diffDias === 1) return 'Mañana';
    if (diffDias < 7) return `En ${diffDias} días`;
    return fechaEvento.toLocaleDateString();
  };

  const getUrgencia = (fecha: string) => {
    const fechaEvento = new Date(fecha);
    const ahora = new Date();
    const diffMs = fechaEvento.getTime() - ahora.getTime();
    const diffDias = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    
    if (diffMs < 0) return 'pasado';
    if (diffDias === 0) return 'hoy';
    if (diffDias <= 3) return 'urgente';
    if (diffDias <= 7) return 'proximo';
    return 'normal';
  };

  if (cargando) {
    return (
      <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6">
        <h2 className="text-lg font-semibold mb-4 flex items-center gap-2">
          <Calendar className="w-5 h-5" />
          Próximos Eventos
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

  const eventosOrdenados = eventos
    .filter(e => !e.completado)
    .sort((a, b) => new Date(a.fecha_evento).getTime() - new Date(b.fecha_evento).getTime());

  return (
    <div className="bg-white dark:bg-dark-800 rounded-xl shadow-soft p-6">
      <h2 className="text-lg font-semibold mb-4 flex items-center gap-2">
        <Calendar className="w-5 h-5" />
        Próximos Eventos
        {eventosOrdenados.length > 0 && (
          <span className="ml-auto text-sm text-gray-500 dark:text-gray-400">
            {eventosOrdenados.length} eventos
          </span>
        )}
      </h2>
      
      <div className="space-y-4">
        {eventosOrdenados.length === 0 ? (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            <Calendar className="w-12 h-12 mx-auto mb-3 opacity-50" />
            <p>No hay eventos próximos</p>
          </div>
        ) : (
          eventosOrdenados.slice(0, 5).map((evento) => {
            const urgencia = getUrgencia(evento.fecha_evento);
            return (
              <div 
                key={evento.id} 
                className={`flex items-start space-x-3 p-3 rounded-lg transition-all group ${
                  urgencia === 'hoy' ? 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800' :
                  urgencia === 'urgente' ? 'bg-orange-50 dark:bg-orange-900/20 border border-orange-200 dark:border-orange-800' :
                  'hover:bg-gray-50 dark:hover:bg-dark-700'
                }`}
              >
                <div className={`flex-shrink-0 w-8 h-8 rounded-full ${getColor(evento.tipo, evento.prioridad)} flex items-center justify-center text-white`}>
                  {getIcono(evento.tipo)}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <p className={`text-sm font-medium ${
                      urgencia === 'hoy' ? 'text-red-900 dark:text-red-100' :
                      urgencia === 'urgente' ? 'text-orange-900 dark:text-orange-100' :
                      'text-gray-900 dark:text-gray-100'
                    }`}>
                      {evento.titulo}
                    </p>
                    {evento.prioridad === 'alta' && (
                      <span className="px-2 py-1 text-xs bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded-full">
                        Alta
                      </span>
                    )}
                  </div>
                  {evento.descripcion && (
                    <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">
                      {evento.descripcion}
                    </p>
                  )}
                  <div className="flex items-center gap-2 mt-2">
                    <p className={`text-xs font-medium ${
                      urgencia === 'hoy' ? 'text-red-600 dark:text-red-400' :
                      urgencia === 'urgente' ? 'text-orange-600 dark:text-orange-400' :
                      'text-gray-500 dark:text-gray-400'
                    }`}>
                      {formatearFecha(evento.fecha_evento)}
                    </p>
                    <span className="text-xs text-gray-300 dark:text-gray-600">•</span>
                    <p className="text-xs text-gray-500 dark:text-gray-400 capitalize">
                      {evento.tipo.replace('_', ' ')}
                    </p>
                  </div>
                </div>
                <button className="opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
                  <ExternalLink className="w-4 h-4 text-gray-400" />
                </button>
              </div>
            );
          })
        )}
      </div>
      
      {eventosOrdenados.length > 5 && (
        <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
          <button className="text-sm text-primary-600 dark:text-primary-400 hover:text-primary-700 dark:hover:text-primary-300 transition-colors">
            Ver todos los eventos ({eventosOrdenados.length}) →
          </button>
        </div>
      )}
    </div>
  );
} 