use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Trivial {
    pub id: String,
    pub value: String,
}

impl Trivial {
    fn new(value: String) -> Self {
        Self {
            id: String::new(),
            value,
        }
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
    async fn test_create_update_read_delete() {
        // --- 1. Init in‑memory SurrealDB ---
        let db = init_db_test().await;

        let table = "trivial";

        // 1️⃣ Create
        let new = Trivial::new("ok".into());
        let id = db.create_struct(table, &new).await.expect("create failed");

        // 2️⃣ Read
        let mut read: Trivial = db.read_by_id(table, &id).await.expect("read failed");

        assert_eq!(read.value, "ok");

        // 3️⃣ Update
        read.value = "ok, updated".into();

        db.update_struct(table, &read).await.expect("update failed");

        // 4️⃣ Read again
        let updated: Trivial = db
            .read_by_id(table, &id)
            .await
            .expect("read after update failed");

        assert_eq!(updated.value, "ok, updated");

        // 5️⃣ Delete
        db.delete_struct(table, updated)
            .await
            .expect("delete failed");

        // 6️⃣ Verify deletion
        let deleted = db.read_by_id::<Trivial>(table, &id).await;
        assert!(deleted.is_err());
    }
}
