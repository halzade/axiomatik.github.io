use crate::db::database_user;
use crate::db::database_user::{SurrealUserError, Role, User};
use bcrypt::{hash, DEFAULT_COST};
use thiserror::Error;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("password too short")]
    PasswordTooShort,

    #[error("bcrypt error: {0}")]
    Bcrypt(String),

    #[error("user database error: {0}")]
    DatabaseError(#[from] SurrealUserError),
}

pub async fn create_user(args: &Vec<String>) {
    if args.len() != 4 {
        info!("Usage: cargo run -- create-user <username> <password>");
        std::process::exit(1);
    }

    let username = &args[2];
    let password = &args[3];

    match create_editor_user(username, password).await {
        Ok(_) => {
            info!("Editor user '{}' created successfully.", username);
            std::process::exit(0);
        }
        Err(e) => {
            error!("Failed to create editor user: {}", e);
            std::process::exit(1);
        }
    }
}

pub async fn delete_user(args: &Vec<String>) -> Result<(), CommandError> {
    if args.len() != 3 {
        info!("Usage: cargo run -- delete-user <username>");
        std::process::exit(1);
    }
    let username = &args[2];

    database_user::delete_user(username).await?;
    Ok(())
}

pub async fn create_editor_user(username: &str, password: &str) -> Result<(), CommandError> {
    if password.len() < 3 {
        return Err(CommandError::PasswordTooShort);
    }

    let password_hash =
        hash(password, DEFAULT_COST).map_err(|e| CommandError::Bcrypt(e.to_string()))?;
    let user = User {
        username: username.to_string(),
        author_name: username.to_string().clone(),
        password_hash,
        needs_password_change: true,
        role: Role::Editor,
    };

    assert!(database_user::create_user(user).await.is_ok());
    Ok(())
}
