#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use http::StatusCode;

    #[tokio::test]
    async fn test_create_and_delete_article() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // create admin user
        #[rustfmt::skip]
        ac.db_user().setup_admin_user()
            .username("admin1")
            .password("strong*admin*password")
            .execute().await?;

        // verify admin in DB
        #[rustfmt::skip]
        ac.db_user().must_see("admin1").await?
            .username("admin1")
            .needs_password_change(false)
            .verify()?;

        // login as admin
        #[rustfmt::skip]
        let auth = ac.login()
            .username("admin1")
            .password("strong*admin*password")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/admin_user")
                .verify().await?;

        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test Article Admin")
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
        ac.web().get_url("/test-article-admin.html").await?
            .must_see_response(StatusCode::OK)
            .body_contains("Test Article Admin")
            .body_contains("This is a test article text.")
            .verify().await?;
        // article was rendered and served

        trust::me::path_exists("web/test-article-admin.html")?;

        // verify article in DB
        ac.db_article()
            .must_see("test-article-admin.html")
            .await?
            .title("Test Article Admin")
            .verify()?;

        // delete article
        #[rustfmt::skip]
        ac.admin(&auth).delete_article()
            .article_file_name("test-article-admin.html")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .header_location("/admin_article")
                .verify().await?;

        // verify the article was deleted from DB
        ac.db_article().must_not_see("test-article-admin.html").await?.verify()?;

        // Cleanup
        trust::me::path_doesnt_exists("web/test-article-admin.html")?;
        trust::me::path_doesnt_exists("web/u/test-article-admin_image_50.png")?;
        trust::me::path_doesnt_exists("web/u/test-article-admin_image_288.png")?;
        trust::me::path_doesnt_exists("web/u/test-article-admin_image_440.png")?;
        trust::me::path_doesnt_exists("web/u/test-article-admin_image_820.png")?;

        Ok(())
    }
}
