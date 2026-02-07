use crate::db::database_article::DatabaseArticle;
use crate::trust::db::db_article_verifier::DatabaseArticleVerifier;
use crate::trust::me::TrustError;
use std::sync::Arc;

#[derive(Debug)]
pub struct DatabaseArticleController {
    dba: Arc<DatabaseArticle>,
}

impl DatabaseArticleController {
    pub fn new(dba: Arc<DatabaseArticle>) -> Self {
        Self { dba }
    }

    pub async fn must_see(
        &self,
        article_file_html: &str,
    ) -> Result<DatabaseArticleVerifier, TrustError> {
        /*
         * retrieve the real data
         */
        let real_o = self.dba.article_by_file_name(article_file_html).await?;
        match real_o {
            Some(real) => {
                // build verifier
                Ok(DatabaseArticleVerifier::new(real))
            }
            None => Err(TrustError::RealData),
        }
    }
}
