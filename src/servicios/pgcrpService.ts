import { invoke } from '@tauri-apps/api/tauri';
import type {
  Pgcrp,
  PgcrpAsignacionSeccion,
  PgcrpAsignacionEstudiante,
  PgcrpEstudianteVista,
  PgcrpEstadisticas,
  ReportePgcrpEstudiante,
  AsignacionSeccionInput,
  AsignacionEstudianteInput,
  PgcrpInput,
} from '../tipos/pgcrp';

// ===== GESTIÓN DE ACTIVIDADES PGCRP =====

export async function obtenerActividadesPgcrp(): Promise<Pgcrp[]> {
  return await invoke('obtener_actividades_pgcrp');
}

export async function crearActividadPgcrp(input: PgcrpInput): Promise<number> {
  return await invoke('crear_actividad_pgcrp', { input });
}

export async function actualizarActividadPgcrp(id_pgcrp: number, input: PgcrpInput): Promise<void> {
  return await invoke('actualizar_actividad_pgcrp', { idPgcrp: id_pgcrp, input });
}

export async function eliminarActividadPgcrp(id_pgcrp: number): Promise<void> {
  return await invoke('eliminar_actividad_pgcrp', { idPgcrp: id_pgcrp });
}

// ===== GESTIÓN DE ASIGNACIONES POR SECCIÓN =====

export async function obtenerAsignacionesSeccion(id_periodo?: number): Promise<PgcrpAsignacionSeccion[]> {
  return await invoke('obtener_asignaciones_seccion', { idPeriodo: id_periodo });
}

export async function asignarPgcrpSeccion(input: AsignacionSeccionInput): Promise<number> {
  return await invoke('asignar_pgcrp_seccion', { input });
}

export async function eliminarAsignacionSeccion(id_asignacion_seccion: number): Promise<void> {
  return await invoke('eliminar_asignacion_seccion', { idAsignacionSeccion: id_asignacion_seccion });
}

// ===== GESTIÓN DE ASIGNACIONES POR ESTUDIANTE =====

export async function obtenerAsignacionesEstudiante(
  id_periodo?: number,
  id_grado_secciones?: number
): Promise<PgcrpAsignacionEstudiante[]> {
  return await invoke('obtener_asignaciones_estudiante', {
    idPeriodo: id_periodo,
    idGradoSecciones: id_grado_secciones,
  });
}

export async function asignarPgcrpEstudiante(input: AsignacionEstudianteInput): Promise<number> {
  return await invoke('asignar_pgcrp_estudiante', { input });
}

export async function eliminarAsignacionEstudiante(id_asignacion_estudiante: number): Promise<void> {
  return await invoke('eliminar_asignacion_estudiante', { idAsignacionEstudiante: id_asignacion_estudiante });
}

// ===== CONSULTAS Y REPORTES =====

export async function obtenerEstudiantesConPgcrp(
  id_grado_secciones: number,
  id_periodo: number
): Promise<PgcrpEstudianteVista[]> {
  return await invoke('obtener_estudiantes_con_pgcrp', {
    idGradoSecciones: id_grado_secciones,
    idPeriodo: id_periodo,
  });
}

export async function obtenerEstadisticasPgcrp(id_periodo: number): Promise<PgcrpEstadisticas> {
  return await invoke('obtener_estadisticas_pgcrp', { idPeriodo: id_periodo });
}

export async function generarReportePgcrp(
  id_periodo: number,
  id_grado?: number,
  id_modalidad?: number
): Promise<ReportePgcrpEstudiante[]> {
  return await invoke('generar_reporte_pgcrp', {
    idPeriodo: id_periodo,
    idGrado: id_grado,
    idModalidad: id_modalidad,
  });
}

// ===== FUNCIONES AUXILIARES =====

export function obtenerColorTipoAsignacion(tipo: string): string {
  switch (tipo) {
    case 'individual':
      return 'bg-blue-100 text-blue-800';
    case 'seccion':
      return 'bg-green-100 text-green-800';
    case 'sin_asignar':
      return 'bg-red-100 text-red-800';
    default:
      return 'bg-gray-100 text-gray-800';
  }
}

export function obtenerTextoTipoAsignacion(tipo: string): string {
  switch (tipo) {
    case 'individual':
      return 'Individual';
    case 'seccion':
      return 'Por Sección';
    case 'sin_asignar':
      return 'Sin Asignar';
    default:
      return 'Desconocido';
  }
}

export function formatearFecha(fecha?: string): string {
  if (!fecha) return 'N/A';
  return new Date(fecha).toLocaleDateString('es-ES', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

// Función para exportar reportes a CSV
export function exportarReporteCSV(reporte: ReportePgcrpEstudiante[], nombreArchivo: string = 'reporte_pgcrp.csv'): void {
  const headers = [
    'Cédula',
    'Nombres',
    'Apellidos',
    'Grado',
    'Sección',
    'Actividad PGCRP',
    'Tipo Asignación',
    'Observaciones',
  ];

  const csvContent = [
    headers.join(','),
    ...reporte.map(row => [
      row.cedula,
      `"${row.nombres}"`,
      `"${row.apellidos}"`,
      `"${row.grado}"`,
      `"${row.seccion}"`,
      `"${row.actividad_pgcrp}"`,
      `"${obtenerTextoTipoAsignacion(row.tipo_asignacion)}"`,
      `"${row.observaciones || ''}"`,
    ].join(','))
  ].join('\n');

  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
  const link = document.createElement('a');
  const url = URL.createObjectURL(blob);
  link.setAttribute('href', url);
  link.setAttribute('download', nombreArchivo);
  link.style.visibility = 'hidden';
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
} 