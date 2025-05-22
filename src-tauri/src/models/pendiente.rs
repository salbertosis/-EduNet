use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AsignaturaPendiente {
    pub id_pendiente: i32,
    pub id_estudiante: i32,
    pub id_asignatura: i32,
    pub id_periodo: i32,
    pub grado: String,
    pub cal_momento1: Option<i32>,
    pub cal_momento2: Option<i32>,
    pub cal_momento3: Option<i32>,
    pub cal_momento4: Option<i32>,
    pub estado: String,
    pub fecha_registro: NaiveDateTime,
    pub id_grado_secciones: i32,
    pub nombre_asignatura: Option<String>,
    pub periodo_escolar: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AsignaturaPendienteInput {
    pub id_asignatura: i32,
    pub id_periodo: i32,
    pub revision: Option<i32>,
} 