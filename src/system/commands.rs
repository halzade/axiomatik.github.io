use crate::db::database_user::{Role, SurrealUserError, User};
use crate::system::server::TheState;
use bcrypt::{hash, DEFAULT_COST};
use thiserror::Error;
use CommandError::Bcrypt;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("bcrypt error: {0}")]
    Bcrypt(String),

    #[error("user database error: {0}")]
    DatabaseError(#[from] SurrealUserError),
}

/**
 * create bootstrap admin
 */
pub async fn create_admin_user(state: &TheState) -> Result<(), CommandError> {
    let username = "admin";
    let password = "admin*";

    let password_hash = hash(password, DEFAULT_COST).map_err(|e| Bcrypt(e.to_string()))?;
    let user = User {
        username: username.to_string(),
        author_name: username.to_string(),
        password_hash,
        needs_password_change: true,
        role: Role::Admin,
    };

    state.dbu.create_user(user).await?;
    Ok(())
}
