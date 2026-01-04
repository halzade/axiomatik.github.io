#[cfg(test)]
mod tests {
    use axiomatik_web::auth::{authenticate_user, create_admin_user};
    use axiomatik_web::db::init_mem_db;

    #[tokio::test]
    async fn test_create_admin_user() {
        let db = init_mem_db().await.expect("Failed to init mem db");

        let username = "admin";
        let password = "password123";

        // Create admin user
        let result = create_admin_user(&db, username, password).await;
        assert!(
            result.is_ok(),
            "Admin user creation failed: {:?}",
            result.err()
        );

        // Verify user exists and can authenticate
        let auth_result = authenticate_user(&db, username, password).await;
        assert!(
            auth_result.is_ok(),
            "Authentication failed for created admin: {:?}",
            auth_result.err()
        );

        let user = auth_result.unwrap();
        assert_eq!(user.username, username);
        assert!(!user.needs_password_change);
    }
}
