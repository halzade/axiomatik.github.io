use crate::db::database_article::DatabaseArticle;
use crate::db::database_system::DatabaseSystem;
use crate::db::database_user::{DatabaseUser, Role, User};
use crate::system::server::TheState;
use crate::trust::me::TrustError;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

impl NexoDb {

    pub async fn db_setup_user(&self, username: &str) -> Result<(), TrustError> {
        self.db_setup_user_with_password(username, "password").await
    }

    pub async fn db_setup_user_with_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), TrustError> {
        // db create user
        self.dbu
            .create_user(User {
                username: username.to_string(),
                author_name: username.to_string(),
                password_hash: hash(password, DEFAULT_COST)?,
                needs_password_change: false,
                role: Role::Editor,
            })
            .await?;
        Ok(())
    }
}
