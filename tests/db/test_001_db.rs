use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SurrealValue)]
struct Trivial {
    pub key: String,
    pub value: String,
}

impl Trivial {
    fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::test_001_db::Trivial;
    use axiomatik_web::db::database;
    use axiomatik_web::db::database::ini_in_memory_db_connection;
    use axiomatik_web::trust::script_base::TrustError;

    #[tokio::test]
    async fn test_connects_and_query() -> Result<(), TrustError> {
        ini_in_memory_db_connection().await?;
        let r = database::db_write().await?;
        let res = r.query("RETURN 1").await?;
        assert!(res.check().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_update_read_delete() -> Result<(), TrustError> {
        ini_in_memory_db_connection().await?;
        {
            // 1. Create
            let w1 = database::db_write().await?;
            let new = Trivial::new("secret", "ok");
            let created_o: Option<Trivial> = w1.create(("trivial", "1")).content(new).await?;

            let created = created_o.unwrap();
            assert_eq!(created.value, "ok");
            assert_eq!(created.key, "secret");
        }
        {
            // 2. Read
            let r1 = database::db_read().await?;
            let res = r1.select(("trivial", "1")).await?;

            let read_o: Option<Trivial> = res.unwrap();
            let read = read_o.unwrap();
            assert_eq!(read.value, "ok");
            assert_eq!(read.key, "secret");
        }
        {
            // 3. Update
            let w2 = database::db_write().await?;
            let update_with = Trivial::new("secret 2", "ok updated");
            let updated: Option<Trivial> = w2.update(("trivial", "1")).content(update_with).await?;

            let updated = updated.unwrap();
            assert_eq!(updated.key, "secret 2");
            assert_eq!(updated.value, "ok updated");
        }
        {
            // 4. Read again
            let r2 = database::db_read().await?;
            let res2 = r2.select(("trivial", "1")).await?;

            let read_o2: Option<Trivial> = res2.unwrap();
            let read2 = read_o2.unwrap();
            assert_eq!(read2.value, "ok updated");
            assert_eq!(read2.key, "secret 2");
        }
        {
            // 5. Delete
            let w3 = database::db_write().await?;
            let deleted: Option<Trivial> = w3.delete(("trivial", "1")).await?;

            let deleted = deleted.unwrap();
            assert_eq!(deleted.key, "secret 2"); // or whatever the key was before
            assert_eq!(deleted.value, "ok updated");
        }
        {
            // 6. Verify deletion
            let r3 = database::db_read().await?;
            let res3: Option<Trivial> = r3.select(("trivial", "1")).await?;
            // was deleted
            assert!(res3.is_none());
        }
        Ok(())
    }
}
