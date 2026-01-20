#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::boundary;
    use axiomatik_web::test_framework::script_base_data::{FAKE_IMAGE_DATA_JPEG, JPEG};
    use axum::http::{header, StatusCode};
    use axum_core::extract::Request;
    use reqwest::Body;
    use std::fs;

    #[tokio::test]
    async fn test_exclusive_main_article_finance() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user2").await;

        let body = ArticleBuilder::new()
            .title("test-Financni trhy v soku")
            .author("Financni Expert")
            .category("finance")
            .text("Dlouhy text o financich")
            .short_text("Kratky text o financich")
            .main()
            .exclusive()
            .image("test.jpg", FAKE_IMAGE_DATA_JPEG, JPEG)
            .image_description("anything")
            .build()
            .unwrap();

        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::SEE_OTHER);

        let updated_index = fs::read_to_string("index.html").unwrap();

        // Check the MAIN_ARTICLE section
        let main_start = updated_index
            .find("<!-- MAIN_ARTICLE -->")
            .expect("MAIN_ARTICLE marker not found");
        let main_end = updated_index
            .find("<!-- /MAIN_ARTICLE -->")
            .expect("/MAIN_ARTICLE marker not found");
        let main_section = &updated_index[main_start..main_end];

        assert!(
            main_section
                .contains(r#"<span class="red">EXKLUZIVNĚ:</span> test-Financni trhy v soku"#),
            "Exclusive tag not found in main article title"
        );

        // Cleanup
        let _ = fs::remove_file("test-financni-trhy-v-soku.html");
        let _ = fs::remove_file("snippets/test-financni-trhy-v-soku.html.txt");
    }
}
