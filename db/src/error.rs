use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("DATABASE_URL must be set")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Error connecting to database")]
    ConnectionError(#[from] diesel::ConnectionError),
    #[error("Database query Error")]
    QueryError(#[from] diesel::result::Error),
}
