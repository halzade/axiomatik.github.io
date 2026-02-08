use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use http::{header, Request};
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Debug, Clone)]
pub struct AccountController {
    app_router: Arc<Router>,
}

impl AccountController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router }
    }

    // pub fn update_author(self) -> Result<(ResponseVerifier), TrustError> {
    pub fn update_author_name(self) -> Result<(), TrustError> {
        // TODO response

        Ok(())
    }

    pub async fn get(&self, auth_cookie: String) -> Result<ResponseVerifier, TrustError> {
        let response = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/account")
                    .header(header::COOKIE, &auth_cookie)
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        Ok(ResponseVerifier::new(response))
    }
}
