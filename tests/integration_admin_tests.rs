#[cfg(test)]
mod tests {
    use axiomatik_web::commands::create_editor_user;
    use axiomatik_web::database::Role::Editor;
    use axiomatik_web::{database_tools, form_login};
    use axiomatik_web::test_framework::script_base;

    #[tokio::test]
    async fn test_print_from_db() {
        script_base::setup_before_tests_once().await;
        
        // Create a user to have something to query
        let username = "testuser";
        let password = "testpassword";
        create_editor_user(username, password).await.unwrap();

        // Execute print_from_db
        let result = database_tools::print_from_db("SELECT username FROM user").await;
        assert!(result.is_ok(), "print_from_db failed: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_create_editor_user() {
        script_base::setup_before_tests_once().await;
        
        let username = "editor1";
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
