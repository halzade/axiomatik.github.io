use crate::db::database::{db_read, db_write, DatabaseError};
use crate::db::database_user::DatabaseUserError::{
    DeleteUserDatabaseError, DeleteUserDeletionError,
};
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use surrealdb::sql::Value;
use surrealdb::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseUserError {
    #[error("user deletion failed")]
    DeleteUserError,

    #[error("user deletion failed, couldn't write into database, {0}")]
    DeleteUserDatabaseError(#[from] DatabaseError),

    #[error("user deletion failed, error while writing into database, {0}")]
    DeleteUserDeletionError(#[from] Error),
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
        let user_o = get_user(&username).await;
        if let Some(user) = user_o {
            if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(get_user_by_id(user_id).await)
    }
}

pub async fn update_user(user: User) -> Option<User> {
    let sdb_r = db_write().await;
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
    let sdb_r = db_write().await;
    match sdb_r {
        Ok(sdb) => sdb
            .create(("user", user.username.clone()))
            .content(user)
            .await
            .unwrap(),
        Err(_) => None,
    }
}
pub async fn delete_user(user_name: &str) -> Result<(), DatabaseUserError> {
    let sdb_r = db_write().await;
    match sdb_r {
        Ok(sdb) => {
            let res: Result<Option<Value>, Error> = sdb.delete(("user", user_name)).await;
            match res {
                Ok(_) => Ok(()),
                Err(e) => Err(DeleteUserDeletionError(e)),
            }
        }
        Err(e) => Err(DeleteUserDatabaseError(e)),
    }
}

pub async fn get_user_by_id(user_id: &str) -> Option<User> {
    if let Ok(sdb) = db_read().await {
        return sdb.select(("user", user_id)).await.unwrap();
    }
    None
}

pub async fn get_user(user_name: &str) -> Option<User> {
    get_user_by_id(user_name).await
}
