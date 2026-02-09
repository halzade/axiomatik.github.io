use crate::db::database;
use crate::db::database::{DatabaseSurreal, SurrealError};
use crate::db::database_system::ArticleStatus::DoesNotExist;
use crate::db::database_system::SurrealSystemError::ViewsNotFound;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb_types::SurrealValue;
use thiserror::Error;
use tracing::warn;
use ArticleStatus::{Invalid, Valid};

const ARTICLE_STATUS_TABLE: &str = "article_update_status";

#[derive(Debug, Error)]
pub enum SurrealSystemError {
    #[error("surreal db error: {0}")]
    Surreal(#[from] surrealdb::Error),

    #[error("article not found: {0}")]
    ArticleNotFound(String),

    #[error("views not found for {0}")]
    ViewsNotFound(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct ArticleViews {
    pub article_file_name: String,
    pub views: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, SurrealValue)]
#[serde(rename_all = "lowercase")]
pub enum ArticleStatus {
    Valid,
    Invalid,
    DoesNotExist,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct ArticleUpdateStatus {
    pub article_file_name: String,
    pub article_status: ArticleStatus,
}

/**
 * access to a database
 * - anything system-related
 */
#[derive(Debug)]
pub struct DatabaseSystem {
    surreal: Arc<DatabaseSurreal>,
}

impl DatabaseSystem {
    pub fn new(db: Arc<DatabaseSurreal>) -> DatabaseSystem {
        DatabaseSystem { surreal: db }
    }

    pub async fn new_from_scratch() -> Result<DatabaseSystem, SurrealError> {
        let surreal = Arc::new(database::init_in_memory_db_connection().await?);
        Ok(DatabaseSystem { surreal })
    }

    /*
     * Increase and read article view count
     */
    // TODO
    pub async fn increase_article_views(
        &self,
        article_file_name: String,
    ) -> Result<u64, SurrealSystemError> {
        let mut response = self
            .surreal
            .db
            .query(
                "UPSERT type::record(\"article_views\", $article_file_name) SET views += 1, article_file_name = $article_file_name RETURN views"
            )
            .bind(("article_file_name", article_file_name.clone()))
            .await?;

        let views: Option<u64> = response.take("views")?;
        match views {
            Some(v) => Ok(v),
            None => {
                warn!("article not found in article_views: {}", article_file_name);
                Ok(0)
            }
        }
    }

    pub async fn write_article_record(
        &self,
        article_file_name: String,
        article_status: ArticleStatus,
    ) -> Result<(), SurrealSystemError> {
        let _: Option<ArticleUpdateStatus> = self
            .surreal
            .db
            .upsert((ARTICLE_STATUS_TABLE, article_file_name.clone()))
            .content(ArticleUpdateStatus { article_file_name, article_status })
            .await?;
        Ok(())
    }

    pub async fn create_article_record(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(article_file_name, Invalid).await
    }

    pub async fn invalidate_article(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(article_file_name, Invalid).await
    }

    pub async fn validate_article(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(article_file_name, Valid).await
    }

    pub async fn read_article_validity(
        &self,
        article_file_name: String,
    ) -> Result<ArticleStatus, SurrealSystemError> {
        let response: Option<ArticleUpdateStatus> =
            self.surreal.db.select((ARTICLE_STATUS_TABLE, article_file_name.clone())).await?;

        match response {
            Some(status) => Ok(status.article_status),
            None => {
                warn!("requested article not found in database: {}", article_file_name);
                Ok(DoesNotExist)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust::me::TrustError;

    #[tokio::test]
    async fn test_increase_article_views() -> Result<(), TrustError> {
        let dbs = DatabaseSystem::new_from_scratch().await?;
        let article_name = "test-article.html".to_string();

        // First view
        let views = dbs.increase_article_views(article_name.clone()).await?;
        assert_eq!(views, 1);

        // Second view
        let views = dbs.increase_article_views(article_name.clone()).await?;
        assert_eq!(views, 2);

        // Different article
        let views = dbs.increase_article_views("other-article.html".to_string()).await?;
        assert_eq!(views, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_article_status_updates() -> Result<(), TrustError> {
        let dbs = DatabaseSystem::new_from_scratch().await?;
        let article_name = "test-article.html".to_string();

        // doesn't exist yet
        let s = dbs.read_article_validity(article_name.clone()).await?;
        assert_eq!(s, DoesNotExist);

        // create record
        dbs.create_article_record(article_name.clone()).await?;
        let s = dbs.read_article_validity(article_name.clone()).await?;
        assert_eq!(s, Invalid);

        // validate it
        dbs.validate_article(article_name.clone()).await?;
        let s = dbs.read_article_validity(article_name.clone()).await?;
        assert_eq!(s, Valid);

        // invalidate it
        dbs.invalidate_article(article_name.clone()).await?;
        let s = dbs.read_article_validity(article_name.clone()).await?;
        assert_eq!(s, Invalid);

        Ok(())
    }
}
