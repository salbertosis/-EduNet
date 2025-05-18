use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistorialAcademico {
    pub id_historial: i32,
    pub id_estudiante: i32,
    pub id_periodo: i32,
    pub id_grado_secciones: i32,
    pub promedio_anual: f64,
    pub estatus: String,
    pub fecha_registro: NaiveDateTime,
    pub periodo_escolar: Option<String>,
    pub grado: Option<String>,
    pub seccion: Option<String>,
} 