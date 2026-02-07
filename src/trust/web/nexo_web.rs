use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use http::Request;
use tower::ServiceExt;

pub struct NexoWeb {
    web_router: Router,
}

impl NexoWeb {
    pub fn new(web_router: Router) -> Self {
        Self { web_router }
    }

    pub async fn get_url(&self, url: &str) -> Result<ResponseVerifier, TrustError> {
        let response = self
            .web_router
            .clone()
            .oneshot(Request::builder().method("GET").uri(url).body(axum::body::Body::empty())?)
            .await?;

        Ok(ResponseVerifier::new(response))
    }
}
