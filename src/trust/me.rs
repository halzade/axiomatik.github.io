use crate::db::database;
use crate::db::database::SurrealError;
use crate::db::database_user::{DatabaseUser, SurrealUserError};
use crate::system::commands::CommandError;
use crate::system::configuration::{ConfigurationError, Mode};
use crate::system::server::{ServerError, TheState};
use crate::system::{data_system, data_updates, logger, server};
use crate::trust::nexo_app::NexoApp;
use crate::trust::nexo_db::NexoDb;
use crate::trust::nexo_web::NexoWeb;
use http::header;
use std::convert::Infallible;
use std::sync::Arc;
use thiserror::Error;
use crate::db::database_article::DatabaseArticle;

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

    #[error("configuration error")]
    TrustConfiguration(#[from] ConfigurationError),

    #[error("server error")]
    TrustServerError(#[from] ServerError),

    #[error("application mode error")]
    ModeError(Mode),
}

pub struct TrustMe {
    nexo_app: Arc<NexoApp>,
    nexo_web: Arc<NexoWeb>,
    nexo_db: Arc<NexoDb>,
}

pub async fn server() -> Result<TrustMe, TrustError> {
    // config
    logger::config();
    data_updates::new();

    // database
    let db = Arc::new(database::init_in_memory_db_connection().await?);
    let dba = Arc::new(DatabaseArticle::new(db.clone()));
    let dbu = Arc::new(DatabaseUser::new(db.clone()));
    let dbs = Arc::new(crate::db::database_system::DatabaseSystem::new(db.clone()));

    // in memory application data
    let ds = Arc::new(data_system::new());
    let dv = Arc::new(data_updates::new());

    // the application state
    let state = TheState { dba, dbu, dbs, ds, dv };

    // server
    let server = server::connect(state.clone()).await?;
    // app
    let app_router = server.start_app_router().await?;
    // web
    let web_router = server.start_web_router().await?;
    server.status_start()?;

    Ok(TrustMe {
        // app and web already have state from server
        nexo_app: Arc::new(NexoApp::new(app_router)),
        nexo_web: Arc::new(NexoWeb::new(web_router)),
        nexo_db: Arc::new(NexoDb::new(state.clone()).await?),
    })
}

impl TrustMe {
    pub fn nexo_app(&self) -> Result<Arc<NexoApp>, TrustError> {
        Ok(self.nexo_app.clone())
    }

    pub fn nexo_web(&self) -> Result<Arc<NexoWeb>, TrustError> {
        Ok(self.nexo_web.clone())
    }

    pub fn surreal(&self) -> Result<Arc<NexoDb>, TrustError> {
        Ok(self.nexo_db.clone())
    }
}

pub fn path_exists(path: &str) {
    assert!(std::path::Path::new(path).exists());
}

pub fn remove_file(path: &str) -> Result<(), TrustError> {
    assert!(std::fs::remove_file(path).is_ok());
    Ok(())
}
