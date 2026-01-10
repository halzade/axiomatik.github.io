use axiomatik_web::script_base::{serialize, setup_app};
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_sql_injection_rejection_in_login() {
    let (app, _db) = setup_app().await;

    // Payload attempting SQL injection
    let injection_payload = [("username", "admin' OR '1'='1"), ("password", "anything")];

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&injection_payload)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::SEE_OTHER); // Should not redirect (successful login)
}

#[tokio::test]
async fn test_malicious_keyword_rejection() {
    let (app, _db) = setup_app().await;

    let malicious_payload = [
        ("username", "admin; drOp daTaBasE user;"),
        ("password", "password"),
    ];

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&malicious_payload)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
