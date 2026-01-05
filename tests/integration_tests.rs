use axiomatik_web::{app, auth, db};
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use chrono::Datelike;
use std::sync::Arc;
use tower::ServiceExt;
use url::form_urlencoded;

async fn setup_app() -> (axum::Router, Arc<db::Database>) {
    let db = Arc::new(db::init_mem_db().await.unwrap());
    (app(db.clone()), db)
}

fn serialize(params: &[(&str, &str)]) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    serializer.extend_pairs(params);
    serializer.finish()
}

#[tokio::test]
async fn test_login() {
    let (app, db) = setup_app().await;

    // 1. Create user via auth module (simulating command)
    auth::create_editor_user(&db, "admin", "password123")
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
    assert_eq!(
        response.headers().get(header::LOCATION).unwrap(),
        "/change-password"
    );
    assert!(response.headers().get(header::SET_COOKIE).is_some());
}

#[tokio::test]
async fn test_change_password() {
    let (app, db) = setup_app().await;

    // Create user who needs password change
    let password_hash = bcrypt::hash("pass1234", bcrypt::DEFAULT_COST).unwrap();
    db.create_user(db::User {
        username: "user1".to_string(),
        author_name: "user1".to_string(),
        password_hash,
        needs_password_change: true,
        role: db::Role::Editor,
    })
    .await
    .unwrap();

    // Login as user1
    let login_params1 = [("username", "user1"), ("password", "pass1234")];
    let login_resp1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&login_params1)))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should redirect to change-password
    assert_eq!(login_resp1.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        login_resp1.headers().get(header::LOCATION).unwrap(),
        "/change-password"
    );
    let cookie1 = login_resp1
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Change password
    let change_params = [("new_password", "new_password_123")];
    let change_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/change-password")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie1)
                .body(Body::from(serialize(&change_params)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(change_resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        change_resp.headers().get(header::LOCATION).unwrap(),
        "/account"
    );

    // Verify change in DB
    let user = db.get_user("user1").await.unwrap().unwrap();
    assert_eq!(user.author_name, "user1");
    assert!(!user.needs_password_change);
}

#[tokio::test]
async fn test_serve_static_html() {
    let (app, _db) = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/jeden-tisic-dnu.html")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_404_fallback() {
    let (app, _db) = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/non-existent-page.html")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8_lossy(&body);

    let expected_404_content = std::fs::read_to_string("404.html").unwrap();
    assert_eq!(true, expected_404_content.len() > 200);
    assert_eq!(body_str, expected_404_content);
}

#[tokio::test]
async fn test_404_fallback_curl() {
    let (app, _db) = setup_app().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let url = format!("http://{}/non-existent-page.html", addr);
    let output = std::process::Command::new("curl")
        .arg("-s")
        .arg(url)
        .output()
        .expect("Failed to execute curl");

    let body_str = String::from_utf8_lossy(&output.stdout);
    let expected_404_content = std::fs::read_to_string("404.html").unwrap();
    assert_eq!(body_str.trim(), expected_404_content.trim());
}

#[tokio::test]
async fn test_account_page() {
    let (app, db) = setup_app().await;

    // 1. Create user
    let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
    db.create_user(db::User {
        username: "account_user".to_string(),
        author_name: "Initial Author".to_string(),
        password_hash,
        needs_password_change: false,
        role: db::Role::Editor,
    })
    .await
    .unwrap();

    // 2. Login
    let login_params = [("username", "account_user"), ("password", "password123")];
    let login_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&login_params)))
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

    // 3. Access account page
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains("account_user"));
    assert!(body_str.contains("Initial Author"));
    assert!(body_str.contains("Moje články"));

    // 4. Update author name
    let update_params = [("author_name", "Updated Author")];
    let update_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/account/update-author")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie)
                .body(Body::from(serialize(&update_params)))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(update_resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        update_resp.headers().get(header::LOCATION).unwrap(),
        "/account"
    );

    // 5. Verify update in DB
    let user = db.get_user("account_user").await.unwrap().unwrap();
    assert_eq!(user.author_name, "Updated Author");

    // 6. Create an article for this user
    let boundary = "---------------------------123456789012345678901234567";
    let body = format!(
        "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        User Article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Updated Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        test-cathegory\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        Content\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Short\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\n\r\n\
        data\r\n\
        --{0}--\r\n",
        boundary
    );

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    // 7. Verify article is on account page
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains("User Article"));

    // 8. Update author name again
    let update_params = [("author_name", "Second Update")];
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/account/update-author")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie)
                .body(Body::from(serialize(&update_params)))
                .unwrap(),
        )
        .await
        .unwrap();

    // 9. Verify article is STILL on account page (linked by username, not author_name)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains("User Article"));
    assert!(body_str.contains("Second Update"));

    // Cleanup files
    let _ = std::fs::remove_file("user-article.html");
    let _ = std::fs::remove_file("snippets/user-article.html.txt");
}

#[tokio::test]
async fn test_create_article() {
    let (app, db) = setup_app().await;

    // 1. Create user who does NOT need password change
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

    // 2. Login
    let login_params = [("username", "admin"), ("password", "password123")];
    let login_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&login_params)))
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

    // 3. Create article (Multipart)
    let boundary = "---------------------------123456789012345678901234567";

    // Create related article and category files for testing
    let related_article_content = "<html><body><!-- SNIPPETS --></body></html>";
    std::fs::write("related-test-article.html", related_article_content).unwrap();
    std::fs::create_dir_all("snippets").unwrap();
    std::fs::write(
        "snippets/related-test-article.html.txt",
        "<div>Related Snippet</div>",
    )
    .unwrap();

    let category_content = "<html><body><!-- SNIPPETS --></body></html>";
    std::fs::write("test-cathegory.html", category_content).unwrap();

    let body = format!(
        "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        Test Article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Test Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        test-cathegory\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        This is a test article text.\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Short text.\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"related_articles\"\r\n\r\n\
        related-test-article.html\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\n\
        Content-Type: image/jpeg\r\n\r\n\
        fake-image-data\r\n\
        --{0}--\r\n",
        boundary
    );

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(header::LOCATION).unwrap(),
        "/test-article.html"
    );

    // Verify files were created
    assert!(std::path::Path::new("test-article.html").exists());

    // Cleanup
    let _ = std::fs::remove_file("test-article.html");
    let _ = std::fs::remove_file("snippets/test-article.html.txt");
    let _ = std::fs::remove_file("related-test-article.html");
    let _ = std::fs::remove_file("snippets/related-test-article.html.txt");
    let _ = std::fs::remove_file("test-cathegory.html");

    // Cleanup uploads
    if let Ok(entries) = std::fs::read_dir("uploads") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".jpg") {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
    }

    // Also cleanup the archive file if it was created
    let now = chrono::Local::now();
    let czech_months = [
        "leden", "unor", "brezen", "duben", "kveten", "cerven", "cervenec", "srpen", "zari",
        "rijen", "listopad", "prosinec",
    ];
    let month_name = czech_months[(now.month() - 1) as usize];
    let archive_filename = format!("archive-test-cathegory-{}-{}.html", month_name, now.year());
    let _ = std::fs::remove_file(archive_filename);
}
