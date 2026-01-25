#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::{content_type_with_boundary};
    use axiomatik_web::test_framework::script_base_data::{FAKE_IMAGE_DATA_JPEG, JPEG};
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_validation_create_article() {
        script_base::setup_before_tests_once().await;

        // 1. Create and login user
        let cookie = script_base::setup_user_and_login("user9").await;

        // 2. Create an article with malicious input
        let body = ArticleBuilder::new()
            .title("Title")
            .author("Author")
            .category("republika")
            .text("Content")
            .short_text("Sho\x07rt")
            .image("test.jpg", FAKE_IMAGE_DATA_JPEG, JPEG)
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

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
