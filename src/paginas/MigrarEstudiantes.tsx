import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../componentes/MensajeGlobalContext';

interface Modalidad {
  id_modalidad: number;
  nombre_modalidad: string;
}

interface Grado {
  id_grado: number;
  nombre_grado: string;
}

interface Seccion {
  id_seccion: number;
  nombre_seccion: string;
}

interface PeriodoEscolar {
  id_periodo: number;
  periodo_escolar: string;
  activo: boolean;
}

export function MigrarEstudiantes() {
  const [activeTab, setActiveTab] = useState<'regulares' | 'nuevos'>('regulares');

  const [modalidades, setModalidades] = useState<Modalidad[]>([]);
  const [grados, setGrados] = useState<Grado[]>([]);
  const [secciones, setSecciones] = useState<Seccion[]>([]);
  const [periodoActivo, setPeriodoActivo] = useState<PeriodoEscolar | null>(null);

  const [selectedModalidad, setSelectedModalidad] = useState('');
  const [selectedGrado, setSelectedGrado] = useState('');
  const [selectedSeccion, setSelectedSeccion] = useState('');
  const [cargando, setCargando] = useState(false);
  const { mostrarMensaje } = useMensajeGlobal();
  const [tipoMigracion, setTipoMigracion] = useState('promovidos');

  // Cargar catálogos al montar
  useEffect(() => {
    const cargarCatalogos = async () => {
      try {
        const periodos: PeriodoEscolar[] = await invoke('listar_periodos_escolares');
        const activo = periodos.find(p => p.activo);
        setPeriodoActivo(activo || null);

        const modalidadesData: Modalidad[] = await invoke('listar_modalidades');
        setModalidades(modalidadesData);

        const gradosData: Grado[] = await invoke('listar_grados');
        setGrados(gradosData);
      } catch (error) {
        mostrarMensaje('Error al cargar catálogos: ' + error, 'error');
      }
    };
    cargarCatalogos();
  }, []);

  // Cargar secciones cuando cambian modalidad, grado o periodoActivo
  useEffect(() => {
    const cargarSecciones = async () => {
      if (selectedModalidad && selectedGrado && periodoActivo) {
        try {
          const seccionesData: any[] = await invoke('obtener_secciones_por_grado_modalidad_periodo', {
            id_grado: Number(selectedGrado),
            id_modalidad: Number(selectedModalidad),
            id_periodo: periodoActivo.id_periodo
          });
          setSecciones(seccionesData);
        } catch (error) {
          console.error('Error al cargar secciones:', error);
          setSecciones([]);
        }
      } else {
        setSecciones([]);
      }
    };
    cargarSecciones();
  }, [selectedModalidad, selectedGrado, periodoActivo]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setCargando(true);
    try {
      await invoke('migrar_estudiantes', {
        modalidad: selectedModalidad ? Number(selectedModalidad) : null,
        grado: selectedGrado ? Number(selectedGrado) : null,
        seccion: selectedSeccion ? Number(selectedSeccion) : null,
        tipo: tipoMigracion
      });
      mostrarMensaje('Migración de estudiantes completada con éxito', 'exito');
    } catch (err) {
      mostrarMensaje('Error al migrar estudiantes: ' + err, 'error');
    } finally {
      setCargando(false);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-[60vh]">
      <div className="bg-white dark:bg-dark-800 rounded-2xl shadow-lg p-8 w-full max-w-md">
        {/* Pestañas */}
        <div className="flex mb-8 border-b border-gray-200 dark:border-dark-600">
          <button
            className={`px-4 py-2 font-semibold focus:outline-none transition-colors border-b-2 ${activeTab === 'regulares' ? 'border-blue-600 text-blue-600' : 'border-transparent text-gray-500 dark:text-gray-300'}`}
            onClick={() => setActiveTab('regulares')}
          >
            Estudiantes Regulares
          </button>
          <button
            className={`ml-4 px-4 py-2 font-semibold focus:outline-none transition-colors border-b-2 ${activeTab === 'nuevos' ? 'border-blue-600 text-blue-600' : 'border-transparent text-gray-500 dark:text-gray-300'}`}
            onClick={() => setActiveTab('nuevos')}
          >
            Estudiantes Nuevos
          </button>
        </div>
        {/* Contenido de pestañas */}
        {activeTab === 'regulares' && (
          <form onSubmit={handleSubmit} className="space-y-3">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Modalidad</label>
              <select
                value={selectedModalidad}
                onChange={e => {
                  setSelectedModalidad(e.target.value);
                  setSelectedGrado('');
                  setSelectedSeccion('');
                }}
                className="w-full px-3 py-1.5 rounded-md border border-gray-300 dark:border-dark-600 bg-gray-50 dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-400 focus:border-blue-400 transition-all text-sm"
                required
              >
                <option value="">Seleccione una modalidad</option>
                {modalidades.map(m => (
                  <option key={m.id_modalidad} value={m.id_modalidad}>{m.nombre_modalidad}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Grado</label>
              <select
                value={selectedGrado}
                onChange={e => {
                  setSelectedGrado(e.target.value);
                  setSelectedSeccion('');
                }}
                className="w-full px-3 py-1.5 rounded-md border border-gray-300 dark:border-dark-600 bg-gray-50 dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-400 focus:border-blue-400 transition-all text-sm"
                required
                disabled={!selectedModalidad}
              >
                <option value="">Seleccione un grado</option>
                {grados.map(g => (
                  <option key={g.id_grado} value={g.id_grado}>{g.nombre_grado}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sección</label>
              <select
                value={selectedSeccion}
                onChange={e => setSelectedSeccion(e.target.value)}
                className="w-full px-3 py-1.5 rounded-md border border-gray-300 dark:border-dark-600 bg-gray-50 dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-400 focus:border-blue-400 transition-all text-sm"
                required
                disabled={!selectedModalidad || !selectedGrado || secciones.length === 0}
              >
                <option value="">Seleccione una sección</option>
                {secciones.map(s => (
                  <option key={s.id_seccion} value={s.id_seccion}>{s.nombre_seccion}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo de migración</label>
              <select
                value={tipoMigracion}
                onChange={e => setTipoMigracion(e.target.value)}
                className="w-full px-3 py-1.5 rounded-md border border-gray-300 dark:border-dark-600 bg-gray-50 dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-400 focus:border-blue-400 transition-all text-sm"
                required
              >
                <option value="promovidos">Solo promovidos</option>
                <option value="repitientes">Solo repitientes</option>
              </select>
            </div>
            <button
              type="submit"
              disabled={cargando}
              className="w-full py-2 bg-blue-600 text-white rounded-md font-semibold hover:bg-blue-700 transition-colors text-base mt-2"
            >
              {cargando ? 'Migrando...' : 'Migrar Estudiantes'}
            </button>
          </form>
        )}
        {activeTab === 'nuevos' && (
          <MigrarEstudiantesNuevosUI modalidades={modalidades} grados={grados} periodoActivo={periodoActivo} />
        )}
      </div>
    </div>
  );
}

function MigrarEstudiantesNuevosUI({ modalidades, grados, periodoActivo }: { modalidades: Modalidad[]; grados: Grado[]; periodoActivo: PeriodoEscolar | null }) {
  const [selectedModalidad, setSelectedModalidad] = useState('');
  const [selectedGrado, setSelectedGrado] = useState('');
  const [secciones, setSecciones] = useState<Seccion[]>([]);
  const [selectedSeccion, setSelectedSeccion] = useState('');
  const [cargando, setCargando] = useState(false);
  const [mensaje, setMensaje] = useState<string | null>(null);
  const { mostrarMensaje } = useMensajeGlobal();

  useEffect(() => {
    if (selectedModalidad && selectedGrado && periodoActivo) {
      const cargarSecciones = async () => {
        try {
          const seccionesData: Seccion[] = await invoke('obtener_secciones_por_grado_modalidad_periodo', {
            id_grado: Number(selectedGrado),
            id_modalidad: Number(selectedModalidad),
            id_periodo: periodoActivo.id_periodo
          });
          setSecciones(seccionesData);
        } catch (error) {
          setSecciones([]);
        }
      };
      cargarSecciones();
    } else {
      setSecciones([]);
    }
  }, [selectedModalidad, selectedGrado, periodoActivo]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setMensaje(null);
    setCargando(true);
    try {
      if (!periodoActivo) {
        setMensaje('No hay período activo seleccionado.');
        setCargando(false);
        return;
      }
      // Obtener id_grado_secciones
      const id_grado_secciones = await invoke<number>('obtener_id_grado_secciones', {
        grado_id: Number(selectedGrado),
        seccion_id: Number(selectedSeccion),
        modalidad_id: Number(selectedModalidad)
      });
      if (!id_grado_secciones) {
        setMensaje('No se encontró la combinación de grado, sección y modalidad.');
        setCargando(false);
        return;
      }
      const insertados = await invoke<number>('migrar_estudiantes_nuevos', {
        id_grado_secciones,
        id_periodo: periodoActivo.id_periodo
      });
      setMensaje(`¡Migración exitosa! Se migraron ${insertados} estudiantes nuevos.`);
      mostrarMensaje('Migración de estudiantes nuevos completada con éxito', 'exito');
    } catch (err: any) {
      setMensaje('Error al migrar estudiantes nuevos: ' + (err?.toString() ?? 'Error desconocido'));
      mostrarMensaje('Error al migrar estudiantes nuevos: ' + (err?.toString() ?? 'Error desconocido'), 'error');
    }
    setCargando(false);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-3">
      <div>
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Modalidad</label>
        <select
          value={selectedModalidad}
          onChange={e => {
            setSelectedModalidad(e.target.value);
            setSelectedGrado('');
            setSelectedSeccion('');
          }}
          className="w-full px-3 py-1.5 rounded-md border border-gray-300 dark:border-dark-600 bg-gray-50 dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-400 focus:border-blue-400 transition-all text-sm"
          required
        >
          <option value="">Seleccione una modalidad</option>
          {modalidades.map(m => (
            <option key={m.id_modalidad} value={m.id_modalidad}>{m.nombre_modalidad}</option>
          ))}
        </select>
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Grado</label>
        <select
          value={selectedGrado}
          onChange={e => {
            setSelectedGrado(e.target.value);
            setSelectedSeccion('');
          }}
          className="w-full px-3 py-1.5 rounded-md border border-gray-300 dark:border-dark-600 bg-gray-50 dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-400 focus:border-blue-400 transition-all text-sm"
          required
          disabled={!selectedModalidad}
        >
          <option value="">Seleccione un grado</option>
          {grados.map(g => (
            <option key={g.id_grado} value={g.id_grado}>{g.nombre_grado}</option>
          ))}
        </select>
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sección</label>
        <select
          value={selectedSeccion}
          onChange={e => setSelectedSeccion(e.target.value)}
          className="w-full px-3 py-1.5 rounded-md border border-gray-300 dark:border-dark-600 bg-gray-50 dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-400 focus:border-blue-400 transition-all text-sm"
          required
          disabled={!selectedModalidad || !selectedGrado || secciones.length === 0}
        >
          <option value="">Seleccione una sección</option>
          {secciones.map(s => (
            <option key={s.id_seccion} value={s.id_seccion}>{s.nombre_seccion}</option>
          ))}
        </select>
      </div>
      <button
        type="submit"
        disabled={cargando}
        className="w-full py-2 bg-blue-600 text-white rounded-md font-semibold hover:bg-blue-700 transition-colors text-base mt-2"
      >
        {cargando ? 'Migrando...' : 'Migrar Estudiantes Nuevos'}
      </button>
      {mensaje && <div className="mt-4 text-center text-sm text-blue-700 dark:text-blue-300">{mensaje}</div>}
    </form>
  );
} 