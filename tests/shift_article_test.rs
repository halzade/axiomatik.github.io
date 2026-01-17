use crate::script_base::serialize;
use axiomatik_web::db;
use axiomatik_web::script_base;
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_shift_main_article_removes_exclusive_tag() {
    let (app, db) = script_base::setup_app().await;
    let original_index = std::fs::read_to_string("index.html").unwrap_or_default();

    // 1. Create user
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

    // Prepare index.html with known markers
    let initial_index = r#"
        <!-- MAIN_ARTICLE -->
        <!-- /MAIN_ARTICLE -->
        <!-- SECOND_ARTICLE -->
        <!-- /SECOND_ARTICLE -->
        <!-- THIRD_ARTICLE -->
        <!-- /THIRD_ARTICLE -->
    "#;
    std::fs::write("index.html", initial_index).unwrap();

    // 3. Create first article as MAIN and EXCLUSIVE
    let boundary = "---------------------------123456789012345678901234567";
    let body1 = format!(
        "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        test-Exclusive Article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"is_exclusive\"\r\n\r\n\
        on\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"is_main\"\r\n\r\n\
        on\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Test Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        republika\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        First article text.\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        First short text.\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"image\"; filename=\"test1.jpg\"\r\n\
        Content-Type: image/jpeg\r\n\r\n\
        fake-image-data-1\r\n\
        --{0}--\r\n",
        boundary
    );

    let response1 = app
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
                .body(Body::from(body1))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response1.status(), StatusCode::SEE_OTHER);

    // Verify it is main and exclusive in index.html
    let index_after1 = std::fs::read_to_string("index.html").unwrap();
    assert!(index_after1.contains(r#"<span class="red">EXKLUZIVNĚ:</span> test-Exclusive Article"#));

    // 4. Create second article as MAIN (not necessarily exclusive)
    let body2 = format!(
        "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        test-New Main Article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"is_main\"\r\n\r\n\
        on\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Test Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        republika\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        Second article text.\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Second short text.\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"image\"; filename=\"test2.jpg\"\r\n\
        Content-Type: image/jpeg\r\n\r\n\
        fake-image-data-2\r\n\
        --{0}--\r\n",
        boundary
    );

    let response2 = app
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
                .body(Body::from(body2))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response2.status(), StatusCode::SEE_OTHER);

    // 5. Verify index.html: New Main is MAIN, and Old Main (Exclusive Article) is SECOND and NO LONGER EXCLUSIVE
    let index_after2 = std::fs::read_to_string("index.html").unwrap();
    
    // Check MAIN_ARTICLE
    let main_start = index_after2.find("<!-- MAIN_ARTICLE -->").unwrap();
    let main_end = index_after2.find("<!-- /MAIN_ARTICLE -->").unwrap();
    let main_content = &index_after2[main_start..main_end];
    assert!(main_content.contains("test-New Main Article"));

    // Check SECOND_ARTICLE
    let second_start = index_after2.find("<!-- SECOND_ARTICLE -->").unwrap();
    let second_end = index_after2.find("<!-- /SECOND_ARTICLE -->").unwrap();
    let second_content = &index_after2[second_start..second_end];
    
    assert!(second_content.contains("test-Exclusive Article"));
    assert!(!second_content.contains(r#"<span class="red">EXKLUZIVNĚ:</span>"#), "EXKLUZIVNĚ tag should be removed when shifted to second article");

    // Cleanup
    let _ = std::fs::remove_file("test-exclusive-article.html");
    let _ = std::fs::remove_file("test-new-main-article.html");
    let _ = std::fs::remove_file("snippets/test-exclusive-article.html.txt");
    let _ = std::fs::remove_file("snippets/test-new-main-article.html.txt");
    std::fs::write("index.html", original_index).unwrap();
}
