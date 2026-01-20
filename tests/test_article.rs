#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::boundary;
    use axiomatik_web::test_framework::script_base_data::{
        FAKE_AUDIO_DATA_MP3, FAKE_IMAGE_DATA_JPEG, JPEG, MP3,
    };
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_create_article() {
        script_base::setup_before_tests_once().await;

        // 1. Create a user who does NOT need password change
        let cookie = script_base::setup_user_and_login("user6").await;

        // 3. Create article (Multipart)
        // Create related article and category files for testing
        let related_article_content = "<html><body><!-- SNIPPETS --></body></html>";
        std::fs::write("related-test-article.html", related_article_content).unwrap();
        std::fs::create_dir_all("snippets").unwrap();
        std::fs::write(
            "snippets/related-test-article.html.txt",
            "<div>Related Snippet</div>",
        )
        .unwrap();

        let body = ArticleBuilder::new()
            .title("Test Article")
            .author("Test Author")
            .category("republika")
            .text("This is a test article text.")
            .short_text("Short text.")
            .related_articles("related-test-article.html")
            .image("test.jpg", FAKE_IMAGE_DATA_JPEG, JPEG)
            .image_description("test description")
            .audio("test.mp3", FAKE_AUDIO_DATA_MP3, MP3)
            .build();

        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, boundary())
                .header(header::COOKIE, &cookie)
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

        // 2. Request the article (to increment views)
        let response_article = script_base::one_shot(
            Request::builder()
                .uri("/test-article.html")
                .body(Body::default())
                .unwrap(),
        )
        .await;
        assert_eq!(response_article.status(), StatusCode::OK);

        // 3. Check the account page for views
        let account_resp = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(account_resp.status(), StatusCode::OK);

        // Verify audio player placement
        let article_content = std::fs::read_to_string("test-article.html").unwrap();
        let audio_pos = article_content.find("<audio").expect("Audio player not found");
        let text_pos = article_content
            .find("This is a test article text.")
            .expect("Article text not found");
        assert!(
            audio_pos < text_pos,
            "Audio player should be before article text"
        );

        // Cleanup
        let _ = std::fs::remove_file("test-article.html");
        let _ = std::fs::remove_file("related-test-article.html");
        let _ = std::fs::remove_file("snippets/test-article.html.txt");
        let _ = std::fs::remove_file("snippets/related-test-article.html.txt");
    }
}
