#[cfg(test)]
mod tests {
    use std::fs;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_exclusive_main_article_finance() {
        let boundary = "---------------------------123456789012345678901234567";
        let body = format!(
            "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        test-Financni trhy v soku\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Financni Expert\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        finance\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        Dlouhy text o financich\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Kratky text o financich\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"is_main\"\r\n\r\n\
        on\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"is_exclusive\"\r\n\r\n\
        on\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\n\
        Content-Type: image/jpeg\r\n\r\n\
        fake-image-data\r\n\
        --{0}--\r\n",
            boundary
        );

        let _ = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/create")
                    .header(
                        header::CONTENT_TYPE,
                        format!("multipart/form-data; boundary={}", boundary),
                    )
                    .header(header::COOKIE, &cookie)
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

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
        fs::write("index.html", original_index).unwrap();
        let _ = fs::remove_file("test-financni-trhy-v-soku.html");
        let _ = fs::remove_file("snippets/test-financni-trhy-v-soku.html.txt");
    }
}
