use crate::db::database_article::DatabaseArticle;
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

    pub fn execute(self) -> Result<(), TrustError> {
        // TODO response

        Ok(())
    }
}
