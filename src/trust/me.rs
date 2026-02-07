use crate::db::database;
use crate::db::database::SurrealError;
use crate::db::database_article::DatabaseArticle;
use crate::db::database_user::{DatabaseUser, SurrealUserError};
use crate::system::commands::CommandError;
use crate::system::configuration::ConfigurationError;
use crate::system::server::{ServerError, TheState};
use crate::system::{data_system, data_updates, logger, server};
use crate::trust::app::article::create_article_request_builder::ArticleBuilderError;
use axum::Router;
use http::header;
use std::convert::Infallible;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrustError {
    #[error("test failed: {0}")]
    TestFailed(String),

    #[error("surreal error: {0}")]
    SurrealError(#[from] SurrealError),

    #[error("surreal user error {0}")]
    SurrealUserError(#[from] SurrealUserError),

    #[error("test surrealdb error {0}")]
    SurrealDatabaseError(#[from] surrealdb::Error),

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

    #[error("configuration error")]
    TrustConfiguration(#[from] ConfigurationError),

    #[error("server error")]
    TrustServerError(#[from] ServerError),

    #[error("article builder error")]
    ArticleBuilder(#[from] ArticleBuilderError),

    #[error("real data error")]
    RealData,

    #[error("validation error:/n{0}")]
    Validation(String),
}

pub fn path_exists(path: &str) {
    assert!(std::path::Path::new(path).exists());
}

pub fn remove_file(path: &str) -> Result<(), TrustError> {
    assert!(std::fs::remove_file(path).is_ok());
    Ok(())
}
