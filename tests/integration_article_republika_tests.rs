#[cfg(test)]
mod tests {
    use axum::{
        http::{header, Request},
    };

    use std::fs;
    use reqwest::Body;
    use tower::ServiceExt;
    use axiomatik_web::test_framework::article_builder::{ArticleBuilder, BOUNDARY};
    use axiomatik_web::test_framework::script_base;

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
    async fn test_republika_article_creation_and_limit() {
        
        let mut test_index = original_index.clone();
        prepare_index_with_articles(
            &mut test_index,
            "<!-- Z_REPUBLIKY -->",
            "<!-- /Z_REPUBLIKY -->",
            10,
        );
        fs::write("index.html", test_index).unwrap();

        let body = ArticleBuilder::new()
            .title("test-Newest Republika")
            .author("Author")
            .category("republika")
            .text("Main text")
            .short_text("Short text of newest article")
            .build()?;

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
            )
            .await;

        let updated_index = fs::read_to_string("index.html").unwrap();
        assert!(updated_index.contains("test-Newest Republika"));

        // Count articles in Z_REPUBLIKY section
        let start =
            updated_index.find("<!-- Z_REPUBLIKY -->").unwrap() + "<!-- Z_REPUBLIKY -->".len();
        let end = updated_index.find("<!-- /Z_REPUBLIKY -->").unwrap();
        let section = &updated_index[start..end];
        let count = section.matches("<article").count();
        assert_eq!(count, 10);
        assert!(!section.contains("Article 10")); // Oldest should be gone

        // Cleanup
        let _ = fs::remove_file("test-newest-republika.html");
        let _ = fs::remove_file("snippets/test-newest-republika.html.txt");
    }
}
