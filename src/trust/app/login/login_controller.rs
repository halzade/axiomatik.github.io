use crate::trust::app::login::login_data::LoginData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;

#[derive(Debug)]
pub struct LoginController {
    input: LoginData,
}

impl LoginController {
    pub fn new() -> Self {
        Self { input: LoginData::new() }
    }

    pub fn execute(self) -> Result<(ResponseVerifier), TrustError> {
        // TODO response

        Ok(())
    }
}
