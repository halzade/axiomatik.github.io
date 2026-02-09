use crate::db::database;
use crate::db::database::{DatabaseSurreal, SurrealError};
use crate::db::database_system::SurrealSystemError::ViewsNotFound;
use crate::system::data_updates::ArticleStatus;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb_types::SurrealValue;
use thiserror::Error;
use tracing::warn;

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

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
// TODO name
pub struct ArticleUpdateStatus {
    pub article_file_name: String,
    pub status: ArticleStatus,
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
        surreal.db.query("DEFINE TABLE article_status SCHEMALESS;").await?;

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
                "INSERT INTO article_views {
                    article_file_name: $article_file_name,
                    views: 1
                }
                ON DUPLICATE KEY UPDATE
                views += 1
                RETURN views",
            )
            .bind(("article_file_name", article_file_name.clone()))
            .await?;

        // TODO .take() may throw
        let article_views: Option<ArticleViews> = response.take(0)?;
        match article_views {
            Some(v) => Ok(v.views),
            None => {
                // TODO maybe better to return zero
                Err(ViewsNotFound(article_file_name))
            }
        }
    }

    pub async fn write_article_record(
        &self,
        article_file_name: String,
        article_status: ArticleStatus,
    ) -> Result<(), SurrealSystemError> {
        self.surreal.db
            .query("INSERT INTO article_status { article_file_name: $article_file_name, status: $status } ON DUPLICATE KEY UPDATE status = $status")
            .bind(("article_file_name", article_file_name.clone()))
            .bind(("status", article_status))
            .await?;
        Ok(())
    }

    pub async fn create_article_record(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(article_file_name, ArticleStatus::Invalid).await
    }

    pub async fn invalidate_article(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(article_file_name, ArticleStatus::Invalid).await
    }

    pub async fn validate_article(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(article_file_name, ArticleStatus::Valid).await
    }

    pub async fn read_article_validity(
        &self,
        article_file_name: String,
    ) -> Result<ArticleStatus, SurrealSystemError> {
        let mut response = self
            .surreal
            .db
            .query(
                "SELECT status
                 FROM article_status
                 WHERE article_file_name = $article_file_name
                 LIMIT 1",
            )
            .bind(("article_file_name", article_file_name.clone()))
            .await?;

        let mut rows: Vec<ArticleUpdateStatus> = response.take(0)?;

        match rows.pop() {
            Some(row) => Ok(row.status),
            None => {
                warn!("requested article not found in database: {}", article_file_name);
                Ok(ArticleStatus::DoesntExist)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing::debug;
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

        println!("doesn't exist yet");
        let s = dbs.read_article_validity(article_name.clone()).await?;
        assert_eq!(s, ArticleStatus::DoesntExist);

        println!("create record exist yet");
        // 1. Create a record (should be Invalid)
        dbs.create_article_record(article_name.clone()).await?;

        println!("check validity");
        let s = dbs.read_article_validity(article_name.clone()).await?;
        assert_eq!(s, ArticleStatus::Invalid);

        println!("validate it");
        // 2. Validate article (should be Valid)
        dbs.validate_article(article_name.clone()).await?;

        let s = dbs.read_article_validity(article_name.clone()).await?;
        assert_eq!(s, ArticleStatus::Valid);

        println!("invalidate it");
        // 3. Invalidate article (should be Invalid)
        dbs.invalidate_article(article_name.clone()).await?;
        let s = dbs.read_article_validity(article_name.clone()).await?;
        assert_eq!(s, ArticleStatus::Invalid);

        println!("finished");
        Ok(())
    }
}
