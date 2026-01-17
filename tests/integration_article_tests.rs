use crate::script_base::serialize;
use axiomatik_web::db;
use axiomatik_web::script_base;
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use chrono::Datelike;
use tower::ServiceExt;

#[tokio::test]
async fn test_create_article() {
    let (app, db) = script_base::setup_app().await;

    // 1. Create user who does NOT need password change
    let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
    db.create_user(database::User {
        username: "admin".to_string(),
        author_name: "admin".to_string(),
        password_hash,
        needs_password_change: false,
        role: database::Role::Editor,
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
    std::fs::write("test-category.html", category_content).unwrap();

    let body = format!(
        "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        Test Article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Test Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        test-category\r\n\
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
        Content-Disposition: form-data; name=\"image_description\"\r\n\r\n\
        test description\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\n\
        Content-Type: image/jpeg\r\n\r\n\
        fake-image-data\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"audio\"; filename=\"test.mp3\"\r\n\
        Content-Type: audio/mpeg\r\n\r\n\
        fake-audio-data\r\n\
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

    // 2. Request the article (to increment views)
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/test-article.html")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // 3. Check account page for views
    let account_resp = app
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

    assert_eq!(account_resp.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(account_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_str.contains("Přečteno: 1x"));

    // Verify audio player placement
    let article_content = std::fs::read_to_string("test-article.html").unwrap();
    let audio_pos = article_content.find("<audio").unwrap();
    let text_pos = article_content.find("This is a test article text.").unwrap();
    assert!(audio_pos < text_pos, "Audio player should be before article text");
    assert!(article_content.contains("<div  class=\"container\">"), "Should contain div with double space as in reference");

    // Cleanup
    let _ = std::fs::remove_file("test-article.html");
    let _ = std::fs::remove_file("snippets/test-article.html.txt");
    let _ = std::fs::remove_file("related-test-article.html");
    let _ = std::fs::remove_file("snippets/related-test-article.html.txt");
    let _ = std::fs::remove_file("test-category.html");

    // Cleanup uploads
    if let Ok(entries) = std::fs::read_dir("uploads") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("test-") && (name.ends_with(".jpg") || name.ends_with(".mp3")) {
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
    let archive_filename = format!("archive-test-category-{}-{}.html", month_name, now.year());
    let _ = std::fs::remove_file(archive_filename);
}
