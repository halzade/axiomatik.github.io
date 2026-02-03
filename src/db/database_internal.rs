use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

const DATABASE: &str = "surrealkv://axiomatik.db";

pub struct DatabaseSurreal {
    // Any is for {Local, Remote}
    pub db: RwLock<Surreal<Any>>,
}

impl DatabaseSurreal {
    pub async fn new(path: &str) -> surrealdb::Result<Self> {
        // Connect using the URI scheme, SurrealDB infers engine from the string.
        let db = surrealdb::engine::any::connect(path).await?;
        // Select namespace and database; this returns metadata, keep the handle in `db`.
        db.use_ns("axiomatik").use_db("axiomatik").await?;

        Ok(Self { db: RwLock::new(db) })
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, Surreal<Any>> {
        self.db.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, Surreal<Any>> {
        self.db.write().await
    }
}

pub async fn init_db() -> DatabaseSurreal {
    DatabaseSurreal::new(DATABASE)
        .await
        .expect("Failed to initialize SurrealKV database")
}

pub async fn init_mem_db() -> DatabaseSurreal {
    DatabaseSurreal::new("mem://")
        .await
        .expect("Failed to initialize in-memory SurrealKV database")
}
