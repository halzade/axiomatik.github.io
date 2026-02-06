use std::convert::Infallible;
use crate::db::database::SurrealError;
use crate::db::database_user::{Role, SurrealUserError, User};
use crate::db::{database, database_user};
use crate::system::commands::CommandError;
use crate::system::{data_updates, logger};
use crate::trust::nexo_app::NexoApp;
use crate::trust::nexo_web::NexoWeb;
use bcrypt::{hash, DEFAULT_COST};
use http::header;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrustError {
    #[error("test failed: {0}")]
    TestFailed(String),

    #[error("surreal error: {0}")]
    TestSurrealError(#[from] SurrealError),

    #[error("surreal user error {0}")]
    TestSurrealUserError(#[from] SurrealUserError),

    #[error("test surrealdb error {0}")]
    TestError(#[from] surrealdb::Error),

    #[error("test command error {0}")]
    TrustCommandError(#[from] CommandError),

    #[error("io error {0}")]
    IoError(#[from] std::io::Error),

    #[error("reqwest error {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("http error {0}")]
    HttpError(#[from] http::Error),
    
    #[error("infallible error {0}")]
    TrustInfallible(#[from] Infallible),

    #[error("serde_json error {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("axum error {0}")]
    AxumError(String),

    #[error("bcrypt error {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("axum framework error {0}")]
    AxumFrameworkError(#[from] axum::Error),

    #[error("header to_str error {0}")]
    HeaderToStrError(#[from] header::ToStrError),
}

pub struct TrustMe {
    nexo_app: Option<NexoApp>,
    nexo_web: Option<NexoWeb>,
}

pub async fn setup() -> Result<(), TrustError> {
    logger::config();
    data_updates::new();
    database::initialize_in_memory_database().await?;
    Ok(())
}

pub fn nexo_app() -> Result<NexoApp, TrustError> {
    let na = NexoApp::new();
    Ok(na)
}

pub fn nexo_web() -> Result<NexoWeb, TrustError> {
    let nw = NexoWeb::new();
    Ok(nw)
}

pub fn path_exists(path: &str) {
    assert!(std::path::Path::new(path).exists());
}

pub fn remove_file(path: &str) -> Result<(), TrustError> {
    assert!(std::fs::remove_file(path).is_ok());
    Ok(())
}

pub async fn db_setup_user(username: &str) -> Result<(), TrustError> {
    // db create user
    database_user::create_user(User {
        username: username.to_string(),
        author_name: username.to_string(),
        password_hash: hash("password", DEFAULT_COST)?,
        needs_password_change: false,
        role: Role::Editor,
    })
    .await?;
    Ok(())
}
