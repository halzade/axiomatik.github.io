use crate::trust::app::login::login_data::LoginData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LoginController {
    app_router: Arc<Router>,
    input: LoginData,
}

impl LoginController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: LoginData::new() }
    }

    pub fn execute(self) -> Result<(ResponseVerifier), TrustError> {
        // TODO response

        Ok(())
    }
}
