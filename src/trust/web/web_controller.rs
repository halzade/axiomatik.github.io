use http::Request;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Debug)]
pub struct WebController {
    web_router: Arc<Router>,
}

impl WebController {
    pub fn new(web_router: Router) -> Self {
        Self {
            web_router: Arc::new(web_router),
        }
    }

    pub async fn get_url(&self, url: &str) -> Result<ResponseVerifier, TrustError> {
        let response = (*self.web_router)
            .clone()
            .oneshot(Request::builder().method("GET").uri(url).body(axum::body::Body::empty())?)
            .await?;

        Ok(ResponseVerifier::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_web_controller() -> Result<(), TrustError> {
        let ac = crate::trust::app_controller::AppController::new().await?;
        let web = ac.get_web();

        web.get_url("/").await?;

        Ok(())
    }
}
