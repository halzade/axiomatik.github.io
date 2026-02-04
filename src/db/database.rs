use crate::db::database::SurrealError::NotInitialized;
use lazy_static::lazy_static;
use std::convert::Infallible;
use std::sync::Arc;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};

const DATABASE_DEV: &str = "rocksdb://axiomatik.db";
const DATABASE_TEST: &str = "mem://";

lazy_static! {
    pub static ref DATABASE: OnceCell<Arc<DatabaseSurreal>> = OnceCell::const_new();
}

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
    pub async fn new(path: &str) -> surrealdb::Result<Self> {
        // infer db engine
        let surreal = surrealdb::engine::any::connect(path).await?;
        surreal.use_ns("axiomatik").use_db("axiomatik").await?;

        Ok(Self {
            db: RwLock::new(surreal),
        })
    }
}

async fn init(db_path: &str) -> Arc<DatabaseSurreal> {
    Arc::new(
        DatabaseSurreal::new(db_path)
            .await
            .expect("Failed to initialize database"),
    )
}

impl From<Infallible> for SurrealError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub async fn initialize_database() -> Result<(), SurrealError> {
    DATABASE
        .get_or_init(|| async { init(DATABASE_DEV).await })
        .await;
    Ok(())
}

pub async fn initialize_in_memory_database() -> Result<(), SurrealError> {
    DATABASE
        .get_or_init(|| async { init(DATABASE_TEST).await })
        .await;
    Ok(())
}

pub async fn db_write() -> Result<Surreal<Any>, SurrealError> {
    let sdb = db()?;
    Ok(sdb.db.write().await.clone())
}

pub async fn db_read() -> Result<Surreal<Any>, SurrealError> {
    let sdb = db()?;
    Ok(sdb.db.read().await.clone())
}

fn db() -> Result<Arc<DatabaseSurreal>, SurrealError> {
    match DATABASE.get() {
        None => Err(NotInitialized),
        Some(db) => Ok(Arc::clone(db)),
    }
}
