use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleViews {
    pub article_file_name: String,
    pub views: i64,
}

/*
 * Increase and read article view count
 */
pub fn increase_article_views(article_base: String) -> u64 {
    // TODO

    0
}
