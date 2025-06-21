import { invoke } from '@tauri-apps/api/tauri';
import { EstudiantePgcrpDetalle, ActividadPgcrp, AsignacionEstudiantePgcrp } from '../tipos/estudiante_pgcrp';

export const estudiantePgcrpService = {
  /**
   * Obtiene la lista de estudiantes de una sección con sus asignaciones PGCRP
   */
  async obtenerEstudiantesSeccionPgcrp(
    idGradoSecciones: number,
    idPeriodo: number
  ): Promise<EstudiantePgcrpDetalle[]> {
    try {
      const resultado = await invoke('obtener_estudiantes_seccion_pgcrp', {
        idGradoSecciones,
        idPeriodo,
      });
      return resultado as EstudiantePgcrpDetalle[];
    } catch (error) {
      console.error('Error al obtener estudiantes PGCRP:', error);
      throw new Error('Error al cargar los estudiantes');
    }
  },

  /**
   * Obtiene la lista de actividades PGCRP disponibles
   */
  async obtenerActividadesPgcrp(): Promise<ActividadPgcrp[]> {
    try {
      const resultado = await invoke('obtener_actividades_pgcrp_estudiante');
      return resultado as ActividadPgcrp[];
    } catch (error) {
      console.error('Error al obtener actividades PGCRP:', error);
      throw new Error('Error al cargar las actividades');
    }
  },

  /**
   * Asigna o actualiza una actividad PGCRP para un estudiante individual
   */
  async asignarPgcrpEstudiante(asignacion: AsignacionEstudiantePgcrp): Promise<string> {
    try {
      const resultado = await invoke('asignar_pgcrp_estudiante_individual', {
        asignacion,
      });
      return resultado as string;
    } catch (error) {
      console.error('Error al asignar PGCRP:', error);
      throw new Error('Error al asignar la actividad PGCRP');
    }
  },

  /**
   * Elimina la asignación PGCRP individual de un estudiante
   */
  async eliminarPgcrpEstudiante(idEstudiante: number, idPeriodo: number): Promise<string> {
    try {
      const resultado = await invoke('eliminar_pgcrp_estudiante_individual', {
        idEstudiante,
        idPeriodo,
      });
      return resultado as string;
    } catch (error) {
      console.error('Error al eliminar asignación PGCRP:', error);
      throw new Error('Error al eliminar la asignación PGCRP');
    }
  },

  /**
   * Exporta datos de estudiantes PGCRP a CSV
   */
  async exportarEstudiantesPgcrpCSV(
    idGradoSecciones: number,
    idPeriodo: number,
    nombreGrado: string,
    nombreSeccion: string
  ): Promise<string> {
    try {
      const estudiantes = await this.obtenerEstudiantesSeccionPgcrp(idGradoSecciones, idPeriodo);
      
      const headers = [
        'Cédula',
        'Nombres',
        'Apellidos',
        'Grado',
        'Sección',
        'PGCRP Individual',
        'PGCRP por Sección',
        'Observaciones',
        'Fecha Asignación'
      ];

      const filas = estudiantes.map(estudiante => [
        estudiante.cedula.toString(),
        estudiante.nombres,
        estudiante.apellidos,
        estudiante.nombre_grado,
        estudiante.nombre_seccion,
        estudiante.actividad_pgcrp || '',
        estudiante.actividad_seccion || '',
        estudiante.observaciones || '',
        estudiante.fecha_asignacion 
          ? new Date(estudiante.fecha_asignacion).toLocaleDateString()
          : ''
      ]);

      const csvContent = [
        headers.join(','),
        ...filas.map(fila => fila.map(campo => `"${campo}"`).join(','))
      ].join('\n');

      // Crear y descargar archivo
      const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
      const link = document.createElement('a');
      const url = URL.createObjectURL(blob);
      link.setAttribute('href', url);
      link.setAttribute('download', `estudiantes_pgcrp_${nombreGrado}_${nombreSeccion}.csv`);
      link.style.visibility = 'hidden';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);

      return 'Exportación completada exitosamente';
    } catch (error) {
      console.error('Error al exportar datos:', error);
      throw new Error('Error al exportar los datos');
    }
  },

  /**
   * Obtiene estadísticas de asignaciones PGCRP por sección
   */
  async obtenerEstadisticasPgcrpSeccion(
    idGradoSecciones: number,
    idPeriodo: number
  ): Promise<{
    totalEstudiantes: number;
    conPgcrpIndividual: number;
    conPgcrpSeccion: number;
    sinAsignar: number;
    porcentajeAsignados: number;
  }> {
    try {
      const estudiantes = await this.obtenerEstudiantesSeccionPgcrp(idGradoSecciones, idPeriodo);
      
      const totalEstudiantes = estudiantes.length;
      const conPgcrpIndividual = estudiantes.filter(e => e.actividad_pgcrp).length;
      const conPgcrpSeccion = estudiantes.filter(e => !e.actividad_pgcrp && e.actividad_seccion).length;
      const sinAsignar = estudiantes.filter(e => !e.actividad_pgcrp && !e.actividad_seccion).length;
      const porcentajeAsignados = totalEstudiantes > 0 
        ? ((totalEstudiantes - sinAsignar) / totalEstudiantes) * 100 
        : 0;

      return {
        totalEstudiantes,
        conPgcrpIndividual,
        conPgcrpSeccion,
        sinAsignar,
        porcentajeAsignados: Math.round(porcentajeAsignados * 100) / 100
      };
    } catch (error) {
      console.error('Error al obtener estadísticas:', error);
      throw new Error('Error al calcular estadísticas');
    }
  }
}; 