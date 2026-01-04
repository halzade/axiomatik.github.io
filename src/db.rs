use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};

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

use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Database {
    pub client: Surreal<Any>,
}

pub struct WrapDb {
    pub db: RwLock<Option<Database>>,
}

impl WrapDb {
    pub fn new() -> Self {
        Self {
            db: RwLock::new(None),
        }
    }

    pub async fn get_db(&self) -> Database {
        let read_guard = self.db.read().await;
        if let Some(db) = &*read_guard {
            return db.clone();
        }
        drop(read_guard);

        let mut write_guard = self.db.write().await;
        if let Some(db) = &*write_guard {
            return db.clone();
        }

        let db = init_db().await.expect("Failed to initialize database");
        *write_guard = Some(db.clone());
        db
    }
}

pub static DB_INSTANCE: std::sync::OnceLock<WrapDb> = std::sync::OnceLock::new();

pub fn get_wrap_db() -> &'static WrapDb {
    DB_INSTANCE.get_or_init(WrapDb::new)
}

impl Database {
    pub async fn get_user(&self, username: &str) -> surrealdb::Result<Option<User>> {
        self.client.select(("user", username)).await
    }

    pub async fn update_user(&self, user: User) -> surrealdb::Result<Option<User>> {
        self.client
            .update(("user", user.username.clone()))
            .content(user)
            .await
    }

    pub async fn create_user(&self, user: User) -> surrealdb::Result<Option<User>> {
        self.client
            .create(("user", user.username.clone()))
            .content(user)
            .await
    }

    pub async fn delete_user(&self, username: &str) -> surrealdb::Result<()> {
        let _: Option<User> = self.client.delete(("user", username)).await?;
        Ok(())
    }

    pub async fn has_users(&self) -> bool {
        let users: Vec<User> = self.client.select("user").await.unwrap_or_default();
        !users.is_empty()
    }
}

pub async fn init_db() -> surrealdb::Result<Database> {
    init_db_with_url("file://axiomatik.db").await
}

pub async fn init_mem_db() -> surrealdb::Result<Database> {
    init_db_with_url("mem://").await
}

async fn init_db_with_url(url: &str) -> surrealdb::Result<Database> {
    let client = connect(url).await?;
    client.use_ns("axiomatik").use_db("axiomatik").await?;

    let db = Database { client };

    Ok(db)
}
