use crate::database_internal;
use crate::database_internal::DatabaseSurreal;
use anyhow::Error;
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::{Response, Surreal};
use tokio::sync::{OnceCell, RwLockReadGuard, RwLockWriteGuard};
use tracing::error;

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
    let sdb_r = db_write().await;
    match sdb_r {
        Ok(sdb) => sdb
            .update(("user", user.username.clone()))
            .content(user)
            .await
            .unwrap(),
        Err(_) => None,
    }
}

pub async fn create_user(user: User) -> Option<User> {
    let sdb_r = db_write().await;
    match sdb_r {
        Ok(sdb) => sdb
            .create(("user", user.username.clone()))
            .content(user)
            .await
            .unwrap(),
        Err(_) => None,
    }
}
pub async fn delete_user(user_name: &str) {
    if let Ok(sdb) = db_write().await {
        let _: Result<Option<surrealdb::sql::Value>, surrealdb::Error> =
            sdb.delete(("user", user_name)).await;
    } else {
        error!("Database not available");
    }
}

pub async fn has_users() -> bool {
    if let Ok(sdb) = db_read().await {
        let response_r = sdb.query("SELECT count() FROM user").await;
        return if let Ok(mut response) = response_r {
            let count: Option<i64> = response.take(0).unwrap_or_default();
            count.unwrap_or(0) > 0
        } else {
            error!("Database not available");
            false
        }
    }
    error!("Database error");
    false
}

pub async fn create_article(article: Article) -> Option<Article> {
    let sdb_r = db_write().await;
    match sdb_r {
        Ok(sdb) => sdb.create("article").content(article).await.unwrap(),
        _ => None,
    }
}

pub async fn get_articles_by_username(username: &str) -> Option<Vec<Article>> {
    if let Ok(sdb) = db_read().await {
        let response_r = sdb
            .query("SELECT * FROM article WHERE created_by = $username ORDER BY date DESC")
            .bind(("username", username.to_string()))
            .await;
        if let Ok(mut response) = response_r {
            return response.take(0).unwrap();
        }
    }
    None
}

// TODO limit to like a 1000 lates articles or something like that
pub async fn get_all_articles() -> Option<Vec<Article>> {
    let sdb_r = db_read().await;
    match sdb_r {
        Ok(sdb) => Some(sdb.select("article").await.unwrap()),
        _ => None,
    }
}

pub async fn get_user(user_name: &str) -> Option<User> {
    if let Ok(sdb) = db_read().await {
        return sdb.select(("user", user_name)).await.unwrap();
    }
    None
}

pub async fn query(query: String) -> Response {
    let sdb = db_read().await.expect("Database not initialized");
    sdb.query(query).await.expect("Query failed")
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

async fn db_read<'lt>() -> Result<RwLockReadGuard<'lt, Surreal<Any>>, Error> {
    let sdb = DATABASE
        .get()
        .ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
    Ok(sdb.db.read().await)
}

async fn db_write<'lt>() -> Result<RwLockWriteGuard<'lt, Surreal<Any>>, Error> {
    let sdb = DATABASE
        .get()
        .ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
    Ok(sdb.db.write().await)
}
