#[cfg(test)]
mod tests {
    use axiomatik_web::db::database_user;
    use axiomatik_web::db::database_user::Role::Editor;
    use axiomatik_web::db::database_user::User;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::test_framework::script_base::serialize;
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;

    #[tokio::test]
    async fn test_change_password() {
        script_base::setup_before_tests_once().await;

        // Create user who needs password change
        let password_hash = bcrypt::hash("pass1234", bcrypt::DEFAULT_COST).unwrap();
        database_user::create_user(User {
            username: "user1".to_string(),
            author_name: "user1".to_string(),
            password_hash,
            needs_password_change: true,
            role: Editor,
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
        let user = database_user::get_user("user1").await.unwrap();
        assert_eq!(user.author_name, "user1");
        assert!(!user.needs_password_change);
    }
}
