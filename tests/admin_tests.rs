#[cfg(test)]
mod tests {
    use axiomatik_web::auth::{authenticate_user, create_editor_user};
    use axiomatik_web::db::{init_mem_db, Role};

    #[tokio::test]
    async fn test_create_editor_user() {
        let db = init_mem_db().await.expect("Failed to init mem db");

        let username = "editor";
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
        assert!(!user.needs_password_change);
        assert_eq!(user.role, Role::Editor);
    }
}
