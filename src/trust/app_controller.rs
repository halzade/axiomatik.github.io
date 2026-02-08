use crate::db::database;
use crate::db::database_article::DatabaseArticle;
use crate::db::database_user::DatabaseUser;
use crate::system::server::TheState;
use crate::system::{data_system, data_updates, logger, server};
use crate::trust::app::account::account_controller::AccountController;
use crate::trust::app::article::create_article_controller::CreateArticleController;
use crate::trust::app::change_password::change_password_controller::ChangePasswordController;
use crate::trust::app::login::login_controller::LoginController;
use crate::trust::db::db_article_controller::DatabaseArticleController;
use crate::trust::db::db_system_controller::DatabaseSystemController;
use crate::trust::db::db_user_controller::DatabaseUserController;
use crate::trust::me::TrustError;
use crate::trust::web::web_controller::WebController;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug)]
pub struct AppController {
    // app
    account: Arc<AccountController>,
    article: Arc<CreateArticleController>,
    change_password: Arc<ChangePasswordController>,
    login: Arc<LoginController>,
    // web
    web: Arc<WebController>,
    // db
    db_article_controller: Arc<DatabaseArticleController>,
    db_user_controller: Arc<DatabaseUserController>,
    db_system_controller: Arc<DatabaseSystemController>,
}

impl AppController {
    pub async fn new() -> Result<AppController, TrustError> {
        debug!("config");
        logger::config();
        data_updates::new();

        debug!("database");
        let db = Arc::new(database::init_in_memory_db_connection().await?);
        let dba = Arc::new(DatabaseArticle::new(db.clone()));
        let dbu = Arc::new(DatabaseUser::new(db.clone()));
        let dbs = Arc::new(crate::db::database_system::DatabaseSystem::new(db.clone()));

        // in memory application data
        let ds = Arc::new(data_system::new());
        let dv = Arc::new(data_updates::new());

        // the application state
        let state = TheState { dba: dba.clone(), dbu: dbu.clone(), dbs: dbs.clone(), ds, dv };

        // server
        let server = server::connect(state.clone()).await?;
        // app
        let app_router = Arc::new(server.start_app_router().await?);
        // web
        let web_router = server.start_web_router().await?;
        server.status_start()?;

        // application controller
        Ok(AppController {
            // app
            account: Arc::new(AccountController::new(app_router.clone())),
            article: Arc::new(CreateArticleController::new(app_router.clone())),
            change_password: Arc::new(ChangePasswordController::new(app_router.clone())),
            login: Arc::new(LoginController::new(app_router)),
            // web
            web: Arc::new(WebController::new(web_router)),
            // surreal
            db_article_controller: Arc::new(DatabaseArticleController::new(dba.clone())),
            db_user_controller: Arc::new(DatabaseUserController::new(dbu.clone())),
            db_system_controller: Arc::new(DatabaseSystemController::new(dbs.clone())),
        })
    }

    pub fn create_article(&self) -> Arc<CreateArticleController> {
        self.article.set_cookie(self.login.get_cookie());
        self.article.clone()
    }

    pub fn change_password(&self) -> Arc<ChangePasswordController> {
        self.change_password.clone()
    }

    pub fn account(&self) -> Arc<AccountController> {
        self.account.clone()
    }

    pub fn login(&self) -> Arc<LoginController> {
        self.login.clone()
    }

    pub fn web(&self) -> Arc<WebController> {
        self.web.clone()
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
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_create_article() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        // create user and login
        ac.db_user().setup_user()
            .username("editor")
            .password("password")
            .execute().await?;

        ac.login()
            .username("editor")
            .password("password")
            .execute().await?;

        /*
         * post Article to router
         */
        #[rustfmt::skip]
        ac.create_article()
            .title("Title 1")
            .author("Editor")
            .category("republika")
            .text("text")
            .execute().await?
            .must_see_response(StatusCode::SEE_OTHER)
            .verify().await?;

        /*
         * verify Article in the database
         */
        #[rustfmt::skip]
        ac.db_article().must_see("title-1.html").await?
            .title("Title 1")
            .text("text")
            .verify()?;

        Ok(())
    }

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
