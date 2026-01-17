#[cfg(test)]
mod tests {
    use axiomatik_web::database;
    use axiomatik_web::test_framework::article_builder::{ArticleBuilder, BOUNDARY};
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::{serialize, FAKE_IMAGE_DATA};
    use axum::http::{header, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_shift_main_article_removes_exclusive_tag() {
        // 1. Create user
        let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
        database::create_user(database::User {
            username: "admin".to_string(),
            author_name: "admin".to_string(),
            password_hash,
            needs_password_change: false,
            role: database::Role::Editor,
        })
        .await
        .unwrap();

        // 2. Login
        let login_params = [("username", "admin"), ("password", "password123")];
        let login_resp = script_base::one_shot(
            axum_core::extract::Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&login_params)))
                .unwrap(),
        )
        .await;

        let cookie = login_resp
            .headers()
            .get(header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // Prepare index.html with known markers
        let initial_index = r#"
        <!-- MAIN_ARTICLE -->
        <!-- /MAIN_ARTICLE -->
        <!-- SECOND_ARTICLE -->
        <!-- /SECOND_ARTICLE -->
        <!-- THIRD_ARTICLE -->
        <!-- /THIRD_ARTICLE -->
    "#;
        std::fs::write("index.html", initial_index).unwrap();

        // 3. Create first article as MAIN and EXCLUSIVE
        let body1 = ArticleBuilder::new()
            .title("test-Exclusive Article")
            .is_exclusive(true)
            .is_main(true)
            .author("Test Author")
            .category("republika")
            .text("First article text.")
            .short_text("First short text.")
            .image("test1.jpg", FAKE_IMAGE_DATA, "image/jpeg")
            .build();

        let response1 = script_base::one_shot(
            axum_core::extract::Request::builder()
                .method("POST")
                .uri("/create")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", BOUNDARY),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body1))
                .unwrap(),
        )
        .await;

        assert_eq!(response1.status(), StatusCode::SEE_OTHER);

        // Verify it is main and exclusive in index.html
        let index_after1 = std::fs::read_to_string("index.html").unwrap();
        assert!(
            index_after1.contains(r#"<span class="red">EXKLUZIVNĚ:</span> test-Exclusive Article"#)
        );

        // 4. Create second article as MAIN (not necessarily exclusive)
        let body2 = ArticleBuilder::new()
            .title("test-New Main Article")
            .is_main(true)
            .author("Test Author")
            .category("republika")
            .text("Second article text.")
            .short_text("Second short text.")
            .image("test2.jpg", FAKE_IMAGE_DATA, "image/jpeg")
            .build()?;

        let response2 = script_base::one_shot(
            axum_core::extract::Request::builder()
                .method("POST")
                .uri("/create")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", BOUNDARY),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body2))
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
        assert!(main_content.contains("test-New Main Article"));

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
        let _ = std::fs::remove_file("test-exclusive-article.html");
        let _ = std::fs::remove_file("test-new-main-article.html");
        let _ = std::fs::remove_file("snippets/test-exclusive-article.html.txt");
        let _ = std::fs::remove_file("snippets/test-new-main-article.html.txt");
        std::fs::write("index.html", original_index).unwrap();
    }
}
