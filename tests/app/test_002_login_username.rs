#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_validation_login_username() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // BEL
        #[rustfmt::skip]
        ac.login()
            .username("adm\x07in")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::BAD_REQUEST)
            .verify().await?;

        Ok(())
    }
}
