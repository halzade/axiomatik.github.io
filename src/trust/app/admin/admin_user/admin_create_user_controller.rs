use crate::trust::app::admin::admin_user::admin_user_data::AdminUserFluent;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use parking_lot::RwLock;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Debug)]
pub struct AdminCreateUserController {
    app_router: Arc<Router>,
    user_cookie: Arc<RwLock<Option<String>>>,
    user_fluent: AdminUserFluent,
}

impl AdminCreateUserController {
    pub fn new(app_router: Arc<Router>, user_cookie: Arc<RwLock<Option<String>>>) -> Self {
        Self {
            app_router,
            user_cookie,
            user_fluent: AdminUserFluent::new(),
        }
    }

    pub fn username(&self, username: &str) -> &Self {
        self.user_fluent.username(username);
        self
    }

    pub fn password(&self, password: &str) -> &Self {
        self.user_fluent.password(password);
        self
    }

    pub fn author_name(&self, author_name: &str) -> &Self {
        self.user_fluent.author_name(author_name);
        self
    }

    pub async fn execute(&self) -> Result<ResponseVerifier, TrustError> {
        let data = self.user_fluent.get_data();
        let username = data.username.unwrap_or_default();
        let author_name = data.author_name.unwrap_or(username.clone());
        let password = data.password.unwrap_or_default();
        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        let response_r = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/admin_user/create")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(header::COOKIE, cookie)
                    .body(Body::from(format!(
                        "username={}&author_name={}&password={}",
                        username, author_name, password
                    )))?,
            )
            .await;

        let response_verifier = ResponseVerifier::from_r(response_r);

        if response_verifier.response.status().is_success()
            || response_verifier.response.status().is_redirection()
        {
            self.user_fluent.reset();
        }

        Ok(response_verifier)
    }
}
