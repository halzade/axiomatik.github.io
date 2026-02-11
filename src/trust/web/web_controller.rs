use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use std::sync::Arc;
use tower::ServiceExt;
use tracing::error;

#[derive(Debug)]
pub struct WebController {
    web_router: Arc<Router>,
}

impl WebController {
    pub fn new(web_router: Router) -> Self {
        Self { web_router: Arc::new(web_router) }
    }

    pub async fn get_url(&self, url: &str) -> Result<ResponseVerifier, TrustError> {
        if !url.starts_with('/') {
            error!("url must start with '/'")
        }
        let response = (*self.web_router)
            .clone()
            .oneshot(Request::builder().method("GET").uri(url).body(Body::empty())?)
            .await?;

        Ok(ResponseVerifier::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust::app_controller::AppController;
    use http::StatusCode;

    #[tokio::test]
    async fn test_web_controller() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.web().get_url("/").await?
            .must_see_response(StatusCode::OK)
            .verify().await?;

        Ok(())
    }
}
