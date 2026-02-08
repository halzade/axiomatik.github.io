#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;
    use axiomatik_web::trust::app_controller::AppController;

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
        ac.login()
            .username("user6")
            .password("password")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        #[rustfmt::skip]
        ac.create_article()
            .title("Test Article")
            .author("Test Author")
            .category("republika")
            .text("This is a test article text.")
            .short_text("Short text.")
            .image_any_png()?
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("test-article.html")
                .verify()?;

        trust::me::path_exists("web/test-article.html");

        // Request the article
        #[rustfmt::skip]
        web.get_url("/test-article.html").await?
            .must_see_response(StatusCode::OK)
            .verify();

        // Cleanup
        trust::me::remove_file("web/test-article.html")?;
        trust::me::remove_file("web/u/test-article_image_50.jpg")?;
        trust::me::remove_file("web/u/test-article_image_288.jpg")?;
        trust::me::remove_file("web/u/test-article_image_440.jpg")?;
        trust::me::remove_file("web/u/test-article_image_820.jpg")?;
        trust::me::remove_file("web/u/test-article_audio.mp3")?;
        Ok(())
    }
}
