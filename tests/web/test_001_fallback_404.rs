#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_404_fallback() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.web().get_url("/non-existent-page.html").await?
            .must_see_response(StatusCode::NOT_FOUND)
            .body("404; stránka nenalezena")
            .verify().await?;

        Ok(())
    }
}
