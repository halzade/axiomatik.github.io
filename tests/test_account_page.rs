#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::ArticleBuilder;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::{boundary, response_to_body, serialize};
    use axiomatik_web::test_framework::script_base_data::{FAKE_IMAGE_DATA_JPEG, JPEG};
    use axiomatik_web::{database};
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_account_page() {
        script_base::setup_before_tests_once().await;

        // 1. Create user
        let cookie = script_base::setup_user_and_login("user8").await;

        // 3. Access account page
        let response_accout = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response_accout.status(), StatusCode::OK);
        let body_account = response_to_body(response_accout).await;
        assert!(body_account.contains("user8"));
        assert!(body_account.contains("Moje články"));

        // 4. Update author name
        let update_params = [("author_name", "Updated Author")];
        let response_update_author = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/account/update-author")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie)
                .body(Body::from(serialize(&update_params)))
                .unwrap(),
        )
        .await;

        assert_eq!(response_update_author.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response_update_author
                .headers()
                .get(header::LOCATION)
                .unwrap(),
            "/account"
        );

        // 5. Verify update in DB
        let user = database::get_user("user8").await.unwrap();
        assert_eq!(user.author_name, "Updated Author");

        // 6. Create an article for this user
        let body = ArticleBuilder::new()
            .title("Test User Article")
            .author("Updated Author")
            .category("zahranici")
            .text("Content")
            .short_text("Short")
            .image("test.jpg", FAKE_IMAGE_DATA_JPEG, JPEG)
            .image_description("test image description")
            .related_articles("related-test-article.html")
            .build();

        let response_create_article = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body.unwrap()))
                .unwrap(),
        )
        .await;

        assert_eq!(response_create_article.status(), StatusCode::SEE_OTHER);

        // 7. Verify the article is on the account page
        let response_accout_2 = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response_accout_2.status(), StatusCode::OK);

        let body = response_to_body(response_accout_2).await;
        assert!(body.contains("Test User Article"));

        // 8. Update author name again
        let update_params = [("author_name", "Second Update")];
        let response_update_author_2 = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/account/update-author")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie)
                .body(Body::from(serialize(&update_params)))
                .unwrap(),
        )
        .await;

        assert_eq!(response_update_author_2.status(), StatusCode::SEE_OTHER);

        // 9. Verify the article is STILL on the account page (linked by username, not author_name)
        let response_accout_3 = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response_accout_3.status(), StatusCode::OK);

        let body = response_to_body(response_accout_3).await;
        assert!(body.contains("Test User Article"));
        assert!(body.contains("Second Update"));

        // Cleanup files
        let _ = std::fs::remove_file("test-user-article.html");
        let _ = std::fs::remove_file("snippets/test-user-article.html.txt");
    }
}
