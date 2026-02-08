#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use http::StatusCode;

    #[tokio::test]
    async fn test_change_password() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // Create user who needs password change
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user1")
            .password("pass1234")
            .needs_password_change(true)
            .execute().await?;

        // Login as user1
        #[rustfmt::skip]
        ac.login()
            .username("user1")
            .password("pass1234")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/change-password")
                .verify().await?;

        #[rustfmt::skip]
        ac.change_password()
            .new_password("new_password_123")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/account")
                .verify().await?;

        // Verify change in DB
        #[rustfmt::skip]
        ac.db_user().must_see("user1").await?
            .username("user1")
            .needs_password_change(false)
            .verify()?;

        Ok(())
    }
}
