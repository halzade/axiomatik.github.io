#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_account_page() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // Create user and login
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user8")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user8")
            .password("password123")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        // Access account page
        #[rustfmt::skip]
        ac.account().get(&auth).await?
            .must_see_response(StatusCode::OK)
            .body_contains("user8")
            .body_contains("Moje články")
            .verify().await?;

        // Update author name
        #[rustfmt::skip]
        ac.account().update_author_name(&auth)
            .author_name("Updated Author")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/account")
                .verify().await?;

        // Verify update in DB
        #[rustfmt::skip]
        ac.db_user()
            .must_see("user8").await?
            .username("user8")
            .author_name("Updated Author")
            .verify()?;

        // Create an article with this user
        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test User Article")
            .author("Updated Author")
            .category("zahranici")
            .text("Content")
            .short_text("Short")
            .image_any_png()?
            .image_desc("test image description")
            .related_articles("related-test-article.html")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        // 7. Verify the article is on the account page
        #[rustfmt::skip]
        ac.account().get(&auth).await?
            .must_see_response(StatusCode::OK)
            .body_contains("user8")
            .body_contains("Test User Article")
            .verify().await?;

        // Update author name again
        #[rustfmt::skip]
        ac.account().update_author_name(&auth)
            .author_name("Second Update")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/account")
                .verify().await?;

        // Verify the article is STILL on the account page (linked by username, not author_name)
        #[rustfmt::skip]
        ac.account()
            .get(&auth).await?
            .must_see_response(StatusCode::OK)
                .body_contains("user8")
                .body_contains("Second Update")
                .body_contains("Test User Article")
                .verify().await?;

        // cleanup files
        trust::me::remove_file("web/test-user-article.html")?;
        trust::me::remove_file("web/u/test-user-article_image_820.png")?;
        trust::me::remove_file("web/u/test-user-article_image_50.png")?;
        trust::me::remove_file("web/u/test-user-article_image_288.png")?;
        trust::me::remove_file("web/u/test-user-article_image_440.png")?;

        Ok(())
    }
}
