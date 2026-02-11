#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use http::StatusCode;

    #[tokio::test]
    async fn test_ping_web() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // health for logged-in user
        #[rustfmt::skip]
        ac.web().get_url("/ping").await?
            .must_see_response(StatusCode::OK)
            .body_contains("\"web ping\"")
            .verify().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_ping_app() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // health for logged-in user
        #[rustfmt::skip]
        ac.web_app("any-cookie").get_url("/ping").await?
            .must_see_response(StatusCode::OK)
            .body_contains("\"app ping\"")
            .verify().await?;

        Ok(())
    }
}
