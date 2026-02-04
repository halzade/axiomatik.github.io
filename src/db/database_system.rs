use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct ArticleViews {
    pub article_file_name: String,
    pub views: i64,
}

/*
 * Increase and read article view count
 */
// TODO
// pub fn increase_article_views(article_base: String) -> u64 {
//
//
// }
