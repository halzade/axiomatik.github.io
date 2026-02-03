use serde::{Deserialize, Serialize};

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
    async fn test_create_read_delete() -> Result<()> {
        // --- 1. Init in‑memory SurrealDB ---
        let db = Surreal::<Mem>::new(()).await?;

        // Use a namespace/database (required, otherwise queries fail)
        db.use_ns("test").use_db("test").await?;

        // --- 2. Clean up any existing data ---
        let _: Vec<Trivial> = db.delete(Resource::from("test_entity")).await?;

        // --- 3. CREATE ---
        let x = Trivial::new("hello");
        let created: Option<Trivial> = db
            .create(Resource::from(("test_entity", "1")))
            .content(x.clone())
            .await?;
        assert_eq!(created, Some(x.clone()));

        // --- 4. READ ---
        let read: Option<Trivial> = db.select(Resource::from(("test_entity", "1"))).await?;
        assert_eq!(read, Some(x.clone()));

        // --- 5. DELETE ---
        let deleted: Option<Trivial> = db.delete(Resource::from(("test_entity", "1"))).await?;
        assert_eq!(deleted, Some(x.clone()));

        // --- 6. VERIFY NONE ---
        let check: Option<Trivial> = db.select(Resource::from(("test_entity", "1"))).await?;
        assert!(check.is_none());

        Ok(())
    }
}