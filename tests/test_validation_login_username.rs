#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::{serialize};
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_validation_login_username() {
        script_base::setup_before_tests_once().await;

        // BEL
        let login_params = [("username", "adm\x07in"), ("password", "password123")];
        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&login_params)))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
