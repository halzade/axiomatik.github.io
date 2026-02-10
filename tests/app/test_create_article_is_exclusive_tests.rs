#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use axum::http::{header, StatusCode};
    use axum_core::extract::Request;
    use reqwest::Body;
    use std::fs;
    use axiomatik_web::trust;
    use axiomatik_web::trust::app_controller::AppController;
    use axiomatik_web::trust::data::media_data::PNG;
    use axiomatik_web::trust::me::TrustError;

    #[tokio::test]
    async fn test_exclusive_main_article_finance() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user2")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        ac.login()
            .username("user2")
            .password("password123")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        
        // let image_data = utils::get_test_image_data();
        // let body = ArticleBuilder::new()
        //     .title("Test Financni Trhy v Šoku")
        //     .author("Financni Expert")
        //     .category("finance")
        //     .text("Dlouhý text o financich")
        //     .short_text("Krátký text o financich")
        //     .main()
        //     .exclusive()
        //     .image("test.png", &image_data, PNG)
        //     .image_desc("anything")
        //     .build()
        //     .unwrap();
        //
        // let response_create = utils::one_shot(
        //     Request::builder()
        //         .method("POST")
        //         .uri("/create")
        //         .header(header::CONTENT_TYPE, content_type_with_boundary())
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::from(body))?,
        // )
        // .await;
        //
        // assert_eq!(response_create.status(), StatusCode::SEE_OTHER);
        //
        // let response_index = utils::one_shot(
        //     http::Request::builder()
        //         .method("GET")
        //         .uri("/index.html")
        //         .body(Body::default())?,
        // )
        // .await;
        //
        // assert_eq!(response_index.status(), StatusCode::OK);
        //
        // let body = to_bytes(response_index.into_body(), usize::MAX)
        //     .await?;
        // let body_str = String::from_utf8(body.to_vec()).unwrap();
        // assert!(
        //     body_str.contains("<span class=\"red\">EXKLUZIVNĚ:</span>Test Financni Trhy v Šoku")
        // );

        // Cleanup
        trust::me::remove_file("web/test-financni-trhy-v-soku.html")?;
        trust::me::remove_file("web/u/test-financni-trhy-v-soku_image_50.jpg")?;
        trust::me::remove_file("web/u/test-financni-trhy-v-soku_image_288.jpg")?;
        trust::me::remove_file("web/u/test-financni-trhy-v-soku_image_440.jpg")?;
        trust::me::remove_file("web/u/test-financni-trhy-v-soku_image_820.jpg")?;
        Ok(())
    }
}
