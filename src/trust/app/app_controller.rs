use crate::trust::app::account::account_controller::AccountController;
use crate::trust::app::article::create_article_controller::CreateArticleController;
use crate::trust::app::change_password::change_password_controller::ChangePasswordController;
use crate::trust::app::login::login_controller::LoginController;
use crate::trust::db::db_article_controller::ArticleDatabaseController;
use crate::trust::me::TrustError;

#[derive(Debug)]
pub struct AppController;

impl AppController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn post_create_article(&self) -> CreateArticleController {
        CreateArticleController::new()
    }

    pub fn post_change_password(&self) -> ChangePasswordController {
        ChangePasswordController::new()
    }

    pub fn post_account_update_author(&self) -> AccountController {
        AccountController::new()
    }

    pub fn post_login(&self) -> LoginController {
        LoginController::new()
    }

    pub(crate) fn db_article_must_see(&self) -> ArticleDatabaseController {
        ArticleDatabaseController::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_article() -> Result<(), TrustError> {
        let ac = AppController::new();

        #[rustfmt::skip]
        ac.post_create_article()
            .title("title")
            .text("text")
            .execute()?;

        #[rustfmt::skip]
        ac.db_article_must_see()
            .title("title")
            .text("text")
            .verify();

        Ok(())
    }
}
