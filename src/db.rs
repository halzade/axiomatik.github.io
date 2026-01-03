use surrealdb::engine::any::{connect, Any};
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use bcrypt::{hash, DEFAULT_COST};

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
}

pub async fn init_db() -> surrealdb::Result<Database> {
    let client = connect("mem://").await?; // Using memory for now, or you can use "file://axiomatik.db"
    client.use_ns("axiomatik").use_db("axiomatik").await?;

    let db = Database { client };

    // Initialize default user if not exists
    if db.get_user("admin").await?.is_none() {
        let default_password = std::fs::read_to_string("default_password.txt")
            .expect("default_password.txt must be present")
            .trim()
            .to_string();
        let password_hash = hash(default_password, DEFAULT_COST).unwrap();
        let default_user = User {
            username: "admin".to_string(),
            password_hash,
            needs_password_change: true,
        };
        let _: Option<User> = db.client.create(("user", "admin")).content(default_user).await?;
        println!("Default admin user created");
    }

    Ok(db)
}
