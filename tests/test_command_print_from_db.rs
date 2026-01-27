#[cfg(test)]
mod tests {
    use tracing::info;
    use axiomatik_web::test_framework::script_base;
    use axiomatik_web::commands::create_editor_user;
    use axiomatik_web::database_tools;

    #[tokio::test]
    async fn test_print_from_db() {
        info!("Executing test_print_from_db");
        script_base::setup_before_tests_once().await;
        info!("Executing test_print_from_db next");
        
        // Create a user to have something to query
        let username = "user10";
        let password = "testpassword";
        create_editor_user(username, password).await.unwrap();

        // Execute print_from_db
        let result = database_tools::print_from_db("SELECT username FROM user").await;
        assert!(result.is_ok(), "print_from_db failed: {:?}", result.err());
    }
}
