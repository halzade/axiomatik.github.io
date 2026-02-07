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
use crate::trust::db::db_article_verifier::DatabaseArticleVerifier;
use crate::trust::db::db_system_controller::DatabaseSystemController;
use crate::trust::db::db_system_verifier::DatabaseSystemVerifier;
use crate::trust::db::db_user_controller::DatabaseUserController;
use crate::trust::me::TrustError;
use std::sync::Arc;
use crate::trust::web::web_controller::WebController;

#[derive(Debug)]
pub struct AppController {
    // app
    account: Arc<AccountController>,
    article: Arc<CreateArticleController>,
    change_password: Arc<ChangePasswordController>,
    login: Arc<LoginController>,
    web: Arc<WebController>,
    // db
    db_article_controller: Arc<DatabaseArticleController>,
    db_user_controller: Arc<DatabaseUserController>,
    db_system_controller: Arc<DatabaseSystemController>,
}

impl AppController {
    pub async fn new() -> Result<AppController, TrustError> {
        // config
        logger::config();
        data_updates::new();

        // database
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

    pub fn post_create_article(&self) -> Arc<CreateArticleController> {
        self.article.clone()
    }

    pub fn post_change_password(&self) -> Arc<ChangePasswordController> {
        self.change_password.clone()
    }

    pub fn post_account_update_author(&self) -> Arc<AccountController> {
        self.account.clone()
    }

    pub fn post_login(&self) -> Arc<LoginController> {
        self.login.clone()
    }

    pub fn get_web(&self) -> Arc<WebController> {
        self.web.clone()
    }

    pub fn db_article_must_see(&self, real_article_url: &str) -> DatabaseArticleVerifier {
        DatabaseArticleVerifier::new(real_article_url)
    }

    pub fn db_system_must_see(&self, real_article_url: &str) -> DatabaseSystemVerifier {
        DatabaseSystemVerifier::new(real_article_url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_article() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        /*
         * post Article to router
         */
        #[rustfmt::skip]
        ac.post_create_article().as_ref().clone()
            .title("Title 1")
            .text("text")
            .execute()?;

        /*
         * verify Article in the database
         */
        #[rustfmt::skip]
        ac.db_article_must_see("title-1.html")
            .title("Title 1")
            .text("text")
            .verify()?;

        Ok(())
    }
}
