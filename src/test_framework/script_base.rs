use axum::Router;
use http::{header, Request, Response};
use std::convert::Into;

use std::string::ToString;
use tower::ServiceExt;

use crate::db::database_user::Role::Editor;
use crate::db::database_user::User;
use crate::db::{database, database_user};
use crate::system::{server, system_data};
use crate::test_framework::article_builder::BOUNDARY;
use crate::logger;
use tokio::sync::OnceCell;
use tracing::log::debug;

pub const CLEANUP: &str = "Failed to cleanup";

// TODO
static APP_ROUTER: OnceCell<Router> = OnceCell::const_new();
const PASSWORD: &str = "password123";

pub async fn setup_before_tests_once() {
    debug!("only once");

    logger::config();
    system_data::init_trivial_data();
    database::initialize_in_memory_database().await;

    let r = server::start_router().await;
    let _ = APP_ROUTER.set(r);

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
