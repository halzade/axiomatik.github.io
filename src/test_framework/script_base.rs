use axum::Router;
use http::{header, Request, Response};
use std::convert::Into;

use std::string::ToString;
use tower::ServiceExt;

use crate::database::Role::Editor;
use crate::database::User;
use crate::{database, logger, server};
use tracing::info;

use std::sync::Arc;
use std::sync::{Once, OnceLock};
use tokio::fs::OpenOptions;
use tokio::sync::{Notify, OnceCell};
use tokio::task::JoinHandle;
use crate::test_framework::article_builder::BOUNDARY;

static APP_ROUTER: OnceCell<Router> = OnceCell::const_new();

static ORIGINAL_INDEX: OnceLock<String> = OnceLock::new();

const PASSWORD: &str = "password123";

pub async fn setup_before_tests_once() {
    logger::config();

    database::initialize_in_memory_database().await;

    // Save original index.html
    // let original_index =
    // std::fs::read_to_string("index.html").expect("Failed to read index.html");
    // ORIGINAL_INDEX.set(original_index).expect("ORIGINAL_INDEX already set");

    let r = server::start_router().await;
    APP_ROUTER.set(r).unwrap();
}

fn after_tests_clean_up() {
    info!("All tests finished!");
    if let Some(content) = ORIGINAL_INDEX.get() {
        info!("rewrite index from original");
        std::fs::write("index.html", content).unwrap();
    }
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

pub fn original_index() -> String {
    ORIGINAL_INDEX.get().unwrap().to_string()
}

pub async fn setup_user_and_login(name: &str) -> String {
    // create user in DB
    database::create_user(new_user(name)).await.unwrap();

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

pub fn boundary() -> String {
    format!("multipart/form-data; boundary={}", BOUNDARY)
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
