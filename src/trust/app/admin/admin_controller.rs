use crate::trust::app::admin::admin_article_data::AdminArticleFluent;
use crate::trust::app::admin::admin_user_data::AdminUserFluent;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use parking_lot::RwLock;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Debug)]
pub struct AdminController {
    app_router: Arc<Router>,
    user_cookie: Arc<RwLock<Option<String>>>,
    user_fluent: AdminUserFluent,
    article_fluent: AdminArticleFluent,
}

impl AdminController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self {
            app_router,
            user_cookie: Arc::new(RwLock::new(None)),
            user_fluent: AdminUserFluent::new(),
            article_fluent: AdminArticleFluent::new(),
        }
    }

    pub fn set_cookie(&self, cookie: Option<String>) {
        *self.user_cookie.write() = cookie;
    }

    pub fn delete_user(&self) -> &Self {
        self
    }

    pub fn delete_article(&self) -> &Self {
        self
    }

    pub fn username(&self, username: &str) -> &Self {
        self.user_fluent.username(username);
        self
    }

    pub fn article_file_name(&self, name: &str) -> &Self {
        self.article_fluent.article_file_name(name);
        self
    }

    pub async fn execute_delete_user(&self) -> Result<ResponseVerifier, TrustError> {
        let username = self.user_fluent.get_data().username.unwrap_or_default();
        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        let response_r = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/admin/users/delete/{}", username))
                    .header(header::COOKIE, cookie)
                    .body(Body::empty())?,
            )
            .await;

        Ok(ResponseVerifier::from_r(response_r))
    }

    pub async fn execute_delete_article(&self) -> Result<ResponseVerifier, TrustError> {
        let name = self.article_fluent.get_data().article_file_name.unwrap_or_default();
        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        let response_r = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/admin/articles/delete/{}", name))
                    .header(header::COOKIE, cookie)
                    .body(Body::empty())?,
            )
            .await;

        Ok(ResponseVerifier::from_r(response_r))
    }
}
