use tracing::error;
use crate::db::database_article_data::Article;
use crate::trust::app::article::create_article_data::ArticleFluent;
use crate::trust::data::utils::error;
use crate::trust::me::TrustError;
use TrustError::Validation;

#[derive(Debug)]
pub struct DatabaseArticleVerifier {
    real: Article,
    pub expected: ArticleFluent,
}

impl DatabaseArticleVerifier {
    pub fn new(real: Article) -> Self {
        Self { real, expected: ArticleFluent::new() }
    }

    pub fn verify(&self) -> Result<(), TrustError> {
        let mut errors: Vec<String> = Vec::new();
        let expected = self.expected.get_data();

        // title
        if let Some(exp) = expected.title {
            let real = self.real.title.as_str();
            if exp != real {
                errors.push(error("title", exp, real));
            }
        }

        // text
        if let Some(exp) = expected.text {
            let real = self.real.text.as_str();
            if exp != real {
                errors.push(error("text", exp, real));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            for e in &errors {
                error!("{}", e);
            }
            Err(Validation(format!("{} incorrect", errors.len())))
        }
    }

    /*
     * fluent interface methods
     */
    pub fn title(&self, title: &str) -> &Self {
        self.expected.title(title);
        self
    }

    pub fn text(&self, text: &str) -> &Self {
        self.expected.text(text);
        self
    }
}
