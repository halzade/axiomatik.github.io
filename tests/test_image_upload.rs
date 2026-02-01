#[cfg(test)]
mod tests {
    use axiomatik_web::trust::article_builder::ArticleBuilder;
    use axiomatik_web::trust::article_builder::ArticleBuilder;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::content_type_with_boundary;
    use axum::http::{header, Request, StatusCode};
    use image::GenericImageView;
    use reqwest::Body;
    use std::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_image_upload_resized_copies() {
        script_base::setup_before_tests_once().await;

        // 1. Set up user and login
        let cookie = script_base::setup_user_and_login("image_tester").await;

        // 2. Read placeholder image
        let image_data =
            fs::read("web/images/placeholder_1024.png").expect("Failed to read placeholder image");
        let png_mime = "image/png";

        // 3. Create an article with a name starting with "text-"
        let title = "text-testing-upload";
        let body = ArticleBuilder::new()
            .title(title)
            .author("Tester")
            .category("republika")
            .text("Test content")
            .short_text("Short text")
            .image("placeholder_1024.png", &image_data, png_mime)
            .image_desc("Description")
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

        // TODO do update dimension verification

        // 4. Verify that four copies with required dimensions were saved in u/
        let file_base = "text-testing-upload";
        let expected_files = vec![
            (format!("u/{}_image_820.png", file_base), 820, None), // None means height is proportional, or we don't strictly check it as per save_file_field_with_name logic (it uses resize(820, height, ...))
            (format!("u/{}_image_50.png", file_base), 50, Some(50)),
            (format!("u/{}_image_288.png", file_base), 288, Some(211)),
            (format!("u/{}_image_440.png", file_base), 440, Some(300)),
        ];

        for (path_str, expected_w, expected_h_opt) in expected_files {
            let path = Path::new(&path_str);
            assert!(path.exists(), "File {} does not exist", path_str);

            let img = image::open(path).expect(&format!("Failed to open saved image {}", path_str));
            let (w, h) = img.dimensions();

            assert_eq!(w, expected_w, "Width mismatch for {}", path_str);
            if let Some(expected_h) = expected_h_opt {
                assert_eq!(h, expected_h, "Height mismatch for {}", path_str);
            }
        }

        // Cleanup
        let _ = fs::remove_file("web/text-testing-upload.html");
        let _ = fs::remove_file(format!("u/{}_image_820.png", file_base));
        let _ = fs::remove_file(format!("u/{}_image_50.png", file_base));
        let _ = fs::remove_file(format!("u/{}_image_288.png", file_base));
        let _ = fs::remove_file(format!("u/{}_image_440.png", file_base));
    }
}
