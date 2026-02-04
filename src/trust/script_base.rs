use axum::Router;
use http::{header, Request, Response};
use std::convert::Into;

use std::string::ToString;
use thiserror::Error;
use tower::ServiceExt;

use crate::db::database::SurrealError;
use crate::db::database_user::Role::Editor;
use crate::db::database_user::{SurrealUserError, User};
use crate::db::{database, database_user};
use crate::system::{data_updates, logger, server};
use crate::trust::article_builder::BOUNDARY;
use tokio::sync::OnceCell;
use tracing::log::debug;
use crate::system::commands::CommandError;

// TODO X, proper test framework
static APP_ROUTER: OnceCell<Router> = OnceCell::const_new();
const PASSWORD: &str = "password123";

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
    HeaderToStrError(#[from] http::header::ToStrError),
}

pub async fn setup_before_tests_once() {
    debug!("only once");

    logger::config();
    data_updates::new();
    database::initialize_in_memory_database().await;

    let s = server::new();
    let r = s.start_server().await;
    let _ = APP_ROUTER.set(r.unwrap());

    debug!("test initialized");
}

pub async fn one_shot(request: Request<reqwest::Body>) -> Response<axum::body::Body> {
    let router = APP_ROUTER.get().unwrap().clone();
    router.oneshot(request).await.unwrap()
}

pub fn serialize(params: &[(&str, &str)]) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer.extend_pairs(params);
    serializer.finish()
}

pub async fn setup_user_and_login(name: &str) -> String {
    // create user in DB
    database_user::create_user(new_user(name)).await.unwrap();

    // login user
    let login_resp = one_shot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(reqwest::Body::from(format!(
                "username={}&password={}",
                name, PASSWORD
            )))
            .unwrap(),
    )
    .await;

    let cookie = login_resp
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    cookie
}

pub fn content_type_with_boundary() -> String {
    format!("multipart/form-data; boundary={}", BOUNDARY)
}

pub async fn response_to_body(response: axum::response::Response) -> String {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await;
    let body_str = String::from_utf8_lossy(&body_bytes.unwrap()).to_string();
    body_str
}

pub fn get_test_image_data() -> Vec<u8> {
    std::fs::read("tests/data/image_1024.png")
        .expect("Test image not found at tests/data/image_1024.png")
}

fn new_user(name: &str) -> User {
    let password_hash = bcrypt::hash(PASSWORD, bcrypt::DEFAULT_COST).unwrap();
    User {
        username: name.into(),
        author_name: name.into(),
        password_hash,
        needs_password_change: false,
        role: Editor,
    }
}
