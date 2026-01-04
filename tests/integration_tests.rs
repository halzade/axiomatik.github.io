use axiomatik_web::{app, db};
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;
use url::form_urlencoded;

async fn setup_app() -> axum::Router {
    let db = Arc::new(db::init_mem_db().await.unwrap());
    app(db)
}

fn serialize(params: &[(&str, &str)]) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    serializer.extend_pairs(params);
    serializer.finish()
}

#[tokio::test]
async fn test_create_user_bootstrap() {
    let app = setup_app().await;

    // Bootstrap user creation (no users in DB)
    let params = [("username", "admin"), ("password", "password123")];
    let body = serialize(&params);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/create-user")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), 100000).await.unwrap();
    assert!(String::from_utf8_lossy(&body).contains("Účet byl úspěšně vytvořen"));
}

#[tokio::test]
async fn test_login() {
    let app = setup_app().await;

    // 1. Create user via bootstrap
    let params = [("username", "admin"), ("password", "password123")];
    let body = serialize(&params);
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/create-user")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    // 2. Try login
    let login_params = [("username", "admin"), ("password", "password123")];
    let login_body = serialize(&login_params);
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(login_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get(header::LOCATION).unwrap(), "/form");
    assert!(response.headers().get(header::SET_COOKIE).is_some());
}

#[tokio::test]
async fn test_change_password() {
    let app = setup_app().await;

    // 1. Create user (not bootstrap, so we need a session)
    // Actually, let's create a bootstrap user, then log in, then create ANOTHER user who will need password change.
    
    // Create bootstrap user
    let params = [("username", "admin"), ("password", "password123")];
    app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/create-user")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(serialize(&params)))
            .unwrap(),
    ).await.unwrap();

    // Login as admin
    let login_params = [("username", "admin"), ("password", "password123")];
    let login_resp = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(serialize(&login_params)))
            .unwrap(),
    ).await.unwrap();
    
    let cookie = login_resp.headers().get(header::SET_COOKIE).unwrap().to_str().unwrap().to_string();

    // Create user 'user1' who needs password change
    let create_params = [("username", "user1"), ("password", "pass1234")];
    app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/create-user")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(header::COOKIE, &cookie)
            .body(Body::from(serialize(&create_params)))
            .unwrap(),
    ).await.unwrap();

    // Login as user1
    let login_params1 = [("username", "user1"), ("password", "pass1234")];
    let login_resp1 = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(serialize(&login_params1)))
            .unwrap(),
    ).await.unwrap();

    // Should redirect to change-password
    assert_eq!(login_resp1.status(), StatusCode::SEE_OTHER);
    assert_eq!(login_resp1.headers().get(header::LOCATION).unwrap(), "/change-password");
    let cookie1 = login_resp1.headers().get(header::SET_COOKIE).unwrap().to_str().unwrap().to_string();

    // Change password
    let change_params = [("new_password", "new_password_123")];
    let change_resp = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/change-password")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(header::COOKIE, &cookie1)
            .body(Body::from(serialize(&change_params)))
            .unwrap(),
    ).await.unwrap();

    assert_eq!(change_resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(change_resp.headers().get(header::LOCATION).unwrap(), "/form");
}

#[tokio::test]
async fn test_create_article() {
    let app = setup_app().await;

    // 1. Login
    let params = [("username", "admin"), ("password", "password123")];
    app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/create-user")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(serialize(&params)))
            .unwrap(),
    ).await.unwrap();

    let login_params = [("username", "admin"), ("password", "password123")];
    let login_resp = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(serialize(&login_params)))
            .unwrap(),
    ).await.unwrap();
    let cookie = login_resp.headers().get(header::SET_COOKIE).unwrap().to_str().unwrap().to_string();

    // 2. Create article (Multipart)
    // We'll use a simple multipart body
    let boundary = "---------------------------123456789012345678901234567";
    let body = format!(
        "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        Test Article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Test Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        zahranici\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        This is a test article text.\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Short text.\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\n\
        Content-Type: image/jpeg\r\n\r\n\
        fake-image-data\r\n\
        --{0}--\r\n",
        boundary
    );

    let response = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header(header::CONTENT_TYPE, format!("multipart/form-data; boundary={}", boundary))
            .header(header::COOKIE, &cookie)
            .body(Body::from(body))
            .unwrap(),
    ).await.unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
}
