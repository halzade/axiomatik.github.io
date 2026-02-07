use crate::trust::app::login::login_data::LoginData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use header::{CONTENT_TYPE, SET_COOKIE};
use http::{header, Request};
use parking_lot::RwLock;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Debug)]
pub struct LoginController {
    app_router: Arc<Router>,
    input: LoginData,
    user_cookie: RwLock<Option<String>>,
}

impl LoginController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: LoginData::new(), user_cookie: RwLock::new(None) }
    }

    pub async fn post_login_with_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<ResponseVerifier, TrustError> {
        let login_response = self
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
            .await?;

        let cookie = login_response.headers().get(SET_COOKIE).cloned();
        if let Some(c) = cookie {
            *self.user_cookie.write() = Some(c.to_str()?.to_string());
        }

        Ok(ResponseVerifier::new(login_response))
    }
}
