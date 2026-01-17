#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_validation_login_username() {
        let (app, db) = setup_app().await;

        // Create user with clean name
        auth::create_editor_user("admin", "password123")
            .await
            .unwrap();

        // BEL
        let login_params = [("username", "adm\x07in"), ("password", "password123")];
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(Body::from(serialize(&login_params)))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_validation_login_password() {
        let (app, db) = setup_app().await;

        // Create user with clean name
        auth::create_editor_user("admin", "password123")
            .await
            .unwrap();

        // DEL
        let login_params = [("username", "admin"), ("password", "passw\x7ford123")];
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(Body::from(serialize(&login_params)))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_validation_create_article() {
        let (app, db) = setup_app().await;

        // 1. Create and login user
        auth::create_editor_user("author1", "pass123")
            .await
            .unwrap();

        // Manual update to bypass password change
        let mut user = database::get_user("author1").await.unwrap().unwrap();
        user.needs_password_change = false;
        database::update_user(user).await.unwrap();

        let login_params = [("username", "author1"), ("password", "pass123")];
        let login_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(Body::from(serialize(&login_params)))
                    .unwrap(),
            )
            .await
            .unwrap();
        let cookie = login_resp
            .headers()
            .get(header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // 2. Create an article with malicious input
        let boundary = "---------------------------123456789012345678901234567";
        let body = format!(
            "--{0}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        Title\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"author\"\r\n\r\n\
        Author\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"category\"\r\n\r\n\
        republika\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"text\"\r\n\r\n\
        Content\r\n\
        --{0}\r\n\
        Content-Disposition: form-data; name=\"short_text\"\r\n\r\n\
        Sho\x07rt\r\n\
        --{0}--\r\n",
            boundary
        );

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/create")
                    .header(
                        header::CONTENT_TYPE,
                        format!("multipart/form-data; boundary={}", boundary),
                    )
                    .header(header::COOKIE, &cookie)
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
