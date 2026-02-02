#[cfg(test)]
mod tests {
    use axiomatik_web::application::login::form_login;
    use axiomatik_web::db::database_user::Role::Editor;
    use axiomatik_web::system::commands::create_editor_user;
    use axiomatik_web::trust::script_base;

    #[tokio::test]
    async fn test_create_editor_user() {
        script_base::setup_before_tests_once().await;

        let username = "user11";
        let password = "password123";

        // Create editor user
        let result = create_editor_user(username, password).await;
        assert!(result.is_ok());

        // Verify user exists and can authenticate
        let auth_result = form_login::authenticate_user(username, password).await;
        assert!(auth_result.is_ok());

        let user = auth_result.unwrap();
        assert_eq!(user.username, username);
        assert!(user.needs_password_change);
        assert_eq!(user.role, Editor);
    }
}
