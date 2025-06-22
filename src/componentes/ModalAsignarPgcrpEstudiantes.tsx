import React, { useState, useEffect } from 'react';
import { X, Search, User, Calendar } from 'lucide-react';
import { Button } from './ui/button';
import { Card } from './ui/card';
import { estudiantePgcrpService } from '../servicios/estudiantePgcrpService';
import { EstudiantePgcrpDetalle, ActividadPgcrp, AsignacionEstudiantePgcrp } from '../tipos/estudiante_pgcrp';
import { useMensajeGlobal } from './MensajeGlobalContext';

interface ModalAsignarPgcrpEstudiantesProps {
  isOpen: boolean;
  onClose: () => void;
  idGradoSecciones: number;
  idPeriodo: number;
  nombreGrado: string;
  nombreSeccion: string;
  onSuccess: () => void;
}

const ModalAsignarPgcrpEstudiantes: React.FC<ModalAsignarPgcrpEstudiantesProps> = ({
  isOpen,
  onClose,
  idGradoSecciones,
  idPeriodo,
  nombreGrado,
  nombreSeccion,
  onSuccess,
}) => {
  const [estudiantes, setEstudiantes] = useState<EstudiantePgcrpDetalle[]>([]);
  const [actividades, setActividades] = useState<ActividadPgcrp[]>([]);
  const [filtroActividad, setFiltroActividad] = useState('');
  const [filtroEstudiante, setFiltroEstudiante] = useState('');
  const [cargando, setCargando] = useState(false);
  const [guardando, setGuardando] = useState(false);
  const [estudianteSeleccionado, setEstudianteSeleccionado] = useState<EstudiantePgcrpDetalle | null>(null);
  const [actividadSeleccionada, setActividadSeleccionada] = useState<number | null>(null);
  const [observaciones, setObservaciones] = useState('');
  const { mostrarMensaje } = useMensajeGlobal();

  useEffect(() => {
    if (isOpen) {
      cargarDatos();
    }
  }, [isOpen, idGradoSecciones, idPeriodo]);

  const cargarDatos = async () => {
    setCargando(true);
    try {
      const [estudiantesData, actividadesData] = await Promise.all([
        estudiantePgcrpService.obtenerEstudiantesSeccionPgcrp(idGradoSecciones, idPeriodo),
        estudiantePgcrpService.obtenerActividadesPgcrp(),
      ]);

      setEstudiantes(estudiantesData);
      setActividades(actividadesData);
    } catch (error) {
      console.error('Error al cargar datos:', error);
      mostrarMensaje('Error al cargar los datos', 'error');
    } finally {
      setCargando(false);
    }
  };

  const handleSeleccionarEstudiante = (estudiante: EstudiantePgcrpDetalle) => {
    setEstudianteSeleccionado(estudiante);
    setActividadSeleccionada(estudiante.id_extra_catedra || null);
    setObservaciones(estudiante.observaciones || '');
  };

  const handleAsignar = async () => {
    if (!estudianteSeleccionado) return;

    setGuardando(true);
    try {
      const asignacion: AsignacionEstudiantePgcrp = {
        id_estudiante: estudianteSeleccionado.id_estudiante,
        id_extra_catedra: actividadSeleccionada || undefined,
        id_periodo: idPeriodo,
        id_grado_secciones: idGradoSecciones,
        observaciones: observaciones.trim() || undefined,
      };

      const mensaje = await estudiantePgcrpService.asignarPgcrpEstudiante(asignacion);
      
      mostrarMensaje(mensaje, 'exito');
      await cargarDatos();
      setEstudianteSeleccionado(null);
      setActividadSeleccionada(null);
      setObservaciones('');
      onSuccess();
    } catch (error) {
      console.error('Error al asignar PGCRP:', error);
      mostrarMensaje('Error al asignar PGCRP', 'error');
    } finally {
      setGuardando(false);
    }
  };

  const handleEliminar = async () => {
    if (!estudianteSeleccionado || !estudianteSeleccionado.id_extra_catedra) return;

    if (!confirm('¿Está seguro de eliminar la asignación PGCRP individual para este estudiante?')) {
      return;
    }

    setGuardando(true);
    try {
      const mensaje = await estudiantePgcrpService.eliminarPgcrpEstudiante(
        estudianteSeleccionado.id_estudiante,
        idPeriodo
      );

      mostrarMensaje(mensaje, 'exito');
      await cargarDatos();
      setEstudianteSeleccionado(null);
      setActividadSeleccionada(null);
      setObservaciones('');
      onSuccess();
    } catch (error) {
      console.error('Error al eliminar asignación PGCRP:', error);
      mostrarMensaje('Error al eliminar asignación PGCRP', 'error');
    } finally {
      setGuardando(false);
    }
  };

  const actividadesFiltradas = actividades.filter(actividad =>
    actividad.nombre.toLowerCase().includes(filtroActividad.toLowerCase())
  );

  const estudiantesFiltrados = estudiantes.filter(estudiante =>
    `${estudiante.nombres} ${estudiante.apellidos}`.toLowerCase().includes(filtroEstudiante.toLowerCase()) ||
    estudiante.cedula.toString().includes(filtroEstudiante)
  );

  const getEstadoPgcrp = (estudiante: EstudiantePgcrpDetalle) => {
    // Usar el campo tipo_asignacion de la base de datos
    if (estudiante.tipo_asignacion === 'individual') {
      return {
        tipo: 'Individual',
        actividad: estudiante.actividad_pgcrp || 'No asignado',
        color: 'text-teal-600 bg-teal-50',
      };
    } else if (estudiante.tipo_asignacion === 'seccion' || estudiante.actividad_pgcrp) {
      return {
        tipo: 'Sección',
        actividad: estudiante.actividad_pgcrp || estudiante.actividad_seccion || 'No asignado',
        color: 'text-green-600 bg-green-50',
      };
    } else {
      return {
        tipo: 'Sin Asignar',
        actividad: 'No asignado',
        color: 'text-gray-500 bg-gray-50',
      };
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white dark:bg-gray-900 rounded-lg shadow-xl w-full max-w-4xl h-[80vh] overflow-hidden">
        {/* Header */}
        <div className="bg-teal-500 text-white p-3 flex justify-between items-center">
          <div>
            <h2 className="text-lg font-bold">Asignar PGCRP Individual</h2>
            <p className="text-teal-100 text-sm">
              {nombreGrado} - Sección {nombreSeccion}
            </p>
          </div>
          <button
            onClick={onClose}
            className="text-white hover:text-gray-200 transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        {cargando ? (
          <div className="p-8 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-teal-500 mx-auto"></div>
            <p className="mt-4 text-gray-600 dark:text-gray-300">Cargando datos...</p>
          </div>
        ) : (
          <div className="flex h-[calc(80vh-70px)]">
            {/* Lista de estudiantes */}
            <div className="w-1/2 border-r border-gray-200 dark:border-gray-700 flex flex-col">
              <div className="p-3 border-b border-gray-200 dark:border-gray-700 dark:bg-gray-800">
                <h3 className="font-semibold text-gray-800 dark:text-gray-200 mb-2 text-sm">Estudiantes</h3>
                <div className="relative">
                  <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={14} />
                  <input
                    type="text"
                    placeholder="Buscar estudiante..."
                    value={filtroEstudiante}
                    onChange={(e) => setFiltroEstudiante(e.target.value)}
                    className="w-full pl-9 pr-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-md focus:ring-2 focus:ring-teal-500 focus:border-transparent bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400"
                  />
                </div>
              </div>

              <div className="flex-1 overflow-y-auto p-2 space-y-1 dark:bg-gray-900">
                {estudiantesFiltrados.map((estudiante) => {
                  const estado = getEstadoPgcrp(estudiante);
                  return (
                    <Card
                      key={estudiante.id_estudiante}
                      className={`p-2 cursor-pointer transition-all hover:shadow-sm dark:border-gray-700 ${
                        estudianteSeleccionado?.id_estudiante === estudiante.id_estudiante
                          ? 'ring-2 ring-teal-500 bg-teal-50 dark:bg-teal-900/20'
                          : 'hover:bg-gray-50 dark:hover:bg-gray-800 dark:bg-gray-800'
                      }`}
                      onClick={() => handleSeleccionarEstudiante(estudiante)}
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex items-center space-x-2">
                          <User className="text-gray-400" size={14} />
                          <div>
                            <p className="font-medium text-gray-900 dark:text-gray-100 text-sm">
                              {estudiante.nombres} {estudiante.apellidos}
                            </p>
                            <p className="text-xs text-gray-500 dark:text-gray-400">
                              C.I: {estudiante.cedula.toLocaleString()}
                            </p>
                          </div>
                        </div>
                        <div className="text-right">
                          <span className={`px-1.5 py-0.5 rounded-full text-xs font-medium ${estado.color}`}>
                            {estado.tipo}
                          </span>
                          <p className="text-xs text-gray-600 dark:text-gray-400 mt-0.5 max-w-20 truncate">
                            {estado.actividad}
                          </p>
                        </div>
                      </div>
                    </Card>
                  );
                })}
              </div>
            </div>

            {/* Panel de asignación */}
            <div className="w-1/2 flex flex-col">
              {estudianteSeleccionado ? (
                <>
                  {/* Información del estudiante */}
                  <div className="p-3 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800">
                    <h3 className="font-semibold text-gray-800 dark:text-gray-200 mb-1 text-sm">
                      Estudiante Seleccionado
                    </h3>
                    <div className="space-y-0.5">
                      <p className="text-xs text-gray-700 dark:text-gray-300">
                        <span className="font-medium">Nombre:</span>{' '}
                        {estudianteSeleccionado.nombres} {estudianteSeleccionado.apellidos}
                      </p>
                      <p className="text-xs text-gray-700 dark:text-gray-300">
                        <span className="font-medium">Cédula:</span>{' '}
                        {estudianteSeleccionado.cedula.toLocaleString()}
                      </p>
                      {estudianteSeleccionado.fecha_asignacion && (
                        <p className="text-xs text-gray-700 dark:text-gray-300">
                          <span className="font-medium">Última asignación:</span>{' '}
                          {new Date(estudianteSeleccionado.fecha_asignacion).toLocaleDateString()}
                        </p>
                      )}
                    </div>
                  </div>

                  {/* Selector de actividad */}
                  <div className="p-3 border-b border-gray-200 dark:border-gray-700 dark:bg-gray-900">
                    <h3 className="font-semibold text-gray-800 dark:text-gray-200 mb-2 text-sm">
                      Actividad PGCRP
                    </h3>
                    <div className="relative mb-2">
                      <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={14} />
                      <input
                        type="text"
                        placeholder="Buscar actividad..."
                        value={filtroActividad}
                        onChange={(e) => setFiltroActividad(e.target.value)}
                        className="w-full pl-9 pr-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-md focus:ring-2 focus:ring-teal-500 focus:border-transparent bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400"
                      />
                    </div>

                    <div className="space-y-1 max-h-24 overflow-y-auto">
                      <label className="flex items-center space-x-2 p-1.5 rounded-md hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer">
                        <input
                          type="radio"
                          name="actividad"
                          value=""
                          checked={actividadSeleccionada === null}
                          onChange={() => setActividadSeleccionada(null)}
                          className="text-teal-600 focus:ring-teal-500"
                        />
                        <span className="text-gray-700 dark:text-gray-300 text-sm">Sin asignación individual</span>
                      </label>
                      {actividadesFiltradas.map((actividad) => (
                        <label
                          key={actividad.id_extra_catedra}
                          className="flex items-center space-x-2 p-1.5 rounded-md hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
                        >
                          <input
                            type="radio"
                            name="actividad"
                            value={actividad.id_extra_catedra}
                            checked={actividadSeleccionada === actividad.id_extra_catedra}
                            onChange={() => setActividadSeleccionada(actividad.id_extra_catedra)}
                            className="text-teal-600 focus:ring-teal-500"
                          />
                          <span className="text-gray-700 dark:text-gray-300 text-sm">{actividad.nombre}</span>
                        </label>
                      ))}
                    </div>
                  </div>

                  {/* Observaciones */}
                  <div className="flex-1 p-3 border-b border-gray-200 dark:border-gray-700 dark:bg-gray-900">
                    <h3 className="font-semibold text-gray-800 dark:text-gray-200 mb-2 text-sm">
                      Observaciones
                    </h3>
                    <textarea
                      value={observaciones}
                      onChange={(e) => setObservaciones(e.target.value)}
                      placeholder="Observaciones adicionales (opcional)..."
                      className="w-full h-16 p-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md focus:ring-2 focus:ring-teal-500 focus:border-transparent resize-none bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400"
                    />
                  </div>

                  {/* Botones de acción - Fijos en la parte inferior */}
                  <div className="p-3 bg-gray-50 dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-2">
                    {estudianteSeleccionado.id_extra_catedra && (
                      <Button
                        onClick={handleEliminar}
                        disabled={guardando}
                        variant="outline"
                        className="text-red-600 border-red-600 hover:bg-red-50 text-sm px-3 py-1.5"
                      >
                        <X size={14} className="mr-1" />
                        Eliminar
                      </Button>
                    )}
                    <Button
                      onClick={handleAsignar}
                      disabled={guardando}
                      className="bg-teal-500 hover:bg-teal-600 text-white text-sm px-3 py-1.5"
                    >
                      {guardando ? (
                        <div className="animate-spin rounded-full h-3 w-3 border-b-2 border-white mr-1"></div>
                      ) : (
                        <Calendar size={14} className="mr-1" />
                      )}
                      {estudianteSeleccionado.id_extra_catedra ? 'Actualizar' : 'Asignar'}
                    </Button>
                  </div>
                </>
              ) : (
                <div className="flex-1 flex items-center justify-center dark:bg-gray-900">
                  <div className="text-center text-gray-500 dark:text-gray-400">
                    <User size={48} className="mx-auto mb-4 text-gray-300 dark:text-gray-600" />
                    <p>Selecciona un estudiante para asignar PGCRP</p>
                  </div>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default ModalAsignarPgcrpEstudiantes; 