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

pub struct DatabaseSystem {
    db: Arc<DatabaseSurreal>,
}

impl DatabaseSystem {
    pub fn new(db: Arc<DatabaseSurreal>) -> DatabaseSystem {
        DatabaseSystem { db }
    }

    pub async fn new_from_scratch() -> Result<DatabaseSystem, SurrealError> {
        let db = Arc::new(database::initialize_in_memory_database().await?);
        Ok(DatabaseSystem { db })
    }
}

/*
 * Increase and read article view count
 */
// TODO
// pub fn increase_article_views(article_base: String) -> u64 {
//
//
// }
