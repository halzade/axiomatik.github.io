use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    use axiomatik_web::db::database_internal::init_db_test;

    #[tokio::test]
    async fn test_connects_and_query() {
        let db_surreal = init_db_test().await;
        let db = db_surreal.db.read().await;
        let res = db.query("RETURN 1").await.unwrap();
        assert!(res.check().is_ok());
    }

    #[tokio::test]
    async fn test_create_update_read_delete() {
        let db = init_db_test().await;

        let key = "1";
        let table = "trivial";

        // 1. Create
        let new = Trivial::new(key, "ok");
        db.create_struct(table, &new).await.expect("create failed");

        // 2. Read
        let mut read: Trivial = db.read_by_key(table, key).await.expect("read failed");

        assert_eq!(read.value, "ok");
        assert_eq!(read.key, "1");

        // 3. Update
        read.value = "ok, updated".into();
        db.update_struct(table, &read).await.expect("update failed");

        // 4. Read again
        let updated: Trivial = db
            .read_by_key(table, key)
            .await
            .expect("read after update failed");

        assert_eq!(updated.value, "ok, updated");
        assert_eq!(read.key, key);

        // 5. Delete
        db.delete_struct(table, updated)
            .await
            .expect("delete failed");

        // 6. Verify deletion
        let deleted = db.read_by_key::<Trivial>(table, key).await;
        assert!(deleted.is_err());
    }
}
