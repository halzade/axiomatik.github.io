use crate::db::database_system::{ArticleStatus, DatabaseSystem};
use crate::system::logger;
use crate::trust::db::db_system_verifier::DatabaseSystemVerifier;
use crate::trust::me::TrustError;
use std::sync::Arc;

#[derive(Debug)]
pub struct DatabaseSystemController {
    dbs: Arc<DatabaseSystem>,
}

impl DatabaseSystemController {
    pub fn new(dbs: Arc<DatabaseSystem>) -> Self {
        Self { dbs }
    }

    /*
     * only for local tests
     */
    pub async fn new_local() -> Result<Self, TrustError> {
        logger::config();
        let dbs = Arc::new(DatabaseSystem::new_from_scratch().await?);
        Ok(Self { dbs })
    }

    pub async fn setup_article_status(
        &self,
        article_file_name: &str,
        article_status: ArticleStatus,
    ) -> Result<(), TrustError> {
        self.dbs.write_article_record(article_file_name, article_status).await?;
        Ok(())
    }

    pub async fn must_see(
        &self,
        article_file_name: &str,
    ) -> Result<DatabaseSystemVerifier, TrustError> {
        /*
         * retrieve the real data
         */
        let real = self.dbs.read_article_validity(article_file_name).await?;
        // build verifier
        Ok(DatabaseSystemVerifier::new(real))
    }
}

#[cfg(test)]
mod tests {
    use crate::db::database_system::ArticleStatus::{Invalid, Valid};
    use crate::trust::db::db_system_controller::DatabaseSystemController;
    use crate::trust::me::TrustError;

    #[tokio::test]
    async fn test_system_verifier_pass() -> Result<(), TrustError> {
        let sc = DatabaseSystemController::new_local().await?;

        sc.setup_article_status("test-article.html", Valid).await?;

        #[rustfmt::skip]
        sc.must_see("test-article.html").await?
            .article_status(Valid)
            .verify()?;

        Ok(())
    }

    #[tokio::test]
    async fn test_system_verifier_fail() -> Result<(), TrustError> {
        let sc = DatabaseSystemController::new_local().await?;

        sc.setup_article_status("test-article.html", Invalid).await?;

        #[rustfmt::skip]
        let err = sc.must_see("test-article.html").await?
            .article_status(Valid)
            .verify(); // wait for error

        assert!(err.is_err());
        assert_eq!(err.unwrap_err().to_string(), "validation error: 1 incorrect");
        Ok(())
    }
}
