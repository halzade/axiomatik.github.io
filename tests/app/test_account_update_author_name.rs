#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;
    use tracing::info;

    #[tokio::test]
    async fn test_account_page() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        info!("create user");

        // Create user
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user8")
            .password("password123")
            .execute().await?;

        info!("login");

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user8")
            .password("password123")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        info!(auth);
        info!("........access account page");

        // can access the account page
        #[rustfmt::skip]
        ac.account().get(auth).await?
            .must_see_response(StatusCode::OK)
            .verify().await?;
        info!("........access account page done");

        // assert!(body_account.contains("user8"));
        // assert!(body_account.contains("Moje články"));
        
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

        // assert_eq!(response_update_author.status(), StatusCode::SEE_OTHER);
        // assert_eq!(
        //     response_update_author
        //         .headers()
        //         .get(header::LOCATION)
        //         .unwrap(),
        //     "/account"
        // );
        // let body_account_updated = response_to_body(response_update_author).await;
        // assert!(body_account_updated.contains("user8"));
        // assert!(body_account_updated.contains("Moje články"));
        // // Verify the updated author
        // assert!(body_account_updated.contains("Updated Author"));

        // Verify update in DB
        #[rustfmt::skip]
        ac.db_user().must_see("user8").await?
            .author_name("Updated Author")
            .verify()?;

        Ok(())
    }
}
