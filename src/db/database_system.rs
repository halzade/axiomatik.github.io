use crate::db::database::DatabaseSurreal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb_types::SurrealValue;

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct ArticleViews {
    pub article_file_name: String,
    pub views: u64,
}

pub struct DatabaseSystem {}

impl DatabaseSystem {
    pub fn new(p0: Arc<DatabaseSurreal>) -> DatabaseSystem {
        todo!()
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
