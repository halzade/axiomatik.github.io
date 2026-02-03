use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    use axiomatik_web::db::database_internal::init_db_test;

    #[tokio::test]
    async fn test_connects_and_query() {
        let db_surreal = init_db_test().await;
        let db = db_surreal.db.read().await;
        let res = db.query("RETURN 1").await.unwrap();
        assert!(res.check().is_ok());
    }

    #[tokio::test]
    async fn test_create_read_delete() -> Result<(), Box<dyn std::error::Error>> {
        // --- 1. Init in‑memory SurrealDB ---
        let db = init_db_test().await;

        // --- 3. CREATE ---
        let x = Trivial::new("hello".to_string());
        let id = db
            .write_struct("test_entity", Some("1"), &x)
            .await
            .map_err(|e| format!("{:?}", e))?;
        assert!(id.contains("test_entity:1") || id.contains("1"));

        // --- 4. READ ---
        let read: Trivial = db
            .read_by_id("test_entity", "1")
            .await
            .map_err(|e| format!("{:?}", e))?;
        assert_eq!(read, x);

        // --- 5. DELETE ---
        {
            let db_inner = db.db.write().await;
            let _: Option<serde_json::Value> = db_inner.delete(("test_entity", "1")).await?;
        }

        // --- 6. VERIFY NONE ---
        let check: Result<Trivial, _> = db.read_by_id("test_entity", "1").await;
        assert!(check.is_err());

        Ok(())
    }
}
