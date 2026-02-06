use std::convert::Infallible;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use thiserror::Error;
use tokio::sync::RwLock;
use crate::system::configuration;
use crate::system::configuration::Mode;

const DATABASE_DEV: &str = "rocksdb://axiomatik.db";
const DATABASE_TEST: &str = "mem://";

#[derive(Debug, Error)]
pub enum SurrealError {
    #[error("Database not initialized")]
    NotInitialized,

    #[error("SurrealDB error: {0}")]
    Surreal(#[from] surrealdb::Error),

    #[error("SurrealDB infallible {0}")]
    SurrealInfallible(Infallible),

    #[error("invalid statement")]
    InvalidStatement,

    #[error("application run mode not set")]
    UnknownApplicationRunMode,

    #[error("record not found in table {0} by key {1}")]
    RecordNotFound(String, String),
}

#[derive(Debug)]
pub struct DatabaseSurreal {
    // Any is for {Local, Remote}
    pub db: RwLock<Surreal<Any>>,
}

impl DatabaseSurreal {
    async fn new(path: &str) -> surrealdb::Result<Self> {
        // infer db engine
        let surreal = surrealdb::engine::any::connect(path).await?;
        surreal.use_ns("axiomatik").use_db("axiomatik").await?;

        Ok(Self { db: RwLock::new(surreal) })
    }

    pub async fn db_write(&self) -> Result<Surreal<Any>, SurrealError> {
        Ok(self.db.write().await.clone())
    }

    pub async fn db_read(&self) -> Result<Surreal<Any>, SurrealError> {
        Ok(self.db.read().await.clone())
    }
}

impl From<Infallible> for SurrealError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub async fn init_db_connection() -> Result<DatabaseSurreal, SurrealError> {
    Ok(DatabaseSurreal::new(DATABASE_DEV).await?)
}

pub async fn init_in_memory_db_connection() -> Result<DatabaseSurreal, SurrealError> {
    Ok(DatabaseSurreal::new(DATABASE_TEST).await?)
}

pub async fn db_by_mode() -> Result<DatabaseSurreal, SurrealError> {
    match configuration::MODE.get() {
        None => {
            Err(SurrealError::UnknownApplicationRunMode)
        }
        Some(mode) => {
            match mode {
                Mode::Testing => {
                    init_in_memory_db_connection().await
                }
                Mode::ApplicationRun => {
                    init_db_connection().await
                }
            }
        }
    }

}