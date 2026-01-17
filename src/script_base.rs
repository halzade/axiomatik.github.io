use crate::database::Role::Editor;
use crate::database::User;
use crate::{database, server};
use axum::Router;
use http::{header, Request, Response};

use std::fs;
use tower::ServiceExt;

use std::sync::{Once, OnceLock};
// use axum_core::body::Body;

static INIT: Once = Once::new();
static APP_ROUTER: OnceLock<Router> = OnceLock::new();

async fn setup_before_tests() {
    INIT.call_once(|| {
        // runs once before any test body that calls setup()
        println!("setup_before_tests()");
        let _ = database::initialize_in_memory_database();
        let router = server::router();
        let _ = APP_ROUTER.set(router);

        let _ = setup_test_environment_with_user_admin();
    });
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

async fn setup_test_environment_with_user_admin() -> String {
    database::create_user(user_admin()).await.unwrap();

    let login_resp = one_shot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(reqwest::Body::from("username=admin&password=password123"))
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

    fs::create_dir_all("snippets").unwrap();
    fs::create_dir_all("uploads").unwrap();

    cookie
}

fn user_admin() -> User {
    let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
    User {
        username: "admin".to_string(),
        author_name: "admin".to_string(),
        password_hash,
        needs_password_change: false,
        role: Editor,
    }
}
