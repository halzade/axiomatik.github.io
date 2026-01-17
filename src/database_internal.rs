use surrealdb::engine::any::{connect, Any};
use surrealdb::Surreal;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct DatabaseSurreal {
    pub db: RwLock<Surreal<Any>>,
}

impl DatabaseSurreal {
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

pub async fn init_db() -> DatabaseSurreal {
    DatabaseSurreal::new("rocksdb://axiomatik.db").await
}

pub async fn init_mem_db() -> DatabaseSurreal {
    DatabaseSurreal::new("mem://").await
}
