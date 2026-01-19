#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::serialize;
    use axum::extract::Request;
    use http::{header, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_malicious_keyword_rejection() {
        script_base::setup_before_tests_once().await;
        
        let malicious_payload = [
            ("username", "admin; drOp daTaBasE user;"),
            ("password", "password"),
        ];

        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&malicious_payload)))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
