#[cfg(test)]
mod tests {
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_sql_injection_rejection_in_login() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.login()
            .username("admin' OR '1'='1")
            .password("anything")
            .execute().await?
            .must_see_response(StatusCode::BAD_REQUEST)
            .verify()?;

        Ok(())
    }
}
