use crate::trust::app::change_password::change_password_data::ChangePasswordData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use std::sync::Arc;

#[derive(Debug)]
pub struct ChangePasswordController {
    app_router: Arc<Router>,
    input: ChangePasswordData,
}

impl ChangePasswordController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: ChangePasswordData::new() }
    }

    // pub fn execute(self) -> Result<(ResponseVerifier), TrustError> {
    pub fn execute(self) -> Result<(), TrustError> {
        // TODO response

        Ok(())
    }
}
