#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_validation_create_article() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // 1. Create and login user
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user9")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user9")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        // 2. Create an article with malicious input
        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Title")
            .author("user9")
            .category("republika")
            .text("Content")
            .short_text("Sho\x07rt")
            .image_any_png()?
            .image_desc("test description")
            .execute().await?
                .must_see_response(StatusCode::BAD_REQUEST)
                .verify().await?;

        Ok(())
    }
}
