#[cfg(test)]
mod tests {
    use axiomatik_web::db::database_user;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::{response_to_body, serialize, TrustError};
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_account_page() -> Result<(), TrustError> {
        script_base::setup_before_tests_once().await;

        // Create user
        let cookie = script_base::setup_user_and_login("user8").await;

        // Access account page
        let response_account = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response_account.status(), StatusCode::OK);
        let body_account = response_to_body(response_account).await;
        assert!(body_account.contains("user8"));
        assert!(body_account.contains("Moje články"));

        // Update author name
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
        let body_account_updated = response_to_body(response_update_author).await;
        assert!(body_account_updated.contains("user8"));
        assert!(body_account_updated.contains("Moje články"));
        // Verify the updated author
        assert!(body_account_updated.contains("Updated Author"));

        // Verify update in DB
        let user = database_user::get_user_by_name("user8").await?.unwrap();
        assert_eq!(user.author_name, "Updated Author");

        // cleanup DB
        assert!(database_user::delete_user("user8").await.is_ok());

        Ok(())
    }
}
