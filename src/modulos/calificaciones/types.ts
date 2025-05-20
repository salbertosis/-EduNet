// Tipos centralizados para el módulo de calificaciones

export interface CalificacionEstudiante {
  id_calificacion?: number;
  id_asignatura: number;
  nombre_asignatura: string;
  lapso_1?: number;
  lapso_1_ajustado?: number;
  lapso_2?: number;
  lapso_2_ajustado?: number;
  lapso_3?: number;
  lapso_3_ajustado?: number;
  nota_final?: number;
  revision?: string;
}

export interface Asignatura {
  id_asignatura: number;
  nombre_asignatura: string;
  id_grado: number;
  id_modalidad: number;
}

export interface HistorialAcademico {
  id_historial: number;
  id_estudiante: number;
  id_periodo: number;
  id_grado_secciones: number;
  promedio_anual: number;
  estatus: string;
  fecha_registro: string;
  periodo_escolar?: string | null;
  grado?: string | null;
  seccion?: string | null;
} 