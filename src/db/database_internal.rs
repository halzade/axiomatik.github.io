use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Trivial {
    value: String,
}

impl Trivial {
    fn new(value: String) -> Self {
        Self { value }
    }
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

        // --- 1. Delete all test_entity records ---
        info!("1 - cleaning old records");
        {
            let w0 = db.write().await;
            let _: Vec<serde_json::Value> = w0
                .delete("test_entity")
                .await
                .expect("Failed to delete test_entity");
        }

        // --- 2. Create a new record ---
        info!("2 - creating a record");

        let write_trivial: Trivial = Trivial::new("ok".into());
        let created_json: serde_json::Value = {
            let w1 = db.write().await;
            w1.create(("test_entity", "1"))
                .content(
                    serde_json::to_value(&write_trivial)
                        .expect("Failed to convert Trivial to JSON"),
                )
                .await
                .expect("Failed to create record")
                .expect("Record was not created")
        };

        // --- 3. Convert JSON to Trivial struct ---
        let created_trivial: Trivial = serde_json::from_value(created_json.clone())
            .expect("Failed to deserialize JSON into Trivial");
        info!("Created record: {:?}", created_trivial);

        assert_eq!(created_trivial, Trivial::new("ok".into()));

        // --- 4. Read the record back ---
        info!("3 - reading the record");
        let read_json: serde_json::Value = {
            let r = db.read().await;
            r.select(("test_entity", "1"))
                .await
                .expect("Failed to read record")
                .expect("Record not found")
        };

        let read_trivial: Trivial =
            serde_json::from_value(read_json.clone()).expect("Deserialize read record");
        assert_eq!(read_trivial, Trivial::new("ok".into()));

        // --- 5. Delete the record ---
        info!("4 - deleting the record");
        {
            let w2 = db.write().await;
            let _: Option<serde_json::Value> = w2
                .delete(("test_entity", "1"))
                .await
                .expect("Failed to delete record");
        }

        // --- 6. Verify deletion ---
        let check_deleted: Option<serde_json::Value> = {
            let r = db.read().await;
            r.select(("test_entity", "1"))
                .await
                .expect("Failed to read after delete")
        };
        assert!(check_deleted.is_none());
        info!("Test completed successfully");
    }
}
