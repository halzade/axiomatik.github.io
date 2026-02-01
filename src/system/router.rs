use crate::application::account::form_account;
use crate::application::article::article;
use crate::application::change_password::form_change_password;
use crate::application::form::form_article_create;
use crate::application::form::form_article_create::FormArticleCreateError;
use crate::application::form::form_article_data_parser::ArticleCreateError;
use crate::application::index::index;
use crate::application::index::index::IndexError;
use crate::application::login::form_login;
use crate::application::republika::republika;
use crate::application::republika::republika::RepublikaError;
use crate::application::search::search;
use crate::db::database_user::{self, Backend};
use crate::system::data_system::{DataSystem, DataSystemError};
use crate::system::data_updates::{DataUpdates, DataUpdatesError};
use crate::system::server::ApplicationStatus;
use crate::system::{data_system, data_updates, heartbeat};
use axum::body::Body;
use axum::extract::OriginalUri;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, get_service, post};
use axum::{middleware, Router};
use axum_core::extract::Request;
use axum_login::AuthManagerLayerBuilder;
use http::StatusCode;
use std::convert::Infallible;
use std::fs;
use thiserror::Error;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::{debug, info};

pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("create article error: {0}")]
    RouterArticleError(#[from] ArticleCreateError),

    #[error("data update error: {0}")]
    RouterDataUpdate(#[from] DataUpdatesError),

    #[error("data update system: {0}")]
    RouterDataSystem(#[from] DataSystemError),

    #[error("form error: {0}")]
    RouterForm(#[from] FormArticleCreateError),

    #[error("index error: {0}")]
    RouterIndexError(#[from] IndexError),

    #[error("republika error: {0}")]
    RouterRepublikaErrorError(#[from] RepublikaError),

    #[error("response infallible: {0}")]
    RouterInfallible(#[from] Infallible),
}

pub struct ApplicationRouter {
    data_system: DataSystem,
    data_updates: DataUpdates,
}

impl ApplicationRouter {
    pub fn new() -> ApplicationRouter {
        ApplicationRouter {
            data_system: data_system::new(),
            data_updates: data_updates::new(),
        }
    }
}

impl IntoResponse for RouterError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl IntoResponse for FormArticleCreateError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl ApplicationRouter {
    #[rustfmt::skip]
    pub async fn start_router(&self, status: ApplicationStatus) -> Router {
        info!("start_router()");

        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(tower_sessions::cookie::SameSite::Lax);

        let backend = Backend;
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        /*
         * Protected routes
         */
        let protected_routes = Router::new()
            .route("/form", get(form_article_create::show_article_create_form))
            .route("/create", post(article::create_article))
            .route("/change-password",
                get(form_change_password::show_change_password)
               .post(form_change_password::handle_change_password),
            )
            .route("/account", get(form_account::show_account))
            .route("/account/update-author", post(form_account::handle_update_author_name))
            .route("/heartbeat", get(heartbeat::handle_heartbeat))
            .layer(middleware::from_fn(auth_middleware));

        /*
         * Unprotected routes
         */

        let ret = Router::new()
            .route("/", get(|| async { Redirect::to("/index.html") }))
            .route("/login",
                get(form_login::show_login)
               .post(form_login::handle_login),
            )
            .route("/search", get(search::handle_search))
            .route("/ping", get("ping success"))
            // serve static content
            .nest_service("/css", ServeDir::new("../../web/css"))
            .nest_service("/js", ServeDir::new("../../web/js"))
            .nest_service("/u", ServeDir::new("web/u"))
            // other files
            .nest_service("/favicon.ico", get_service(ServeFile::new("../../web/favicon.ico")))
            // HTML files
            .route("/*.html", get(|ori_uri: OriginalUri, request| async move {
                    self.serve_html_content(ori_uri, request).await
                }
            ))
            // web app
            .merge(protected_routes)
            // everything already served, user requested for non-existent content
            .fallback(show_404)
            .layer(auth_layer)
            .with_state(status);

        info!("start_router() finished");
        ret
    }

    pub async fn serve_html_content(
        &self,
        ori_uri: OriginalUri,
        request: Request<Body>,
    ) -> Result<Response, RouterError> {
        // TODO X validate ori_uri contain only alphanumeric and dash, and one dot
        // TODO X re-renders should be somehow one thread only

        /*
         * What kind of content is it?
         */
        match &ori_uri {
            OriginalUri(uri) => match uri.path() {
                "/index.html" => {
                    if !self.data_updates.index_valid() {
                        index::render_index().await?;
                        self.data_updates.index_invalidate();
                    }
                }
                "/finance.html" => {
                    // TODO do
                }
                "/news.html" => {
                    // TODO do
                }
                "/republika.html" => {
                    if !self.data_updates.republika_valid() {
                        republika::render_republika(&self.data_system).await?;
                        self.data_updates.republika_validate();
                    }
                }
                "/technologie.html" => {
                    // TODO do
                }
                "/veda.html" => {
                    // TODO do
                }
                "/zahranici.html" => {
                    // TODO do
                }
                _ => {
                    // forgot some or Article
                    // TODO do
                }
            },
        };

        Ok(ServeFile::new(ori_uri.path().to_string())
            .oneshot(request)
            .await?
            .into_response())
    }
}

async fn auth_middleware(auth_session: AuthSession, req: Request<Body>, next: Next) -> Response {
    let req = req;
    match auth_session.user {
        Some(user) => {
            // change password
            if user.needs_password_change && req.uri().path() != "/change-password" {
                return Redirect::to("/change-password").into_response();
            }

            // continue
            if user.role == database_user::Role::Editor {
                return next.run(req).await;
            }
        }
        None => {
            return Redirect::to("/login").into_response();
        }
    }

    // login
    Redirect::to("/login").into_response()
}

async fn show_404() -> impl IntoResponse {
    debug!("show_404 called");

    match fs::read_to_string("../../web/404.html") {
        Ok(content) => (StatusCode::NOT_FOUND, Html(content)),
        Err(err) => {
            debug!("Failed to read 404.html: {err}");
            (StatusCode::NOT_FOUND, Html("404 Not Found".to_string()))
        }
    }
}
