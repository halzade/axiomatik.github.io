use crate::trust::app::account::account_data::AccountData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AccountController {
    app_router: Arc<Router>,
    input: AccountData,
}

impl AccountController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: AccountData::new() }
    }

    pub fn update_author(self) -> Result<(ResponseVerifier), TrustError> {
        // TODO response

        Ok(())
    }
}
