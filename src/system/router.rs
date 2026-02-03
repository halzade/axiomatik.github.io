use crate::application::account::form_account;
use crate::application::article::article;
use crate::application::article::article::ArticleError;
use crate::application::change_password::form_change_password;
use crate::application::finance::finance;
use crate::application::finance::finance::FinanceError;
use crate::application::form::form_article_create;
use crate::application::form::form_article_create::FormArticleCreateError;
use crate::application::form::form_article_data_parser::ArticleCreateError;
use crate::application::index::index;
use crate::application::index::index::IndexError;
use crate::application::login::form_login;
use crate::application::news::news;
use crate::application::news::news::NewsError;
use crate::application::republika::republika;
use crate::application::republika::republika::RepublikaError;
use crate::application::search::search;
use crate::application::technologie::technologie;
use crate::application::technologie::technologie::TechnologieError;
use crate::application::veda::veda;
use crate::application::veda::veda::VedaError;
use crate::application::zahranici::zahranici;
use crate::application::zahranici::zahranici::ZahraniciError;
use crate::db::database_user::{self, Backend};
use crate::system::data_system::{DataSystem, DataSystemError};
use crate::system::data_updates::{ArticleStatus, DataUpdates, DataUpdatesError};
use crate::system::server::ApplicationStatus;
use crate::system::{data_system, data_updates, heartbeat};
use axum::body::Body;
use axum::extract::OriginalUri;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{middleware, Router};
use axum_core::extract::Request;
use axum_login::AuthManagerLayerBuilder;
use http::StatusCode;
use std::convert::Infallible;
use std::sync::Arc;
use thiserror::Error;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};
use tower_sessions::cookie::SameSite::Strict;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::{info, warn};

pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("create article error: {0}")]
    RouterArticleCreateError(#[from] ArticleCreateError),

    #[error("data update error: {0}")]
    RouterDataUpdate(#[from] DataUpdatesError),

    #[error("index error: {0}")]
    RouterIndexError(#[from] IndexError),

    #[error("finance error: {0}")]
    RouterFinanceError(#[from] FinanceError),

    #[error("news error: {0}")]
    RouterNewsError(#[from] NewsError),

    #[error("republika error: {0}")]
    RouterRepublikaError(#[from] RepublikaError),

    #[error("technologie error: {0}")]
    RouterTechnologieError(#[from] TechnologieError),

    #[error("veda error: {0}")]
    RouterVedaError(#[from] VedaError),

    #[error("zahranici error: {0}")]
    RouterZahraniciError(#[from] ZahraniciError),

    #[error("article error: {0}")]
    RouterArticleError(#[from] ArticleError),

    #[error("data update system: {0}")]
    RouterDataSystem(#[from] DataSystemError),

    #[error("form error: {0}")]
    RouterForm(#[from] FormArticleCreateError),

    #[error("response infallible: {0}")]
    RouterInfallible(#[from] Infallible),
}

pub struct ApplicationRouter {
    data_system: DataSystem,
    data_updates_a: Arc<DataUpdates>,
}

impl ApplicationRouter {
    pub fn new() -> ApplicationRouter {
        ApplicationRouter {
            data_system: data_system::new(),
            data_updates_a: Arc::new(data_updates::new()),
        }
    }
}

impl IntoResponse for RouterError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl IntoResponse for ArticleError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl IntoResponse for FormArticleCreateError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl ApplicationRouter {
    #[rustfmt::skip]
    pub async fn start_router(self: Arc<Self>, status: ApplicationStatus) -> Router {
        info!("start_router()");

        let self_a1 = self.clone();
        let self_a2 = self.clone();

        // TODO don't use default memory story, use redis or something
        let session_layer = SessionManagerLayer::new(MemoryStore::default())
            // true only https
            .with_secure(true)
            .with_http_only(true)
            // creating articles doesn't require any clos site features
            .with_same_site(Strict);

        let auth_layer = AuthManagerLayerBuilder::new(Backend, session_layer).build();

        /*
         * Protected routes
         */
        let protected_routes = Router::new()
            .route("/form", get(form_article_create::show_article_create_form))
            .route("/create", post(move |auth, multipart| {
                article::create_article(self_a1.data_updates_a.clone(), auth, multipart)
            }))
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
            .route("/login",
                get(form_login::show_login)
               .post(form_login::handle_login),
            )
            .route("/search", get(search::handle_search))
            .route("/ping", get("ping success"))
            // serve static files
            .nest_service("/web/", ServeDir::new("/web/"))
            /*
             * catch web requests and maybe updat invalid HTML file
             * redirect the request to the web directory
             */
            .route("/", get(move |ori_uri: OriginalUri, request|
                async move {
                    self_a2.serve_static_content(ori_uri, request).await
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

    /**
     * serve all static files for web
     * 1. HTML file requests
     * - serve a static HTML file if valid
     * - rebuild the HTML file if invalid
     * 2. image, video, CSS, js requests
     * - serve static files
     */
    pub async fn serve_static_content(
        &self,
        ori_uri: OriginalUri,
        request: Request<Body>,
    ) -> Result<Response, RouterError> {
        /*
         * request url
         */
        let url = ori_uri.path().to_string();

        /*
         * What kind of content is it?
         */
        if url.starts_with("/css/")
            || url.starts_with("/js/")
            || url.starts_with("/u/")
            || url == "/favicon.ico"
        {
            return serve_this(url, request).await;
        }
        // it is an HTML file request, HTML content may need refresh

        match url.as_str() {
            "/index.html" => {
                if !self.data_updates_a.index_valid() {
                    self.data_updates_a.index_validate();

                    index::render_index(&self.data_system).await?;
                }
                serve_this(url, request).await
            }
            "/finance.html" => {
                if !self.data_updates_a.finance_valid() {
                    self.data_updates_a.finance_validate();

                    finance::render_finance(&self.data_system).await?;
                }
                serve_this(url, request).await
            }
            "/news.html" => {
                if !self.data_updates_a.news_valid() {
                    self.data_updates_a.news_validate();

                    news::render_news(&self.data_system).await?;
                }
                serve_this(url, request).await
            }
            "/republika.html" => {
                if !self.data_updates_a.republika_valid() {
                    self.data_updates_a.republika_validate();

                    republika::render_republika(&self.data_system).await?;
                }
                serve_this(url, request).await
            }
            "/technologie.html" => {
                if !self.data_updates_a.technologie_valid() {
                    self.data_updates_a.technologie_validate();

                    technologie::render_technologie(&self.data_system).await?;
                }
                serve_this(url, request).await
            }
            "/veda.html" => {
                if !self.data_updates_a.veda_valid() {
                    self.data_updates_a.veda_validate();

                    veda::render_veda(&self.data_system).await?;
                }
                serve_this(url, request).await
            }
            "/zahranici.html" => {
                if !self.data_updates_a.zahranici_valid() {
                    self.data_updates_a.zahranici_validate();

                    zahranici::render_zahranici(&self.data_system).await?;
                }
                serve_this(url, request).await
            }
            _ => {
                // 404 or Article
                match self.data_updates_a.article_valid(&url) {
                    ArticleStatus::Valid => {
                        // no change, serve the file
                        serve_this(url, request).await
                    }
                    ArticleStatus::Invalid => {
                        // article was invalidated, render article HTML
                        // new article was u
                        article::render_article(&url, &self.data_system).await?;
                        self.data_updates_a.article_validate(&url);
                        serve_this(url, request).await
                    }
                    ArticleStatus::DoesntExist => {
                        // requested url doesn't exist
                        serve_404().await
                    }
                }
            }
        }
    }
}

async fn serve_this(path: String, request: Request<Body>) -> Result<Response, RouterError> {
    Ok(ServeFile::new(format!("web/{}", path))
        .oneshot(request)
        .await?
        .into_response())
}

async fn serve_404() -> Result<Response, RouterError> {
    Ok((
        StatusCode::NOT_FOUND,
        Html("404, stránka nenalezená".to_string()),
    )
        .into_response())
}

async fn auth_middleware(auth_session: AuthSession, req: Request<Body>, next: Next) -> Response {
    let req = req;
    info!("auth_middleware: user={:?}, path={}", auth_session.user, req.uri().path());
    match auth_session.user {
        Some(user) => {
            // change password
            if user.needs_password_change && req.uri().path() != "/change-password" {
                info!("auth_middleware: needs_password_change redirect");
                return Redirect::to("/change-password").into_response();
            }

            // continue
            if user.role == database_user::Role::Editor {
                info!("auth_middleware: role=Editor, continue");
                return next.run(req).await;
            }
            info!("auth_middleware: role NOT Editor: {:?}", user.role);
        }
        None => {
            info!("auth_middleware: No user in session");
            return Redirect::to("/login").into_response();
        }
    }

    // login
    info!("auth_middleware: default redirect to /login");
    Redirect::to("/login").into_response()
}

async fn show_404() -> impl IntoResponse {
    warn!("router fallback");
    (
        StatusCode::NOT_FOUND,
        Html("404, stránka nenalezená".to_string()),
    )
}
