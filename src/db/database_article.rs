use crate::db::database::DatabaseError;
use crate::db::database_article_data::{Article, MiniArticleData, ShortArticleData};
use tracing::error;

// TODO

pub async fn create_article(article: Article) -> Option<Article> {
    let sdb_wg = crate::db::database::db_write().await.ok()?;
    let res = sdb_wg
        .create("article")
        .content(serde_json::to_value(&article).ok()?)
        .await;
    match res {
        Ok(Some(v)) => serde_json::from_value::<Article>(v).ok(),
        Ok(None) => None,
        Err(e) => {
            error!("Failed to create article: {}", e);
            None
        }
    }
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
    let vals: Vec<serde_json::Value> = response.take(0)?;
    let mut articles = Vec::with_capacity(vals.len());
    for v in vals {
        if let Ok(a) = serde_json::from_value::<Article>(v) {
            articles.push(a);
        }
    }
    Ok(articles)
}

pub async fn articles_by_author(username: &str, limit: u32) -> Result<Vec<Article>, DatabaseError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE created_by = $username ORDER BY date DESC LIMIT $limit")
        .bind(("username", username.to_string()))
        .bind(("limit", limit))
        .await?;
    let vals: Vec<serde_json::Value> = response.take(0)?;
    let mut articles = Vec::with_capacity(vals.len());
    for v in vals {
        if let Ok(a) = serde_json::from_value::<Article>(v) {
            articles.push(a);
        }
    }
    Ok(articles)
}

pub async fn article_by_file_name(filename: &str) -> Result<Option<Article>, DatabaseError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE article_file_name = $filename")
        .bind(("filename", filename.to_string()))
        .await?;
    let vals: Vec<serde_json::Value> = response.take(0)?;
    for v in vals {
        if let Ok(a) = serde_json::from_value::<Article>(v) {
            return Ok(Some(a));
        }
    }
    Ok(None)
}

pub async fn related_articles(related: &[String]) -> Result<Vec<ShortArticleData>, DatabaseError> {
    if related.is_empty() {
        return Ok(Vec::new());
    }
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query(
            "SELECT id, title, article_file_name, summary
             FROM article
             WHERE article_file_name IN $related",
        )
        .bind(("related", related.to_vec()))
        .await?;
    let vals: Vec<serde_json::Value> = response.take(0)?;
    let mut articles = Vec::with_capacity(vals.len());
    for v in vals {
        if let Ok(a) = serde_json::from_value::<ShortArticleData>(v) {
            articles.push(a);
        }
    }
    Ok(articles)
}

pub async fn articles_by_category(
    category: &str,
    limit: u32,
) -> Result<Vec<ShortArticleData>, DatabaseError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE category = $category ORDER BY date DESC LIMIT $limit")
        .bind(("category", category.to_string()))
        .bind(("limit", limit))
        .await?;
    let vals: Vec<serde_json::Value> = response.take(0)?;
    let mut articles = Vec::with_capacity(vals.len());
    for v in vals {
        if let Ok(a) = serde_json::from_value::<ShortArticleData>(v) {
            articles.push(a);
        }
    }
    Ok(articles)
}

// TODO X actually most read
pub async fn articles_most_read(limit: u32) -> Result<Vec<MiniArticleData>, DatabaseError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE ORDER BY date DESC LIMIT $limit")
        .bind(("limit", limit))
        .await?;
    let vals: Vec<serde_json::Value> = response.take(0)?;
    let mut articles = Vec::with_capacity(vals.len());
    for v in vals {
        if let Ok(a) = serde_json::from_value::<MiniArticleData>(v) {
            articles.push(a);
        }
    }
    Ok(articles)
}

pub async fn articles_by_words(
    search_words: Vec<String>,
    limit: u32,
) -> Result<Vec<ShortArticleData>, DatabaseError> {
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
    let vals: Vec<serde_json::Value> = response.take(0)?;
    let mut articles = Vec::with_capacity(vals.len());
    for v in vals {
        if let Ok(a) = serde_json::from_value::<ShortArticleData>(v) {
            articles.push(a);
        }
    }

    Ok(articles)
}
