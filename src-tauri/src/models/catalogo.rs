use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeriodoEscolar {
    pub id_periodo: i32,
    pub periodo_escolar: String,
    pub activo: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Grado {
    pub id_grado: i32,
    pub nombre_grado: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Modalidad {
    pub id_modalidad: i32,
    pub nombre_modalidad: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asignatura {
    pub id_asignatura: i32,
    pub nombre_asignatura: String,
    pub id_grado: i32,
    pub id_modalidad: i32,
    pub id_docente: Option<i32>,
    pub nombres_docente: Option<String>,
    pub apellidos_docente: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeccionCatalogo {
    pub id_seccion: i32,
    pub nombre_seccion: String,
}

#[derive(Debug, Serialize)]
pub struct SeccionCompleta {
    pub id_seccion: i32,
    pub nombre_seccion: String,
    #[serde(rename = "idGradoSecciones")]
    pub id_grado_secciones: i32,
} 