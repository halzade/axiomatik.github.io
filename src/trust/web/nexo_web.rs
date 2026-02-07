use axum::Router;
use axum_core::response::Response;
use crate::system::configuration;
use crate::system::router_web::WebRouter;
use crate::trust::me::TrustError;
use crate::trust::response_verifier::ResponseVerifier;
use tower::ServiceExt;
use http::Request;

pub struct NexoWeb {
    web_router: Router,
}

impl NexoWeb {
    pub fn new(web_router: Router) -> Self {
        Self { web_router }
    }

    pub async fn get_url(&self, url: &str) -> Result<ResponseVerifier, TrustError> {
        let response = self.web_router
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(url)
                    .body(axum::body::Body::empty())?,
            )
            .await?;

        Ok(ResponseVerifier::new_from_response(response).await?)
    }
}