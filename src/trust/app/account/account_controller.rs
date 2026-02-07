use crate::trust::app::account::account_data::AccountData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;

#[derive(Debug)]
pub struct AccountController {
    input: AccountData,
}

impl AccountController {
    pub fn new() -> Self {
        Self { input: AccountData::new() }
    }

    pub fn execute(self) -> Result<(ResponseVerifier), TrustError> {
        // TODO response

        Ok(())
    }
}
