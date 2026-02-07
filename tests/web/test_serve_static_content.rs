#[cfg(test)]
mod tests {
    use axiomatik_web::trust::utils;
    use axiomatik_web::trust::utils::TrustError;
    use axum::http::{Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_serve_static_content() -> Result<(), TrustError> {
        utils::setup_before_tests_once().await;

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
