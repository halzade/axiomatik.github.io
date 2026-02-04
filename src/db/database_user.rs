use crate::db::database::{db_read, db_write, SurrealError};
use crate::db::database_user::DatabaseUserError::*;
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]
pub enum DatabaseUserError {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Role {
    Editor,
}

#[derive(Clone, Debug)]
pub struct Backend;

impl axum_login::AuthnBackend for Backend {
    type User = User;
    type Credentials = (String, String);
    type Error = Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let (username, password) = creds;

        debug!("Authenticating user {:?}", username);
        let user_o = get_user_by_name(&username).await;
        if let Some(user) = user_o {
            if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }

    async fn get_user(
        &self,
        user_name: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(get_user_by_name(user_name).await)
    }
}

pub async fn update_user_author_name(
    user_name: &str,
    new_author_name: &str,
) -> Result<(), DatabaseUserError> {
    let sdb = db_write().await.map_err(UpdateUserDatabaseError)?;
    let _: Option<serde_json::Value> = sdb
        .update(("user", user_name))
        .merge(serde_json::json!({
            "author_name": new_author_name,
        }))
        .await
        .map_err(UpdateUserExecutionError)?;
    Ok(())
}

pub async fn update_user_password(
    user_name: &str,
    new_password_hash: &str,
) -> Result<(), DatabaseUserError> {
    let sdb = db_write().await.map_err(UpdateUserDatabaseError)?;
    let _: Option<serde_json::Value> = sdb
        .update(("user", user_name))
        .merge(serde_json::json!({
            "password_hash": new_password_hash,
            "needs_password_change": false,
        }))
        .await
        .map_err(UpdateUserExecutionError)?;
    Ok(())
}

pub async fn create_user(user: User) -> Result<(), DatabaseUserError> {
    let sdb = db_write().await.map_err(CreateUserDatabaseError)?;
    let _: Option<serde_json::Value> = sdb
        .create(("user", user.username.clone()))
        .content(serde_json::to_value(&user).map_err(|e| CreateUserError(e.to_string()))?)
        .await
        .map_err(CreateUserExecutionError)?;
    debug!("User created successfully.");
    Ok(())
}

pub async fn delete_user(user_name: &str) -> Result<(), DatabaseUserError> {
    let sdb_r = db_write().await;
    match sdb_r {
        Ok(sdb) => {
            let res: Result<Option<serde_json::Value>, surrealdb::Error> = sdb.delete(("user", user_name)).await;
            match res {
                Ok(_) => {
                    debug!("User {} deleted successfully.", user_name);
                    Ok(())
                }
                Err(e) => Err(DeleteUserDeletionError(e)),
            }
        }
        Err(e) => Err(DeleteUserDatabaseError(e)),
    }
}

// TODO X Result
pub async fn get_user_by_name(user_id: &str) -> Option<User> {
    if let Ok(sdb) = db_read().await {
        if let Ok(opt_val) = sdb.select(("user", user_id)).await {
            if let Some(val) = opt_val {
                return serde_json::from_value::<User>(val).ok();
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::database::initialize_in_memory_database;

    #[tokio::test]
    async fn test_create_update_delete_user() {
        initialize_in_memory_database().await;

        let user = User {
            username: "testuser_x".to_string(),
            author_name: "Test Author".to_string(),
            password_hash: "hash".to_string(),
            needs_password_change: false,
            role: Role::Editor,
        };

        // create
        create_user(user.clone())
            .await
            .expect("Failed to create user");

        let fetched_user = get_user_by_name("testuser_x")
            .await
            .expect("User not found");
        assert_eq!(fetched_user.username, "testuser_x");
        assert_eq!(fetched_user.author_name, "Test Author");

        // update
        update_user_author_name("testuser_x", "New Author Name")
            .await
            .expect("Failed to update user");

        let fetched_user = get_user_by_name("testuser_x")
            .await
            .expect("User not found");
        assert_eq!(fetched_user.author_name, "New Author Name");

        // delete
        delete_user("testuser_x")
            .await
            .expect("Failed to delete user");

        let fetched_user = get_user_by_name("testuser_x").await;
        assert!(fetched_user.is_none());
    }

    #[tokio::test]
    async fn test_get_nonexistent_user() {
        initialize_in_memory_database().await;

        let fetched_user = get_user_by_name("nonexistent").await;

        assert!(fetched_user.is_none());
    }
}
