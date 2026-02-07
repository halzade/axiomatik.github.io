use crate::trust::app::article::create_article_data::ArticleData;
use crate::trust::data::utils::error;
use crate::trust::me::TrustError;
use TrustError::{RealData, Validation};
use crate::db::database_article_data::Article;

#[derive(Debug)]
pub struct DatabaseArticleVerifier {
    real: Article,
    pub expected: ArticleData,
}

impl DatabaseArticleVerifier {
    pub fn new(real: Article) -> Self {
        Self { real, expected: ArticleData::new() }
    }

    pub fn title(&self, title: &str) -> &Self {
        self.expected.title(title);
        self
    }

    pub fn text(&self, text: &str) -> &Self {
        self.expected.text(text);
        self
    }

    pub fn verify(&self) -> Result<(), TrustError> {

        let mut errors: Vec<String> = Vec::new();

        // title
        if let Some(exp) = &self.expected.title {
            let real = self.real.title;
            if exp != real {
                errors.push(error("title", exp, real));
            }
        }

        // text
        if let Some(exp) = &self.expected.text {
            let real = self.real.text;
            errors.push(error("text", exp, real));
        }

        if errors.is_empty() { Ok(()) } else { Err(Validation(errors.join("\n"))) }
    }
}
