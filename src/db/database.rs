use crate::db::database_internal;
use crate::db::database_internal::DatabaseSurreal;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use thiserror::Error;
use tokio::sync::{OnceCell, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database not initialized")]
    NotInitialized,

    #[error("SurrealDB error: {0}")]
    Surreal(#[from] surrealdb::Error),
}

static DATABASE: OnceCell<DatabaseSurreal> = OnceCell::const_new();

/*
 * Technical Methods
 */

pub async fn initialize_database() {
    DATABASE.get_or_init(database_internal::init_db).await;
}

pub async fn initialize_in_memory_database() {
    DATABASE.get_or_init(database_internal::init_mem_db).await;
}

pub(crate) async fn db_read<'lt>() -> Result<RwLockReadGuard<'lt, Surreal<Any>>, DatabaseError> {
    let sdb = DATABASE.get().ok_or(DatabaseError::NotInitialized)?;
    Ok(sdb.db.read().await)
}

pub(crate) async fn db_write<'lt>() -> Result<RwLockWriteGuard<'lt, Surreal<Any>>, DatabaseError> {
    let sdb = DATABASE.get().ok_or(DatabaseError::NotInitialized)?;
    Ok(sdb.db.write().await)
}
