#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::{serialize};
    use axiomatik_web::{commands};
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_login() {
        script_base::setup_before_tests_once().await;

        // 1. Create a user via auth module (simulating command)
        commands::create_editor_user("admin1", "password123")
            .await
            .unwrap();

        // 2. Try login
        let login_params = [("username", "admin1"), ("password", "password123")];
        let login_body = serialize(&login_params);
        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(login_body))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response.headers().get(header::LOCATION).unwrap(),
            "/change-password"
        );
        assert!(response.headers().get(header::SET_COOKIE).is_some());
        let cookie_header = response.headers().get(header::SET_COOKIE).unwrap().to_str().unwrap();
        assert!(cookie_header.contains("HttpOnly"));
        assert!(cookie_header.contains("Secure"));
        assert!(cookie_header.contains("SameSite=Strict"));
        assert!(cookie_header.contains("Path=/"));
    }
}
