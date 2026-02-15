use crate::db::database;
use crate::db::database_article::DatabaseArticle;
use crate::db::database_system::DatabaseSystem;
use crate::db::database_user::DatabaseUser;
use crate::system::server::TheState;
use crate::system::{configuration, data_system, data_updates, logger, server};
use crate::trust::app::account::account_controller::AccountController;
use crate::trust::app::admin::admin_controller::AdminController;
use crate::trust::app::article::create_article_controller::CreateArticleController;
use crate::trust::app::change_password::change_password_controller::ChangePasswordController;
use crate::trust::app::login::login_controller::LoginController;
use crate::trust::db::db_article_controller::DatabaseArticleController;
use crate::trust::db::db_system_controller::DatabaseSystemController;
use crate::trust::db::db_user_controller::DatabaseUserController;
use crate::trust::me::TrustError;
use crate::trust::web::auth_web_controller::AuthorizedWebController;
use crate::trust::web::web_controller::WebController;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug)]
pub struct AppController {
    // app
    account: Arc<AccountController>,
    admin: Arc<AdminController>,
    article: Arc<CreateArticleController>,
    change_password: Arc<ChangePasswordController>,
    login: Arc<LoginController>,
    web_auth: Arc<AuthorizedWebController>,
    // web
    web: Arc<WebController>,
    // db
    db_article_controller: Arc<DatabaseArticleController>,
    db_user_controller: Arc<DatabaseUserController>,
    db_system_controller: Arc<DatabaseSystemController>,
}

impl AppController {
    pub async fn new() -> Result<Self, TrustError> {
        debug!("config");
        logger::config();
        data_updates::new();

        debug!("database");
        let surreal = Arc::new(database::init_in_memory_db_connection().await?);
        let dba = Arc::new(DatabaseArticle::new(surreal.clone()));
        let dbu = Arc::new(DatabaseUser::new(surreal.clone()));
        let dbs = Arc::new(DatabaseSystem::new(surreal.clone()));

        // if there are no articles at all, create the table
        surreal.db.query("DEFINE TABLE article SCHEMALESS;").await?;

        // in memory application data
        let ds = Arc::new(data_system::new());
        let dv = Arc::new(data_updates::new());

        // the application state
        let config = configuration::get_config()?;
        let state = TheState {
            dba: dba.clone(),
            dbu: dbu.clone(),
            dbs: dbs.clone(),
            ds,
            dv,
            start_time: chrono::Utc::now(),
            config,
        };

        // server
        let server = server::connect(&state).await?;
        // app
        let app_router = Arc::new(server.start_app_router().await?);
        // web
        let web_router = server.start_web_router().await?;
        server.status_start()?;

        // application controller
        Ok(Self {
            // app
            account: Arc::new(AccountController::new(app_router.clone())),
            admin: Arc::new(AdminController::new(app_router.clone())),
            article: Arc::new(CreateArticleController::new(app_router.clone())),
            change_password: Arc::new(ChangePasswordController::new(app_router.clone())),
            login: Arc::new(LoginController::new(app_router.clone())),
            web_auth: Arc::new(AuthorizedWebController::new(app_router.clone())),
            // web
            web: Arc::new(WebController::new(web_router)),
            // surreal
            db_article_controller: Arc::new(DatabaseArticleController::new(dba.clone())),
            db_user_controller: Arc::new(DatabaseUserController::new(dbu.clone())),
            db_system_controller: Arc::new(DatabaseSystemController::new(dbs.clone())),
        })
    }

    pub fn create_article(&self, auth: &str) -> Arc<CreateArticleController> {
        self.article.set_cookie(Some(auth.to_string()));
        self.article.clone()
    }

    pub fn change_password(&self, auth: &str) -> Arc<ChangePasswordController> {
        self.change_password.set_cookie(Some(auth.to_string()));
        self.change_password.clone()
    }

    pub fn account(&self) -> Arc<AccountController> {
        self.account.clone()
    }

    pub fn login(&self) -> Arc<LoginController> {
        self.login.clone()
    }

    pub fn web_app(&self, auth: &str) -> Arc<AuthorizedWebController> {
        self.web_auth.set_cookie(Some(auth.to_string()));
        self.web_auth.clone()
    }

    pub fn web(&self) -> Arc<WebController> {
        self.web.clone()
    }

    pub fn admin(&self, auth: &str) -> Arc<AdminController> {
        self.admin.set_cookie(Some(auth.to_string()));
        self.admin.clone()
    }

    pub fn db_article(&self) -> Arc<DatabaseArticleController> {
        self.db_article_controller.clone()
    }

    pub fn db_user(&self) -> Arc<DatabaseUserController> {
        self.db_user_controller.clone()
    }

    pub fn db_system(&self) -> Arc<DatabaseSystemController> {
        self.db_system_controller.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::database_user::Role::Editor;

    #[tokio::test]
    async fn test_create_user() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        /*
         * create a user in the database
         */
        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("tester")
            .password("password")
            .execute().await?;

        /*
         * verify user in the database
         */
        #[rustfmt::skip]
        ac.db_user().must_see("tester")
            .await?
            .username("tester")
            .author_name("tester")
            .role(Editor)
            .verify()?;

        Ok(())
    }
}
