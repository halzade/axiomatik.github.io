use crate::db::database_user::{DatabaseUser, Role, User};
use crate::trust::me::TrustError;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

#[derive(Debug)]
pub struct DatabaseUserController {
    dbu: Arc<DatabaseUser>,
}

impl DatabaseUserController {
    pub fn new(dbu: Arc<DatabaseUser>) -> Self {
        Self { dbu }
    }

    pub fn execute(self) -> Result<(), TrustError> {
        // TODO response

        Ok(())
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
