use crate::trust::me::TrustError;
use axum::Router;
use http::{header, Request};
use parking_lot::RwLock;
use std::sync::Arc;
use tower::ServiceExt;
use crate::db::database_article::DatabaseArticle;
use crate::db::database_system::DatabaseSystem;
use crate::db::database_user::DatabaseUser;

pub struct NexoApp {
    app_router: Arc<Router>,
    user_cookie: RwLock<Option<String>>,
    pub dba: Arc<DatabaseArticle>,
    pub dbs: Arc<DatabaseSystem>,
    pub dbu: Arc<DatabaseUser>,
}

impl NexoApp {
    pub fn new(app_router: Router) -> Self {
        Self { app_router: Arc::new(app_router), user_cookie: Default::default() }
    }

    pub fn post_create_article(&self) -> ArticleBuilder {
        ArticleBuilder::new()
    }

    pub async fn post_login(&self, username: &str) -> Result<ResponseVerifier, TrustError> {
        self.post_login_with_password(username, "password").await
    }

    pub async fn post_login_with_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<ResponseVerifier, TrustError> {
        let login_response = (*self.app_router)
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(reqwest::Body::from(format!(
                        "username={}&password={}",
                        username, password
                    )))?,
            )
            .await?;

        let cookie = login_response.headers().get(header::SET_COOKIE).cloned();
        if let Some(c) = cookie {
            *self.user_cookie.write() = Some(c.to_str()?.to_string());
        }

        Ok(ResponseVerifier::new_from_response(login_response).await?)
    }
}
