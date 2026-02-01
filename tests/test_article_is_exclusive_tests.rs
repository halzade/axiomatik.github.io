#[cfg(test)]
mod tests {
    use axiomatik_web::trust::article_builder::ArticleBuilder;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::content_type_with_boundary;
    use axum::http::{header, StatusCode};
    use axum_core::extract::Request;
    use reqwest::Body;
    use std::fs;
    use axiomatik_web::trust::script_base_data::PNG;

    #[tokio::test]
    async fn test_exclusive_main_article_finance() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user2").await;

        let image_data = script_base::get_test_image_data();
        let body = ArticleBuilder::new()
            .title("test-Financni trhy v soku")
            .author("Financni Expert")
            .category("finance")
            .text("Dlouhý text o financich")
            .short_text("Krátký text o financich")
            .main()
            .exclusive()
            .image("test.png", &image_data, PNG)
            .image_desc("anything")
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
        let _ = fs::remove_file("web/test-financni-trhy-v-soku.html");
        let _ = fs::remove_file("web/u/test-financni-trhy-v-soku_image_820.jpg");
        let _ = fs::remove_file("web/u/test-financni-trhy-v-soku_image_50.jpg");
        let _ = fs::remove_file("web/u/test-financni-trhy-v-soku_image_288.jpg");
        let _ = fs::remove_file("web/u/test-financni-trhy-v-soku_image_440.jpg");
    }
}
