use crate::db::database::{db_read, db_write, SurrealError};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use surrealdb_types::SurrealValue;
use thiserror::Error;
use tracing::{debug, info};

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

#[derive(Clone, Debug)]
pub struct Backend;

/**
 * user authentication
 */
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
        let user_r = get_user_by_name(&username).await;
        match user_r {
            Ok(user_o) => {
                match user_o {
                    Some(user) => {
                        if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
                            /*
                             * user was authenticated
                             */
                            return Ok(Some(user));
                        }
                        Ok(None)
                    }
                    None => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    async fn get_user(
        &self,
        user_name: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user_r = get_user_by_name(user_name).await;
        match user_r {
            Ok(user_o) => Ok(user_o),
            _ => Ok(None),
        }
    }
}

pub async fn update_user_author_name(
    user_name: &str,
    new_author_name: &str,
) -> Result<(), SurrealUserError> {
    let sdb = db_write().await?;
    let _: Option<User> = sdb
        .update(("user", user_name.to_string()))
        .merge(("author_name", new_author_name.to_string()))
        .await?;
    Ok(())
}

pub async fn update_user_password(
    user_name: String,
    new_password_hash: String,
) -> Result<(), SurrealUserError> {
    let sdb = db_write().await?;
    let _: Option<User> = sdb
        .update(("user", user_name))
        .merge(json!({
            "password_hash": new_password_hash,
            "needs_password_change": false
        }))
        .await?;
    Ok(())
}

pub async fn create_user(user: User) -> Result<(), SurrealUserError> {
    let sdb = db_write().await?;
    let _: Option<User> = sdb
        .create(("user", user.username.clone()))
        .content(user)
        .await?;
    info!("User created successfully.");
    Ok(())
}

pub async fn delete_user(user_name: &str) -> Result<(), SurrealUserError> {
    let sdb = db_write().await?;
    let _: Option<User> = sdb.delete(("user", user_name)).await?;
    info!("User {} deleted successfully.", user_name);
    Ok(())
}

// TODO Result
pub async fn get_user_by_name(user_id: &str) -> Result<Option<User>, SurrealUserError> {
    let sdb = db_read().await?;
    let user_o = sdb.select(("user", user_id)).await?;
    Ok(user_o)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::database::initialize_in_memory_database;

    #[tokio::test]
    async fn test_create_update_delete_user() -> Result<(), SurrealUserError> {
        initialize_in_memory_database().await?;

        let user = User {
            username: "tester".to_string(),
            author_name: "Test Author".to_string(),
            password_hash: "hash".to_string(),
            needs_password_change: false,
            role: Role::Editor,
        };

        // create
        create_user(user.clone()).await?;

        let fetched_user = get_user_by_name("tester").await?.unwrap();
        assert_eq!(fetched_user.username, "tester");
        assert_eq!(fetched_user.author_name, "Test Author");

        // TODO update_user_password

        // update
        update_user_author_name("tester", "New Author Name")
            .await
            .expect("Failed to update user");

        let fetched_user = get_user_by_name("tester").await?.unwrap();
        assert_eq!(fetched_user.author_name, "New Author Name");

        // delete
        delete_user("tester").await.expect("Failed to delete user");

        let fetched_user = get_user_by_name("tester").await?;
        assert!(fetched_user.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_nonexistent_user() -> Result<(), SurrealUserError> {
        initialize_in_memory_database().await?;

        let fetched_user = get_user_by_name("nonexistent").await?;

        assert!(fetched_user.is_none());
        Ok(())
    }
}
