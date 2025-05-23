use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Docente {
    pub id_docente: i32,
    pub cedula: i64,
    pub apellidos: String,
    pub nombres: String,
    pub especialidad: String,
    pub telefono: Option<String>,
    pub correoelectronico: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FiltroDocentes {
    pub cedula: Option<String>,
    pub apellidos: Option<String>,
    pub especialidad: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NuevoDocente {
    pub cedula: i64,
    pub apellidos: String,
    pub nombres: String,
    pub especialidad: String,
    pub telefono: Option<String>,
    pub correoelectronico: Option<String>,
} 