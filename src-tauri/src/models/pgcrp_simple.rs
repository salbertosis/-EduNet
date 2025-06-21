use serde::{Deserialize, Serialize};

// Modelo para actividades PGCRP (tabla pgcrp)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActividadPgcrp {
    pub id_pgcrp: i32,
    pub nombre: String,
}

// Modelo para asignación de PGCRP por sección
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AsignacionPgcrpSeccion {
    pub id_grado_secciones: i32,
    pub id_pgcrp: i32,
    pub id_periodo: i32,
    pub fecha_asignacion: Option<String>,
}

// Modelo completo con información de la actividad
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AsignacionPgcrpCompleta {
    pub id_grado_secciones: i32,
    pub id_pgcrp: i32,
    pub id_periodo: i32,
    pub nombre_actividad: String,
    pub fecha_asignacion: Option<String>,
}

// Input para crear asignación
#[derive(Debug, Deserialize)]
pub struct AsignacionPgcrpInput {
    pub id_grado_secciones: i32,
    pub id_pgcrp: i32,
    pub id_periodo: i32,
} 