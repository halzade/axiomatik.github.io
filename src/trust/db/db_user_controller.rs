use crate::db::database_user::{DatabaseUser, Role, User};
use crate::system::logger;
use crate::trust::db::db_user_verifier::DatabaseUserVerifier;
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

    /*
     * only for local tests
     */
    pub async fn new_local() -> Result<Self, TrustError> {
        logger::config();
        let dbu = Arc::new(DatabaseUser::new_from_scratch().await?);
        Ok(Self { dbu })
    }

    pub async fn must_see(&self, username: &str) -> Result<DatabaseUserVerifier, TrustError> {
        /*
         * retrieve the real data
         */
        let real_o = self.dbu.get_user_by_name(username).await?;
        match real_o {
            Some(real) => {
                // build verifier
                Ok(DatabaseUserVerifier::new(real))
            }
            None => Err(TrustError::RealData),
        }
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

#[cfg(test)]
mod tests {
    use crate::db::database_user::Role;
    use crate::trust::db::db_user_controller::DatabaseUserController;
    use crate::trust::me::TrustError;
    use Role::Editor;

    #[tokio::test]
    async fn test_user_verifier_pass() -> Result<(), TrustError> {
        let uc = DatabaseUserController::new_local().await?;

        uc.db_setup_user_with_password("tester", "password").await?;

        #[rustfmt::skip]
        uc.must_see("tester").await?
            .username("tester")
            .author_name("tester")
            .role(Editor)
            .verify()?;

        Ok(())
    }

    #[tokio::test]
    async fn test_user_verifier_fail() -> Result<(), TrustError> {
        let uc = DatabaseUserController::new_local().await?;

        uc.db_setup_user_with_password("tester", "password").await?;

        #[rustfmt::skip]
        let err = uc.must_see("tester").await?
            .username("testerX")
            .author_name("testerY")
            .role(Editor)
            .verify();

        assert!(err.is_err());
        assert_eq!(err.unwrap_err().to_string(), "validation error: 2 incorrect");

        Ok(())
    }
}
