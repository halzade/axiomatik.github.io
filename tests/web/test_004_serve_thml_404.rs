#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_serve_html_404() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
          ac.web().get_url("/test-no-user-article.html").await?
            .must_see_response(StatusCode::OK)
            .verify().await?;

        Ok(())
    }
}
