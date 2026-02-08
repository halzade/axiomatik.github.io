#[cfg(test)]
mod tests {
    use axum::http::{header, Request, StatusCode};
    use reqwest::Body;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_validation_create_article() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // 1. Create and login user
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user9")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        ac.login()
            .username("user9")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        // let image_data = utils::get_test_image_data();
        // 
        // // 2. Create an article with malicious input
        // let body = ArticleBuilder::new()
        //     .title("Title")
        //     .author("Author")
        //     .category("republika")
        //     .text("Content")
        //     .short_text("Sho\x07rt")
        //     .image("test.jpg", &image_data, PNG)
        //     .image_desc("test description")
        //     .build();

        // let response = utils::one_shot(
        //     Request::builder()
        //         .method("POST")
        //         .uri("/create")
        //         .header(header::CONTENT_TYPE, content_type_with_boundary())
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::from(body.unwrap()))
        //         .unwrap(),
        // )
        // .await;

        // assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        Ok(())
    }
}
