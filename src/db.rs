use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Role {
    Editor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub author_name: String,
    pub password_hash: String,
    pub needs_password_change: bool,
    pub role: Role,
}

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
    pub category: String,
    pub related_articles: String,
    pub views: i64,
}

pub struct Database {
    pub db: RwLock<Surreal<Any>>,
}

impl Database {
    pub async fn new(url: &str) -> Self {
        let client = connect(url).await;
        client.unwrap().use_ns("axiomatik").use_db("axiomatik").await;
        Self {
            db: RwLock::new(client.unwrap())
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<Surreal<Any>> {
        self.db.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<Surreal<Any>> {
        self.db.write().await
    }

    pub async fn get_user(&self, username: &str) -> surrealdb::Result<Option<User>> {
        self.db.read().await.select(("user", username)).await
    }

    pub async fn update_user(&self, user: User) -> surrealdb::Result<Option<User>> {
        self.db
            .write()
            .await
            .update(("user", user.username.clone()))
            .content(user)
            .await
    }

    pub async fn create_user(&self, user: User) -> surrealdb::Result<Option<User>> {
        self.db
            .write()
            .await
            .create(("user", user.username.clone()))
            .content(user)
            .await
    }

    pub async fn delete_user(&self, username: &str) -> surrealdb::Result<()> {
        self.db.write().await.delete(("user", username));
        Ok(())
    }

    pub async fn has_users(&self) -> bool {
        // TODO simplify
        let users: Vec<User> = self
            .db
            .read()
            .await
            .select("user")
            .await
            .unwrap_or_default();
        !users.is_empty()
    }

    pub async fn create_article(&self, article: Article) -> surrealdb::Result<Option<Article>> {
        self.db
            .write()
            .await
            .create("article")
            .content(article)
            .await
    }

    pub async fn get_articles_by_username(
        &self,
        username: &str,
    ) -> surrealdb::Result<Vec<Article>> {
        let mut response = self
            .db
            .read
            .await
            .query("SELECT * FROM article WHERE created_by = $username ORDER BY date DESC")
            .bind(("username", username.to_string()))
            .await?;
        response.take(0)
    }

    pub async fn increment_article_views(&self, file_name: &str) -> surrealdb::Result<i64> {
        let mut response = self
            .db
            .read()
            .await
            .query(
                "UPDATE article SET views += 1 WHERE article_file_name = $file_name RETURN views",
            )
            .bind(("file_name", file_name.to_string()))
            .await?;
        let views: Option<i64> = response.take("views")?;
        Ok(views.unwrap_or(0))
    }
}

pub async fn init_mem_db() -> Database {
    Database::new("mem://").await
}

pub async fn init_db() -> Database {
    Database::new("rocksdb://axiomatik.db").await
}
