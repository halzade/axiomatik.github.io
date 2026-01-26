#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::content_type_with_boundary;
    use axum::http::{header, StatusCode};
    use reqwest::Body;
    use axiomatik_web::test_framework::script_base_data::PNG;

    #[tokio::test]
    async fn test_shift_main_article_removes_exclusive_tag() {
        script_base::setup_before_tests_once().await;

        // Create user
        let cookie = script_base::setup_user_and_login("user3").await;

        let image_data = script_base::get_test_image_data();
        // Create the first article as MAIN and EXCLUSIVE
        let body1 = ArticleBuilder::new()
            .title("test-Exclusive Article")
            .exclusive()
            .main()
            .author("Test Author")
            .category("republika")
            .text("First article text.")
            .short_text("First short text.")
            .image("test1.jpg", &image_data, PNG)
            .image_description("test description")
            .build();

        let response1 = script_base::one_shot(
            axum_core::extract::Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, content_type_with_boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body1.unwrap()))
                .unwrap(),
        )
        .await;

        assert_eq!(response1.status(), StatusCode::SEE_OTHER);

        // Verify it is main and exclusive in index.html
        let index_after1 = std::fs::read_to_string("index.html").unwrap();
        assert!(
            index_after1.contains(r#"<span class="red">EXKLUZIVNĚ:</span> test-Exclusive Article"#)
        );

        let image_data = script_base::get_test_image_data();
        
        // 4. Create the second article as MAIN (not necessarily exclusive)
        let body2 = ArticleBuilder::new()
            .title("Test New Main Article")
            .main()
            .author("Test Author")
            .category("republika")
            .text("Second article text.")
            .short_text("Second short text.")
            .image("test2.jpg", &image_data, PNG)
            .image_description("test description")
            .build();

        let response2 = script_base::one_shot(
            axum_core::extract::Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, content_type_with_boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body2.unwrap()))
                .unwrap(),
        )
        .await;

        assert_eq!(response2.status(), StatusCode::SEE_OTHER);

        // 5. Verify index.html: New Main is MAIN, and Old Main (Exclusive Article) is SECOND and NO LONGER EXCLUSIVE
        let index_after2 = std::fs::read_to_string("index.html").unwrap();

        // Check MAIN_ARTICLE
        let main_start = index_after2.find("<!-- MAIN_ARTICLE -->").unwrap();
        let main_end = index_after2.find("<!-- /MAIN_ARTICLE -->").unwrap();
        let main_content = &index_after2[main_start..main_end];
        assert!(main_content.contains("Test New Main Article"));

        // Check SECOND_ARTICLE
        let second_start = index_after2.find("<!-- SECOND_ARTICLE -->").unwrap();
        let second_end = index_after2.find("<!-- /SECOND_ARTICLE -->").unwrap();
        let second_content = &index_after2[second_start..second_end];

        assert!(second_content.contains("test-Exclusive Article"));
        assert!(
            !second_content.contains(r#"<span class="red">EXKLUZIVNĚ:</span>"#),
            "EXKLUZIVNĚ tag should be removed when shifted to second article"
        );

        // Cleanup
        let _ = std::fs::remove_file("web/test-exclusive-article.html");
        let _ = std::fs::remove_file("web/test-new-main-article.html");
        let _ = std::fs::remove_file("web/uploads/test-exclusive-article_image_820.jpg");
        let _ = std::fs::remove_file("web/uploads/test-exclusive-article_image_50.jpg");
        let _ = std::fs::remove_file("web/uploads/test-exclusive-article_image_288.jpg");
        let _ = std::fs::remove_file("web/uploads/test-exclusive-article_image_440.jpg");
        let _ = std::fs::remove_file("web/uploads/test-new-main-article_image_820.jpg");
        let _ = std::fs::remove_file("web/uploads/test-new-main-article_image_50.jpg");
        let _ = std::fs::remove_file("web/uploads/test-new-main-article_image_288.jpg");
        let _ = std::fs::remove_file("web/uploads/test-new-main-article_image_440.jpg");
    }
}
