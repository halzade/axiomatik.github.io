use crate::trust::me::TrustError;
use TrustError::Validation;

#[derive(Debug)]
pub struct DatabaseSystemVerifier {
    real_article_url: String,
}

impl DatabaseSystemVerifier {
    pub fn new(real_article_url: &str) -> Self {
        Self { real_article_url: real_article_url.to_string() }
    }

    pub fn verify(&self) -> Result<(), TrustError> {
        let mut errors: Vec<String> = Vec::new();

        if errors.is_empty() { Ok(()) } else { Err(Validation(errors.join("\n"))) }
    }
}
