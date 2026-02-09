use crate::db::database;
use crate::db::database::{DatabaseSurreal, SurrealError};
use crate::db::database_article::SurrealArticleError::ArticleNotFound;
use crate::db::database_article_data::{
    AccountArticleData, Article, MiniArticleData, ShortArticleData,
};
use regex;
use std::convert::Into;
use std::string::ToString;
use std::sync::Arc;
use thiserror::Error;
use tracing::log::debug;

const ARTICLE: &str = "article";

#[derive(Debug, Error)]
pub enum SurrealArticleError {
    #[error("surreal db error: {0}")]
    Surreal(#[from] surrealdb::Error),

    #[error("article not found: {0}")]
    ArticleNotFound(String),
}

/**
 * access to a database
 * - anything article related
 */
#[derive(Debug)]
pub struct DatabaseArticle {
    surreal: Arc<DatabaseSurreal>,
}

impl DatabaseArticle {
    pub fn new(db: Arc<DatabaseSurreal>) -> DatabaseArticle {
        DatabaseArticle { surreal: db }
    }

    pub async fn new_from_scratch() -> Result<DatabaseArticle, SurrealError> {
        let surreal = Arc::new(database::init_in_memory_db_connection().await?);
        Ok(DatabaseArticle { surreal })
    }

    // TODO Id, file name won't work for requests, need uuid.
    pub async fn create_article(&self, article: Article) -> Result<(), SurrealArticleError> {
        debug!("create_article: {:?}", article);
        let _: Option<Article> = self
            .surreal
            .db
            .create((ARTICLE, article.article_file_name.clone()))
            .content(article)
            .await?;
        Ok(())
    }

