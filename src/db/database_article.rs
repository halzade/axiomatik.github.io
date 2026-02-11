use crate::db::database;
use crate::db::database::{DatabaseSurreal, SurrealError};
use crate::db::database_article::SurrealArticleError::ArticleNotFound;
use crate::db::database_article_data::{
    AccountArticleData, Article, MainArticleData, MiniArticleData, ShortArticleData, TopArticleData,
};
use crate::db::database_system::{ArticleViews, SurrealSystemError};
use regex;
use serde::{Deserialize, Serialize};
use std::convert::{Infallible, Into};
use std::string::ToString;
use std::sync::Arc;
use surrealdb::types::SurrealValue;
use thiserror::Error;
use tracing::log::debug;

const ARTICLE: &str = "article";

#[derive(Debug, Error)]
pub enum SurrealArticleError {
    #[error("surreal db error {0}")]
    Surreal(#[from] surrealdb::Error),

    #[error("article not found {0}")]
    ArticleNotFound(String),

    // TODO never throw Infallible
    #[error("article infallible {0}")]
    ArticleInfallible(#[from] Infallible),
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
        debug!("article_by_file_name: article_file_name={}", article_file_name);

        let article_o: Option<Article> =
            self.surreal.db.select((ARTICLE, article_file_name.to_string())).await?;
        match article_o {
            None => Err(ArticleNotFound(article_file_name.into())),
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

    pub async fn article_top_three(
        &self,
    ) -> Result<(MainArticleData, TopArticleData, TopArticleData), SurrealArticleError> {
        let mut query_response_set = self
            .surreal
            .db
            .query("SELECT * FROM article WHERE is_main = true ORDER BY date LIMIT 3")
            .await?;
        let mut top_articles: Vec<MainArticleData> = query_response_set.take(0)?;
        let main: MainArticleData = top_articles.pop().unwrap_or(MainArticleData::empty());
        let second_m: MainArticleData = top_articles.pop().unwrap_or(MainArticleData::empty());
        let third_m: MainArticleData = top_articles.pop().unwrap_or(MainArticleData::empty());

        Ok((main, TopArticleData::try_from(second_m)?, TopArticleData::try_from(third_m)?))
    }

    pub async fn articles_most_read(
        &self,
        most_read_vec: Vec<ArticleViews>,
    ) -> Result<Vec<MiniArticleData>, SurrealArticleError> {
        debug!("articles_most_read amount={:?}, {:?}", most_read_vec.len(), most_read_vec);

        let mut response = self
            .surreal
            .db
            .query("SELECT * FROM $article_file_name")
            .bind(("$article_file_name", most_read_vec))
            .await?;
        let most_read_articles: Vec<MiniArticleData> = response.take(0)?;
        Ok(most_read_articles)
    }

    pub async fn most_read_by_views(&self) -> Result<Vec<MiniArticleData>, SurrealSystemError> {
        let mut response = self
            .surreal
            .db
            .query("SELECT type::record('article', article_file_name) AS article, views FROM article_views ORDER BY views DESC LIMIT 3 FETCH article")
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
    use crate::db::database_system::{ArticleViews, DatabaseSystem};
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

        let av = ArticleViews { article_file_name: "test-title-7.html".to_string(), views: 1 };

        let most_red = db.articles_most_read(vec![av]).await?;

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

    #[tokio::test]
    async fn test_most_read_article_one() -> Result<(), TrustError> {
        let dbs = DatabaseSystem::new_from_scratch().await?;
        let dba = DatabaseArticle::new_from_scratch().await?;

        let article_1 = "test-11.html".to_string();

        dba.create_article(easy_article("Test 11", "user AA1", "text")).await?;
        dbs.increase_article_views(article_1.clone()).await?;
        let most_read = dba.most_read_by_views().await?;

        assert_eq!(most_read.len(), 1);
        assert_eq!(most_read[0].article_file_name, article_1);

        Ok(())
    }

    #[tokio::test]
    async fn test_most_read_article_none() -> Result<(), TrustError> {
        let dbs = DatabaseSystem::new_from_scratch().await?;
        let dba = DatabaseArticle::new_from_scratch().await?;

        let most_read = dba.most_read_by_views().await?;
        assert_eq!(most_read.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_most_read_article() -> Result<(), TrustError> {
        let dbs = DatabaseSystem::new_from_scratch().await?;
        let dba = DatabaseArticle::new_from_scratch().await?;

        let article_1 = "test-1.html".to_string();
        let article_2 = "test-2.html".to_string();
        let article_3 = "test-3.html".to_string();
        let article_4 = "test-4.html".to_string();
        let article_5 = "test-5.html".to_string();

        dba.create_article(easy_article("Test 1", "user A1", "text")).await?;
        dba.create_article(easy_article("Test 2", "user A2", "text")).await?;
        dba.create_article(easy_article("Test 3", "user A3", "text")).await?;
        dba.create_article(easy_article("Test 4", "user A4", "text")).await?;
        dba.create_article(easy_article("Test 5", "user A5", "text")).await?;

        dbs.increase_article_views(article_1.clone()).await?;

        dbs.increase_article_views(article_2.clone()).await?;
        dbs.increase_article_views(article_2.clone()).await?;

        dbs.increase_article_views(article_3.clone()).await?;
        dbs.increase_article_views(article_3.clone()).await?;
        dbs.increase_article_views(article_3.clone()).await?;

        dbs.increase_article_views(article_4.clone()).await?;
        dbs.increase_article_views(article_4.clone()).await?;
        dbs.increase_article_views(article_4.clone()).await?;
        dbs.increase_article_views(article_4.clone()).await?;

        dbs.increase_article_views(article_5.clone()).await?;

        let most_read = dba.most_read_by_views().await?;
        assert_eq!(most_read.len(), 3);
        assert_eq!(most_read[0].article_file_name, article_4);
        assert_eq!(most_read[1].article_file_name, article_3);
        assert_eq!(most_read[2].article_file_name, article_2);

        Ok(())
    }
}
