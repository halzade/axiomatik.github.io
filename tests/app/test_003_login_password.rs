#[cfg(test)]
mod tests {
    use axiomatik_web::trust::utils;
    use axiomatik_web::trust::utils::{serialize, TrustError};
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_validation_login_password() -> Result<(), TrustError> {
        utils::setup_before_tests_once().await;

        // DEL
        let login_params = [("username", "admin"), ("password", "passw\x7ford123")];
        let response = utils::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&login_params)))?,
        )
        .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }
}
