use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pgcrp {
    pub id_pgcrp: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub activo: bool,
    pub fecha_creacion: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PgcrpAsignacionSeccion {
    pub id_asignacion_seccion: i32,
    pub id_grado_secciones: i32,
    pub id_pgcrp: i32,
    pub id_periodo: i32,
    pub fecha_asignacion: Option<NaiveDateTime>,
    pub activo: bool,
    // Campos adicionales para consultas
    pub nombre_pgcrp: Option<String>,
    pub grado: Option<String>,
    pub seccion: Option<String>,
    pub modalidad: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PgcrpAsignacionEstudiante {
    pub id_asignacion_estudiante: i32,
    pub id_estudiante: i32,
    pub id_pgcrp: i32,
    pub id_periodo: i32,
    pub fecha_asignacion: Option<NaiveDateTime>,
    pub activo: bool,
    pub observaciones: Option<String>,
    // Campos adicionales para consultas
    pub nombre_pgcrp: Option<String>,
    pub nombres_estudiante: Option<String>,
    pub apellidos_estudiante: Option<String>,
    pub cedula_estudiante: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PgcrpEstudianteVista {
    pub id_estudiante: i32,
    pub cedula: i64,
    pub nombres: String,
    pub apellidos: String,
    pub id_grado_secciones: i32,
    pub id_periodo: i32,
    pub id_pgcrp_asignado: Option<i32>,
    pub nombre_pgcrp_asignado: Option<String>,
    pub tipo_asignacion: String, // 'individual', 'seccion', 'sin_asignar'
    pub observaciones_individuales: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AsignacionSeccionInput {
    pub id_grado_secciones: i32,
    pub id_pgcrp: i32,
    pub id_periodo: i32,
}

#[derive(Debug, Deserialize)]
pub struct AsignacionEstudianteInput {
    pub id_estudiante: i32,
    pub id_pgcrp: i32,
    pub id_periodo: i32,
    pub observaciones: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PgcrpInput {
    pub nombre: String,
    pub descripcion: Option<String>,
}

// Estructura para reportes y estadísticas
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PgcrpEstadisticas {
    pub total_estudiantes: i32,
    pub estudiantes_con_asignacion: i32,
    pub estudiantes_sin_asignacion: i32,
    pub asignaciones_por_seccion: i32,
    pub asignaciones_individuales: i32,
    pub actividades_utilizadas: Vec<ActividadEstadistica>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActividadEstadistica {
    pub id_pgcrp: i32,
    pub nombre_actividad: String,
    pub total_estudiantes: i32,
    pub asignaciones_seccion: i32,
    pub asignaciones_individuales: i32,
}

// Estructura para el reporte de actividades extracátedra
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportePgcrpEstudiante {
    pub cedula: i64,
    pub nombres: String,
    pub apellidos: String,
    pub grado: String,
    pub seccion: String,
    pub actividad_pgcrp: String,
    pub tipo_asignacion: String,
    pub observaciones: Option<String>,
} 