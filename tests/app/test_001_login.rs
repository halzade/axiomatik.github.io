#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_login() -> Result<(), TrustError> {
        // setup
        let ac = AppController::new().await?;

        // create user
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("admin1")
            .password("password123")
            .execute()
            .await?;

        //  login
        #[rustfmt::skip]
        ac.login()
            .username("admin1")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/change-password")
                .header_cookie(&["HttpOnly", "Secure", "SameSite=Strict", "Path=/"])
                .verify().await?;

        Ok(())
    }
}
