use crate::trust::app::article::create_article_data::ArticleFluent;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use std::sync::Arc;
use http::{header, Request, StatusCode};
use reqwest::Body;
use tower::ServiceExt;
use crate::trust::data::utils::content_type_with_boundary;

#[derive(Debug, Clone)]
pub struct CreateArticleController {
    app_router: Arc<Router>,
    input: ArticleFluent,
}

impl CreateArticleController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: ArticleFluent::new() }
    }

    pub fn title(&self, title: &str) -> &Self {
        self.input.title(title);
        self
    }

    pub fn text(&self, text: &str) -> &Self {
        self.input.text(text);
        self
    }

    // pub fn execute(&self) -> Result<(ResponseVerifier), TrustError> {
    pub fn execute(&self) -> Result<(), TrustError> {
        ;

        // let response = self.app_router.oneshot(
        //     Request::builder()
        //         .method("POST")
        //         .uri("/create")
        //         .header(header::CONTENT_TYPE, content_type_with_boundary())
        //         .header(header::COOKIE, &cookie)
        //         .body(Body::from(body.unwrap()))
        //         .unwrap(),
        // )
        //     .await?;

        // assert_eq!(response.status(), StatusCode::SEE_OTHER);

        Ok(())
    }
}
