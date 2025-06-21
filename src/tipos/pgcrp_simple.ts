// Tipos simples para PGCRP por sección

export interface ActividadPgcrp {
  id_pgcrp: number;
  nombre: string;
}

export interface AsignacionPgcrpSeccion {
  id_grado_secciones: number;
  id_pgcrp: number;
  id_periodo: number;
  fecha_asignacion?: string;
}

export interface AsignacionPgcrpCompleta {
  id_grado_secciones: number;
  id_pgcrp: number;
  id_periodo: number;
  nombre_actividad: string;
  fecha_asignacion?: string;
}

export interface AsignacionPgcrpInput {
  id_grado_secciones: number;
  id_pgcrp: number;
  id_periodo: number;
} 