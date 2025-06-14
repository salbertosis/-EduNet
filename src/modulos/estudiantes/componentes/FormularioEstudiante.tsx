import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

interface Estudiante {
  id: number;
  cedula: string;
  apellidos: string;
  nombres: string;
  genero?: string;
  fecha_nacimiento?: string;
  fecha_ingreso?: string;
  municipionac?: string;
  paisnac?: string;
  entidadfed?: string;
  ciudadnac?: string;
  estadonac?: string;
  id_grado?: number;
  nombre_grado?: string;
  id_seccion?: number;
  nombre_seccion?: string;
  id_modalidad?: number;
  nombre_modalidad?: string;
  id_periodoactual?: number;
  estado?: string;
  fecha_retiro?: string;
}

interface Seccion {
  id_seccion: number;
  nombre_seccion: string;
  id_grado_secciones: number;
}

interface FormularioEstudianteProps {
  estudiante?: Estudiante;
  onGuardar: () => void;
  onCancelar: () => void;
}

export function FormularioEstudiante({ estudiante, onGuardar, onCancelar }: FormularioEstudianteProps) {
  const [formData, setFormData] = useState<Partial<Estudiante>>({
    cedula: '',
    apellidos: '',
    nombres: '',
    genero: '',
    fecha_nacimiento: '',
    fecha_ingreso: '',
    municipionac: '',
    paisnac: '',
    entidadfed: '',
    ciudadnac: '',
    estadonac: '',
    id_grado: undefined,
    id_seccion: undefined,
    id_modalidad: undefined,
    id_periodoactual: undefined,
    estado: 'Activo',
    fecha_retiro: '',
  });
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [periodosEscolares, setPeriodosEscolares] = useState<{ id_periodo: number; periodo_escolar: string }[]>([]);
  const [secciones, setSecciones] = useState<{ id_seccion: number; nombre_seccion: string; id_grado_secciones: number }[]>([]);
  const { mostrarMensaje } = useMensajeGlobal();

  useEffect(() => {
    if (estudiante) {
      setFormData(estudiante);
    }
  }, [estudiante]);

  useEffect(() => {
    if (success) {
      const timer = setTimeout(() => {
        setSuccess(null);
        onGuardar();
      }, 2500);
      return () => clearTimeout(timer);
    }
  }, [success]);

  useEffect(() => {
    // Obtener periodos escolares al montar el componente
    invoke('listar_periodos_escolares')
      .then((data: any) => {
        setPeriodosEscolares(data);
      })
      .catch((err) => {
        console.error('Error al obtener periodos escolares:', err);
      });
  }, []);

  useEffect(() => {
    const cargarSecciones = async () => {
      console.log('Intentando cargar secciones con:', {
        idGrado: formData.id_grado,
        idModalidad: formData.id_modalidad,
        idPeriodo: formData.id_periodoactual
      });
      
      if (formData.id_grado && formData.id_modalidad && formData.id_periodoactual) {
        try {
          const seccionesData = await invoke<Seccion[]>('obtener_secciones_por_grado_modalidad_periodo', {
            idGrado: formData.id_grado,
            idModalidad: formData.id_modalidad,
            idPeriodo: formData.id_periodoactual
          });
          console.log('Secciones cargadas:', seccionesData);
          setSecciones(seccionesData);
        } catch (err) {
          console.error('Error al cargar secciones:', err);
          mostrarMensaje('Error al cargar las secciones', 'error');
        }
      } else {
        console.log('No se cargan secciones porque faltan datos:', {
          idGrado: formData.id_grado,
          idModalidad: formData.id_modalidad,
          idPeriodo: formData.id_periodoactual
        });
        setSecciones([]);
      }
    };

    cargarSecciones();
  }, [formData.id_grado, formData.id_modalidad, formData.id_periodoactual]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    console.log('handleChange:', { name, value });
    setFormData(prev => {
      const newData = {
        ...prev,
        [name]: name === 'cedula' ? Number(value) : name.startsWith('id_') ? Number(value) :
          name === 'id_periodoactual' ? Number(value) : value,
      };
      console.log('Nuevo formData:', newData);
      return newData;
    });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    setError(null);
    setSuccess(null);

    try {
      const cleanData = {
        cedula: formData.cedula,
        nombres: formData.nombres,
        apellidos: formData.apellidos,
        genero: formData.genero,
        fecha_nacimiento: formData.fecha_nacimiento,
        fecha_ingreso: formData.fecha_ingreso,
        municipionac: formData.municipionac,
        paisnac: formData.paisnac,
        entidadfed: formData.entidadfed,
        ciudadnac: formData.ciudadnac,
        estadonac: formData.estadonac,
        id_grado: formData.id_grado,
        id_seccion: formData.id_seccion,
        id_modalidad: formData.id_modalidad,
        id_periodoactual: formData.id_periodoactual,
        estado: formData.estado,
        fecha_retiro: formData.fecha_retiro ? formData.fecha_retiro : null,
      };
      console.log('[DEBUG] cleanData a enviar:', cleanData);
      if (estudiante?.id) {
        await invoke('actualizar_estudiante', {
          id: estudiante.id,
          estudiante: cleanData,
        });
        mostrarMensaje('Estudiante actualizado correctamente', 'exito');
        setSuccess('Estudiante actualizado correctamente');
      } else {
        await invoke('crear_estudiante', {
          estudiante: cleanData,
        });
        mostrarMensaje('Estudiante creado correctamente', 'exito');
        setSuccess('Estudiante creado correctamente');
      }
    } catch (err) {
      console.error('Error al guardar estudiante:', err);
      if (typeof err === 'string' && err.includes('cédula')) {
        mostrarMensaje(err, 'error');
        setError(err);
      } else {
        mostrarMensaje('Error al guardar el estudiante. Por favor, intente nuevamente.', 'error');
        setError('Error al guardar el estudiante. Por favor, intente nuevamente.');
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="h-screen max-h-screen flex flex-col bg-transparent">
      {/* Espacio superior real */}
      <div className="pt-8" />
      <form onSubmit={handleSubmit} className="flex-1 overflow-y-auto max-w-6xl mx-auto p-8 bg-white dark:bg-dark-800 rounded-2xl shadow-lg mt-2 mb-2 flex flex-col justify-between relative" style={{minHeight: '80vh'}}>
        {/* Mensaje de éxito visual */}
        {success && (
          <div className="mb-6 p-4 rounded-lg bg-green-100 border border-green-300 text-green-800 text-center font-semibold animate-fade-in">
            {success}
          </div>
        )}
        {/* Mensaje de error visual */}
        {error && (
          <div className="mb-6 p-4 rounded-lg bg-red-100 border border-red-300 text-red-800 text-center font-semibold">
            {error}
          </div>
        )}
        <div className="space-y-8 pb-8">
          {/* DATOS PERSONALES */}
          <section className="bg-gray-50 dark:bg-dark-700 p-6 rounded-xl">
            <h3 className="text-2xl font-semibold text-primary-600 mb-6">Datos Personales</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Nombres</label>
                <input 
                  type="text" 
                  name="nombres" 
                  value={formData.nombres || ''} 
                  onChange={handleChange} 
                  required 
                  placeholder="Ej: Juan Carlos" 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Apellidos</label>
                <input 
                  type="text" 
                  name="apellidos" 
                  value={formData.apellidos || ''} 
                  onChange={handleChange} 
                  required 
                  placeholder="Ej: Pérez Gómez" 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Cédula</label>
                <input 
                  type="number" 
                  name="cedula" 
                  value={formData.cedula || ''} 
                  onChange={handleChange} 
                  required 
                  placeholder="Ej: 12345678" 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Género</label>
                <select 
                  name="genero" 
                  value={formData.genero || ''} 
                  onChange={handleChange} 
                  required 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione</option>
                  <option value="M">Masculino</option>
                  <option value="F">Femenino</option>
                  <option value="O">Otro</option>
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Fecha de nacimiento</label>
                <input 
                  type="date" 
                  name="fecha_nacimiento" 
                  value={formData.fecha_nacimiento || ''} 
                  onChange={handleChange} 
                  required 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
            </div>
          </section>

          {/* DATOS ACADÉMICOS */}
          <section className="bg-gray-50 dark:bg-dark-700 p-6 rounded-xl">
            <h3 className="text-2xl font-semibold text-primary-600 mb-6">Datos Académicos</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Grado</label>
                <select 
                  name="id_grado" 
                  value={formData.id_grado || ''} 
                  onChange={handleChange} 
                  required 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione un grado</option>
                  <option value={1}>1er Año</option>
                  <option value={2}>2do Año</option>
                  <option value={3}>3er Año</option>
                  <option value={4}>4to Año</option>
                  <option value={5}>5to Año</option>
                  <option value={6}>6to Año</option>
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Sección</label>
                <select 
                  name="id_seccion" 
                  value={formData.id_seccion || ''} 
                  onChange={handleChange} 
                  required 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione una sección</option>
                  {secciones.map((seccion) => (
                    <option key={seccion.id_seccion} value={seccion.id_grado_secciones}>
                      {seccion.nombre_seccion}
                    </option>
                  ))}
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Modalidad</label>
                <select 
                  name="id_modalidad" 
                  value={formData.id_modalidad || ''} 
                  onChange={handleChange} 
                  required 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione una modalidad</option>
                  <option value={1}>Ciencias</option>
                  <option value={2}>Telemática</option>
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Año Escolar</label>
                <select
                  name="id_periodoactual"
                  value={formData.id_periodoactual || ''}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione un año escolar</option>
                  {periodosEscolares.map(periodo => (
                    <option key={periodo.id_periodo} value={periodo.id_periodo}>
                      {periodo.periodo_escolar}
                    </option>
                  ))}
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Fecha de ingreso</label>
                <input 
                  type="date" 
                  name="fecha_ingreso" 
                  value={formData.fecha_ingreso || ''} 
                  onChange={handleChange} 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
              {estudiante && (
                <>
                  <div className="space-y-2">
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Estado</label>
                    <select 
                      name="estado" 
                      value={formData.estado || ''} 
                      onChange={handleChange} 
                      required 
                      className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                    >
                      <option value="">Seleccione estado</option>
                      <option value="Activo">Activo</option>
                      <option value="Retirado">Retirado</option>
                    </select>
                  </div>
                  <div className="space-y-2">
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Fecha de retiro</label>
                    <input 
                      type="date" 
                      name="fecha_retiro" 
                      value={formData.fecha_retiro || ''} 
                      onChange={handleChange} 
                      className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                    />
                  </div>
                </>
              )}
            </div>
          </section>

          {/* DATOS DE NACIMIENTO */}
          <section className="bg-gray-50 dark:bg-dark-700 p-6 rounded-xl">
            <h3 className="text-2xl font-semibold text-primary-600 mb-6">Datos de Nacimiento</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Municipio</label>
                <input 
                  type="text" 
                  name="municipionac" 
                  value={formData.municipionac || ''} 
                  onChange={handleChange} 
                  placeholder="Ej: Maracaibo" 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">País</label>
                <input 
                  type="text" 
                  name="paisnac" 
                  value={formData.paisnac || ''} 
                  onChange={handleChange} 
                  placeholder="Ej: Venezuela" 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Entidad Federal</label>
                <input 
                  type="text" 
                  name="entidadfed" 
                  value={formData.entidadfed || ''} 
                  onChange={handleChange} 
                  placeholder="Ej: Zulia" 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Ciudad</label>
                <input 
                  type="text" 
                  name="ciudadnac" 
                  value={formData.ciudadnac || ''} 
                  onChange={handleChange} 
                  placeholder="Ej: Maracaibo" 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Estado</label>
                <input 
                  type="text" 
                  name="estadonac" 
                  value={formData.estadonac || ''} 
                  onChange={handleChange} 
                  placeholder="Ej: Zulia" 
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all" 
                />
              </div>
            </div>
          </section>
        </div>
        {/* Espacio inferior real */}
        <div className="h-8" />
        {/* Botones de acción fijos dentro del formulario */}
        <div className="sticky bottom-0 left-0 right-0 bg-white dark:bg-dark-800 py-4 px-8 border-t border-gray-200 dark:border-dark-700 z-10 flex justify-end space-x-4 max-w-6xl mx-auto">
          <button
            type="button"
            onClick={onCancelar}
            className="px-6 py-2.5 bg-gray-200 dark:bg-dark-700 text-gray-700 dark:text-gray-200 rounded-lg hover:bg-gray-300 dark:hover:bg-dark-600 transition-all duration-200 font-medium"
            disabled={isSubmitting}
          >
            Cancelar
          </button>
          <button
            type="submit"
            className="px-6 py-2.5 bg-primary-500 text-white rounded-lg hover:bg-primary-600 flex items-center gap-2 transition-all duration-200 font-medium disabled:opacity-60 disabled:cursor-not-allowed"
            disabled={isSubmitting}
          >
            <span>{estudiante ? 'Actualizar' : 'Crear'}</span>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-5 h-5">
              <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 12.75l6 6 9-13.5" />
            </svg>
          </button>
        </div>
      </form>
      {/* Espacio inferior real */}
      <div className="pb-8" />
    </div>
  );
} 