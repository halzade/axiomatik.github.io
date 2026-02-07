use crate::db::database_user::User;
use crate::trust::app::article::create_article_data::ArticleData;
use crate::trust::me::TrustError;
use TrustError::Validation;

#[derive(Debug)]
pub struct DatabaseUserVerifier {
    real_article_url: String,
    pub expected: User,
}

impl DatabaseUserVerifier {
    pub fn new(real_article_url: &str) -> Self {
        Self { real_article_url: real_article_url.to_string(), expected: ArticleData::new() }
    }

    pub fn verify(&self) -> Result<(), TrustError> {
        let real = User::new();

        let mut errors: Vec<String> = Vec::new();

        if errors.is_empty() { Ok(()) } else { Err(Validation(errors.join("\n"))) }
    }
}
