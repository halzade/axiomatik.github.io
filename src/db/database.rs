use std::convert::Infallible;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use thiserror::Error;

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

// TODO need to sign into the database

#[derive(Debug)]
pub struct DatabaseSurreal {
    /*
     * surreal<Any> is already internally synchronized
     * no connection pool, acts as a shared async client
     * any is for {Local, Remote}, but Local already implements connection
     * initialize from
     * - main - for production, or
     * - trust::me - for tests
     */
    pub db: Surreal<Any>,
}

impl DatabaseSurreal {
    async fn new(path: &str) -> surrealdb::Result<Self> {
        // infer db engine
        let surreal = surrealdb::engine::any::connect(path).await?;
        surreal.use_ns("axiomatik").use_db("axiomatik").await?;

        Ok(Self { db: surreal })
    }
}

impl From<Infallible> for SurrealError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub async fn init_db_connection() -> Result<DatabaseSurreal, SurrealError> {
    let surreal = DatabaseSurreal::new(DATABASE_DEV).await?;
    prepare_as_if_empty(&surreal).await?;
    Ok(surreal)
}

/*
 * only for tests
 */
pub async fn init_in_memory_db_connection() -> Result<DatabaseSurreal, SurrealError> {
    let surreal = DatabaseSurreal::new(DATABASE_TEST).await?;
    prepare_as_if_empty(&surreal).await?;
    Ok(surreal)
}

async fn prepare_as_if_empty(surreal: &DatabaseSurreal) -> Result<(), SurrealError> {
    surreal.db
        .query(r#"
        DEFINE TABLE article SCHEMALESS;
        
        DEFINE INDEX user ON article FIELDS user;
        DEFINE INDEX article_file_name ON article FIELDS article_file_name UNIQUE;
        
        DEFINE TABLE article_update_status SCHEMALESS;
        DEFINE INDEX article_file_name ON article_update_status FIELDS article_file_name UNIQUE;
        "#,)
        .await?;
    Ok(())
}
