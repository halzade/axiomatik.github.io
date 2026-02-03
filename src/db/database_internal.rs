use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

const DATABASE: &str = "surrealkv://axiomatik.db";
const DATABASE_TEST: &str = "surrealkv://axiomatik.test";

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
    DatabaseSurreal::new(DATABASE_TEST)
        .await
        .expect("Failed to initialize SurrealKV test database")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connects_and_query() {
        let db = init_mem_db().await;
        let s = db.read().await;
        let res = s.query("RETURN 1").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_create_read_delete() {
        let db = init_mem_db().await;

        // write
        let w1 = db.write().await;
        w1.create("test_entity")
            .content(serde_json::to_value("ok").ok())
            .await
            .expect("Failed to create ok entity");

        // read
        let r1 = db.read().await;
        let v = r1
            .query("test_entity")
            .await
            .expect("Failed to read ok entity")
            .take(0).ok();

        assert_eq!(v, Some("ok"));

        // delete
        let w2 = db.write().await;
        w2.delete("test_entity")
            .await
            .expect("Failed to delet ok entity");

        // cleaned up
        let r1 = db.read().await;
        let v = r1
            .query("test_entity")
            .await
            .expect("Failed to read ok entity")
            .take(0).is_err();

        // assert_eq!(v, None);
    }
}