    /**
     * used for
     * - articles on the account page
     */
    pub async fn articles_by_username(
        &self,
        username: &str,
        limit: u32,
    ) -> Result<Vec<AccountArticleData>, SurrealArticleError> {
        debug!("articles_by_username: username={}, limit={}", username, limit);

        let mut response = self
            .surreal
            .db
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
     * - article must always exist if article_status is Invalid
     */
    pub async fn article_by_file_name(
        &self,
        article_file_name: &str,
    ) -> Result<Article, SurrealArticleError> {
        let real_filename = article_file_name.strip_prefix('/').unwrap_or(&article_file_name);
        debug!("article_by_file_name: article_file_name={}", real_filename);

        let article_o: Option<Article> =
            self.surreal.db.select((ARTICLE, real_filename.to_string())).await?;
        match article_o {
            None => Err(ArticleNotFound(real_filename.into())),
            Some(article) => Ok(article),
        }
    }

    /**
     * used for
     * - related articles on the Article page
     */
    pub async fn related_articles(
        &self,
        related: Vec<String>,
    ) -> Result<Vec<ShortArticleData>, SurrealArticleError> {
        debug!("related_articles: related={:?}", related);

        if related.is_empty() {
            return Ok(Vec::new());
        }
        let mut response = self
            .surreal
            .db
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
        &self,
        category: &str,
        limit: u32,
    ) -> Result<Vec<ShortArticleData>, SurrealArticleError> {
        let mut response = self
            .surreal
            .db
            .query(
                "SELECT * FROM article WHERE category = $category ORDER BY date DESC LIMIT $limit",
            )
            .bind(("category", category.to_string()))
            .bind(("limit", limit))
            .await?;
        let category_articles: Vec<ShortArticleData> = response.take(0)?;
        Ok(category_articles)
    }

    // TODO X actually most read
    // TODO only wanted data
    pub async fn articles_most_read(
        &self,
        limit: u32,
    ) -> Result<Vec<MiniArticleData>, SurrealArticleError> {
        debug!("articles_most_read: limit={}", limit);

        let mut response = self
            .surreal
            .db
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
        &self,
        search_words: Vec<String>,
        limit: u32,
    ) -> Result<Vec<ShortArticleData>, SurrealArticleError> {
        debug!("articles_by_words: search_words={:?}, limit={}", search_words, limit);

        if search_words.is_empty() {
            return Ok(Vec::new());
        }

        /*
         * build search condition
         */
        let mut conditions = Vec::new();
        for i in 0..search_words.len() {
            conditions.push(format!("string::matches(text, $w{})", i));
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
        let mut q = self.surreal.db.query(query);
        for (i, word) in search_words.iter().enumerate() {
            let pattern = format!(r"(?i)\b{}\b", regex::escape(word));
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
}

#[cfg(test)]
mod tests {
    use crate::db::database_article::DatabaseArticle;
    use crate::trust::app::article::create_article_request_builder::easy_article;
    use crate::trust::me::TrustError;

    #[tokio::test]
    async fn test_create_article() -> Result<(), TrustError> {
        let db = DatabaseArticle::new_from_scratch().await?;

        let a = easy_article("Test Title 1", "user_x", "text");
        db.create_article(a).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_username() -> Result<(), TrustError> {
        let db = DatabaseArticle::new_from_scratch().await?;
        // prepare user article
        db.create_article(easy_article("Test Title 1", "user_xx", "text")).await?;

        let articles = db.articles_by_username("user_xx", 100).await?;
        assert_eq!(articles.len(), 1);
        let a = articles.get(0).unwrap();
        assert_eq!(a.title, "Test Title 1");
        Ok(())
    }

    #[tokio::test]
    async fn test_article_by_file_name() -> Result<(), TrustError> {
        let db = DatabaseArticle::new_from_scratch().await?;

        // prepare the article
        db.create_article(easy_article("Test Title X", "userN", "text")).await?;

        let article_o = db.article_by_file_name("test-title-x.html").await?;
        assert_eq!(article_o.title, "Test Title X");
        Ok(())
    }

    #[tokio::test]
    async fn test_related_articles() -> Result<(), TrustError> {
        let db = DatabaseArticle::new_from_scratch().await?;

        // prepare the article
        db.create_article(easy_article("Related 1", "userB", "text")).await?;
        db.create_article(easy_article("Related 2", "userC", "text")).await?;

        // execute
        let related_articles =
            db.related_articles(vec!["related-1.html".into(), "related-2.html".into()]).await?;

        assert_eq!(related_articles.len(), 2);
        // descending order by date
        assert_eq!(related_articles[0].title, "Related 2");
        assert_eq!(related_articles[1].title, "Related 1");
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_category() -> Result<(), TrustError> {
        let db = DatabaseArticle::new_from_scratch().await?;

        // TODO
        // prepare the article
        // db.create_article(
        //     ArticleBuilder::article().title("Article 1").category("republika").build(),
        // )
        // .await?;

        let articles = db.articles_by_category("republika", 100).await?;

        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Article 1");
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_most_read() -> Result<(), TrustError> {
        let db = DatabaseArticle::new_from_scratch().await?;

        // prepare the article
        db.create_article(easy_article("Test Title 7", "userN", "text")).await?;

        let most_red = db.articles_most_read(1).await?;

        assert_eq!(most_red.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_articles_by_words() -> Result<(), TrustError> {
        let db = DatabaseArticle::new_from_scratch().await?;

        db.create_article(easy_article("Title 1", "user1", "text abc")).await?;
        db.create_article(easy_article("Title 2", "user1", "text other")).await?;
        db.create_article(easy_article("Title 3", "user2", "xyz text")).await?;

        let articles = db.articles_by_words(vec!["abc".into(), "XYZ".into()], 100).await?;

        assert_eq!(articles.len(), 2);
        // descending order by date
        let a1 = articles.get(0).unwrap();
        let a2 = articles.get(1).unwrap();
        assert_eq!(a1.title, "Title 3");
        assert_eq!(a2.title, "Title 1");
        Ok(())
    }
}
