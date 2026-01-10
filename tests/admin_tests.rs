#[cfg(test)]
mod tests {
    use axiomatik_web::auth::{authenticate_user, create_editor_user};
    use axiomatik_web::db::{init_mem_db, Role};
    use axiomatik_web::db_tool::print_from_db;

    #[tokio::test]
    async fn test_print_from_db() {
        let db = init_mem_db().await;
        
        // Create a user to have something to query
        let username = "testuser";
        let password = "testpassword";
        create_editor_user(&db, username, password).await.unwrap();

        // Execute print_from_db
        let result = print_from_db(&db, "SELECT username FROM user").await;
        assert!(result.is_ok(), "print_from_db failed: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_create_editor_user() {
        let db = init_mem_db().await;

        let username = "editor1";
        let password = "password123";

        // Create editor user
        let result = create_editor_user(&db, username, password).await;
        assert!(
            result.is_ok(),
            "Editor user creation failed: {:?}",
            result.err()
        );

        // Verify user exists and can authenticate
        let auth_result = authenticate_user(&db, username, password).await;
        assert!(
            auth_result.is_ok(),
            "Authentication failed for created editor: {:?}",
            auth_result.err()
        );

        let user = auth_result.unwrap();
        assert_eq!(user.username, username);
        assert!(user.needs_password_change);
        assert_eq!(user.role, Role::Editor);
    }
}
