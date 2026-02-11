#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use http::StatusCode;

    #[tokio::test]
    async fn test_create_editor_user() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // no public access to health
        #[rustfmt::skip]
        ac.web().get_url("/health").await?
            .must_see_response(StatusCode::NOT_FOUND)
            .verify().await?;

        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user21")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user21")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        // health for logged in user
        #[rustfmt::skip]
        ac.web().get_url_authorized("/health", &auth).await?
            .must_see_response(StatusCode::OK)
            .body_contains("ok")
            .verify().await?;

        Ok(())
    }
}
