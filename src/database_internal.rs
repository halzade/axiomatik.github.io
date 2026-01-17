use surrealdb::engine::any::{connect, Any};
use surrealdb::Surreal;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Database {
    pub db: RwLock<Surreal<Any>>,
}

impl Database {
    async fn new(url: &str) -> Self {
        let client = connect(url).await.unwrap();
        client
            .use_ns("axiomatik")
            .use_db("axiomatik")
            .await
            .unwrap();
        Self {
            db: RwLock::new(client),
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, Surreal<Any>> {
        self.db.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, Surreal<Any>> {
        self.db.write().await
    }
}

pub async fn init_db() -> Database {
    Database::new("rocksdb://axiomatik.db").await
}

pub async fn init_mem_db() -> Database {
    Database::new("mem://").await
}
