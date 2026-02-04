#[cfg(test)]
mod tests {
    use axiomatik_web::application::login::form_login;
    use axiomatik_web::db::database_user::delete_user;
    use axiomatik_web::db::database_user::Role::Editor;
    use axiomatik_web::system::commands::create_editor_user;
    use axiomatik_web::trust::script_base;
    use axiomatik_web::trust::script_base::TrustError;

    #[tokio::test]
    async fn test_create_editor_user() -> Result<(), TrustError> {
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
        assert_eq!(user.role, Editor);
        assert!(user.needs_password_change);

        // clean up
        assert!(delete_user("user11").await.is_ok());

        Ok(())
    }
}
