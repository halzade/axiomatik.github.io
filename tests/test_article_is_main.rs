#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::boundary;
    use axiomatik_web::test_framework::script_base_data::{FAKE_IMAGE_DATA_JPEG, JPEG};
    use axum_core::extract::Request;
    use http::header;
    use reqwest::Body;
    use std::fs;

    #[tokio::test]
    async fn test_veda_article_is_main_rotation() {
        script_base::setup_before_tests_once().await;

        let cookie = script_base::setup_user_and_login("user4").await;

        let body = ArticleBuilder::new()
            .title("test-New Veda Main")
            .author("Author Veda")
            .category("veda")
            .text("Main text of veda article")
            .short_text("Short text of veda article")
            .is_main()
            .image("test.jpg", FAKE_IMAGE_DATA_JPEG, JPEG)
            .image_description("test description")
            .build()
            .unwrap();

        script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await;

        let updated_index = fs::read_to_string("index.html").unwrap();

        // Check MAIN_ARTICLE: should contain test-New Veda Main
        let main_start = updated_index.find("<!-- MAIN_ARTICLE -->").unwrap();
        let main_end = updated_index.find("<!-- /MAIN_ARTICLE -->").unwrap();
        let main_section = &updated_index[main_start..main_end];
        assert!(main_section.contains("test-New Veda Main"));
        assert!(main_section.contains("uploads/")); // Image should be there

        // Check SECOND_ARTICLE: should contain Old Main Article (stripped of image, class changed to first-article, h1 changed to h2)
        let second_start = updated_index.find("<!-- SECOND_ARTICLE -->").unwrap();
        let second_end = updated_index.find("<!-- /SECOND_ARTICLE -->").unwrap();
        let second_section = &updated_index[second_start..second_end];
        assert!(second_section.contains("Old Main Article"));
        assert!(
            second_section.contains("class=\"first-article\"")
                || second_section.contains("class='first-article'")
        );
        assert!(second_section.contains("<h2>Old Main Article</h2>"));
        assert!(!second_section.contains("<img")); // Image should be stripped

        // Check THIRD_ARTICLE: should contain Old Second Article (class changed to second-article)
        let third_start = updated_index.find("<!-- THIRD_ARTICLE -->").unwrap();
        let third_end = updated_index.find("<!-- /THIRD_ARTICLE -->").unwrap();
        let third_section = &updated_index[third_start..third_end];
        assert!(third_section.contains("Old Second Article"));
        assert!(
            third_section.contains("class=\"second-article\"")
                || third_section.contains("class='second-article'")
        );

        // Cleanup
        let _ = fs::remove_file("test-new-veda-main.html");
        let _ = fs::remove_file("snippets/test-new-veda-main.html.txt");
    }
}
