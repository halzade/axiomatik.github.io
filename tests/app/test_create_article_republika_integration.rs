#[cfg(test)]
mod tests {
    use axum::http::{header, Request, StatusCode};

    use reqwest::Body;
    use std::fs;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_republika_article_creation_and_limit() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user5")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        ac.login()
            .username("user5")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify()?;

        let cookie = ac.login().get_cookie().unwrap();

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
