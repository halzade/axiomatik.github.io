#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::boundary;
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;
    use std::fs;
    use std::path::Path;
    use image::GenericImageView;

    #[tokio::test]
    async fn test_image_upload_resized_copies() {
        script_base::setup_before_tests_once().await;

        // 1. Setup user and login
        let cookie = script_base::setup_user_and_login("image_tester").await;

        // 2. Read placeholder image
        let image_data = fs::read("images/placeholder.png").expect("Failed to read placeholder image");
        let png_mime = "image/png";

        // 3. Create article with name starting with "text-"
        let title = "text-testing-upload";
        let body = ArticleBuilder::new()
            .title(title)
            .author("Tester")
            .category("republika")
            .text("Test content")
            .short_text("Short text")
            .image("placeholder.png", &image_data, png_mime)
            .image_description("Description")
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

        // 4. Verify that four copies with required dimensions were saved in uploads/
        let base_name = "text-testing-upload";
        let expected_files = vec![
            (format!("uploads/{}_image_820.png", base_name), 820, None), // None means height is proportional or we don't strictly check it as per save_file_field_with_name logic (it uses resize(820, height, ...))
            (format!("uploads/{}_image_50.png", base_name), 50, Some(50)),
            (format!("uploads/{}_image_288.png", base_name), 288, Some(211)),
            (format!("uploads/{}_image_440.png", base_name), 440, Some(300)),
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
        let _ = fs::remove_file("text-testing-upload.html");
        let _ = fs::remove_file("snippets/text-testing-upload.html.txt");
        let _ = fs::remove_file(format!("uploads/{}_image_820.png", base_name));
        let _ = fs::remove_file(format!("uploads/{}_image_50.png", base_name));
        let _ = fs::remove_file(format!("uploads/{}_image_288.png", base_name));
        let _ = fs::remove_file(format!("uploads/{}_image_440.png", base_name));
    }
}
