use axiomatik_web::{app, db};
use axum::{
    body::Body,
    http::{Request, header},
};
use std::sync::Arc;
use tower::ServiceExt;
use std::fs;

#[tokio::test]
async fn test_republika_limit_10_articles() {
    // Prepare index.html with 10 articles in Z_REPUBLIKY section
    let mut articles_html = String::new();
    for i in 1..=10 {
        articles_html.push_str(&format!(
            r#"<article class="card">
                <a href="article-{0}.html"><h2>Article {0}</h2></a>
                <a href="article-{0}.html"><p>Short text {0}</p></a>
            </article>"#,
            i
        ));
    }

    let original_index = format!(r#"<!DOCTYPE html>
<html>
<body>
    <section class="article-grid">
        <!-- Z_REPUBLIKY -->{}<!-- /Z_REPUBLIKY -->
    </section>
</body>
</html>"#, articles_html);
    fs::write("index.html", original_index).unwrap();

    // Setup app
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
    }).await.unwrap();

    let login_resp = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from("username=admin&password=password123"))
            .unwrap()
    ).await.unwrap();
    
    let cookie = login_resp.headers().get(header::SET_COOKIE).unwrap().to_str().unwrap().to_string();

    // Create new article in "republika" category (it should become the 11th, so one should be removed)
    let boundary = "---------------------------123456789012345678901234567";
    let body = format!(
        "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        Newest Article\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        republika\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        Main text\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Short text of newest article\r\n\
        --{0}--\r\n",
        boundary
    );

    let _ = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header(header::CONTENT_TYPE, format!("multipart/form-data; boundary={}", boundary))
            .header(header::COOKIE, &cookie)
            .body(Body::from(body))
            .unwrap()
    ).await.unwrap();

    // Verify index.html update
    let updated_index = fs::read_to_string("index.html").unwrap();
    
    // Should contain the newest article
    assert!(updated_index.contains("Newest Article"));
    assert!(updated_index.contains("Short text of newest article"));
    
    // Count articles
    let count = updated_index.matches("<article").count();
    assert_eq!(count, 10, "There should be exactly 10 articles");

    // Article 10 should be removed as it was the oldest in our setup (Articles 1-10 were present, Newest was added to top)
    // Wait, in my setup I pushed them in order 1, 2, ..., 10. 
    // The implementation prepends to the top.
    // So if we have [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] and we prepend "Newest", it becomes ["Newest", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // truncate(10) will keep ["Newest", 1, 2, 3, 4, 5, 6, 7, 8, 9]
    // So Article 10 should be gone.
    assert!(!updated_index.contains("Article 10"), "Oldest article should be removed");
    assert!(updated_index.contains("Article 1"), "Article 1 should still be there");

    // Cleanup
    let _ = fs::remove_file("index.html");
    let _ = fs::remove_file("newest-article.html");
    let _ = fs::remove_dir_all("snippets");
}
