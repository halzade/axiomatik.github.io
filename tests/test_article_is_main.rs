#[cfg(test)]
mod tests {
    use axiomatik_web::trust::article_builder::ArticleBuilder;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::content_type_with_boundary;
    use axum::http::{header, StatusCode};
    use axum_core::extract::Request;
    use reqwest::Body;
    use std::fs;
    use axiomatik_web::test_framework::script_base_data::PNG;

    #[tokio::test]
    async fn test_veda_article_is_main_rotation() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user4").await;
        
        let image_data = script_base::get_test_image_data();
        let body = ArticleBuilder::new()
            .title("Test New Veda Main")
            .author("Author Veda")
            .category("veda")
            .text("Main text of veda article")
            .short_text("Short text of veda article")
            .main()
            .image("test.jpg", &image_data, PNG)
            .image_desc("test description")
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

        // Check MAIN_ARTICLE: should contain Test New Veda Main
        let main_start = updated_index.find("<!-- MAIN_ARTICLE -->").unwrap();
        let main_end = updated_index.find("<!-- /MAIN_ARTICLE -->").unwrap();
        let main_section = &updated_index[main_start..main_end];
        assert!(main_section.contains("Test New Veda Main"));
        assert!(main_section.contains("u/")); // Image should be there

        // Cleanup
        let _ = fs::remove_file("web/test-new-veda-main.html");
        let _ = fs::remove_file("web/u/test-new-veda-main_image_820.jpg");
        let _ = fs::remove_file("web/u/test-new-veda-main_image_50.jpg");
        let _ = fs::remove_file("web/u/test-new-veda-main_image_288.jpg");
        let _ = fs::remove_file("web/u/test-new-veda-main_image_440.jpg");
    }
}
