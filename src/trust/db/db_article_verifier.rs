use crate::trust::app::article::create_article_data::ArticleData;
use crate::trust::data::utils::error;
use crate::trust::me::TrustError;
use TrustError::{RealData, Validation};
use crate::db::database_article_data::Article;

#[derive(Debug)]
pub struct DatabaseArticleVerifier {
    real_article_url: String,
    pub expected: ArticleData,
}

impl DatabaseArticleVerifier {
    pub fn new(real_article_url: &str) -> Self {
        Self { real_article_url: real_article_url.to_string(), expected: ArticleData::new() }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.expected = self.expected.title(title);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.expected = self.expected.text(text);
        self
    }

    pub fn verify(&self) -> Result<(), TrustError> {
        
        
        let real = Article::new();
        
        let mut errors: Vec<String> = Vec::new();

        // title
        if let Some(exp) = &self.expected.title {
            let real = self.real.title.as_ref().ok_or(RealData)?;
            if exp != real {
                errors.push(error("title", exp, real));
            }
        }

        // text
        if let Some(exp) = &self.expected.text {
            let real = self.real.text.as_ref().ok_or(RealData)?;
            errors.push(error("text", exp, real));
        }

        if errors.is_empty() { Ok(()) } else { Err(Validation(errors.join("\n"))) }
    }
}
