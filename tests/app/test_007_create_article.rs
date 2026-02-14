#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_create_article() -> Result<(), TrustError> {
        // setup
        let ac = AppController::new().await?;

        // create user and login
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user6")
            .password("password")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user6")
            .password("password")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test Article")
            .author("Test Author")
            .category("republika")
            .text("This is a test article text.\n\nMore text.")
            .short_text("Short text.")
            .image_any_png()?
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/account")
                .verify().await?;

        // article isn't rendered yet

        // Request the article
        #[rustfmt::skip]
        ac.web().get_url("/test-article.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Article")
            .body_contains("This is a test article text.")
            .verify().await?;
        // article was rendered and served

        trust::me::path_exists("web/test-article.html")?;

        // Cleanup
        trust::me::remove_file("web/test-article.html")?;
        trust::me::remove_file("web/u/test-article_image_50.png")?;
        trust::me::remove_file("web/u/test-article_image_288.png")?;
        trust::me::remove_file("web/u/test-article_image_440.png")?;
        trust::me::remove_file("web/u/test-article_image_820.png")?;
        Ok(())
    }
}
