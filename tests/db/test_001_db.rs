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
    use axiomatik_web::db::database::initialize_in_memory_database;

    #[tokio::test]
    async fn test_connects_and_query() -> Result<(), database::SurrealError> {
        initialize_in_memory_database().await?;
        let r = database::db_write().await?;
        let res = r.query("RETURN 1").await?;
        assert!(res.check().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_update_read_delete() -> Result<(), database::SurrealError> {
        initialize_in_memory_database().await?;

        let key = "1";
        let table = "trivial";

        {
            // 1. Create
            let w1 = database::db_write().await.unwrap();
            let new = Trivial::new(key, "ok");
            w1.create("trivial").content(new).await.unwrap();
        }
        {
            // 2. Read
            let r1 = database::db_read().await.unwrap();
            let res = r1.select(("trivial", "1")).await.unwrap();
            let mut read: Trivial = res.unwrap();
            assert_eq!(read.value, "ok");
            assert_eq!(read.key, "1");
        }
        // {
        //     // 3. Update
        //     let w2 = database::db_write().await?;
        //     w2.update("trivial").content({}).await?;
        // }
        // 4. Read again
        // let updated: Trivial = db
        //     .read_by_key(table, key)
        //     .await
        //     .expect("read after update failed");
        //
        // assert_eq!(updated.value, "ok, updated");
        // assert_eq!(read.key, key);
        //
        // // 5. Delete
        // db.delete_struct(table, updated)
        //     .await
        //     .expect("delete failed");
        //
        // // 6. Verify deletion
        // let deleted = db.read_by_key::<Trivial>(table, key).await;
        // assert!(deleted.is_err());
        Ok(())
    }
}
