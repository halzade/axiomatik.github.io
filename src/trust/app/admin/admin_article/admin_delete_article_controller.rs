use crate::trust::app::admin::admin_article::admin_article_data::AdminArticleFluent;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use parking_lot::RwLock;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Debug)]
pub struct AdminDeleteArticleController {
    app_router: Arc<Router>,
    user_cookie: Arc<RwLock<Option<String>>>,
    article_fluent: AdminArticleFluent,
}

impl AdminDeleteArticleController {
    pub fn new(app_router: Arc<Router>, user_cookie: Arc<RwLock<Option<String>>>) -> Self {
        Self {
            app_router,
            user_cookie,
            article_fluent: AdminArticleFluent::new(),
        }
    }

    pub fn article_file_name(&self, name: &str) -> &Self {
        self.article_fluent.article_file_name(name);
        self
    }

    pub async fn execute(&self) -> Result<ResponseVerifier, TrustError> {
        let name = self.article_fluent.get_data().article_file_name.unwrap_or_default();
        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        let response_r = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/admin_article/delete/{}", name))
                    .header(header::COOKIE, cookie)
                    .body(Body::empty())?,
            )
            .await;

        let response_verifier = ResponseVerifier::from_r(response_r);

        if response_verifier.response.status().is_success()
            || response_verifier.response.status().is_redirection()
        {
            self.article_fluent.reset();
        }

        Ok(response_verifier)
    }
}
