#[cfg(test)]
mod tests {
    use axum::http::{header, Request, StatusCode};

    use axiomatik_web::trust::article_builder::ArticleBuilder;
    use axiomatik_web::trust::utils;
    use axiomatik_web::trust::utils::{content_type_with_boundary, TrustError};
    use axiomatik_web::trust::media_data::PNG;
    use reqwest::Body;
    use std::fs;

    #[tokio::test]
    async fn test_republika_article_creation_and_limit() -> Result<(), TrustError> {
        utils::setup_before_tests_once().await;

        let cookie = utils::setup_user_and_login("user5").await;

        let image_data = utils::get_test_image_data();
        let body = ArticleBuilder::new()
            .title("Test Newest Republika")
            .author("Author")
            .category("republika")
            .text("Main text")
            .short_text("Short text of newest article")
            .image("test.jpg", &image_data, PNG)
            .image_desc("test description")
            .build();

        let response = utils::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, content_type_with_boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body.unwrap()))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::SEE_OTHER);

        let updated_index = fs::read_to_string("index.html").unwrap();
        assert!(updated_index.contains("Test Newest Republika"));

        // Count articles in Z_REPUBLIKY section
        assert!(updated_index.contains("Test Newest Republika"));

        // Cleanup
        assert!(fs::remove_file("web/test-newest-republika.html").is_ok());
        assert!(fs::remove_file("web/u/test-newest-republika_image_820.jpg").is_ok());
        assert!(fs::remove_file("web/u/test-newest-republika_image_50.jpg").is_ok());
        assert!(fs::remove_file("web/u/test-newest-republika_image_288.jpg").is_ok());
        assert!(fs::remove_file("web/u/test-newest-republika_image_440.jpg").is_ok());

        Ok(())
    }
}
