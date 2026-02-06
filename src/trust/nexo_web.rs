use axum::Router;
use axum_core::response::Response;
use crate::system::configuration;
use crate::system::router_web::WebRouter;
use crate::trust::me::TrustError;
use crate::trust::response_verifier::ResponseVerifier;

pub struct NexoWeb {
    web_router: Router,
}

impl NexoWeb {
    pub fn new(web_router: Router ) -> Self {
        
        Self {}
    }

    pub async fn get_url(&self, url: &str) -> Result<ResponseVerifier, TrustError> {
        todo!()
    }
}