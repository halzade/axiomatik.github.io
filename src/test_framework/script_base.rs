use axum::Router;
use http::{header, Request, Response};

use std::string::ToString;
use tower::ServiceExt;

use crate::database::Role::Editor;
use crate::database::User;
use crate::{database, server};
use ctor::{ctor, dtor};
use std::sync::OnceLock;
use tokio::sync::OnceCell;
use tracing::info;

static APP_ROUTER: OnceLock<Router> = OnceLock::new();
static ORIGINAL_INDEX: OnceCell<String> = OnceCell::const_new();

pub const FAKE_IMAGE_DATA: Vec<u8> = Vec::new();
pub const FAKE_AUDIO_DATA: Vec<u8> = Vec::new();

pub const JPEG: &str = "image/jpeg";

const PASSWORD: &str = "password123";

#[ctor]
fn setup_before_tests() {
    info!("setup_before_tests()");

    info!("init database");
    let _ = database::initialize_in_memory_database();

    info!("init server");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let router = rt.block_on(server::router());
    let _ = APP_ROUTER.set(router);

    let original_index = std::fs::read_to_string("index.html").unwrap();
    ORIGINAL_INDEX.set(original_index).unwrap();
}

#[dtor]
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
