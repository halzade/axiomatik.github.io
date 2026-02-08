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

        info!("create user ok");

        info!("login");

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user8")
            .password("password123")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        info!("login ok");

        info!("........access account page");

        // can access the account page
        #[rustfmt::skip]
        ac.account().get(&auth).await?
            .must_see_response(StatusCode::OK)
            .verify().await?;

        info!("........access account page ok");

        // Update author name
        #[rustfmt::skip]
        ac.account().update_author_name(&auth, "Updated Author").await?
            .must_see_response(StatusCode::SEE_OTHER)
            .header_location("/account")
            .verify().await?;

        // request account with updated author name
        #[rustfmt::skip]
        ac.account().get(&auth).await?
            .must_see_response(StatusCode::OK)
            .body_contains("user8")
            .body_contains("Moje články")
            .body_contains("Updated Author")
            .verify().await?;

        // verify update in DB
        #[rustfmt::skip]
        ac.db_user().must_see("user8").await?
            .author_name("Updated Author")
            .verify()?;

        Ok(())
    }
}
