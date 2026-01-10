use axum::body::Body;
use http::{Request, header};
use std::fs;
use std::sync::Arc;
use tower::ServiceExt;
use url::form_urlencoded;
use crate::{app, db};

pub fn serialize(params: &[(&str, &str)]) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    serializer.extend_pairs(params);
    serializer.finish()
}

pub async fn setup_app() -> (axum::Router, Arc<db::Database>) {
    let db = Arc::new(db::init_mem_db().await);
    (app(db.clone()), db)
}

pub async fn setup_test_environment() -> (axum::Router, Arc<db::Database>, String, String) {
    let db = Arc::new(db::Database::new("mem://").await);
    let app = app(db.clone());

    // Create user and login
    let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
    db.create_user(db::User {
        username: "admin".to_string(),
        author_name: "admin".to_string(),
        password_hash,
        needs_password_change: false,
        role: db::Role::Editor,
    })
    .await
    .unwrap();

    let login_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from("username=admin&password=password123"))
                .unwrap(),
        )
        .await
        .unwrap();

    let cookie = login_resp
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let original_index = fs::read_to_string("index.html").expect("Failed to read index.html");

    fs::create_dir_all("snippets").unwrap();
    fs::create_dir_all("uploads").unwrap();

    (app, db, cookie, original_index)
}
