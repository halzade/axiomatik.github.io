#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_exclusive_main_article_finance() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user2")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user2")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test Financni Trhy v Šoku")
            .author("Financni Expert")
            .category("finance")
            .text("Dlouhý text o financich")
            .short_text("Krátký text o financich")
            .is_main(true)
            .is_exclusive(true)
            .image_any_png()?
            .image_desc("anything")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        // trigger render article
        #[rustfmt::skip]
        ac.web().get_url("/test-financni-trhy-v-soku.html").await?
            .must_see_response(StatusCode::OK)
            .verify().await?;

        // verify index
        #[rustfmt::skip]
        ac.web().get_url("/index.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("<span class=\"red\">EXKLUZIVNĚ:</span>Test Financni Trhy v Šoku")
            .verify().await?;

        // Cleanup
        trust::me::remove_file("web/test-financni-trhy-v-soku.html")?;
        trust::me::remove_file("web/u/test-financni-trhy-v-soku_image_50.jpg")?;
        trust::me::remove_file("web/u/test-financni-trhy-v-soku_image_288.jpg")?;
        trust::me::remove_file("web/u/test-financni-trhy-v-soku_image_440.jpg")?;
        trust::me::remove_file("web/u/test-financni-trhy-v-soku_image_820.jpg")?;
        Ok(())
    }
}
