#[cfg(test)]
mod tests {
    use axiomatik_web::trust::article_builder::ArticleBuilder;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::content_type_with_boundary;
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;
    use std::fs;
    use axiomatik_web::test_framework::script_base_data::PNG;

    #[tokio::test]
    async fn test_zahranici_article_creation_and_limit() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user7").await;

        let image_data = script_base::get_test_image_data();
        let body = ArticleBuilder::new()
            .title("Test Newest Zahranici")
            .author("Author")
            .category("zahranici")
            .text("Main text")
            .short_text("Short text of newest article")
            .image("test.jpg", &image_data, PNG)
            .image_description("test description")
            .build()
            .unwrap();

        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, content_type_with_boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::SEE_OTHER);

        let updated_index = fs::read_to_string("index.html").unwrap();
        assert!(updated_index.contains("Test Newest Zahranici"));

        // Count articles in ZE_ZAHRANICI section
        let start =
            updated_index.find("<!-- ZE_ZAHRANICI -->").unwrap() + "<!-- ZE_ZAHRANICI -->".len();
        let end = updated_index.find("<!-- /ZE_ZAHRANICI -->").unwrap();
        let section = &updated_index[start..end];

        assert!(section.contains("Test Newest Zahranici"));

        // Cleanup
        let _ = fs::remove_file("web/test-newest-zahranici.html");
        let _ = fs::remove_file("web/u/test-newest-zahranici_image_820.jpg");
        let _ = fs::remove_file("web/u/test-newest-zahranici_image_50.jpg");
        let _ = fs::remove_file("web/u/test-newest-zahranici_image_288.jpg");
        let _ = fs::remove_file("web/u/test-newest-zahranici_image_440.jpg");
    }
}
