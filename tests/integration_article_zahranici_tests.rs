#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{header, Request},
    };
    use std::fs;
    use tower::ServiceExt;

    fn prepare_index_with_articles(
        index_content: &mut String,
        marker_start: &str,
        marker_end: &str,
        count: usize,
    ) {
        let mut articles_html = String::new();
        for i in 1..=count {
            articles_html.push_str(&format!(
                r#"<article class="card">
                <a href="article-{0}.html"><h2>Article {0}</h2></a>
                <a href="article-{0}.html"><p>Short text {0}</p></a>
            </article>"#,
                i
            ));
        }

        if let (Some(start), Some(end)) = (
            index_content.find(marker_start),
            index_content.find(marker_end),
        ) {
            index_content.replace_range(start + marker_start.len()..end, &articles_html);
        } else {
            panic!("Markers {} and {} not found", marker_start, marker_end);
        }
    }

    #[tokio::test]
    async fn test_zahranici_article_creation_and_limit() {
        let (app, _db, cookie, original_index) = script_base::setup_test_environment().await;
        let mut test_index = original_index.clone();
        prepare_index_with_articles(
            &mut test_index,
            "<!-- ZE_ZAHRANICI -->",
            "<!-- /ZE_ZAHRANICI -->",
            10,
        );
        fs::write("index.html", test_index).unwrap();

        let boundary = "---------------------------123456789012345678901234567";
        let body = format!(
            "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        test-Newest Zahranici\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        zahranici\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        Main text\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Short text of newest article\r\n\
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
        assert!(updated_index.contains("test-Newest Zahranici"));

        // Count articles in ZE_ZAHRANICI section
        let start =
            updated_index.find("<!-- ZE_ZAHRANICI -->").unwrap() + "<!-- ZE_ZAHRANICI -->".len();
        let end = updated_index.find("<!-- /ZE_ZAHRANICI -->").unwrap();
        let section = &updated_index[start..end];
        let count = section.matches("<article").count();
        assert_eq!(count, 10);
        assert!(!section.contains("Article 10")); // Oldest should be gone

        // Cleanup
        fs::write("index.html", original_index).unwrap();
        let _ = fs::remove_file("test-newest-zahranici.html");
        let _ = fs::remove_file("snippets/test-newest-zahranici.html.txt");
    }
}
