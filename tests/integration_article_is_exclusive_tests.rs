#[cfg(test)]
mod tests {
    use axum_core::extract::Request;
    use http::header;
    use reqwest::Body;
    use std::fs;
    use axiomatik_web::test_framework::article_builder::{ArticleBuilder, BOUNDARY};
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::FAKE_IMAGE_DATA;

    #[tokio::test]
    async fn test_exclusive_main_article_finance() {
        let body = ArticleBuilder::new()
            .title("test-Financni trhy v soku")
            .author("Financni Expert")
            .category("finance")
            .text("Dlouhy text o financich")
            .short_text("Kratky text o financich")
            .is_main(true)
            .is_exclusive(true)
            .image("test.jpg", FAKE_IMAGE_DATA)
            .build()
            .unwrap();

        script_base::one_shot(Request::builder()
                    .method("POST")
                    .uri("/create")
                    .header(
                        header::CONTENT_TYPE,
                        format!("multipart/form-data; boundary={}", BOUNDARY),
                    )
                    .header(header::COOKIE, &cookie)
                    .body(Body::from(body))
                    .unwrap(),
            ).await;

        let updated_index = fs::read_to_string("index.html").unwrap();

        // Check MAIN_ARTICLE section
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
