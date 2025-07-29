use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActividadReciente {
    pub id: i32,
    pub tipo_actividad: String,
    pub descripcion: String,
    pub usuario: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub id_estudiante: Option<i32>,
    pub id_docente: Option<i32>,
    pub id_periodo: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NuevaActividad {
    pub tipo_actividad: String,
    pub descripcion: String,
    pub usuario: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub id_estudiante: Option<i32>,
    pub id_docente: Option<i32>,
    pub id_periodo: Option<i32>,
}

impl ActividadReciente {
    pub async fn crear(
        db: &tokio_postgres::Client,
        nueva_actividad: NuevaActividad,
    ) -> Result<ActividadReciente, tokio_postgres::Error> {
        println!("[DEBUG] Creando actividad en base de datos: {}", nueva_actividad.descripcion);
        
        let row = db
            .query_one(
                r#"
                INSERT INTO actividad_reciente 
                (tipo_actividad, descripcion, usuario, metadata, id_estudiante, id_docente, id_periodo)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, tipo_actividad, descripcion, usuario, metadata, timestamp, id_estudiante, id_docente, id_periodo
                "#,
                &[
                    &nueva_actividad.tipo_actividad,
                    &nueva_actividad.descripcion,
                    &nueva_actividad.usuario,
                    &nueva_actividad.metadata,
                    &nueva_actividad.id_estudiante,
                    &nueva_actividad.id_docente,
                    &nueva_actividad.id_periodo,
                ],
            )
            .await?;

        let actividad = ActividadReciente::from_row(&row)?;
        println!("[DEBUG] Actividad creada exitosamente con ID: {}", actividad.id);
        Ok(actividad)
    }

    pub async fn obtener_recientes(
        db: &tokio_postgres::Client,
        limite: i64,
    ) -> Result<Vec<ActividadReciente>, tokio_postgres::Error> {
        let rows = db
            .query(
                r#"
                SELECT id, tipo_actividad, descripcion, usuario, metadata, timestamp, id_estudiante, id_docente, id_periodo
                FROM actividad_reciente
                ORDER BY timestamp DESC
                LIMIT $1
                "#,
                &[&limite],
            )
            .await?;

        let mut actividades = Vec::new();
        for row in rows {
            actividades.push(ActividadReciente::from_row(&row)?);
        }

        Ok(actividades)
    }

    pub async fn limpiar_antiguas(db: &tokio_postgres::Client) -> Result<u64, tokio_postgres::Error> {
        let result = db
            .execute(
                r#"
                DELETE FROM actividad_reciente 
                WHERE timestamp < CURRENT_TIMESTAMP - INTERVAL '30 days'
                "#,
                &[],
            )
            .await?;

        Ok(result)
    }

    fn from_row(row: &Row) -> Result<ActividadReciente, tokio_postgres::Error> {
        // Convertir el timestamp de PostgreSQL a DateTime<Utc>
        let timestamp_raw: chrono::NaiveDateTime = row.try_get("timestamp")?;
        let timestamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(timestamp_raw, chrono::Utc);
        
        Ok(ActividadReciente {
            id: row.try_get("id")?,
            tipo_actividad: row.try_get("tipo_actividad")?,
            descripcion: row.try_get("descripcion")?,
            usuario: row.try_get("usuario")?,
            metadata: row.try_get("metadata")?,
            timestamp,
            id_estudiante: row.try_get("id_estudiante")?,
            id_docente: row.try_get("id_docente")?,
            id_periodo: row.try_get("id_periodo")?,
        })
    }
} 