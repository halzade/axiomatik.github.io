use crate::trust::app::change_password::change_password_data::ChangePasswordData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;

#[derive(Debug, Clone)]
pub struct ChangePasswordController {
    input: ChangePasswordData,
}

impl ChangePasswordController {
    pub fn new() -> Self {
        Self { input: ChangePasswordData::new() }
    }

    pub fn execute(self) -> Result<(ResponseVerifier), TrustError> {
        // TODO response

        Ok(())
    }
}
