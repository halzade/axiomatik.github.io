#[cfg(test)]
mod tests {
    use axum::http::{header, Request, StatusCode};

    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::content_type_with_boundary;
    use reqwest::Body;
    use std::fs;
    use axiomatik_web::test_framework::script_base_data::PNG;

    #[tokio::test]
    async fn test_republika_article_creation_and_limit() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user5").await;

        let image_data = script_base::get_test_image_data();
        let body = ArticleBuilder::new()
            .title("Test Newest Republika")
            .author("Author")
            .category("republika")
            .text("Main text")
            .short_text("Short text of newest article")
            .image("test.jpg", &image_data, PNG)
            .image_description("test description")
            .build();

        let response = script_base::one_shot(
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
        let start =
            updated_index.find("<!-- Z_REPUBLIKY -->").unwrap() + "<!-- Z_REPUBLIKY -->".len();
        let end = updated_index.find("<!-- /Z_REPUBLIKY -->").unwrap();
        let section = &updated_index[start..end];

        assert!(section.contains("Test Newest Republika"));

        // Cleanup
        let _ = fs::remove_file("web/test-newest-republika.html");
        let _ = fs::remove_file("web/u/test-newest-republika_image_820.jpg");
        let _ = fs::remove_file("web/u/test-newest-republika_image_50.jpg");
        let _ = fs::remove_file("web/u/test-newest-republika_image_288.jpg");
        let _ = fs::remove_file("web/u/test-newest-republika_image_440.jpg");
    }
}
