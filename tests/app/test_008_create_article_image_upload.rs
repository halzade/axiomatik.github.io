#[cfg(test)]
mod tests {
    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;
    use image::GenericImageView;

    #[tokio::test]
    async fn test_image_upload_resized_copies() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // 1. Set up user and login
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("image_tester")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("image_tester")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        #[rustfmt::skip]
        ac.create_article(&auth)
            .title("Test Image Upload")
            .author("Tester")
            .category("republika")
            .text("Test content")
            .short_text("Short text")
            .image_any_png()?
            .image_desc("Description")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        trust::me::path_exists("web/u/test-image-upload_image_820.png")?;
        trust::me::path_exists("web/u/test-image-upload_image_50.png")?;
        trust::me::path_exists("web/u/test-image-upload_image_288.png")?;
        trust::me::path_exists("web/u/test-image-upload_image_440.png")?;

        assert_eq!(820, image::open("web/u/test-image-upload_image_820.png")?.dimensions().0);
        assert_eq!((50, 50), image::open("web/u/test-image-upload_image_50.png")?.dimensions());
        assert_eq!((288, 211), image::open("web/u/test-image-upload_image_288.png")?.dimensions());
        assert_eq!((440, 300), image::open("web/u/test-image-upload_image_440.png")?.dimensions());

        // Cleanup
        trust::me::remove_file("web/u/test-image-upload_image_820.png")?;
        trust::me::remove_file("web/u/test-image-upload_image_50.png")?;
        trust::me::remove_file("web/u/test-image-upload_image_288.png")?;
        trust::me::remove_file("web/u/test-image-upload_image_440.png")?;

        Ok(())
    }
}
