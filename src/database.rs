use crate::database_internal;
use crate::database_internal::DatabaseSurreal;
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::{Response, Surreal};
use tokio::sync::{OnceCell, RwLockReadGuard, RwLockWriteGuard};

static DATABASE: OnceCell<DatabaseSurreal> = OnceCell::const_new();

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
    pub audio_url: Option<String>,
    pub category: String,
    pub related_articles: String,
    pub views: i64,
}

pub async fn update_user(user: User) -> Option<User> {
    db_write()
        .await
        .update(("user", user.username.clone()))
        .content(user)
        .await
        .unwrap()
}

pub async fn create_user(user: User) -> Option<User> {
    db_write()
        .await
        .create(("user", user.username.clone()))
        .content(user)
        .await
        .unwrap()
}

pub async fn delete_user(user_name: &str) {
    let _: Result<Option<User>, _> = db_write().await.delete(("user", user_name)).await;
}

pub async fn has_users() -> bool {
    let response = db_read().await.query("SELECT count() FROM user").await.ok();

    if let Some(mut response) = response {
        let count: Option<i64> = response.take(0).unwrap_or_default();
        count.unwrap_or(0) > 0
    } else {
        false
    }
}

pub async fn create_article(article: Article) -> Option<Article> {
    db_write()
        .await
        .create("article")
        .content(article)
        .await
        .unwrap()
}

pub async fn get_articles_by_username(username: &str) -> Vec<Article> {
    // TODO
    db_read()
        .await
        .query("SELECT * FROM article WHERE created_by = $username ORDER BY date DESC")
        .bind(("username", username.to_string()))
        .await
        .unwrap()
        .take(0)
        .unwrap()
}

pub async fn get_all_articles() -> Vec<Article> {
    // TODO don't need all article data
    db_read().await.select("article").await.unwrap()
}

pub async fn get_user(user_name: &str) -> Option<User> {
    db_read().await.select(("user", user_name)).await.unwrap()
}

pub async fn query(query: String) -> Response {
    db_read().await.query(query).await.unwrap()
}

/*
 * Technical Methods
 */

pub async fn initialize_database() {
    DATABASE.get_or_init(database_internal::init_db).await;
}

pub async fn initialize_in_memory_database() {
    DATABASE.get_or_init(database_internal::init_mem_db).await;
}

async fn db_read<'lt>() -> RwLockReadGuard<'lt, Surreal<Any>> {
    DATABASE.get().unwrap().db.read().await
}

async fn db_write<'lt>() -> RwLockWriteGuard<'lt, Surreal<Any>> {
    DATABASE.get().unwrap().db.write().await
}
