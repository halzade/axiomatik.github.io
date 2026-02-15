#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use http::StatusCode;

    #[tokio::test]
    async fn test_001_create_and_delete_user() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // create admin user
        #[rustfmt::skip]
        ac.db_user().setup_admin_user()
            .username("admin1")
            .password("strong*admin*password")
            .needs_password_change(true)
            .execute().await?;

        // Login as admin
        #[rustfmt::skip]
        let auth = ac.login()
            .username("admin1")
            .password("strong*admin*password")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/TODO")
                .verify().await?;

        // Create an Editor user
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user_a1")
            .password("password")
            .needs_password_change(true)
            .execute().await?;

        // Verify user in DB
        #[rustfmt::skip]
        ac.db_user().must_see("admin1").await?
            .username("admin1")
            .needs_password_change(false)
            .verify()?;

        // TODO admin delete user

        // Verify user in DB
        #[rustfmt::skip]
        ac.db_user().must_not_see("user_a1").await?
            .verify()?;

        Ok(())
    }
}
