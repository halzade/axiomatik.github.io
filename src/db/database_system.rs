use crate::db::database;
use crate::db::database::{DatabaseSurreal, SurrealError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb_types::SurrealValue;

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct ArticleViews {
    pub article_file_name: String,
    pub views: u64,
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
        let db = Arc::new(database::init_in_memory_db_connection().await?);
        Ok(DatabaseSystem { surreal: db })
    }

    /*
     * Increase and read article view count
     */
    // TODO
    pub async fn increase_article_views(
        &self,
        article_file_name: String,
    ) -> Result<u64, SurrealError> {
        let mut response = self
            .surreal
            .db
            .query("INSERT INTO article_views { article_file_name: $article_file_name, views: 1 } ON DUPLICATE KEY UPDATE views += 1")
            .bind(("article_file_name", article_file_name.clone()))
            .await?;

        let article_views: Option<ArticleViews> = response.take(0)?;
        match article_views {
            Some(v) => Ok(v.views),
            None => {
                let mut response = self
                    .surreal
                    .db
                    .query("SELECT * FROM article_views WHERE article_file_name = $article_file_name")
                    .bind(("article_file_name", article_file_name))
                    .await?;
                let v: Option<ArticleViews> = response.take(0)?;
                Ok(v.map(|v| v.views).unwrap_or(0))
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
        let article_name = "test-article".to_string();

        // First view
        let views = dbs.increase_article_views(article_name.clone()).await?;
        assert_eq!(views, 1);

        // Second view
        let views = dbs.increase_article_views(article_name.clone()).await?;
        assert_eq!(views, 2);

        // Different article
        let views = dbs.increase_article_views("other-article".to_string()).await?;
        assert_eq!(views, 1);

        Ok(())
    }
}
