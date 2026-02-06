#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_create_article() -> Result<(), TrustError> {
        let server = trust::me::server()?;
        let app = server.nexo_app()?;
        let web = server.nexo_web()?;
        let db = server.surreal()?;

        db.setup_user("user6").await?;
        app.post_login("user6").await?;

        #[rustfmt::skip]
        app.post_create_article()
            .title("Test Article")
            .author("Test Author")
            .category("republika")
            .text("This is a test article text.")
            .short_text("Short text.")
            .image_any_png()
            .execute()?
            .must_see_response(StatusCode::SEE_OTHER)
                .headers_location("test-article.html")
                .verify();

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
