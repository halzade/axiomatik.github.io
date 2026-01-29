use crate::db::database::DatabaseError;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub author: String,
    pub created_by: String,
    pub date: String,
    pub title: String,
    pub text: String,
    pub short_text: String,
    pub article_file_name: String,
    pub image_url: String,
    pub image_description: String,
    pub video_url: Option<String>,
    pub audio_url: Option<String>,
    pub category: String,
    pub related_articles: Vec<String>,
    pub is_main: bool,
    pub is_exclusive: bool,
    pub views: i64,
}

pub async fn create_article(article: Article) -> Option<Article> {
    let sdb_wg = crate::db::database::db_write().await.ok()?;
    let article_r: Result<Option<Article>, _> = sdb_wg.create("article").content(article).await;
    article_r.unwrap_or_else(|e| {
        error!("Failed to create article: {}", e);
        None
    })
}

pub async fn articles_by_username(
    username: &str,
    limit: u32,
) -> Result<Vec<Article>, DatabaseError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE created_by = $username ORDER BY date DESC LIMIT $limit")
        .bind(("username", username.to_string()))
        .bind(("limit", limit))
        .await?;
    let articles: Vec<Article> = response.take(0)?;
    Ok(articles)
}

pub async fn articles_by_author(username: &str, limit: u32) -> Result<Vec<Article>, DatabaseError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE created_by = $username ORDER BY date DESC LIMIT $limit")
        .bind(("username", username.to_string()))
        .bind(("limit", limit))
        .await?;
    let articles: Vec<Article> = response.take(0)?;
    Ok(articles)
}

pub async fn article_by_file_name(filename: &str) -> Result<Option<Article>, DatabaseError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE article_file_name = $filename")
        .bind(("filename", filename.to_string()))
        .await?;
    let articles: Vec<Article> = response.take(0)?;
    Ok(articles.into_iter().next())
}

// TODO sort the Articles
pub async fn articles_by_category(
    category: &str,
    limit: u32,
) -> Result<Vec<Article>, DatabaseError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE category = $category ORDER BY date DESC LIMIT $limit")
        .bind(("category", category.to_string()))
        .bind(("limit", limit))
        .await?;
    let articles: Vec<Article> = response.take(0)?;
    Ok(articles)
}

pub async fn articles_by_words(
    search_words: Vec<String>,
    limit: u32,
) -> Result<Vec<Article>, DatabaseError> {
    if search_words.is_empty() {
        return Ok(Vec::new());
    }
    let sdb = crate::db::database::db_read().await?;
    // Build WHERE conditions like:
    // string::contains(text, $w0) AND string::contains(text, $w1) ...
    let mut conditions = Vec::new();
    for i in 0..search_words.len() {
        conditions.push(format!("string::contains(text, $w{})", i));
    }
    let query = format!(
        "SELECT * FROM article \
         WHERE {} \
         ORDER BY date DESC \
         LIMIT $limit",
        conditions.join(" AND ")
    );

    let mut q = sdb.query(query);

    for (i, word) in search_words.iter().enumerate() {
        q = q.bind((format!("w{}", i), word.clone()));
    }
    q = q.bind(("limit", limit));

    let mut response = q.await?;
    let articles: Vec<Article> = response.take(0)?;

    Ok(articles)
}
