use crate::db::database::SurrealError;
use crate::db::database_article_data::{
    AccountArticleData, Article, MiniArticleData, ShortArticleData,
};
use regex;

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
             WHERE user = $username \
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
            "SELECT article_file_name, title, short_text, image_288_path, image_desc, date \
            FROM article \
            WHERE article_file_name IN $related \
            ORDER BY date DESC",
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
// TODO only wanted data
pub async fn articles_most_read(limit: u32) -> Result<Vec<MiniArticleData>, SurrealError> {
    let sdb = crate::db::database::db_read().await?;
    let mut response = sdb
        .query("SELECT * FROM article ORDER BY date DESC LIMIT $limit")
        .bind(("limit", limit))
        .await?;
    let most_read_articles: Vec<MiniArticleData> = response.take(0)?;
    Ok(most_read_articles)
}

/*
 * used for
 * - search query in the topbar
 */
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
        conditions.push(format!("string::matches(string::lower(text), $w{})", i));
    }
    /*
     * build search query
     */
    let query = format!(
        "SELECT * FROM article
         WHERE {}
         ORDER BY date DESC
         LIMIT $limit",
        conditions.join(" OR ")
    );
    let mut q = sdb.query(query);
    for (i, word) in search_words.iter().enumerate() {
        let pattern = format!(r"\b{}\b", regex::escape(&word.to_lowercase()));
        q = q.bind((format!("w{}", i), pattern));
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
    use crate::trust::article_easy_builder::ArticleBuilder;
    use crate::trust::script_base::TrustError;

    #[tokio::test]
    async fn test_create_article() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        let a = easy_article("Test Title 1", "user_x", "text");
        create_article(a).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_username() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;
        // prepare user article
        create_article(easy_article("Test Title 1", "userN", "text")).await?;

        let articles = articles_by_username("userN", 100).await?;
        assert_eq!(articles.len(), 1);
        let a = articles.get(0).unwrap();
        assert_eq!(a.title, "Test Title 1");
        Ok(())
    }

    #[tokio::test]
    async fn test_article_by_file_name() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;

        // prepare the article
        create_article(easy_article("Test Title X", "userN", "text")).await?;

        let article_o = article_by_file_name("test-title-x.html").await?;
        assert!(article_o.is_some());
        assert_eq!(article_o.unwrap().title, "Test Title X");
        Ok(())
    }

    #[tokio::test]
    async fn test_related_articles() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;

        // prepare the article
        create_article(easy_article("Related 1", "userB", "text")).await?;
        create_article(easy_article("Related 2", "userC", "text")).await?;

        // execute
        let related_articles =
            related_articles(vec!["related-1.html".into(), "related-2.html".into()]).await?;

        assert_eq!(related_articles.len(), 2);
        // descending order by date
        assert_eq!(related_articles[0].title, "Related 2");
        assert_eq!(related_articles[1].title, "Related 1");
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_category() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;

        // prepare the article
        create_article(
            ArticleBuilder::article()
                .title("Article 1")
                .category("republika")
                .build(),
        )
        .await?;

        let articles = articles_by_category("republika", 100).await?;

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Article 1");
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_most_read() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;

        // prepare the article
        create_article(easy_article("Test Title 7", "userN", "text")).await?;

        let most_red = articles_most_read(100).await?;

        assert_eq!(most_red.len(), 1);
        assert_eq!(most_red[0].title, "Test Title 7");
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_words() -> Result<(), TrustError> {
        initialize_in_memory_database().await?;

        create_article(easy_article("Title 1", "user1", "text abc")).await?;
        create_article(easy_article("Title 2", "user1", "text other")).await?;
        create_article(easy_article("Title 3", "user2", "xyz text")).await?;

        let articles = articles_by_words(vec!["abc.".into(), "XYZ".into()], 100).await?;

        assert_eq!(articles.len(), 2);
        let a1 = articles.get(0).unwrap();
        let a2 = articles.get(1).unwrap();
        assert_eq!(a1.title, "Title 1");
        assert_eq!(a2.title, "Title 3");
        Ok(())
    }
}
