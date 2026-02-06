#[cfg(test)]
mod tests {
    use axiomatik_web::system::commands;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::{serialize, TrustError};
    use axum::http::{header, Request, StatusCode};
    use header::{CONTENT_TYPE, LOCATION, SET_COOKIE};
    use reqwest::Body;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_login() -> Result<(), TrustError> {
        script_base::setup_before_tests_once().await;

        // 1. Create a user via auth module (simulating command)
        commands::create_editor_user("admin1", "password123").await?;

        // 2. Try login
        let login_params = [("username", "admin1"), ("password", "password123")];
        let login_body = serialize(&login_params);
        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(login_body))?,
        )
        .await;

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(LOCATION).unwrap(),
            "/change-password"
        );
        assert!(response.headers().get(SET_COOKIE).is_some());
        let cookie_header = response
            .headers()
            .get(SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap();
        assert!(cookie_header.contains("HttpOnly"));
        assert!(cookie_header.contains("Secure"));
        assert!(cookie_header.contains("SameSite=Strict"));
        assert!(cookie_header.contains("Path=/"));
        Ok(())
    }
}
