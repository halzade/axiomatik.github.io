use std::convert::Infallible;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use thiserror::Error;
use tokio::sync::RwLock;

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

    #[error("record not found in table {0} by key {1}")]
    RecordNotFound(String, String),
}

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

pub async fn initialize_database() -> Result<DatabaseSurreal, SurrealError> {
    Ok(DatabaseSurreal::new(DATABASE_DEV).await?)
}

pub async fn initialize_in_memory_database() -> Result<DatabaseSurreal, SurrealError> {
    Ok(DatabaseSurreal::new(DATABASE_TEST).await?)
}
