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

pub struct Database {
    pub client: Surreal<Any>,
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

    pub async fn delete_user(&self, username: &str) -> surrealdb::Result<Option<User>> {
        self.client.delete(("user", username)).await
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
