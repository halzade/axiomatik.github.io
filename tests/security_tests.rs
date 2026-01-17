#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::serialize;
    use axum::extract::Request;
    use http::{header, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_sql_injection_rejection_in_login() {
        let injection_payload = [("username", "admin' OR '1'='1"), ("password", "anything")];

        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&injection_payload)))
                .unwrap(),
        )
        .await;

        // TODO Should not redirect (successful login)
        assert_ne!(response.status(), StatusCode::SEE_OTHER);
    }

    #[tokio::test]
    async fn test_malicious_keyword_rejection() {
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
