use tokio_postgres::{NoTls, Client};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::settings::get_connection_string;
use crate::utils::error::{AppError, AppResult};

pub type DbPool = Arc<Mutex<Client>>;

pub async fn init_db() -> AppResult<DbPool> {
    let connection_string = get_connection_string();
    
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Error al conectar a la base de datos: {}", e)))?;

    // Spawn the connection on a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error en la conexión a la base de datos: {}", e);
        }
    });

    Ok(Arc::new(Mutex::new(client)))
} 