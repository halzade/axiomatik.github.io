#[cfg(test)]
mod tests {
    use axum::http::{header, Request};
    use http::StatusCode;
    use reqwest::Body;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_change_password() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // Create user who needs password change
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user1")
            .password("pass1234")
            .needs_password_change(true)
            .execute().await?;

        // Login as user1
        #[rustfmt::skip]
        ac.login()
            .username("user1")
            .password("pass1234")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .header_location("/change-password")
            .verify().await?;

        let cookie1 = ac.login().get_cookie().unwrap();

        // Change password
        let change_params = [("new_password", "new_password_123")];
        // let change_resp = utils::one_shot(
        //     Request::builder()
        //         .method("POST")
        //         .uri("/change-password")
        //         .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        //         .header(header::COOKIE, &cookie1)
        //         .body(Body::from(serialize(&change_params)))?,
        // )
        // .await;

        // assert_eq!(change_resp.status(), StatusCode::SEE_OTHER);
        // assert_eq!(
        //     change_resp.headers().get(header::LOCATION).unwrap(),
        //     "/account"
        // );

        // Verify change in DB
        #[rustfmt::skip]
        ac.db_user().must_see("user1").await?
            .username("user1")
            .needs_password_change(false)
            .verify()?;

        Ok(())
    }
}
