// Tipos para el sistema PGCRP (Actividades Extracátedra)

export interface Pgcrp {
  id_pgcrp: number;
  nombre: string;
  descripcion?: string;
  activo: boolean;
  fecha_creacion?: string;
}

export interface PgcrpAsignacionSeccion {
  id_asignacion_seccion: number;
  id_grado_secciones: number;
  id_pgcrp: number;
  id_periodo: number;
  fecha_asignacion?: string;
  activo: boolean;
  // Campos adicionales para consultas
  nombre_pgcrp?: string;
  grado?: string;
  seccion?: string;
  modalidad?: string;
}

export interface PgcrpAsignacionEstudiante {
  id_asignacion_estudiante: number;
  id_estudiante: number;
  id_pgcrp: number;
  id_periodo: number;
  fecha_asignacion?: string;
  activo: boolean;
  observaciones?: string;
  // Campos adicionales para consultas
  nombre_pgcrp?: string;
  nombres_estudiante?: string;
  apellidos_estudiante?: string;
  cedula_estudiante?: number;
}

export interface PgcrpEstudianteVista {
  id_estudiante: number;
  cedula: number;
  nombres: string;
  apellidos: string;
  id_grado_secciones: number;
  id_periodo: number;
  id_pgcrp_asignado?: number;
  nombre_pgcrp_asignado?: string;
  tipo_asignacion: 'individual' | 'seccion' | 'sin_asignar';
  observaciones_individuales?: string;
}

export interface AsignacionSeccionInput {
  id_grado_secciones: number;
  id_pgcrp: number;
  id_periodo: number;
}

export interface AsignacionEstudianteInput {
  id_estudiante: number;
  id_pgcrp: number;
  id_periodo: number;
  observaciones?: string;
}

export interface PgcrpInput {
  nombre: string;
  descripcion?: string;
}

// Tipos para reportes y estadísticas
export interface PgcrpEstadisticas {
  total_estudiantes: number;
  estudiantes_con_asignacion: number;
  estudiantes_sin_asignacion: number;
  asignaciones_por_seccion: number;
  asignaciones_individuales: number;
  actividades_utilizadas: ActividadEstadistica[];
}

export interface ActividadEstadistica {
  id_pgcrp: number;
  nombre_actividad: string;
  total_estudiantes: number;
  asignaciones_seccion: number;
  asignaciones_individuales: number;
}

export interface ReportePgcrpEstudiante {
  cedula: number;
  nombres: string;
  apellidos: string;
  grado: string;
  seccion: string;
  actividad_pgcrp: string;
  tipo_asignacion: string;
  observaciones?: string;
}

// Tipos para filtros y selecciones
export interface FiltrosPgcrp {
  id_periodo?: number;
  id_grado?: number;
  id_modalidad?: number;
  id_grado_secciones?: number;
}

// Tipos para la interfaz de gestión
export interface EstadoGestionPgcrp {
  actividades: Pgcrp[];
  asignacionesSeccion: PgcrpAsignacionSeccion[];
  asignacionesEstudiante: PgcrpAsignacionEstudiante[];
  estudiantesVista: PgcrpEstudianteVista[];
  estadisticas?: PgcrpEstadisticas;
  cargando: boolean;
  error?: string;
} 