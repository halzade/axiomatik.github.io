#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::content_type_with_boundary;
    use axiomatik_web::test_framework::script_base_data::{FAKE_AUDIO_DATA_MP3, MP3, PNG};
    use axum::http::{header, Request, StatusCode};
    use header::{CONTENT_TYPE, COOKIE};
    use reqwest::Body;
    use std::fs::read_to_string;

    #[tokio::test]
    async fn test_create_article() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user6").await;

        let image_data = script_base::get_test_image_data();
        let body = ArticleBuilder::new()
            .title("Test Article")
            .author("Test Author")
            .category("republika")
            .text("This is a test article text.")
            .short_text("Short text.")
            .related_articles("related-test-article.html")
            .image("test.jpg", &image_data, PNG)
            .image_description("test description")
            .audio("test.mp3", FAKE_AUDIO_DATA_MP3, MP3)
            .build();

        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(CONTENT_TYPE, content_type_with_boundary())
                .header(COOKIE, &cookie)
                .body(Body::from(body.unwrap()))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(header::LOCATION).unwrap(),
            "test-article.html"
        );

        // Verify files were created
        assert!(std::path::Path::new("test-article.html").exists());

        // Request the article
        let response_article = script_base::one_shot(
            Request::builder()
                .uri("/test-article.html")
                .body(Body::default())
                .unwrap(),
        )
        .await;
        assert_eq!(response_article.status(), StatusCode::OK);

        // Check the account page for views
        let account_resp = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(account_resp.status(), StatusCode::OK);

        // Verify audio player placement
        let article_content = read_to_string("test-article.html").unwrap();
        let audio_pos = article_content
            .find("<audio")
            .expect("Audio player not found");
        let text_pos = article_content
            .find("This is a test article text.")
            .expect("Article text not found");
        assert!(
            audio_pos < text_pos,
            "Audio player should be before article text"
        );

        // Cleanup
        let _ = std::fs::remove_file("web/test-article.html");
        let _ = std::fs::remove_file("web/u/test-article_image_820.jpg");
        let _ = std::fs::remove_file("web/u/test-article_image_50.jpg");
        let _ = std::fs::remove_file("web/u/test-article_image_288.jpg");
        let _ = std::fs::remove_file("web/u/test-article_image_440.jpg");
        let _ = std::fs::remove_file("web/u/test-article_audio.mp3");
    }
}
