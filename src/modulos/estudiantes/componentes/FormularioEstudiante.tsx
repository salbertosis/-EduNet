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
  // Campos legacy para compatibilidad
  municipionac?: string;
  paisnac?: string;
  entidadfed?: string;
  ciudadnac?: string;
  estadonac?: string;
  // Datos académicos
  id_grado?: number;
  nombre_grado?: string;
  id_seccion?: number;
  nombre_seccion?: string;
  id_modalidad?: number;
  nombre_modalidad?: string;
  id_periodoactual?: number;
  estado?: string;
  fecha_retiro?: string;
  // Datos de nacimiento con IDs
  paisnac_id?: number;
  estado_nac_id?: number;
  municipio_nac_id?: number;
  ciudad_nac_id?: number;
  // Datos de nacimiento con nombres
  pais_nombre?: string;
  estado_nombre?: string;
  municipio_nombre?: string;
  ciudad_nombre?: string;
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
    // Datos de nacimiento con IDs
    paisnac_id: undefined,
    estado_nac_id: undefined,
    municipio_nac_id: undefined,
    ciudad_nac_id: undefined,
    // Datos de nacimiento con nombres
    pais_nombre: '',
    estado_nombre: '',
    municipio_nombre: '',
    ciudad_nombre: '',
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
  const [cargandoEstudiante, setCargandoEstudiante] = useState(false);
  const [debugPaises, setDebugPaises] = useState<any[]>([]);

  useEffect(() => {
    const cargarDatosNacimiento = async () => {
      if (estudiante) {
        setCargandoEstudiante(true);
        try {
          // 1. Cargar países (siempre)
          const paisesData = await invoke('listar_paises') as any[];
          setPaises(paisesData);
          // 2. Cargar estados si hay país
          let estadosData: any[] = [];
          if (estudiante.paisnac_id) {
            estadosData = await invoke('listar_estados_por_pais', { id_pais: Number(estudiante.paisnac_id) }) as any[];
            setEstados(estadosData);
          } else {
            setEstados([]);
          }
        } catch (error) {
          console.error('[ERROR] Error al cargar datos de nacimiento:', error);
          setEstados([]);
        } finally {
          // Forzar una actualización completa del formData
          setFormData({
            ...estudiante,
            // Asegurar que todos los campos de nacimiento estén incluidos
            paisnac_id: estudiante?.paisnac_id,
            estado_nac_id: estudiante?.estado_nac_id,
            municipio_nac_id: estudiante?.municipio_nac_id,
            ciudad_nac_id: estudiante?.ciudad_nac_id,
            pais_nombre: estudiante?.pais_nombre,
            estado_nombre: estudiante?.estado_nombre,
            municipio_nombre: estudiante?.municipio_nombre,
            ciudad_nombre: estudiante?.ciudad_nombre,
          });
          console.log('[DEBUG] formData establecido:', estudiante);
          console.log('[DEBUG] Valores de nacimiento en formData:', {
            paisnac_id: estudiante?.paisnac_id,
            estado_nac_id: estudiante?.estado_nac_id,
            municipio_nac_id: estudiante?.municipio_nac_id,
            ciudad_nac_id: estudiante?.ciudad_nac_id
          });
          console.log('[DEBUG] Verificando que los datos se asignen correctamente...');
          console.log('[DEBUG] estudiante.paisnac_id:', estudiante?.paisnac_id, 'tipo:', typeof estudiante?.paisnac_id);
          console.log('[DEBUG] estudiante.estado_nac_id:', estudiante?.estado_nac_id, 'tipo:', typeof estudiante?.estado_nac_id);
          console.log('[DEBUG] estudiante.municipio_nac_id:', estudiante?.municipio_nac_id, 'tipo:', typeof estudiante?.municipio_nac_id);
          console.log('[DEBUG] estudiante.ciudad_nac_id:', estudiante?.ciudad_nac_id, 'tipo:', typeof estudiante?.ciudad_nac_id);
          
          // Verificar si el estudiante tiene datos de nacimiento
          console.log('[DEBUG] ===== VERIFICACIÓN DE DATOS DE NACIMIENTO =====');
          console.log('[DEBUG] ¿Tiene país?:', estudiante?.paisnac_id !== null && estudiante?.paisnac_id !== undefined);
          console.log('[DEBUG] ¿Tiene estado?:', estudiante?.estado_nac_id !== null && estudiante?.estado_nac_id !== undefined);
          console.log('[DEBUG] ¿Tiene municipio?:', estudiante?.municipio_nac_id !== null && estudiante?.municipio_nac_id !== undefined);
          console.log('[DEBUG] ¿Tiene ciudad?:', estudiante?.ciudad_nac_id !== null && estudiante?.ciudad_nac_id !== undefined);
          console.log('[DEBUG] ==============================================');
          console.log('[DEBUG] Nombres de nacimiento:', {
            pais_nombre: estudiante?.pais_nombre,
            estado_nombre: estudiante?.estado_nombre,
            municipio_nombre: estudiante?.municipio_nombre,
            ciudad_nombre: estudiante?.ciudad_nombre
          });
          
          // Mostrar los datos de nacimiento que ya vienen del backend
          console.log('[DEBUG] ===== DATOS COMPLETOS DEL ESTUDIANTE =====');
          console.log('[DEBUG] Datos académicos:', {
            id_grado: estudiante?.id_grado,
            nombre_grado: estudiante?.nombre_grado,
            id_seccion: estudiante?.id_seccion,
            nombre_seccion: estudiante?.nombre_seccion,
            id_modalidad: estudiante?.id_modalidad,
            nombre_modalidad: estudiante?.nombre_modalidad
          });
          console.log('[DEBUG] Datos de nacimiento:', {
            paisnac_id: estudiante?.paisnac_id,
            estado_nac_id: estudiante?.estado_nac_id,
            municipio_nac_id: estudiante?.municipio_nac_id,
            ciudad_nac_id: estudiante?.ciudad_nac_id,
            pais_nombre: estudiante?.pais_nombre,
            estado_nombre: estudiante?.estado_nombre,
            municipio_nombre: estudiante?.municipio_nombre,
            ciudad_nombre: estudiante?.ciudad_nombre
          });
          console.log('[DEBUG] ===========================================');
          
          // Verificar si los datos se están asignando correctamente al formData
          console.log('[DEBUG] ===== VERIFICACIÓN DE FORMDATA =====');
          console.log('[DEBUG] formData después de asignar estudiante:', {
            paisnac_id: formData.paisnac_id,
            estado_nac_id: formData.estado_nac_id,
            municipio_nac_id: formData.municipio_nac_id,
            ciudad_nac_id: formData.ciudad_nac_id
          });
          console.log('[DEBUG] ===========================================');
          
          // Verificar si el estudiante tiene los datos correctos
          console.log('[DEBUG] ===== VERIFICACIÓN DEL ESTUDIANTE =====');
          console.log('[DEBUG] Estudiante recibido:', {
            id: estudiante?.id,
            paisnac_id: estudiante?.paisnac_id,
            estado_nac_id: estudiante?.estado_nac_id,
            municipio_nac_id: estudiante?.municipio_nac_id,
            ciudad_nac_id: estudiante?.ciudad_nac_id
          });
          console.log('[DEBUG] ===========================================');
          
          // Cargar datos dependientes después de asignar el estudiante
          if (estudiante?.paisnac_id) {
            console.log('[DEBUG] Cargando estados para país:', estudiante.paisnac_id);
            invoke('listar_estados_por_pais', { id_pais: Number(estudiante.paisnac_id) })
              .then((data: any) => {
                console.log('[DEBUG] Estados cargados para edición:', data);
                setEstados(data);
              })
              .catch((err) => {
                console.error('[ERROR] Error al cargar estados:', err);
                setEstados([]);
              });
          }
          
          if (estudiante?.estado_nac_id) {
            console.log('[DEBUG] Cargando municipios para estado:', estudiante.estado_nac_id);
            invoke('listar_municipios_por_estado', { id_estado: Number(estudiante.estado_nac_id) })
              .then((data: any) => {
                console.log('[DEBUG] Municipios cargados para edición:', data);
                setMunicipios(data);
              })
              .catch((err) => {
                console.error('[ERROR] Error al cargar municipios:', err);
                setMunicipios([]);
              });
          }
          
          if (estudiante?.municipio_nac_id) {
            console.log('[DEBUG] Cargando ciudades para municipio:', estudiante.municipio_nac_id);
            invoke('listar_ciudades_por_municipio', { id_municipio: Number(estudiante.municipio_nac_id) })
              .then((data: any) => {
                console.log('[DEBUG] Ciudades cargadas para edición:', data);
                setCiudades(data);
              })
              .catch((err) => {
                console.error('[ERROR] Error al cargar ciudades:', err);
                setCiudades([]);
              });
          }
          
          // Marcar como terminado después de cargar los datos dependientes
          setCargandoEstudiante(false);
        }
      }
    };

    cargarDatosNacimiento();
  }, [estudiante]);

  // Verificar cuando formData cambia
  useEffect(() => {
    console.log('[DEBUG] ===== FORMDATA CAMBIÓ =====');
    console.log('[DEBUG] Nuevo formData:', {
      paisnac_id: formData.paisnac_id,
      estado_nac_id: formData.estado_nac_id,
      municipio_nac_id: formData.municipio_nac_id,
      ciudad_nac_id: formData.ciudad_nac_id
    });
    console.log('[DEBUG] ===========================');
  }, [formData]);

  useEffect(() => {
    if (success) {
      const timer = setTimeout(() => {
        setSuccess(null);
        onGuardar();
      }, 2500);
      return () => clearTimeout(timer);
    }
  }, [success, onGuardar]);

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
    console.log('[DEBUG] Cargando países...');
    invoke('listar_paises')
      .then((data: any) => {
        console.log('[DEBUG] Países cargados:', data);
        console.log('[DEBUG] Tipo de datos de países:', typeof data, Array.isArray(data));
        console.log('[DEBUG] Detalle de países:', data.map((p: any) => ({ id: p.id, nombre: p.nombre })));
        setPaises(data);
        setDebugPaises(data); // Estado de debug
      })
      .catch((err) => {
        console.error('[ERROR] Error al cargar países:', err);
        setPaises([]);
      });
  }, []);

  // Cargar estados cuando cambia el país
  useEffect(() => {
    if (cargandoEstudiante) return; // No ejecutar si se está cargando un estudiante
    
    if (formData.paisnac_id) {
      console.log('[DEBUG] Solicitando estados para país:', formData.paisnac_id);
      invoke('listar_estados_por_pais', { id_pais: Number(formData.paisnac_id) })
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
      // Solo resetear si no se está cargando un estudiante
      if (!cargandoEstudiante) {
        setMunicipios([]);
        setCiudades([]);
        setFormData((prev) => ({ ...prev, estado_nac_id: undefined, municipio_nac_id: undefined, ciudad_nac_id: undefined }));
      }
    }
  }, [formData.paisnac_id, cargandoEstudiante]);

  // Cargar municipios cuando cambia el estado
  useEffect(() => {
    if (cargandoEstudiante) return; // No ejecutar si se está cargando un estudiante
    
    if (formData.estado_nac_id) {
      console.log('[DEBUG] Solicitando municipios para estado:', formData.estado_nac_id);
      invoke('listar_municipios_por_estado', { id_estado: Number(formData.estado_nac_id) })
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
      // Solo resetear si no se está cargando un estudiante
      if (!cargandoEstudiante) {
        setCiudades([]);
        setFormData((prev) => ({ ...prev, municipio_nac_id: undefined, ciudad_nac_id: undefined }));
      }
    }
  }, [formData.estado_nac_id, cargandoEstudiante]);

  // Cargar ciudades cuando cambia el municipio
  useEffect(() => {
    if (cargandoEstudiante) return; // No ejecutar si se está cargando un estudiante
    
    if (formData.municipio_nac_id) {
      console.log('[DEBUG] Solicitando ciudades para municipio:', formData.municipio_nac_id);
      invoke('listar_ciudades_por_municipio', { id_municipio: Number(formData.municipio_nac_id) })
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
      // Solo resetear si no se está cargando un estudiante
      if (!cargandoEstudiante) {
        setFormData((prev) => ({ ...prev, ciudad_nac_id: undefined }));
      }
    }
  }, [formData.municipio_nac_id, cargandoEstudiante]);

  // Debug: Log cuando cambia formData
  useEffect(() => {
    console.log('[DEBUG] formData actualizado:', {
      paisnac_id: formData.paisnac_id,
      estado_nac_id: formData.estado_nac_id,
      municipio_nac_id: formData.municipio_nac_id,
      ciudad_nac_id: formData.ciudad_nac_id
    });
  }, [formData.paisnac_id, formData.estado_nac_id, formData.municipio_nac_id, formData.ciudad_nac_id]);

  useEffect(() => {
    if (formData.id_grado && formData.id_modalidad) {
      invoke('obtener_id_grado_secciones', {
        grado_id: Number(formData.id_grado),
        seccion_id: Number(formData.id_seccion),
        modalidad_id: Number(formData.id_modalidad)
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
        grado_id: Number(formData.id_grado),
        seccion_id: Number(formData.id_seccion),
        modalidad_id: Number(formData.id_modalidad)
      });
      const id_grado_secciones = await invoke('obtener_id_grado_secciones', {
        grado_id: Number(formData.id_grado),
        seccion_id: Number(formData.id_seccion),
        modalidad_id: Number(formData.id_modalidad)
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
        paisnac_id: formData.paisnac_id ? Number(formData.paisnac_id) : null,
        estado_nac_id: formData.estado_nac_id ? Number(formData.estado_nac_id) : null,
        municipio_nac_id: formData.municipio_nac_id ? Number(formData.municipio_nac_id) : null,
        ciudad_nac_id: formData.ciudad_nac_id ? Number(formData.ciudad_nac_id) : null,
        id_grado: formData.id_grado ? Number(formData.id_grado) : null,
        id_seccion: formData.id_seccion ? Number(formData.id_seccion) : null,
        id_modalidad: formData.id_modalidad ? Number(formData.id_modalidad) : null,
        id_periodoactual: formData.id_periodoactual ? Number(formData.id_periodoactual) : null,
        estado: formData.estado || 'Activo',
        fecha_retiro: formData.fecha_retiro ? formData.fecha_retiro : null,
      };

      // 3. Enviar el estudiante (crear o actualizar según el caso)
      console.log('[DEBUG] Enviando estudiante al backend:', cleanData);
      
      if (estudiante && estudiante.id) {
        // Modo edición
        await invoke('actualizar_estudiante', { 
          id: estudiante.id, 
          estudiante: cleanData 
        });
        console.log('[DEBUG] Estudiante actualizado correctamente');
        mostrarMensaje('Estudiante actualizado correctamente', 'exito');
        setSuccess('Estudiante actualizado correctamente');
      } else {
        // Modo creación
        await invoke('crear_estudiante', { estudiante: cleanData });
        console.log('[DEBUG] Estudiante creado correctamente');
        mostrarMensaje('Estudiante creado correctamente', 'exito');
        setSuccess('Estudiante creado correctamente');
      }
    } catch (err) {
      console.error('[ERROR] Error al guardar estudiante:', err);
      const mensaje = estudiante && estudiante.id ? 
        'Error al actualizar el estudiante. Por favor, intente nuevamente.' :
        'Error al crear el estudiante. Por favor, intente nuevamente.';
      mostrarMensaje(mensaje, 'error');
      setError(mensaje);
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
            <h3 className="text-2xl font-semibold text-primary-600 mb-6">
              Datos de Nacimiento
              {cargandoEstudiante && (
                <span className="ml-2 text-sm text-blue-600">(Cargando datos...)</span>
              )}
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">País</label>
                <select
                  name="paisnac_id"
                  value={formData.paisnac_id?.toString() || ''}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  {/* Debug: Estado actual */}
                  {(() => {
                    console.log('[DEBUG] Renderizando dropdown países. formData.paisnac_id:', formData.paisnac_id, 'paises.length:', paises.length, 'paises:', paises);
                    console.log('[DEBUG] debugPaises.length:', debugPaises.length, 'debugPaises:', debugPaises);
                    console.log('[DEBUG] Valor del select:', formData.paisnac_id?.toString(), 'Tipo:', typeof formData.paisnac_id);
                    return null;
                  })()}

                  <option value="">Seleccione un país</option>
                  {paises.map((pais) => (
                    <option key={pais.id} value={pais.id}>{pais.nombre}</option>
                  ))}
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Estado</label>
                <select
                  name="estado_nac_id"
                  value={formData.estado_nac_id?.toString() || ''}
                  onChange={handleChange}
                  required
                  className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
                >
                  {/* Debug: Estado actual */}
                  {(() => {
                    console.log('[DEBUG] Renderizando dropdown estados. formData.estado_nac_id:', formData.estado_nac_id, 'estados.length:', estados.length, 'estados:', estados);
                    return null;
                  })()}
                  <option value="">Seleccione un estado</option>
                  {estados.map((estado) => (
                    <option key={estado.id} value={estado.id}>{estado.nombre}</option>
                  ))}
                </select>
              </div>
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Municipio</label>
                <select
                  name="municipio_nac_id"
                  value={formData.municipio_nac_id?.toString() || ''}
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
                  name="ciudad_nac_id"
                  value={formData.ciudad_nac_id?.toString() || ''}
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