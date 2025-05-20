use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalificacionEstudiante {
    pub id_calificacion: Option<i32>,
    pub id_asignatura: i32,
    pub nombre_asignatura: String,
    pub lapso_1: Option<i32>,
    pub lapso_1_ajustado: Option<i32>,
    pub lapso_2: Option<i32>,
    pub lapso_2_ajustado: Option<i32>,
    pub lapso_3: Option<i32>,
    pub lapso_3_ajustado: Option<i32>,
    pub nota_final: Option<i32>,
    pub revision: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CalificacionInput {
    pub id_calificacion: Option<i32>,
    pub id_estudiante: i32,
    pub id_asignatura: i32,
    pub id_periodo: i32,
    pub lapso_1: Option<i32>,
    pub lapso_1_ajustado: Option<i32>,
    pub lapso_2: Option<i32>,
    pub lapso_2_ajustado: Option<i32>,
    pub lapso_3: Option<i32>,
    pub lapso_3_ajustado: Option<i32>,
    pub revision: Option<i32>,
    pub nota_final: Option<i32>,
} 