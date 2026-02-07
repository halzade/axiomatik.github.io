use crate::trust::app::article::create_article_data::ArticleData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CreateArticleController {
    app_router: Arc<Router>,
    input: ArticleData,
}

impl CreateArticleController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: ArticleData::new() }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.input = self.input.title(title);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.input = self.input.text(text);
        self
    }

    pub fn execute(self) -> Result<(ResponseVerifier), TrustError> {
        // TODO response

        Ok(())
    }
}
