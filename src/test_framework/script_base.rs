use axum::Router;
use http::{header, Request, Response};
use std::convert::Into;

use std::string::ToString;
use tower::ServiceExt;

use crate::database::Role::Editor;
use crate::database::User;
use crate::{database, logger, server};

use crate::test_framework::article_builder::BOUNDARY;
use tokio::sync::OnceCell;
use tracing::log::debug;
use tracing::trace;

static APP_ROUTER: OnceCell<Router> = OnceCell::const_new();
const PASSWORD: &str = "password123";

static SETUP_ONCE: OnceCell<()> = OnceCell::const_new();

pub async fn setup_before_tests_once() {
    trace!("many times 1");
    if SETUP_ONCE.get().is_some() {
        trace!("many times 2");
        return;
    }

    SETUP_ONCE
        .get_or_init(|| async {
            debug!("only once");

            logger::config();
            database::initialize_in_memory_database().await;

            // Create required directories
            let _ = std::fs::create_dir_all("uploads");
            let _ = std::fs::create_dir_all("snippets");

            let r = server::start_router().await;
            let _ = APP_ROUTER.set(r);
        })
        .await;

    trace!("many times 3");
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

pub async fn response_to_body(response: axum::response::Response) -> String {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await;
    let body_str = String::from_utf8_lossy(&body_bytes.unwrap()).to_string();
    body_str
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
