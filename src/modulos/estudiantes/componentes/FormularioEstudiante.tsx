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
  const [grados, setGrados] = useState<any[]>([]);
  const [secciones, setSecciones] = useState<any[]>([]);
  const [modalidades, setModalidades] = useState<any[]>([]);
  const { mostrarMensaje } = useMensajeGlobal();
  const [paises, setPaises] = useState<any[]>([]);
  const [estados, setEstados] = useState<any[]>([]);
  const [ciudades, setCiudades] = useState<any[]>([]);
  const [municipios, setMunicipios] = useState<any[]>([]);
  const [idSecciones, setIdSecciones] = useState<number | null>(null);

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
    // Cargar grados al montar
    invoke('listar_grados')
      .then((data: any) => setGrados(data))
      .catch(() => setGrados([]));
  }, []);

  useEffect(() => {
    // Cargar secciones al montar
    invoke('listar_secciones')
      .then((data: any) => setSecciones(data))
      .catch(() => setSecciones([]));
  }, []);

  useEffect(() => {
    // Cargar modalidades al montar
    invoke('listar_modalidades')
      .then((data: any) => setModalidades(data))
      .catch(() => setModalidades([]));
  }, []);

  // Cargar países al montar
  useEffect(() => {
    invoke('listar_paises').then((data: any) => setPaises(data)).catch(() => setPaises([]));
  }, []);

  // Cargar estados cuando cambia el país
  useEffect(() => {
    if (formData.paisnac) {
      console.log('[DEBUG] Solicitando estados para país:', formData.paisnac);
      invoke('listar_estados_por_pais', { idPais: Number(formData.paisnac), id_pais: Number(formData.paisnac) })
        .then((data: any) => {
          console.log('[DEBUG] Estados recibidos:', data);
          setEstados(data);
        })
        .catch((err) => {
          console.error('[ERROR] Error al cargar estados:', err);
          setEstados([]);
        });
    } else {
      setEstados([]);
    }
    setMunicipios([]);
    setCiudades([]);
    setFormData((prev) => ({ ...prev, estadonac: '', municipionac: '', ciudadnac: '' }));
  }, [formData.paisnac]);

  // Cargar municipios cuando cambia el estado
  useEffect(() => {
    if (formData.estadonac) {
      console.log('[DEBUG] Solicitando municipios para estado:', formData.estadonac);
      invoke('listar_municipios_por_estado', { idEstado: Number(formData.estadonac), id_estado: Number(formData.estadonac) })
        .then((data: any) => {
          console.log('[DEBUG] Municipios recibidos:', data);
          setMunicipios(data);
        })
        .catch((err) => {
          console.error('[ERROR] Error al cargar municipios:', err);
          setMunicipios([]);
        });
    } else {
      setMunicipios([]);
    }
    setCiudades([]);
    setFormData((prev) => ({ ...prev, municipionac: '', ciudadnac: '' }));
  }, [formData.estadonac]);

  // Cargar ciudades cuando cambia el municipio
  useEffect(() => {
    if (formData.municipionac) {
      console.log('[DEBUG] Solicitando ciudades para municipio:', formData.municipionac);
      invoke('listar_ciudades_por_municipio', { idMunicipio: Number(formData.municipionac), id_municipio: Number(formData.municipionac) })
        .then((data: any) => {
          console.log('[DEBUG] Ciudades recibidas:', data);
          setCiudades(data);
        })
        .catch((err) => {
          console.error('[ERROR] Error al cargar ciudades:', err);
          setCiudades([]);
        });
    } else {
      setCiudades([]);
    }
    setFormData((prev) => ({ ...prev, ciudadnac: '' }));
  }, [formData.municipionac]);

  useEffect(() => {
    if (formData.id_grado && formData.id_modalidad) {
      invoke('obtener_id_seccion', {
        gradoId: Number(formData.id_grado),
        seccionId: Number(formData.id_seccion),
        modalidadId: Number(formData.id_modalidad)
      })
        .then((id) => {
          setIdSecciones(Number(id));
        })
        .catch(() => setIdSecciones(null));
    } else {
      setIdSecciones(null);
    }
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

    if (!formData.id_grado || !formData.id_seccion || !formData.id_modalidad) {
      mostrarMensaje('Debe seleccionar grado, sección y modalidad válidos.', 'error');
      setIsSubmitting(false);
      return;
    }

    try {
      // 1. Obtener el id_grado_secciones
      console.log('[DEBUG] Enviando a obtener_id_grado_secciones:', {
        gradoId: Number(formData.id_grado),
        seccionId: Number(formData.id_seccion),
        modalidadId: Number(formData.id_modalidad)
      });
      const id_grado_secciones = await invoke('obtener_id_grado_secciones', {
        gradoId: Number(formData.id_grado),
        seccionId: Number(formData.id_seccion),
        modalidadId: Number(formData.id_modalidad)
      }).catch((err) => {
        console.error('[ERROR] invoke obtener_id_grado_secciones:', err);
        throw err;
      });
      console.log('[DEBUG] Respuesta de obtener_id_grado_secciones:', id_grado_secciones);

      const id_grado_secciones_num = Number(id_grado_secciones);
      console.log('[DEBUG] id_grado_secciones_num validado:', id_grado_secciones_num);
      if (!id_grado_secciones_num) {
        mostrarMensaje('No se encontró la combinación de grado, sección y modalidad.', 'error');
        setError('No se encontró la combinación de grado, sección y modalidad.');
        setIsSubmitting(false);
        return;
      }
      console.log('[DEBUG] Continúa con la inserción del estudiante');

      // 2. Armar el payload solo con id_grado_secciones
      const cleanData = {
        cedula: formData.cedula,
        nombres: formData.nombres,
        apellidos: formData.apellidos,
        genero: formData.genero || null,
        fecha_nacimiento: formData.fecha_nacimiento ? formData.fecha_nacimiento : null,
        id_grado_secciones: id_grado_secciones_num,
        fecha_ingreso: formData.fecha_ingreso ? formData.fecha_ingreso : null,
        paisnac_id: formData.paisnac ? Number(formData.paisnac) : null,
        estado_nac_id: formData.estadonac ? Number(formData.estadonac) : null,
        municipio_nac_id: formData.municipionac ? Number(formData.municipionac) : null,
        ciudad_nac_id: formData.ciudadnac ? Number(formData.ciudadnac) : null,
        id_grado: formData.id_grado ? Number(formData.id_grado) : null,
        id_seccion: formData.id_seccion ? Number(formData.id_seccion) : null,
        id_modalidad: formData.id_modalidad ? Number(formData.id_modalidad) : null,
        id_periodoactual: formData.id_periodoactual ? Number(formData.id_periodoactual) : null,
        estado: 'Activo',
        fecha_retiro: formData.fecha_retiro ? formData.fecha_retiro : null,
      };

      // 3. Enviar el estudiante
      console.log('[DEBUG] Enviando estudiante al backend:', cleanData);
      await invoke('crear_estudiante', { estudiante: cleanData });
      console.log('[DEBUG] Estudiante creado correctamente');
      mostrarMensaje('Estudiante creado correctamente', 'exito');
      setSuccess('Estudiante creado correctamente');
    } catch (err) {
      console.error('[ERROR] Error al crear estudiante:', err);
      mostrarMensaje('Error al guardar el estudiante. Por favor, intente nuevamente.', 'error');
      setError('Error al guardar el estudiante. Por favor, intente nuevamente.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col bg-transparent">
      {/* Espacio superior real */}
      <div className="pt-8" />
      <form onSubmit={handleSubmit} className="max-w-6xl mx-auto p-8 bg-white dark:bg-dark-800 rounded-2xl shadow-lg mt-2 mb-2 flex flex-col justify-between relative">
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
                  {grados.map((grado) => (
                    <option key={grado.id_grado} value={grado.id_grado}>{grado.nombre_grado}</option>
                  ))}
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
                    <option key={seccion.id} value={seccion.id}>{seccion.nombre}</option>
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
                  {modalidades.map((modalidad) => (
                    <option key={modalidad.id_modalidad} value={modalidad.id_modalidad}>{modalidad.nombre_modalidad}</option>
                  ))}
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
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">País</label>
                <select
                  name="paisnac"
                  value={formData.paisnac || ''}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione un país</option>
                  {paises.map((pais) => (
                    <option key={pais.id} value={pais.id}>{pais.nombre}</option>
                  ))}
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Estado</label>
                <select
                  name="estadonac"
                  value={formData.estadonac || ''}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione un estado</option>
                  {estados.map((estado) => (
                    <option key={estado.id} value={estado.id}>{estado.nombre}</option>
                  ))}
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Municipio</label>
                <select
                  name="municipionac"
                  value={formData.municipionac || ''}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione un municipio</option>
                  {municipios.map((municipio) => (
                    <option key={municipio.id} value={municipio.id}>{municipio.nombre}</option>
                  ))}
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Ciudad</label>
                <select
                  name="ciudadnac"
                  value={formData.ciudadnac || ''}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  <option value="">Seleccione una ciudad</option>
                  {ciudades.map((ciudad) => (
                    <option key={ciudad.id} value={ciudad.id}>{ciudad.nombre}</option>
                  ))}
                </select>
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