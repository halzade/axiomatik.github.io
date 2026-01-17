#[cfg(test)]
mod tests {
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;
    use axiomatik_web::{commands, database};
    use axiomatik_web::test_framework::article_builder::{ArticleBuilder, BOUNDARY};
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::serialize;

    #[tokio::test]
    async fn test_validation_login_username() {
        // Create user with clean name
        commands::create_editor_user("admin", "password123")
            .await
            .unwrap();

        // BEL
        let login_params = [("username", "adm\x07in"), ("password", "password123")];
        let response = script_base::one_shot(Request::builder()
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
        // Create user with clean name
        commands::create_editor_user("admin", "password123")
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
        // 1. Create and login user
        commands::create_editor_user("author1", "pass123")
            .await
            .unwrap();

        // Manual update to bypass password change
        let mut user = database::get_user("author1").await.unwrap();
        user.needs_password_change = false;
        database::update_user(user).await.unwrap();

        let login_params = [("username", "author1"), ("password", "pass123")];
        let response = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&login_params)))
                .unwrap(),
        )
        .await;

        let cookie = response
            .headers()
            .get(header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

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
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", BOUNDARY),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
