use crate::db::database_user::{DatabaseUser, Role, User};
use crate::system::logger;
use crate::trust::app::login::login_data::LoginFluent;
use crate::trust::db::db_user_verifier::DatabaseUserVerifier;
use crate::trust::me::TrustError;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

#[derive(Debug)]
pub struct DatabaseUserController {
    dbu: Arc<DatabaseUser>,
}

#[derive(Debug, Clone)]
pub struct SetupUserController {
    dbu: Arc<DatabaseUser>,
    input: LoginFluent,
}

impl DatabaseUserController {
    pub const fn new(dbu: Arc<DatabaseUser>) -> Self {
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

    pub fn setup_user(&self) -> SetupUserController {
        SetupUserController::new(self.dbu.clone())
    }

    pub fn setup_admin_user(&self) -> SetupUserController {
        SetupAdminController::new(self.dbu.clone())
    }

    pub async fn must_see(&self, username: &str) -> Result<DatabaseUserVerifier, TrustError> {
        /*
         * retrieve the real data
         */
        let real_o = self.dbu.get_user_by_name(username).await?;
        real_o.map_or_else(|| Err(TrustError::RealData), |real| Ok(DatabaseUserVerifier::new(real)))
    }
}

impl SetupUserController {
    pub fn new(dbu: Arc<DatabaseUser>) -> Self {
        Self { dbu, input: LoginFluent::new() }
    }

    pub fn username(&self, username: &str) -> &Self {
        self.input.username(username);
        self
    }

    pub fn password(&self, password: &str) -> &Self {
        self.input.password(password);
        self
    }

    pub fn needs_password_change(&self, needs: bool) -> &Self {
        self.input.needs_password_change(needs);
        self
    }

    pub async fn execute(&self) -> Result<(), TrustError> {
        let data = self.input.get_data();
        let username = data.username.unwrap_or_default();
        let password = data.password.unwrap_or_default();

        // db create user
        self.dbu
            .create_user(User {
                username: username.to_string(),
                author_name: username.to_string(),
                password_hash: hash(password, DEFAULT_COST)?,
                needs_password_change: data.needs_password_change,
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

        uc.setup_user().username("tester").password("password").execute().await?;

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

        uc.setup_user().username("tester").password("password").execute().await?;

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
