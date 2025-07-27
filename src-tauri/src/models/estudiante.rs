use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use postgres_types::{ToSql, FromSql};

#[derive(Debug, Serialize, Deserialize, Clone, ToSql, FromSql)]
#[postgres(name = "estado_estudiante")]
pub enum EstadoEstudiante {
    #[postgres(name = "activo")]
    Activo,
    #[postgres(name = "retirado")]
    Retirado,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Estudiante {
    pub id: i32,
    pub cedula: i64,
    pub nombres: String,
    pub apellidos: String,
    pub genero: Option<String>,
    pub fecha_nacimiento: NaiveDate,
    pub id_grado_secciones: Option<i32>,
    pub fecha_ingreso: Option<NaiveDate>,
    // Campos legacy para compatibilidad
    pub municipionac: Option<String>,
    pub paisnac: Option<String>,
    pub entidadfed: Option<String>,
    pub ciudadnac: Option<String>,
    pub estadonac: Option<String>,
    // Datos académicos
    pub id_grado: Option<i32>,
    pub nombre_grado: Option<String>,
    pub id_seccion: Option<i32>,
    pub nombre_seccion: Option<String>,
    pub id_modalidad: Option<i32>,
    pub nombre_modalidad: Option<String>,
    pub id_periodoactual: Option<i32>,
    pub estado: EstadoEstudiante,
    pub fecha_retiro: Option<NaiveDate>,
    // Datos de nacimiento con IDs
    pub paisnac_id: Option<i32>,
    pub estado_nac_id: Option<i32>,
    pub municipio_nac_id: Option<i32>,
    pub ciudad_nac_id: Option<i32>,
    // Datos de nacimiento con nombres
    pub pais_nombre: Option<String>,
    pub estado_nombre: Option<String>,
    pub municipio_nombre: Option<String>,
    pub ciudad_nombre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FiltroEstudiantes {
    pub cedula: Option<String>,
    pub apellidos: Option<String>,
    pub grado: Option<i32>,
    pub modalidad: Option<i32>,
    pub estado: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NuevoEstudiante {
    pub cedula: i64,
    pub nombres: String,
    pub apellidos: String,
    pub genero: Option<String>,
    pub fecha_nacimiento: NaiveDate,
    pub id_grado_secciones: Option<i32>,
    pub fecha_ingreso: Option<NaiveDate>,
    pub paisnac_id: Option<i32>,
    pub estado_nac_id: Option<i32>,
    pub municipio_nac_id: Option<i32>,
    pub ciudad_nac_id: Option<i32>,
    pub id_grado: Option<i32>,
    pub id_seccion: Option<i32>,
    pub id_modalidad: Option<i32>,
    pub id_periodoactual: Option<i32>,
    pub estado: EstadoEstudiante,
    pub fecha_retiro: Option<NaiveDate>,
} 