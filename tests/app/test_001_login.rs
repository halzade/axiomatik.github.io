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
        ac.db_user().db_setup_user_with_password("admin1", "password123").await?;

        // Try login
        ac.login().post_login_with_password("admin1", "password123").await?
            .must_see_response(StatusCode::SEE_OTHER)
            .header_location("/change-password")
            .header_cookie(&["HttpOnly", "Secure", "SameSite=Strict", "Path=/"])
            .verify();

        Ok(())
    }
}
