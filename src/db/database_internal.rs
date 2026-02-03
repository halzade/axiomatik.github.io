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
    use tracing::info;

    #[tokio::test]
    async fn test_connects_and_query() {
        let db = init_mem_db().await;
        let s = db.read().await;
        let res = s.query("RETURN 1").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_create_read_delete() {
        info!("test_create_read_delete()");
        let db = init_mem_db().await;

        info!("1");
        // Delete all test_entity records before starting
        {
            let w0 = db.write().await;
            let _: Vec<surrealdb::types::Value> = w0
                .delete("test_entity")
                .await
                .expect("Failed to delete all test_entity before test");
            // w0 dropped here
        }

        info!("2");
        // write
        let created: surrealdb::types::Value = {
            let w1 = db.write().await;

            let res: Option<surrealdb::types::Value> = w1
                .create(("test_entity", "1"))
                .content(serde_json::json!({ "value": "ok" }))
                .await
                .expect("Failed to create ok entity");

            res.expect("Expected created record")
            // w1 dropped here
        };

        let created_json: serde_json::Value =
            serde_json::to_value(&created).expect("serialize created Value to JSON");

        assert_eq!(
            created_json.get("value").and_then(|v| v.as_str()),
            Some("ok")
        );

        info!("4");
        // delete
        {
            let w2 = db.write().await;
            let _: Option<surrealdb::types::Value> = w2
                .delete(("test_entity", "1"))
                .await
                .expect("Failed to delete ok entity");
            // w2 dropped here
        }

        // cleaned up
        let v2: Option<surrealdb::types::Value> = {
            let r2 = db.read().await;
            let res: Option<surrealdb::types::Value> = r2
                .select(("test_entity", "1"))
                .await
                .expect("Failed to read ok entity");
            // r2 dropped here
            res
        };

        assert!(v2.is_none());
    }
}
