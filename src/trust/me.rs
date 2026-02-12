use crate::data::image_processor::ImageProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article::SurrealArticleError;
use crate::db::database_system::SurrealSystemError;
use crate::db::database_user::SurrealUserError;
use crate::system::commands::CommandError;
use crate::system::configuration::ConfigurationError;
use crate::system::server::ServerError;
use http::header;
use image::ImageError;
use std::convert::Infallible;
use std::fs;
use std::path::Path;
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

    #[error("real data error")]
    RealData,

    #[error("validation error: {0}")]
    Validation(String),

    #[error("login did not give cookie")]
    NoCookie,

    #[error("db system error")]
    SurrealSystem(#[from] SurrealSystemError),

    #[error("db system error")]
    SurrealArticle(#[from] SurrealArticleError),

    #[error("image error")]
    TrustImage(#[from] ImageError),

    #[error("image processor error")]
    ImageProcessor(#[from] ImageProcessorError),
}

pub fn path_exists(path: &str) -> Result<(), TrustError> {
    assert!(Path::new(path).exists());
    Ok(())
}

pub fn remove_file(path: &str) -> Result<(), TrustError> {
    assert!(fs::remove_file(path).is_ok());
    Ok(())
}
