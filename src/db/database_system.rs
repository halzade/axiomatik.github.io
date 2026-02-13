use crate::db::database;
use crate::db::database::{DatabaseSurreal, SurrealError};
use crate::db::database_system::ArticleStatus::DoesNotExist;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
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
    pub const fn new(db: Arc<DatabaseSurreal>) -> Self {
        Self { surreal: db }
    }

    pub async fn new_from_scratch() -> Result<Self, SurrealError> {
        let surreal = Arc::new(database::init_in_memory_db_connection().await?);
        Ok(Self { surreal })
    }

    pub async fn write_article_record(
        &self,
        article_file_name: &str,
        article_status: ArticleStatus,
    ) -> Result<(), SurrealSystemError> {
        let _: Option<ArticleUpdateStatus> = self
            .surreal
            .db
            .upsert((ARTICLE_STATUS_TABLE, article_file_name))
            .content(ArticleUpdateStatus {
                article_file_name: article_file_name.into(),
                article_status,
            })
            .await?;
        Ok(())
    }

    pub async fn invalidate_all_article(&self) -> Result<(), SurrealSystemError> {
        self.surreal
            .db
            .query(format!("UPDATE {} SET article_status = $status", ARTICLE_STATUS_TABLE))
            .bind(("status", Invalid))
            .await?;
        Ok(())
    }

    pub async fn create_article_record(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(&article_file_name, Invalid).await
    }

    pub async fn invalidate_article(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(&article_file_name, Invalid).await
    }

    pub async fn validate_article(
        &self,
        article_file_name: String,
    ) -> Result<(), SurrealSystemError> {
        self.write_article_record(&article_file_name, Valid).await
    }

    pub async fn read_article_validity(
        &self,
        article_file_name: &str,
    ) -> Result<ArticleStatus, SurrealSystemError> {
        let response: Option<ArticleUpdateStatus> =
            self.surreal.db.select((ARTICLE_STATUS_TABLE, article_file_name)).await?;

        response.map_or_else(|| {
                warn!("requested article not found in database: {}", article_file_name);
                Ok(DoesNotExist)
            }, |status| Ok(status.article_status))
    }

    pub async fn health(&self) -> Result<String, SurrealSystemError> {
        match self.surreal.db.health().await {
            Ok(()) => Ok("ok".into()),
            Err(_) => Ok("db error".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust::me::TrustError;

    #[tokio::test]
    async fn test_article_status_updates() -> Result<(), TrustError> {
        let dbs = DatabaseSystem::new_from_scratch().await?;
        let article_name = "test-article.html".to_string();

        // doesn't exist yet
        let s = dbs.read_article_validity(&article_name).await?;
        assert_eq!(s, DoesNotExist);

        // create record
        dbs.create_article_record(article_name.clone()).await?;
        let s = dbs.read_article_validity(&article_name.clone()).await?;
        assert_eq!(s, Invalid);

        // validate it
        dbs.validate_article(article_name.clone()).await?;
        let s = dbs.read_article_validity(&article_name.clone()).await?;
        assert_eq!(s, Valid);

        // invalidate it
        dbs.invalidate_article(article_name.clone()).await?;
        let s = dbs.read_article_validity(&article_name.clone()).await?;
        assert_eq!(s, Invalid);

        // validate and then invalidate all
        dbs.validate_article(article_name.clone()).await?;
        let article_name2 = "test-article2.html".to_string();
        dbs.validate_article(article_name2.clone()).await?;

        dbs.invalidate_all_article().await?;

        let s1 = dbs.read_article_validity(&article_name).await?;
        let s2 = dbs.read_article_validity(&article_name2).await?;
        assert_eq!(s1, Invalid);
        assert_eq!(s2, Invalid);

        Ok(())
    }
}
