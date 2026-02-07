#[cfg(test)]
mod tests {
    use axum::http::{Request, StatusCode};
    use reqwest::Body;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_serve_static_content() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        let response = utils::one_shot(
            Request::builder()
                .method("GET")
                .uri("/favicon.ico")
                .body(Body::default())?,
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }
}
