#[cfg(test)]
mod tests {
    use axiomatik_web::commands;
    use axiomatik_web::test_framework::article_builder::{ArticleBuilder, BOUNDARY};
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::{boundary, serialize};
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_validation_login_username() {
        script_base::setup_before_tests_once().await;

        // Create user with clean name
        commands::create_editor_user("admin2", "password123")
            .await
            .unwrap();

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

    #[tokio::test]
    async fn test_validation_login_password() {
        script_base::setup_before_tests_once().await;

        // Create user with clean name
        commands::create_editor_user("admin3", "password123")
            .await
            .unwrap();

        // DEL
        let login_params = [("username", "admin"), ("password", "passw\x7ford123")];
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

    #[tokio::test]
    async fn test_validation_create_article() {
        script_base::setup_before_tests_once().await;

        // 1. Create and login user
        let cookie = script_base::setup_user_and_login("user9").await;

        // 2. Create an article with malicious input
        let body = ArticleBuilder::new()
            .title("Title")
            .author("Author")
            .category("republika")
            .text("Content")
            .short_text("Sho\x07rt")
            .build();

        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(header::CONTENT_TYPE, boundary())
                .header(header::COOKIE, &cookie)
                .body(Body::from(body.unwrap()))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
