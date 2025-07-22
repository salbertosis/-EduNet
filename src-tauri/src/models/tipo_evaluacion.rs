use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TipoEvaluacion {
    pub id: i32,
    pub codigo: String,
    pub nombre: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RespuestaTiposEvaluacion {
    pub exito: bool,
    pub mensaje: String,
    pub tipos_evaluacion: Option<Vec<TipoEvaluacion>>,
}

impl RespuestaTiposEvaluacion {
    pub fn exito(tipos_evaluacion: Vec<TipoEvaluacion>) -> Self {
        Self {
            exito: true,
            mensaje: "Tipos de evaluación obtenidos exitosamente".to_string(),
            tipos_evaluacion: Some(tipos_evaluacion),
        }
    }

    pub fn error(mensaje: String) -> Self {
        Self {
            exito: false,
            mensaje,
            tipos_evaluacion: None,
        }
    }
} 