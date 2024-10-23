pub mod errors;
pub mod model;
pub mod repository;
mod settings;

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to connect to PostgreSQL: {0}")]
    ConnectionError(sqlx::Error),
    #[error("Transaction failed: {0}")]
    TransactionError(sqlx::Error),
    #[error("Reading from DB failed: {0}")]
    ReadError(sqlx::Error),
    #[error("Writing to DB failed: {0}")]
    WriteError(sqlx::Error),
    #[error("Data integrity error: {0}")]
    DataIntegrity(String),
    #[error("Data not found: {0}")]
    NotFound(String),
    #[error("Entity already exists: {0}")]
    AlreadyExists(String),
    #[error("Cannot delete entity \"{0}\" with active references to it")]
    CannotDeleteReferenced(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Can't save password")]
    Hash,
}

pub type Result<T> = std::result::Result<T, Error>;

pub async fn connect(settings: &settings::PostgresSettings) -> Result<PgPool> {
    let conn_opts = PgConnectOptions::from_url(&settings.url).map_err(Error::ConnectionError)?;

    let mut pool_opts = PgPoolOptions::new();

    if let Some(min) = &settings.min_connections {
        pool_opts = pool_opts.min_connections(*min);
    }
    if let Some(max) = &settings.max_connections {
        pool_opts = pool_opts.max_connections(*max);
    }

    pool_opts
        .connect_with(conn_opts)
        .await
        .map_err(Error::ConnectionError)
}
