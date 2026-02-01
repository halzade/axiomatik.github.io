use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub author_name: String,
    pub password_hash: String,
    pub needs_password_change: bool,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Role {
    Editor,
}

pub async fn update_user(user: User) -> Option<User> {
    let sdb_r = crate::db::database::db_write().await;
    match sdb_r {
        Ok(sdb) => sdb
            .update(("user", user.username.clone()))
            .content(user)
            .await
            .unwrap(),
        Err(_) => None,
    }
}

pub async fn create_user(user: User) -> Option<User> {
    let sdb_r = crate::db::database::db_write().await;
    match sdb_r {
        Ok(sdb) => sdb
            .create(("user", user.username.clone()))
            .content(user)
            .await
            .unwrap(),
        Err(_) => None,
    }
}
pub async fn delete_user(user_name: &str) {
    if let Ok(sdb) = crate::db::database::db_write().await {
        let _: Result<Option<surrealdb::sql::Value>, surrealdb::Error> =
            sdb.delete(("user", user_name)).await;
    } else {
        error!("Database not available");
    }
}

// TODO X implement get_current_user(), use axum authentication
// this takes username from cookie, bad
pub async fn get_user(user_name: &str) -> Option<User> {
    if let Ok(sdb) = crate::db::database::db_read().await {
        return sdb.select(("user", user_name)).await.unwrap();
    }
    None
}
