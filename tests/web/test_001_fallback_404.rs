#[cfg(test)]
mod tests {
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::TrustError;
    use axum::http::{Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_404_fallback() -> Result<(), TrustError> {
        script_base::setup_before_tests_once().await;

        let response = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/non-existent-page.html")
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8_lossy(&body);

        assert_eq!(body_str, "404, stránka nenalezená");

        Ok(())
    }
}
