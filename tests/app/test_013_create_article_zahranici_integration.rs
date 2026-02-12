#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_zahranici_article_creation_and_limit() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user7")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user7")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test Newest Zahraňičí")
            .author("Author")
            .category("zahranici")
            .text("Main text")
            .short_text("Short text of newest article")
            .image_any_png()?
            .image_desc("test description")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        // index
        #[rustfmt::skip]
        ac.web().get_url("/index.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Newest Zahraňičí")
            .verify().await?;

        // article
        #[rustfmt::skip]
        ac.web().get_url("/test-newest-zahranici.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Newest Zahraňičí")
            .verify().await?;

        // category
        #[rustfmt::skip]
        ac.web().get_url("/zahranici.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Newest Zahraňičí")
            .verify().await?;

        // Cleanup
        trust::me::remove_file("web/test-newest-zahranici.html")?;
        trust::me::remove_file("web/u/test-newest-zahranici_image_820.png")?;
        trust::me::remove_file("web/u/test-newest-zahranici_image_50.png")?;
        trust::me::remove_file("web/u/test-newest-zahranici_image_288.png")?;
        trust::me::remove_file("web/u/test-newest-zahranici_image_440.png")?;

        Ok(())
    }
}
