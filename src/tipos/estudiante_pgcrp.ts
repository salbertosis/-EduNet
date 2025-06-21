export interface EstudiantePgcrp {
  id_estudiante: number;
  cedula: number;
  nombres: string;
  apellidos: string;
  id_extra_catedra?: number;
  actividad_pgcrp?: string;
  id_periodo: number;
  observaciones?: string;
  fecha_asignacion?: string;
}

export interface AsignacionEstudiantePgcrp {
  id_estudiante: number;
  id_extra_catedra?: number;
  id_periodo: number;
  observaciones?: string;
}

export interface EstudiantePgcrpDetalle {
  id_estudiante: number;
  cedula: number;
  nombres: string;
  apellidos: string;
  nombre_grado: string;
  nombre_seccion: string;
  nombre_modalidad: string;
  id_extra_catedra?: number;
  actividad_pgcrp?: string;
  actividad_seccion?: string;
  observaciones?: string;
  fecha_asignacion?: string;
}

export interface ActividadPgcrp {
  id_extra_catedra: number;
  nombre: string;
}

export interface EstadisticasPgcrpSeccion {
  totalEstudiantes: number;
  conPgcrpIndividual: number;
  conPgcrpSeccion: number;
  sinAsignar: number;
  porcentajeAsignados: number;
}

// Tipos para filtros y búsquedas
export interface FiltroEstudiantePgcrp {
  texto?: string;
  tipoAsignacion?: 'todos' | 'individual' | 'seccion' | 'sin_asignar';
  actividad?: number;
}

// Tipos para reportes
export interface ReporteEstudiantePgcrp {
  seccion: string;
  estudiante: string;
  cedula: number;
  pgcrpActual: string;
  tipoAsignacion: 'Individual' | 'Por Sección' | 'Sin Asignar';
  observaciones?: string;
  fechaAsignacion?: string;
}

// Tipo para el estado de una asignación PGCRP
export interface EstadoPgcrp {
  tipo: 'Individual' | 'Por Sección' | 'Sin Asignar';
  actividad: string;
  color: string;
} 