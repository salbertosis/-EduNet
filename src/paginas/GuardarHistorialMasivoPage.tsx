import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../componentes/MensajeGlobalContext';

interface PeriodoEscolar {
  id_periodo: number;
  periodo_escolar: string;
  activo: boolean;
}

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

const GuardarHistorialMasivoPage = () => {
  const [periodos, setPeriodos] = useState<PeriodoEscolar[]>([]);
  const [modalidades, setModalidades] = useState<Modalidad[]>([]);
  const [grados, setGrados] = useState<Grado[]>([]);
  const [secciones, setSecciones] = useState<Seccion[]>([]);

  const [selectedPeriodo, setSelectedPeriodo] = useState<string>('');
  const [selectedModalidad, setSelectedModalidad] = useState<string>('');
  const [selectedGrado, setSelectedGrado] = useState<string>('');
  const [selectedSeccion, setSelectedSeccion] = useState<string>('');

  const { mostrarMensaje } = useMensajeGlobal();

  // Cargar períodos, modalidades, grados y secciones
  useEffect(() => {
    const cargarCatalogos = async () => {
      try {
        const periodosData: PeriodoEscolar[] = await invoke('listar_periodos_escolares');
        setPeriodos(periodosData);

        const modalidadesData: Modalidad[] = await invoke('listar_modalidades');
        setModalidades(modalidadesData);

        const gradosData: Grado[] = await invoke('listar_grados');
        setGrados(gradosData);

        // Asumimos que inicialmente no hay selección para cargar secciones
        // Las secciones se cargarán dinámicamente según grado y modalidad seleccionados
      } catch (error) {
        console.error('Error al cargar catálogos:', error);
        mostrarMensaje(`Error al cargar catálogos: ${error}`, 'error');
      }
    };
    cargarCatalogos();
  }, []);

  useEffect(() => {
    const cargarSecciones = async () => {
      if (selectedGrado && selectedModalidad) {
        try {
          const seccionesData: Seccion[] = await invoke('obtener_secciones_por_grado_modalidad_periodo', {
            idGrado: Number(selectedGrado),
            idModalidad: Number(selectedModalidad),
            idPeriodo: Number(selectedPeriodo) // Aunque no lo usemos para filtrar secciones en sí, lo mandamos si es requerido
          });
          setSecciones(seccionesData);
        } catch (error) {
          console.error('Error al cargar secciones:', error);
          mostrarMensaje(`Error al cargar secciones: ${error}`, 'error');
        }
      } else {
        setSecciones([]);
      }
    };
    cargarSecciones();
  }, [selectedGrado, selectedModalidad, selectedPeriodo]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      // Convertir a null si no hay selección para los Optional<i32> en Rust
      const idModalidad = selectedModalidad ? Number(selectedModalidad) : null;
      const idGrado = selectedGrado ? Number(selectedGrado) : null;
      const idSeccion = selectedSeccion ? Number(selectedSeccion) : null; // 'todas' es vacío, se convierte a null
      const idPeriodo = selectedPeriodo ? Number(selectedPeriodo) : null; // Periodo es requerido, pero por consistencia con otros filtros

      if (!idPeriodo) { // Asegurarse de que el período siempre esté seleccionado
        mostrarMensaje('Debe seleccionar un período escolar.', 'advertencia');
        return;
      }

      const result: string = await invoke('guardar_historial_masivo', {
        idModalidad,
        idGrado,
        idSeccion,
      });

      mostrarMensaje(result, 'exito');
      // Limpiar el formulario o resetear selecciones si es necesario
      setSelectedModalidad('');
      setSelectedGrado('');
      setSelectedSeccion('');
      // No reseteamos el período porque suele ser el punto de partida
    } catch (error) {
      console.error('Error al guardar historial masivo:', error);
      mostrarMensaje(`Error al guardar historial masivo: ${error}`, 'error');
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">Guardar Historial Académico Masivo</h1>
      <form onSubmit={handleSubmit} className="bg-white dark:bg-dark-800 p-6 rounded-lg shadow-md">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <div>
            <label htmlFor="periodo" className="block text-gray-700 dark:text-gray-300 text-sm font-bold mb-2">Período Escolar:</label>
            <select
              id="periodo"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline bg-gray-100 dark:bg-dark-700 border-gray-300 dark:border-dark-600"
              value={selectedPeriodo}
              onChange={(e) => setSelectedPeriodo(e.target.value)}
              required
            >
              <option value="">Seleccione un período</option>
              {periodos.map((p) => (
                <option key={p.id_periodo} value={p.id_periodo}>
                  {p.periodo_escolar} {p.activo ? '(Activo)' : ''}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label htmlFor="modalidad" className="block text-gray-700 dark:text-gray-300 text-sm font-bold mb-2">Modalidad:</label>
            <select
              id="modalidad"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline bg-gray-100 dark:bg-dark-700 border-gray-300 dark:border-dark-600"
              value={selectedModalidad}
              onChange={(e) => setSelectedModalidad(e.target.value)}
            >
              <option value="">Todas las Modalidades</option>
              {modalidades.map((m) => (
                <option key={m.id_modalidad} value={m.id_modalidad}>
                  {m.nombre_modalidad}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label htmlFor="grado" className="block text-gray-700 dark:text-gray-300 text-sm font-bold mb-2">Grado:</label>
            <select
              id="grado"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline bg-gray-100 dark:bg-dark-700 border-gray-300 dark:border-dark-600"
              value={selectedGrado}
              onChange={(e) => setSelectedGrado(e.target.value)}
            >
              <option value="">Todos los Grados</option>
              {grados.map((g) => (
                <option key={g.id_grado} value={g.id_grado}>
                  {g.nombre_grado}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label htmlFor="seccion" className="block text-gray-700 dark:text-gray-300 text-sm font-bold mb-2">Sección:</label>
            <select
              id="seccion"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline bg-gray-100 dark:bg-dark-700 border-gray-300 dark:border-dark-600"
              value={selectedSeccion}
              onChange={(e) => setSelectedSeccion(e.target.value)}
              disabled={!selectedGrado || !selectedModalidad}
            >
              <option value="">Todas las Secciones</option>
              {secciones.map((s) => (
                <option key={s.id_seccion} value={s.id_seccion}>
                  {s.nombre_seccion}
                </option>
              ))}
            </select>
          </div>
        </div>
        <button
          type="submit"
          className="bg-emerald-600 hover:bg-emerald-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
        >
          Guardar Historial
        </button>
      </form>
    </div>
  );
};

export default GuardarHistorialMasivoPage; 