pub mod api;
pub mod models;
pub mod utils;

use tauri::State;
use tokio::sync::Mutex;
use tokio_postgres::NoTls;

pub struct AppState {
    pub db: Mutex<tokio_postgres::Client>,
}

pub async fn init_db() -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=edunet port=5432",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error de conexión: {}", e);
        }
    });

    Ok(client)
} 