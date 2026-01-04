use axiomatik_web::{app, db};
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use std::sync::Arc;
use tower::ServiceExt;

async fn setup_app() -> (axum::Router, Arc<db::Database>) {
    let db = Arc::new(db::init_mem_db().await.unwrap());
    (app(db.clone()), db)
}

#[tokio::test]
async fn test_unauthorized_access_to_form() {
    let (app, _) = setup_app().await;

    // Try to access /form without any session
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/form")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // It should redirect to /login
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get(header::LOCATION).unwrap(), "/login");
}

#[tokio::test]
async fn test_root_redirect_to_form_then_login() {
    let (app, _) = setup_app().await;

    // Try to access / which redirects to /form
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get(header::LOCATION).unwrap(), "index.html");
}
