use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use parking_lot::RwLock;
use std::sync::Arc;
use tower::ServiceExt;
use tracing::error;

#[derive(Debug)]
pub struct AuthorizedWebController {
    app_router: Arc<Router>,
    user_cookie: Arc<RwLock<Option<String>>>,
}

impl AuthorizedWebController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self { app_router, user_cookie: Arc::new(RwLock::new(None)) }
    }

    pub async fn get_url(&self, url: &str) -> Result<ResponseVerifier, TrustError> {
        if !url.starts_with('/') {
            error!("url must start with '/'")
        }

        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        let response_r = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(url)
                    .header(header::COOKIE, cookie)
                    .body(Body::empty())?,
            )
            .await;

        Ok(ResponseVerifier::from_r(response_r))
    }

    pub fn set_cookie(&self, cookie: Option<String>) {
        *self.user_cookie.write() = cookie;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust::app_controller::AppController;
    use http::StatusCode;

    #[tokio::test]
    async fn test_auth_web_controller() -> Result<(), TrustError> {
        let ac = AppController::new().await?;

        #[rustfmt::skip]
        ac.db_user().setup_user()
            .username("user8")
            .password("password123")
            .execute().await?;

        #[rustfmt::skip]
        let auth = ac.login()
            .username("user8")
            .password("password123")
            .execute().await?
                .must_see_response(StatusCode::SEE_OTHER)
                .verify().await?;

        #[rustfmt::skip]
        ac.web_app(&auth).get_url("/health").await?
            .must_see_response(StatusCode::OK)
            .verify().await?;

        Ok(())
    }
}
