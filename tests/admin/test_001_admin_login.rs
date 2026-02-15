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
            .execute().await?;

        // verify admin user in DB
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

        ac.web_app(&auth).get_url("/admin_user").await?
            .must_see_response(StatusCode::OK)
            .verify().await?;

        ac.web_app(&auth).get_url("/admin_article").await?
            .must_see_response(StatusCode::OK)
            .verify().await?;

        Ok(())
    }
}
