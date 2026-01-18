#[cfg(test)]
mod tests {
    use axiomatik_web::test_framework::article_builder::{ArticleBuilder, BOUNDARY};
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::test_framework::script_base::{serialize};
    use axiomatik_web::{commands, database};
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;
    use axiomatik_web::test_framework::script_base_data::{FAKE_IMAGE_DATA_JPEG, JPEG};

    #[tokio::test]
    async fn test_login() {
        script_base::setup_before_tests_once().await;

        // 1. Create a user via auth module (simulating command)
        commands::create_editor_user("admin", "password123")
            .await
            .unwrap();

        // 2. Try login
        let login_params = [("username", "admin"), ("password", "password123")];
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
    }

    #[tokio::test]
    async fn test_change_password() {
        script_base::setup_before_tests_once().await;

        // Create user who needs password change
        let password_hash = bcrypt::hash("pass1234", bcrypt::DEFAULT_COST).unwrap();
        database::create_user(database::User {
            username: "user1".to_string(),
            author_name: "user1".to_string(),
            password_hash,
            needs_password_change: true,
            role: database::Role::Editor,
        })
        .await
        .unwrap();

        // Login as user1
        let login_params1 = [("username", "user1"), ("password", "pass1234")];
        let login_resp1 = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(serialize(&login_params1)))
                .unwrap(),
        )
        .await;

        // Should redirect to change-password
        assert_eq!(login_resp1.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            login_resp1.headers().get(header::LOCATION).unwrap(),
            "/change-password"
        );
        let cookie1 = login_resp1
            .headers()
            .get(header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // Change password
        let change_params = [("new_password", "new_password_123")];
        let change_resp = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/change-password")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie1)
                .body(Body::from(serialize(&change_params)))
                .unwrap(),
        )
        .await;

        assert_eq!(change_resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            change_resp.headers().get(header::LOCATION).unwrap(),
            "/account"
        );

        // Verify change in DB
        let user = database::get_user("user1").await.unwrap();
        assert_eq!(user.author_name, "user1");
        assert!(!user.needs_password_change);
    }

    #[tokio::test]
    async fn test_serve_static_html() {
        script_base::setup_before_tests_once().await;

        let response = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/jeden-tisic-dnu.html")
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_404_fallback() {
        script_base::setup_before_tests_once().await;

        let response = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/non-existent-page.html")
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8_lossy(&body);

        let expected_404_content = std::fs::read_to_string("404.html").unwrap();
        assert_eq!(true, expected_404_content.len() > 200);
        assert_eq!(body_str, expected_404_content);
    }

    #[tokio::test]
    async fn test_404_fallback_curl() {

        // TODO start only if not running
        // let (app, _db) = setup_app().await;
        // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
        // tokio::spawn(async move {
        //     axum::serve(listener, app).await.unwrap();
        // });

        let output = std::process::Command::new("curl")
            .arg("-s")
            .arg("http://127.0.0.1:3000/non-existent-page.html")
            .output()
            .expect("Failed to execute curl");

        let body_str = String::from_utf8_lossy(&output.stdout);
        let expected_404_content = std::fs::read_to_string("404.html").unwrap();
        assert_eq!(body_str.trim(), expected_404_content.trim());
    }

    #[tokio::test]
    async fn test_account_page() {
        script_base::setup_before_tests_once().await;

        // 1. Create user
        let cookie = script_base::setup_user_and_login("user8").await;

        // 3. Access account page
        let response = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("user8"));
        assert!(body_str.contains("Initial Author"));
        assert!(body_str.contains("Moje články"));

        // 4. Update author name
        let update_params = [("author_name", "Updated Author")];
        let update_resp = script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/account/update-author")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie)
                .body(Body::from(serialize(&update_params)))
                .unwrap(),
        )
        .await;

        assert_eq!(update_resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            update_resp.headers().get(header::LOCATION).unwrap(),
            "/account"
        );

        // 5. Verify update in DB
        let user = database::get_user("user8").await.unwrap();
        assert_eq!(user.author_name, "Updated Author");

        // 6. Create an article for this user
        let body = ArticleBuilder::new()
            .title("test-User Article")
            .author("Updated Author")
            .category("test-category")
            .text("Content")
            .short_text("Short")
            .image("test.jpg", FAKE_IMAGE_DATA_JPEG, JPEG)
            .build();

        script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={}", BOUNDARY),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body.unwrap()))
                .unwrap(),
        )
        .await;

        // 7. Verify the article is on the account page
        let response = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("test-User Article"));

        // 8. Update author name again
        let update_params = [("author_name", "Second Update")];
        script_base::one_shot(
            Request::builder()
                .method("POST")
                .uri("/account/update-author")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, &cookie)
                .body(Body::from(serialize(&update_params)))
                .unwrap(),
        )
        .await;

        // 9. Verify the article is STILL on the account page (linked by username, not author_name)
        let response = script_base::one_shot(
            Request::builder()
                .method("GET")
                .uri("/account")
                .header(header::COOKIE, &cookie)
                .body(Body::default())
                .unwrap(),
        )
        .await;

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8_lossy(&body);
        assert!(body_str.contains("test-User Article"));
        assert!(body_str.contains("Second Update"));

        // Cleanup files
        let _ = std::fs::remove_file("test-user-article.html");
        let _ = std::fs::remove_file("snippets/test-user-article.html.txt");
    }
}
