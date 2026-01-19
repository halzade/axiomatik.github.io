#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::boundary;
    use axiomatik_web::test_framework::script_base_data::{FAKE_IMAGE_DATA_JPEG, JPEG};
    use axum::http::{header, Request};
    use reqwest::Body;
    use std::fs;

    #[tokio::test]
    async fn test_zahranici_article_creation_and_limit() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user7").await;

        let body = ArticleBuilder::new()
            .title("test-Newest Zahranici")
            .author("Author")
            .category("zahranici")
            .text("Main text")
            .short_text("Short text of newest article")
            .image("test.jpg", FAKE_IMAGE_DATA_JPEG, JPEG)
            .image_description("test description")
            .build()
            .unwrap();

        script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await;

        let updated_index = fs::read_to_string("index.html").unwrap();
        assert!(updated_index.contains("test-Newest Zahranici"));

        // Count articles in ZE_ZAHRANICI section
        let start =
            updated_index.find("<!-- ZE_ZAHRANICI -->").unwrap() + "<!-- ZE_ZAHRANICI -->".len();
        let end = updated_index.find("<!-- /ZE_ZAHRANICI -->").unwrap();
        let section = &updated_index[start..end];
        let count = section.matches("<article").count();
        assert_eq!(count, 10);
        assert!(!section.contains("Article 10")); // Oldest should be gone

        // Cleanup
        let _ = fs::remove_file("test-newest-zahranici.html");
        let _ = fs::remove_file("snippets/test-newest-zahranici.html.txt");
    }
}
