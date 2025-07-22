use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Institucion {
    pub id: Option<i32>,
    pub codigo: String,
    pub denominacion: String,
    pub direccion: String,
    pub telefono: Option<String>,
    pub municipio: String,
    pub entidad_federal: String,
    pub cdcee: Option<String>,
    pub director: String,
    pub cedula_director: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Institucion {
    pub fn new(
        codigo: String,
        denominacion: String,
        direccion: String,
        telefono: Option<String>,
        municipio: String,
        entidad_federal: String,
        cdcee: Option<String>,
        director: String,
        cedula_director: String,
    ) -> Self {
        Self {
            id: None,
            codigo,
            denominacion,
            direccion,
            telefono,
            municipio,
            entidad_federal,
            cdcee,
            director,
            cedula_director,
            created_at: None,
            updated_at: None,
        }
    }
} 