use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EstudiantePgcrp {
    pub id_estudiante: i32,
    pub cedula: i64,
    pub nombres: String,
    pub apellidos: String,
    pub id_extra_catedra: Option<i32>,
    pub actividad_pgcrp: Option<String>,
    pub id_periodo: i32,
    pub observaciones: Option<String>,
    pub fecha_asignacion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsignacionEstudiantePgcrp {
    pub id_estudiante: i32,
    pub id_extra_catedra: Option<i32>,
    pub id_periodo: i32,
    pub observaciones: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstudiantePgcrpDetalle {
    pub id_estudiante: i32,
    pub cedula: i64,
    pub nombres: String,
    pub apellidos: String,
    pub nombre_grado: String,
    pub nombre_seccion: String,
    pub nombre_modalidad: String,
    pub id_extra_catedra: Option<i32>, // ID de la actividad PGCRP individual
    pub actividad_pgcrp: Option<String>,
    pub actividad_seccion: Option<String>, // PGCRP asignado por sección
    pub observaciones: Option<String>,
    pub fecha_asignacion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActividadPgcrp {
    pub id_extra_catedra: i32,
    pub nombre: String,
} 