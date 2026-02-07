use crate::db::database;
use crate::db::database_article::DatabaseArticle;
use crate::db::database_system::DatabaseSystem;
use crate::db::database_user::{DatabaseUser, Role, User};
use crate::trust::me::TrustError;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

pub struct NexoDb {
    pub db_article: Arc<DatabaseArticle>,
    pub db_system: Arc<DatabaseSystem>,
    pub db_user: Arc<DatabaseUser>,
}

impl NexoDb {
    pub async fn new() -> Result<NexoDb, TrustError> {
        
        let db_article = DatabaseArticle::new(db_a.clone());
        let db_system = DatabaseSystem::new(db_a.clone());
        let db_user = DatabaseUser::new(db_a.clone());
        Ok(NexoDb {
            db_article: Arc::new(db_article),
            db_system: Arc::new(db_system),
            db_user: Arc::new(db_user),
        })
    }

    pub async fn db_setup_user(&self, username: &str) -> Result<(), TrustError> {
        // db create user
        self.db_user
            .create_user(User {
                username: username.to_string(),
                author_name: username.to_string(),
                password_hash: hash("password", DEFAULT_COST)?,
                needs_password_change: false,
                role: Role::Editor,
            })
            .await?;
        Ok(())
    }
}
