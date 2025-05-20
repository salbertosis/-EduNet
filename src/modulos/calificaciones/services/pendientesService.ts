import { invoke } from '@tauri-apps/api/tauri';

export async function guardarPendientesAPI(idEstudiante: number, pendientes: any[]) {
  try {
    return await invoke('guardar_asignaturas_pendientes', { idEstudiante, pendientes });
  } catch (error: any) {
    let mensaje = 'Ocurrió un error al guardar las asignaturas pendientes. Por favor, intenta nuevamente.';
    if (error && error.message) {
      if (
        error.message.includes('llave duplicada') ||
        error.message.includes('violación de unicidad') ||
        error.message.includes('duplicate key')
      ) {
        // Intentar obtener el nombre de la asignatura del primer pendiente
        let nombre = '';
        if (pendientes && pendientes.length > 0 && pendientes[0].nombre_asignatura) {
          nombre = pendientes[0].nombre_asignatura;
        }
        mensaje = nombre
          ? `La asignatura "${nombre}" ya está registrada como pendiente para este estudiante en el periodo y año escolar actual.`
          : 'Una de las asignaturas ya está registrada como pendiente para este estudiante en el periodo y año escolar actual.';
      } else {
        mensaje = `Error al guardar pendientes: ${error.message}`;
      }
    }
    throw new Error(mensaje);
  }
}

export async function cargarPendientesAPI(idEstudiante: number) {
  try {
    return await invoke('obtener_asignaturas_pendientes_estudiante', { idEstudiante });
  } catch (error: any) {
    let mensaje = 'Ocurrió un error al cargar las asignaturas pendientes. Por favor, intenta nuevamente.';
    if (error && error.message) {
      mensaje = `Error al cargar pendientes: ${error.message}`;
    }
    throw new Error(mensaje);
  }
} 