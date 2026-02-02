#[cfg(test)]
mod tests {
    use axiomatik_web::trust::script_base;
    use axum::http::{Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_serve_static() {
        script_base::setup_before_tests_once().await;

        let response = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/favicon.ico")
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}
