use crate::trust::article::article_data::ArticleData;
use crate::trust::article::article_verifier::ArticleVerifier;
use crate::trust::me::TrustError;

#[derive(Debug)]
pub struct ArticleController {
    input: ArticleData,
}

impl ArticleController {

    pub fn create_article(&self) -> CreateArticleController {

    }

    // change_password
    // update-author

    pub fn new() -> Self {
        Self { input: ArticleData::new() }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.input = self.input.title(title);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.input = self.input.text(text);
        self
    }

    pub fn execute(self) -> Result<Self, TrustError> {
        let received_data = ArticleData::new();

        Ok(Self {
            real: ArticleData { title: received_data.title, text: received_data.text },
            expected: ArticleData::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_framework() -> Result<(), TrustError> {
        let article_controller = ArticleController::new();

        #[rustfmt::skip]
        let resp = article_controller
            .title("title")
            .text("text")
            .execute()?;

        #[rustfmt::skip]
        article_controller.must_see(resp)
            .title("title")
            .text("text")
            .verify();

        Ok(())
    }
}
