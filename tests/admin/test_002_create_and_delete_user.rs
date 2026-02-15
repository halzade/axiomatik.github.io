#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use http::StatusCode;

    #[tokio::test]
    async fn test_create_and_delete_user() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // create admin user
        #[rustfmt::skip]
        ac.db_user().setup_admin_user()
            .username("admin1")
            .password("strong*admin*password")
            .execute().await?;

        // verify admin in DB
        #[rustfmt::skip]
        ac.db_user().must_see("admin1").await?
            .username("admin1")
            .needs_password_change(false)
            .verify()?;

        // login as admin
        #[rustfmt::skip]
        let auth = ac.login()
            .username("admin1")
            .password("strong*admin*password")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/admin_user")
                .verify().await?;

        #[rustfmt::skip]
        ac.admin(&auth).create_user()
            .username("user_a1")
            .password("password")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/admin_user")
                .verify().await?;

        // verify created user
        #[rustfmt::skip]
        ac.db_user().must_see("user_a1").await?
            .username("user_a1")
            .needs_password_change(true)
            .verify()?;

        // delete user
        #[rustfmt::skip]
        ac.admin(&auth).delete_user()
            .username("user_a1")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/admin_user")
                .verify().await?;

        // verify created user
        #[rustfmt::skip]
        ac.db_user().must_not_see("user_a1").await?
            .verify()?;

        Ok(())
    }
}
