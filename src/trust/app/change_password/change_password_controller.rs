use crate::trust::app::change_password::change_password_data::{
    ChangePasswordData, ChangePasswordFluent,
};
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use parking_lot::RwLock;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Debug, Clone)]
pub struct ChangePasswordController {
    app_router: Arc<Router>,
    input: ChangePasswordFluent,
    user_cookie: Arc<RwLock<Option<String>>>,
}

impl ChangePasswordController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self {
            app_router,
            input: ChangePasswordFluent::new(),
            user_cookie: Arc::new(RwLock::new(None)),
        }
    }

    pub fn set_cookie(&self, cookie: Option<String>) {
        *self.user_cookie.write() = cookie;
    }

    pub fn new_password(&self, password: &str) -> &Self {
        self.input.new_password(password);
        self
    }

    pub async fn execute(&self) -> Result<ResponseVerifier, TrustError> {
        let data = self.input.get_data();
        let new_password = data.new_password.unwrap_or_default();
        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        let response_r = self
            .app_router
            .as_ref()
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/change-password")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(header::COOKIE, cookie)
                    .body(Body::from(format!("new_password={}", new_password)))?,
            )
            .await;

        let response_verifier = ResponseVerifier::from_r(response_r);

        // Clear input after execution if successful
        if response_verifier.response.status().is_success()
            || response_verifier.response.status().is_redirection()
        {
            *self.input.data.write() = ChangePasswordData::new();
        }

        Ok(response_verifier)
    }
}
