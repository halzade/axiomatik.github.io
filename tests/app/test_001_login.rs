#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_login() -> Result<(), TrustError> {
        // setup
        let server = trust::me::server().await?;
        let app = server.nexo_app()?;
        let surreal = server.surreal()?;

        // create user
        surreal.db_setup_user_with_password("admin1", "password123").await?;

        // Try login
        app.post_login_with_password("admin1", "password123")
            .await?
            .must_see_response(StatusCode::SEE_OTHER)
            .header_location("/change-password")
            .header_cookie(&["HttpOnly", "Secure", "SameSite=Strict", "Path=/"])
            .verify();

        Ok(())
    }
}
