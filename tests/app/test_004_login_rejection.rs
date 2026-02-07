#[cfg(test)]
mod tests {
    use axiomatik_web::trust::utils;
    use axiomatik_web::trust::utils::{serialize, TrustError};
    use axum::extract::Request;
    use http::{header, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_sql_injection_rejection_in_login() -> Result<(), TrustError> {
        utils::setup_before_tests_once().await;

        let injection_payload = [("username", "admin' OR '1'='1"), ("password", "anything")];

        let response = utils::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&injection_payload)))?,
        )
        .await;

        assert_ne!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }
}
