use crate::db::database;
use crate::db::database::{DatabaseSurreal, SurrealError};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use surrealdb_types::SurrealValue;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum SurrealUserError {
    #[error("user deletion failed")]
    DeleteUserError,

    #[error("user deletion failed, couldn't write into database, {0}")]
    DeleteUserDatabaseError(#[from] SurrealError),

    #[error("user deletion failed, error while writing into database, {0}")]
    DeleteUserDeletionError(#[from] surrealdb::Error),

    #[error("user update failed, {0}")]
    UpdateUserError(String),

    #[error("user update failed, couldn't write into database, {0}")]
    UpdateUserDatabaseError(SurrealError),

    #[error("user update failed, error while writing into database, {0}")]
    UpdateUserExecutionError(surrealdb::Error),

    #[error("user creation failed, {0}")]
    CreateUserError(String),

    #[error("user creation failed, couldn't write into database, {0}")]
    CreateUserDatabaseError(SurrealError),

    #[error("user creation failed, error while writing into database, {0}")]
    CreateUserExecutionError(surrealdb::Error),
}

/*
 * key is username
 */
#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct User {
    pub username: String,
    pub author_name: String,
    pub password_hash: String,
    pub needs_password_change: bool,
    pub role: Role,
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.username.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, SurrealValue)]
pub enum Role {
    Editor,
}

/**
 * access to a database
 * - anything user-related
 */
#[derive(Debug)]
pub struct DatabaseUser {
    surreal: Arc<DatabaseSurreal>,
}

impl DatabaseUser {
    pub fn new(db: Arc<DatabaseSurreal>) -> DatabaseUser {
        DatabaseUser { surreal: db }
    }

    pub async fn new_from_scratch() -> Result<DatabaseUser, SurrealError> {
        let db = Arc::new(database::init_in_memory_db_connection().await?);
        Ok(DatabaseUser { surreal: db })
    }

    pub async fn update_user_author_name(
        &self,
        user_name: &str,
        new_author_name: &str,
    ) -> Result<(), SurrealUserError> {
        let _: Option<User> = self
            .surreal
            .db
            .update(("user", user_name.to_string()))
            .merge(json!({"author_name": new_author_name.to_string()}))
            .await?;
        Ok(())
    }

    pub async fn update_user_password(
        &self,
        user_name: String,
        new_password_hash: String,
    ) -> Result<(), SurrealUserError> {
        let _: Option<User> = self
            .surreal
            .db
            .update(("user", user_name))
            .merge(json!({
                "password_hash": new_password_hash,
                "needs_password_change": false
            }))
            .await?;
        Ok(())
    }

    pub async fn create_user(&self, user: User) -> Result<(), SurrealUserError> {
        let _: Option<User> =
            self.surreal.db.create(("user", user.username.clone())).content(user).await?;
        info!("User created successfully.");
        Ok(())
    }

    pub async fn delete_user(&self, user_name: &str) -> Result<(), SurrealUserError> {
        let _: Option<User> = self.surreal.db.delete(("user", user_name)).await?;
        info!("User {} deleted successfully.", user_name);
        Ok(())
    }

    pub async fn get_user_by_name(&self, user_id: &str) -> Result<Option<User>, SurrealUserError> {
        let user_o = self.surreal.db.select(("user", user_id)).await?;
        Ok(user_o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_update_delete_user() -> Result<(), SurrealUserError> {
        let db = DatabaseUser::new_from_scratch().await?;

        let user = User {
            username: "tester".to_string(),
            author_name: "Test Author".to_string(),
            password_hash: "hash".to_string(),
            needs_password_change: false,
            role: Role::Editor,
        };

        // create
        db.create_user(user.clone()).await?;

        let fetched_user = db.get_user_by_name("tester").await?.unwrap();
        assert_eq!(fetched_user.username, "tester");
        assert_eq!(fetched_user.author_name, "Test Author");

        // update
        db.update_user_author_name("tester", "New Author Name").await?;

        let fetched_user = db.get_user_by_name("tester").await?.unwrap();
        assert_eq!(fetched_user.author_name, "New Author Name");

        // delete
        db.delete_user("tester").await.expect("Failed to delete user");

        let fetched_user = db.get_user_by_name("tester").await?;
        assert!(fetched_user.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_nonexistent_user() -> Result<(), SurrealUserError> {
        let db = DatabaseUser::new_from_scratch().await?;

        let user = User {
            username: "tester1".to_string(),
            author_name: "Test Author".to_string(),
            password_hash: "hash".to_string(),
            needs_password_change: false,
            role: Role::Editor,
        };

        // create some user (create user table)
        db.create_user(user.clone()).await?;

        // nonexistent user
        let fetched_user = db.get_user_by_name("nonexistent").await?;
        assert!(fetched_user.is_none());

        // delete first user
        db.delete_user("tester1").await?;

        Ok(())
    }
}
