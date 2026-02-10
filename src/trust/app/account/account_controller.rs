use crate::trust::app::account::account_data::{
    AccountUpdateAuthorData, AccountUpdateAuthorFluent,
};
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use std::sync::Arc;
use tower::ServiceExt;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct AccountController {
    app_router: Arc<Router>,
    input: AccountUpdateAuthorFluent,
    user_cookie: Arc<parking_lot::RwLock<Option<String>>>,
}

impl AccountController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, input: AccountUpdateAuthorFluent::new(), user_cookie: Arc::new(parking_lot::RwLock::new(None)) }
    }

    pub fn set_cookie(&self, cookie: Option<String>) {
        *self.user_cookie.write() = cookie;
    }

    // start fluent chain for updating author name; sets auth cookie
    pub fn update_author_name(&self, auth_cookie: &str) -> &Self {
        self.set_cookie(Some(auth_cookie.to_string()));
        self
    }

    pub fn author_name(&self, author_name: &str) -> &Self {
        self.input.author_name(author_name);
        self
    }

    pub async fn execute(&self) -> Result<ResponseVerifier, TrustError> {
        let data: AccountUpdateAuthorData = self.input.get_data();
        let author_name = data.author_name.unwrap_or_default();
        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        debug!("update author name: {}", author_name);
        let response = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/account/update-author")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(header::COOKIE, cookie)
                    .body(Body::from(format!("author_name={}", author_name)))?,
            )
            .await?;
        debug!("update author name done");

        // Clear input after execution if successful
        if response.status().is_success() || response.status().is_redirection() {
            *self.input.data.write() = AccountUpdateAuthorData::new();
        }

        Ok(ResponseVerifier::new(response))
    }

    pub async fn get(&self, auth_cookie: &str) -> Result<ResponseVerifier, TrustError> {
        debug!("get account page");
        let response = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/account")
                    .header(header::COOKIE, &auth_cookie.to_string())
                    .body(Body::empty())?,
            )
            .await?;
        debug!("get account page done");
        Ok(ResponseVerifier::new(response))
    }
}
