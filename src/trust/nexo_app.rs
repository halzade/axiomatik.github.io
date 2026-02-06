use crate::trust::article_builder::ArticleBuilder;
use crate::trust::me::TrustError;
use axum::Router;
use http::{header, Request};
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceExt;
use tracing::info;
use crate::system::{configuration, server};

pub struct NexoApp {
    app_router: Arc<Router>,
    user_cookie: RwLock<Option<String>>,
}

impl NexoApp {
    pub fn new(app_router: Router) -> Self {

        Self {
            app_router: Arc::new(app_router),
            user_cookie: Default::default(),
        }
    }

    pub fn post_create_article(&self) -> ArticleBuilder {
        ArticleBuilder::new()
    }

    pub async fn post_login(&self, username: &str) -> Result<(), TrustError> {
        let login_response = self
            .app_router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(reqwest::Body::from(format!(
                        "username={}&password={}",
                        username, "password"
                    )))?,
            )
            .await?;

        *self.user_cookie.write() =
            login_response.headers().get(header::SET_COOKIE).unwrap().to_str().unwrap().to_string();

        Ok(())
    }
}
