use crate::trust::me::TrustError;
use TrustError::Validation;

#[derive(Debug)]
pub struct DatabaseSystemVerifier {}

impl DatabaseSystemVerifier {
    pub fn new(real_article_url: &str) -> Self {
        Self {}
    }

    pub fn verify(&self) -> Result<(), TrustError> {
        let errors: Vec<String> = Vec::new();

        if errors.is_empty() { Ok(()) } else { Err(Validation(errors.join("\n"))) }
    }
}
