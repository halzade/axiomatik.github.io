use crate::trust::app::login::login_data::LoginData;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use std::sync::Arc;
use http::{header, Request};

#[derive(Debug, Clone)]
pub struct LoginController {
    app_router: Arc<Router>,
    input: LoginData,
}

impl LoginController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: LoginData::new() }
    }

    // pub fn execute(self) -> Result<(ResponseVerifier), TrustError> {
    pub fn execute(self) -> Result<(), TrustError> {
        // TODO response

        Ok(())
    }

    pub async fn post_login_with_password(
        &self,
        username: &str,
        password: &str,
    // ) -> Result<ResponseVerifier, TrustError> {
    ) -> Result<(), TrustError> {
        // let login_response = self.app_router
        //     .oneshot(
        //         Request::builder()
        //             .method("POST")
        //             .uri("/login")
        //             .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        //             .body(reqwest::Body::from(format!(
        //                 "username={}&password={}",
        //                 username, password
        //             )))?,
        //     )
        //     .await?;

        // let cookie = login_response.headers().get(header::SET_COOKIE).cloned();
        // if let Some(c) = cookie {
        //     *self.user_cookie.write() = Some(c.to_str()?.to_string());
        // }

        // Ok(ResponseVerifier::new(login_response).await?)
        Ok(())
    }
}
