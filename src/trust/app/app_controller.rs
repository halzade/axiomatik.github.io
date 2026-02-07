use crate::trust::app::account::account_controller::AccountController;
use crate::trust::app::article::create_article_controller::CreateArticleController;
use crate::trust::app::change_password::change_password_controller::ChangePasswordController;
use crate::trust::app::login::login_controller::LoginController;
use crate::trust::me::TrustError;

#[derive(Debug)]
pub struct AppController;

impl AppController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_article(&self) -> CreateArticleController {
        CreateArticleController::new()
    }

    pub fn change_password(&self) -> ChangePasswordController {
        ChangePasswordController::new()
    }

    pub fn account_update_author(&self) -> AccountController {
        AccountController::new()
    }

    pub fn login(&self) -> LoginController {
        LoginController::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_article() -> Result<(), TrustError> {
        let ac = AppController::new();

        #[rustfmt::skip]
        let resp = ac.create_article()
            .title("title")
            .text("text")
            .execute()?;

        #[rustfmt::skip]
        ac.must_see_response(resp)
            .title("title")
            .text("text")
            .verify();

        Ok(())
    }
}
