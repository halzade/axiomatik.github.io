#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_republika_article_creation_and_limit() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user5")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user5")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test Newest Republika")
            .author("Author")
            .category("republika")
            .text("Main text")
            .short_text("Short text of newest article")
            .image_any_png()?
            .image_desc("test description")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        // article
        #[rustfmt::skip]
        ac.web().get_url("/test-newest-republika.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Newest Republika")
            .verify().await?;

        // index
        #[rustfmt::skip]
        ac.web().get_url("/index.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Newest Republika")
            .verify().await?;

        // category
        #[rustfmt::skip]
        ac.web().get_url("/republika.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Newest Republika")
            .verify().await?;

        // Cleanup
        trust::me::remove_file("web/test-newest-republika.html")?;
        trust::me::remove_file("web/u/test-newest-republika_image_820.jpg")?;
        trust::me::remove_file("web/u/test-newest-republika_image_50.jpg")?;
        trust::me::remove_file("web/u/test-newest-republika_image_288.jpg")?;
        trust::me::remove_file("web/u/test-newest-republika_image_440.jpg")?;

        Ok(())
    }
}
