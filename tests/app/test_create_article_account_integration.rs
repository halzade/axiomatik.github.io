#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;
    use std::fs::remove_file;

    #[tokio::test]
    async fn test_account_page() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // Create user and login
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user8")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        ac.login()
            .username("user8")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        let cookie = ac.login().get_cookie().unwrap();

        // Access account page
        // let response_account = utils::one_shot(
        //     Request::builder()
        //         .method("GET")
        //         .uri("/account")
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::default())?,
        // )
        // .await;
        // 
        // assert_eq!(response_account.status(), StatusCode::OK);
        // let body_account = response_to_body(response_account).await;
        // assert!(body_account.contains("user8"));
        // assert!(body_account.contains("Moje články"));
        // 
        // // Update author name
        // let update_params = [("author_name", "Updated Author")];
        // let response_update_author = utils::one_shot(
        //     Request::builder()
        //         .method("POST")
        //         .uri("/account/update-author")
        //         .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::from(serialize(&update_params)))?,
        // )
        // .await;
        // 
        // assert_eq!(response_update_author.status(), StatusCode::SEE_OTHER);
        // assert_eq!(response_update_author.headers().get(header::LOCATION).unwrap(), "/account");
        // 
        // // Verify update in DB
        // let user = database_user::get_user_by_name("user8").await?.unwrap();
        // assert_eq!(user.author_name, "Updated Author");
        // 
        // // Create an article with this user
        // let image_data = utils::get_test_image_data();
        // let body = ArticleBuilder::new()
        //     .title("Test User Article")
        //     .author("Updated Author")
        //     .category("zahranici")
        //     .text("Content")
        //     .short_text("Short")
        //     .image("test.png", &image_data, PNG)
        //     .image_desc("test image description")
        //     .related_articles("related-test-article.html")
        //     .build();
        // 
        // let response_create_article = utils::one_shot(
        //     Request::builder()
        //         .method("POST")
        //         .uri("/create")
        //         .header(header::CONTENT_TYPE, content_type_with_boundary())
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::from(body?))?,
        // )
        // .await;
        // 
        // assert_eq!(response_create_article.status(), StatusCode::SEE_OTHER);
        // 
        // // 7. Verify the article is on the account page
        // let response_account_2 = utils::one_shot(
        //     Request::builder()
        //         .method("GET")
        //         .uri("/account")
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::default())?,
        // )
        // .await;
        // 
        // assert_eq!(response_account_2.status(), StatusCode::OK);
        // 
        // let body = response_to_body(response_account_2).await;
        // assert!(body.contains("Test User Article"));
        // 
        // // Update author name again
        // let update_params = [("author_name", "Second Update")];
        // let response_update_author_2 = utils::one_shot(
        //     Request::builder()
        //         .method("POST")
        //         .uri("/account/update-author")
        //         .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::from(serialize(&update_params)))?,
        // )
        // .await;
        // 
        // assert_eq!(response_update_author_2.status(), StatusCode::SEE_OTHER);
        // 
        // // Verify the article is STILL on the account page (linked by username, not author_name)
        // let response_account_3 = utils::one_shot(
        //     Request::builder()
        //         .method("GET")
        //         .uri("/account")
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::default())?,
        // )
        // .await;
        // 
        // assert_eq!(response_account_3.status(), StatusCode::OK);
        // 
        // let body = response_to_body(response_account_3).await;
        // assert!(body.contains("Test User Article"));
        // assert!(body.contains("Second Update"));
        // 
        // // cleanup DB
        // assert!(database_user::delete_user("user8").await.is_ok());

        // cleanup files
        assert!(remove_file("web/test-user-article.html").is_ok());
        assert!(remove_file("web/u/test-user-article_image_820.png").is_ok());
        assert!(remove_file("web/u/test-user-article_image_50.png").is_ok());
        assert!(remove_file("web/u/test-user-article_image_288.png").is_ok());
        assert!(remove_file("web/u/test-user-article_image_440.png").is_ok());

        Ok(())
    }
}
