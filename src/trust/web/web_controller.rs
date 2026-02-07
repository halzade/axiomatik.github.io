use crate::trust::article::create_article_data::ArticleData;
use crate::trust::article::create_article_verifier::CreateArticleVerifier;
use crate::trust::me::TrustError;

#[derive(Debug)]
pub struct WebController;

impl WebController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_article(&self) -> CreateArticleController {}

    // app controller
    // create_article
    // change_password
    // account_update_author
    // login
}

#[cfg(test)]
mod tests {
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
