use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SurrealValue)]
struct Trivial {
    pub key: String,
    pub value: String,
}

impl Trivial {
    fn new(key: &str, value: &str) -> Self {
        Self { key: key.into(), value: value.into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::test_001_db::Trivial;
    use axiomatik_web::db::database::init_in_memory_db_connection;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_connects_and_query() -> Result<(), TrustError> {
        let dbs = init_in_memory_db_connection().await?;
        let res = dbs.db.query("RETURN 1").await?;
        assert!(res.check().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_update_read_delete() -> Result<(), TrustError> {
        let dbs = init_in_memory_db_connection().await?;
        {
            // 1. Create
            let new = Trivial::new("secret", "ok");
            let created_o: Option<Trivial> = dbs.db.create(("trivial", "1")).content(new).await?;

            let created = created_o.unwrap();
            assert_eq!(created.value, "ok");
            assert_eq!(created.key, "secret");
        }
        {
            // 2. Read
            let res = dbs.db.select(("trivial", "1")).await?;

            let read_o: Option<Trivial> = res.unwrap();
            let read = read_o.unwrap();
            assert_eq!(read.value, "ok");
            assert_eq!(read.key, "secret");
        }
        {
            // 3. Update
            let update_with = Trivial::new("secret 2", "ok updated");
            let updated: Option<Trivial> =
                dbs.db.update(("trivial", "1")).content(update_with).await?;

            let updated = updated.unwrap();
            assert_eq!(updated.key, "secret 2");
            assert_eq!(updated.value, "ok updated");
        }
        {
            // 4. Read again
            let res2 = dbs.db.select(("trivial", "1")).await?;

            let read_o2: Option<Trivial> = res2.unwrap();
            let read2 = read_o2.unwrap();
            assert_eq!(read2.value, "ok updated");
            assert_eq!(read2.key, "secret 2");
        }
        {
            // 5. Delete
            let deleted: Option<Trivial> = dbs.db.delete(("trivial", "1")).await?;

            let deleted = deleted.unwrap();
            assert_eq!(deleted.key, "secret 2"); // or whatever the key was before
            assert_eq!(deleted.value, "ok updated");
        }
        {
            // 6. Verify deletion
            let res3: Option<Trivial> = dbs.db.select(("trivial", "1")).await?;
            // was deleted
            assert!(res3.is_none());
        }
        Ok(())
    }
}
