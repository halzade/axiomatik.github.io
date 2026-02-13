use crate::trust::app::login::login_data::LoginFluent;
use crate::trust::app::login::response_verifier_login::LoginResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use header::CONTENT_TYPE;
use http::{header, Request};
use std::sync::Arc;
use tower::ServiceExt;
use tracing::debug;

#[derive(Debug)]
pub struct LoginController {
    app_router: Arc<Router>,
    input: LoginFluent,
}

impl LoginController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: LoginFluent::new() }
    }

    pub fn username(&self, username: &str) -> &Self {
        self.input.username(username);
        self
    }

    pub fn password(&self, password: &str) -> &Self {
        self.input.password(password);
        self
    }

    pub async fn execute(&self) -> Result<LoginResponseVerifier, TrustError> {
        let data = self.input.get_data();
        let username = data.username.unwrap_or_default();
        let password = data.password.unwrap_or_default();

        let login_response_r = self
            .app_router
            .as_ref()
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(Body::from(format!("username={}&password={}", username, password)))?,
            )
            .await;

        debug!("login done");

        LoginResponseVerifier::from_r(login_response_r)
    }
}
