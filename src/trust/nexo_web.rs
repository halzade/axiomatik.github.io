use axum_core::response::Response;
use crate::trust::me::TrustError;
use crate::trust::response_verifier::ResponseVerifier;

pub struct NexoWeb {

}

impl NexoWeb {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_url(&self, url: &str) -> Result<ResponseVerifier, TrustError> {
        todo!()
    }
}