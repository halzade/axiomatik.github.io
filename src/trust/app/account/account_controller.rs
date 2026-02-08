use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use std::sync::Arc;
use tower::ServiceExt;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct AccountController {
    app_router: Arc<Router>,
}

impl AccountController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router }
    }

    pub async fn update_author_name(
        &self,
        auth_cookie: &str,
        author_name: &str,
    ) -> Result<ResponseVerifier, TrustError> {
        debug!("update author name: {}", author_name);
        let response = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/account/update-author")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(header::COOKIE, auth_cookie.to_string())
                    .body(Body::from(format!("author_name={}", author_name)))?,
            )
            .await?;
        debug!("update author name done");
        Ok(ResponseVerifier::new(response))
    }

    pub async fn get(&self, auth_cookie: &str) -> Result<ResponseVerifier, TrustError> {
        debug!("get account page");
        let response = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/account")
                    .header(header::COOKIE, &auth_cookie.to_string())
                    .body(Body::empty())?,
            )
            .await?;
        debug!("get account page done");
        Ok(ResponseVerifier::new(response))
    }
}
