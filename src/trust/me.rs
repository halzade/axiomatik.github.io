use crate::db::database;
use crate::db::database::SurrealError;
use crate::db::database_user::SurrealUserError;
use crate::system::commands::CommandError;
use crate::system::{data_updates, logger};
use crate::trust::nexo_app::NexoApp;
use crate::trust::nexo_web::NexoWeb;
use crate::trust::response_verifier::ResponseVerifier;
use axum_core::response::Response;
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

pub async fn setup_user_and_login(username: &str) -> Result<(), TrustError> {
    todo!();

    Ok(())
}

pub fn path_exists(path: &str) {
    assert!(std::path::Path::new(path).exists());
}

pub fn remove_file(path: &str) -> Result<(), TrustError> {
    assert!(std::fs::remove_file(path).is_ok());
    Ok(())
}
