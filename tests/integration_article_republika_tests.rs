#[cfg(test)]
mod tests {
    use axum::http::{header, Request};

    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::boundary;
    use reqwest::Body;
    use std::fs;

    #[tokio::test]
    async fn test_republika_article_creation_and_limit() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user5").await;

        let body = ArticleBuilder::new()
            .title("Test Newest Republika")
            .author("Author")
            .category("republika")
            .text("Main text")
            .short_text("Short text of newest article")
            .build();

        script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body.unwrap()))
                .unwrap(),
        )
        .await;

        let updated_index = fs::read_to_string("index.html").unwrap();
        assert!(updated_index.contains("Test Newest Republika"));

        // Count articles in Z_REPUBLIKY section
        let start =
            updated_index.find("<!-- Z_REPUBLIKY -->").unwrap() + "<!-- Z_REPUBLIKY -->".len();
        let end = updated_index.find("<!-- /Z_REPUBLIKY -->").unwrap();
        let section = &updated_index[start..end];
        let count = section.matches("<article").count();
        assert_eq!(count, 10);
        assert!(!section.contains("Article 10")); // Oldest should be gone

        // Cleanup
        let _ = fs::remove_file("test-newest-republika.html");
        let _ = fs::remove_file("snippets/test-newest-republika.html.txt");
    }
}
