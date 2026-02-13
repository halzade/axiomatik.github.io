use crate::db::database_article::DatabaseArticle;
use crate::system::logger;
use crate::trust::app::article::create_article_easy_builder::ArticleBuilder;
use crate::trust::db::db_article_verifier::DatabaseArticleVerifier;
use crate::trust::me::TrustError;
use std::sync::Arc;

#[derive(Debug)]
pub struct DatabaseArticleController {
    dba: Arc<DatabaseArticle>,
}

impl DatabaseArticleController {
    pub const fn new(dba: Arc<DatabaseArticle>) -> Self {
        Self { dba }
    }

    /*
     * only for local tests
     */
    pub async fn new_local() -> Result<Self, TrustError> {
        logger::config();
        let dba = Arc::new(DatabaseArticle::new_from_scratch().await?);
        Ok(Self { dba })
    }

    pub async fn db_setup_article(&self, title: &str, text: &str) -> Result<(), TrustError> {
        #[rustfmt::skip]
        self.dba.create_article(ArticleBuilder::article()
            .title(title)
            .text(text)
            .build()
        ).await?;
        Ok(())
    }

    pub async fn must_see(
        &self,
        article_file_html: &str,
    ) -> Result<DatabaseArticleVerifier, TrustError> {
        /*
         * retrieve the real data
         */
        let real = self.dba.article_by_file_name(article_file_html).await?;
        // build verifier
        Ok(DatabaseArticleVerifier::new(real))
    }
}

#[cfg(test)]
mod tests {
    use crate::trust::db::db_article_controller::DatabaseArticleController;
    use crate::trust::me::TrustError;

    #[tokio::test]
    async fn test_article_verifier_pass() -> Result<(), TrustError> {
        let ac = DatabaseArticleController::new_local().await?;

        ac.db_setup_article("Test Article 1", "Content of the article").await?;

        #[rustfmt::skip]
        ac.must_see("test-article-1.html").await?
            .title("Test Article 1")
            .text("Content of the article")
            .verify()?;

        Ok(())
    }

    #[tokio::test]
    async fn test_article_verifier_fail() -> Result<(), TrustError> {
        let ac = DatabaseArticleController::new_local().await?;

        ac.db_setup_article("Test Article 2", "Content of the article").await?;

        #[rustfmt::skip]
        let err = ac.must_see("test-article-2.html").await?
            .title("Wrong Title")
            .text("Wrong content")
            .verify();

        assert!(err.is_err());
        assert_eq!(err.unwrap_err().to_string(), "validation error: 2 incorrect");
        Ok(())
    }
}
