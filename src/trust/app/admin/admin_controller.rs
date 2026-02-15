use crate::trust::app::admin::admin_article::admin_delete_article_controller::AdminDeleteArticleController;
use crate::trust::app::admin::admin_user::admin_create_user_controller::AdminCreateUserController;
use crate::trust::app::admin::admin_user::admin_delete_user_controller::AdminDeleteUserController;
use axum::Router;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug)]
pub struct AdminController {
    user_cookie: Arc<RwLock<Option<String>>>,
    create_user_controller: AdminCreateUserController,
    delete_user_controller: AdminDeleteUserController,
    delete_article_controller: AdminDeleteArticleController,
}

impl AdminController {
    pub fn new(app_router: Arc<Router>) -> Self {
        let cookie = Arc::new(RwLock::new(None));

        Self {
            user_cookie: cookie.clone(),
            create_user_controller: AdminCreateUserController::new(
                app_router.clone(),
                cookie.clone(),
            ),
            delete_user_controller: AdminDeleteUserController::new(
                app_router.clone(),
                cookie.clone(),
            ),
            delete_article_controller: AdminDeleteArticleController::new(app_router, cookie),
        }
    }

    pub fn set_cookie(&self, cookie: Option<String>) {
        *self.user_cookie.write() = cookie;
    }

    pub const fn create_user(&self) -> &AdminCreateUserController {
        &self.create_user_controller
    }

    pub const fn delete_user(&self) -> &AdminDeleteUserController {
        &self.delete_user_controller
    }

    pub const fn delete_article(&self) -> &AdminDeleteArticleController {
        &self.delete_article_controller
    }
}
