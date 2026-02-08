#[cfg(test)]
mod tests {
    use axiomatik_web::db::database_user;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::data::utils::{response_to_body, serialize};
    use axiomatik_web::trust::me::TrustError;
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_account_page() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // Create user
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
        let response_account = utils::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())?,
        )
        .await;

        assert_eq!(response_account.status(), StatusCode::OK);
        let body_account = response_to_body(response_account).await;
        assert!(body_account.contains("user8"));
        assert!(body_account.contains("Moje články"));

        // Update author name
        let update_params = [("author_name", "Updated Author")];
        let response_update_author = utils::one_shot(
            Request::builder()
                .method("POST")
                .uri("/account/update-author")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie)
                .body(Body::from(serialize(&update_params)))?,
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
        #[rustfmt::skip]
        ac.db_user().must_see("user8").await?
            .author_name("Updated Author")
            .verify()?;

        Ok(())
    }
}
