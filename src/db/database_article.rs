use crate::db::database::SurrealError;
use crate::db::database_article_data::{
    AccountArticleData, Article, MiniArticleData, ShortArticleData,
};

pub async fn create_article(article: Article) -> Result<(), SurrealError> {
    let sdb = crate::db::database::db_write().await?;

    // TODO Id, file name won't work for requests, need uuid.
    let _: Option<Article> = sdb.create("article").content(article).await?;
    Ok(())
}

/**
 * used for
 * - articles on the account page
 */
pub async fn articles_by_username(
    username: &str,
    limit: u32,
) -> Result<Vec<AccountArticleData>, SurrealError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query(
            "SELECT * FROM article \
             WHERE created_by = $username \
             ORDER BY date DESC \
             LIMIT $limit",
        )
        .bind(("username", username.to_string()))
        .bind(("limit", limit))
        .await?;
    let account_articles: Vec<AccountArticleData> = response.take(0)?;
    Ok(account_articles)
}

/**
 * used for
 * - rendering Article template
 */
pub async fn article_by_file_name(filename: &str) -> Result<Option<Article>, SurrealError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE article_file_name = $filename")
        .bind(("filename", filename.to_string()))
        .await?;
    let article_o: Option<Article> = response.take(0)?;
    Ok(article_o)
}

/**
 * used for
 * - related articles on the Article page
 */
pub async fn related_articles(related: Vec<String>) -> Result<Vec<ShortArticleData>, SurrealError> {
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
    let short_article_data: Vec<ShortArticleData> = response.take(0)?;
    Ok(short_article_data)
}

pub async fn articles_by_category(
    category: &str,
    limit: u32,
) -> Result<Vec<ShortArticleData>, SurrealError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE category = $category ORDER BY date DESC LIMIT $limit")
        .bind(("category", category.to_string()))
        .bind(("limit", limit))
        .await?;
    let category_articles: Vec<ShortArticleData> = response.take(0)?;
    Ok(category_articles)
}

// TODO X actually most read
pub async fn articles_most_read(limit: u32) -> Result<Vec<MiniArticleData>, SurrealError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article WHERE ORDER BY date DESC LIMIT $limit")
        .bind(("limit", limit))
        .await?;
    let most_read_articles: Vec<MiniArticleData> = response.take(0)?;
    Ok(most_read_articles)
}

pub async fn articles_by_words(
    search_words: Vec<String>,
    limit: u32,
) -> Result<Vec<ShortArticleData>, SurrealError> {
    if search_words.is_empty() {
        return Ok(Vec::new());
    }
    let sdb = crate::db::database::db_read().await?;

    /*
     * build search condition
     */
    let mut conditions = Vec::new();
    for i in 0..search_words.len() {
        conditions.push(format!("string::contains(text, $w{})", i));
    }
    /*
     * build search query
     */
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

    /*
     * execute search query
     */
    let mut response = q.await?;
    let matching_articles: Vec<ShortArticleData> = response.take(0)?;
    Ok(matching_articles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::database::initialize_in_memory_database;
    use crate::trust::article_builder::easy_article;
    use crate::trust::script_base::TrustError;

    #[tokio::test]
    async fn test_create_article() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        let a = easy_article("user_x", "Test Title 1");
        create_article(a).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_username() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        let articles = articles_by_username("user_x", 100).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_article_by_file_name() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        let article_o = article_by_file_name("file").await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_related_articles() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        let articles = related_articles(vec!["file".into()]).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_category() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        let articles = articles_by_category("category", 100).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_articles_most_read() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        let a = articles_most_read(100).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_words() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        let articles = articles_by_words(vec!["one.".into(), "two".into()], 100).await?;

        Ok(())
    }
}
