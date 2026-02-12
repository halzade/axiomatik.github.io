#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_related_articles() -> Result<(), TrustError> {
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

        // the first article (related)
        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test Related")
            .author("Author")
            .category("republika")
            .text("Main text")
            .short_text("Short text of newest article")
            .image_any_png()?
            .image_desc("test description")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        #[rustfmt::skip]
        ac.web().get_url("/test-related.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Related")
            .verify().await?;

        // the article "test-any-x.html" is Validated

        // the article
        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test This Article XX")
            .author("Author")
            .category("republika")
            .text("Main text")
            .short_text("Short text of newest article")
            .image_any_png()?
            .image_desc("test description")
            .related_articles("test-related.html")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        // the article "test-related.html" is Invalidated

        // verify related article
        #[rustfmt::skip]
        ac.web().get_url("/test-related.html").await?
            .must_see_response(StatusCode::OK)
            // original article
            .body_contains("Test This Article XX")
            .verify().await?;

        // verify the article
        #[rustfmt::skip]
        ac.web().get_url("/test-this-article-xx.html").await?
            .must_see_response(StatusCode::OK)
            // related article
            .body_contains("Test Related")
            .verify().await?;

        // clean up the article
        trust::me::remove_file("web/test-this-article-xx.html")?;
        trust::me::remove_file("web/u/test-this-article-xx_image_820.png")?;
        trust::me::remove_file("web/u/test-this-article-xx_image_50.png")?;
        trust::me::remove_file("web/u/test-this-article-xx_image_288.png")?;
        trust::me::remove_file("web/u/test-this-article-xx_image_440.png")?;

        // clean up the related article
        trust::me::remove_file("web//test-related.html.html")?;
        trust::me::remove_file("web/u//test-related.html_image_820.png")?;
        trust::me::remove_file("web/u//test-related.html_image_50.png")?;
        trust::me::remove_file("web/u//test-related.html_image_288.png")?;
        trust::me::remove_file("web/u//test-related.html_image_440.png")?;

        Ok(())
    }
}
