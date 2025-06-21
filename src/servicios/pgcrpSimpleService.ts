import { invoke } from '@tauri-apps/api/tauri';
import { 
  ActividadPgcrp, 
  AsignacionPgcrpInput, 
  AsignacionPgcrpCompleta 
} from '../tipos/pgcrp_simple';

// Obtener todas las actividades PGCRP disponibles
export async function obtenerActividadesPgcrpSimple(): Promise<ActividadPgcrp[]> {
  return await invoke('obtener_actividades_pgcrp_simple');
}

// Asignar actividad PGCRP a una sección
export async function asignarPgcrpSeccionSimple(input: AsignacionPgcrpInput): Promise<void> {
  return await invoke('asignar_pgcrp_seccion_simple', { input });
}

// Obtener la asignación PGCRP actual de una sección
export async function obtenerPgcrpSeccion(
  id_grado_secciones: number, 
  id_periodo: number
): Promise<AsignacionPgcrpCompleta | null> {
  return await invoke('obtener_pgcrp_seccion', { 
    id_grado_secciones: id_grado_secciones, 
    id_periodo: id_periodo 
  });
}

// Eliminar asignación PGCRP de una sección
export async function eliminarPgcrpSeccion(
  id_grado_secciones: number, 
  id_periodo: number
): Promise<void> {
  return await invoke('eliminar_pgcrp_seccion', { 
    id_grado_secciones: id_grado_secciones, 
    id_periodo: id_periodo 
  });
} 