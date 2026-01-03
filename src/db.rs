use surrealdb::engine::any::{connect, Any};
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub needs_password_change: bool,
}

pub struct Database {
    pub client: Surreal<Any>,
}

impl Database {
    pub async fn get_user(&self, username: &str) -> surrealdb::Result<Option<User>> {
        self.client.select(("user", username)).await
    }

    pub async fn update_user(&self, user: User) -> surrealdb::Result<Option<User>> {
        self.client.update(("user", user.username.clone())).content(user).await
    }

    pub async fn create_user(&self, user: User) -> surrealdb::Result<Option<User>> {
        self.client.create(("user", user.username.clone())).content(user).await
    }

    pub async fn has_users(&self) -> bool {
        let users: Vec<User> = self.client.select("user").await.unwrap_or_default();
        !users.is_empty()
    }
}

pub async fn init_db() -> surrealdb::Result<Database> {
    let client = connect("file://axiomatik.db").await?; 
    client.use_ns("axiomatik").use_db("axiomatik").await?;

    let db = Database { client };

    Ok(db)
}
