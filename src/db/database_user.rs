use crate::db::database::{db_read, db_write, DatabaseError};
use crate::db::database_user::DatabaseUserError::*;
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use surrealdb::Error;
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]
pub enum DatabaseUserError {
    #[error("user deletion failed")]
    DeleteUserError,

    #[error("user deletion failed, couldn't write into database, {0}")]
    DeleteUserDatabaseError(#[from] DatabaseError),

    #[error("user deletion failed, error while writing into database, {0}")]
    DeleteUserDeletionError(#[from] Error),

    #[error("user update failed, {0}")]
    UpdateUserError(String),

    #[error("user update failed, couldn't write into database, {0}")]
    UpdateUserDatabaseError(DatabaseError),

    #[error("user update failed, error while writing into database, {0}")]
    UpdateUserExecutionError(Error),

    #[error("user creation failed, {0}")]
    CreateUserError(String),

    #[error("user creation failed, couldn't write into database, {0}")]
    CreateUserDatabaseError(DatabaseError),

    #[error("user creation failed, error while writing into database, {0}")]
    CreateUserExecutionError(Error),
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

pub async fn update_user(user: User) -> Result<(), DatabaseUserError> {
    let sdb = db_write().await.map_err(UpdateUserDatabaseError)?;
    let _: Option<User> = sdb
        .update(("user", user.username.clone()))
        .content(user)
        .await
        .map_err(UpdateUserExecutionError)?;
    Ok(())
}

pub async fn create_user(user: User) -> Result<(), DatabaseUserError> {
    let sdb = db_write().await.map_err(CreateUserDatabaseError)?;
    let _: Option<User> = sdb
        .create(("user", user.username.clone()))
        .content(user)
        .await
        .map_err(CreateUserExecutionError)?;
    debug!("User created successfully.");
    Ok(())
}

pub async fn delete_user(user_name: &str) -> Result<(), DatabaseUserError> {
    let sdb_r = db_write().await;
    match sdb_r {
        Ok(sdb) => {
            let res: Result<Option<User>, Error> = sdb.delete(("user", user_name)).await;
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

pub async fn get_user_by_name(user_id: &str) -> Option<User> {
    if let Ok(sdb) = db_read().await {
        return sdb.select(("user", user_id)).await.unwrap();
    }
    None
}
