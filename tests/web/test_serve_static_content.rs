#[cfg(test)]
mod tests {
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::TrustError;
    use axum::http::{Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_serve_static_content() -> Result<(), TrustError> {
        script_base::setup_before_tests_once().await;

        let response = script_base::one_shot(
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
