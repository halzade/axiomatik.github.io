use crate::application::account::form_account;
use crate::application::account::form_account::{AccountError, FormAccount};
use crate::application::article::article;
use crate::application::article::article::ArticleError;
use crate::application::change_password::form_change_password;
use crate::application::change_password::form_change_password::ChangePasswordError;
use crate::application::finance::finance::FinanceError;
use crate::application::form::form_article_create;
use crate::application::form::form_article_create::FormArticleCreateError;
use crate::application::form::form_article_data_parser::ArticleCreateError;
use crate::application::index::index::IndexError;
use crate::application::login::form_login;
use crate::application::news::news::NewsError;
use crate::application::republika::republika::RepublikaError;
use crate::application::technologie::technologie::TechnologieError;
use crate::application::veda::veda::VedaError;
use crate::application::zahranici::zahranici::ZahraniciError;
use crate::db::database_user::{self, Backend};
use crate::system::data_system::{DataSystem, DataSystemError};
use crate::system::data_updates::{DataUpdates, DataUpdatesError};
use crate::system::router_web::WebRouter;
use crate::system::server::ApplicationStatus;
use crate::system::{data_system, data_updates, heartbeat};
use axum::body::Body;
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
use tower_sessions::cookie::SameSite::Strict;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::{info, warn};
use crate::db::database::DatabaseSurreal;

pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Error)]
pub enum AppRouterError {
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

    #[error("change password error: {0}")]
    RouterChangePasswordError(#[from] ChangePasswordError),

    #[error("account error: {0}")]
    RouterAccountError(#[from] AccountError),
}

pub struct ApplicationRouter {
    data_system: DataSystem,
    data_updates_a: Arc<DataUpdates>,

    form_account: Arc<FormAccount>,
}

impl ApplicationRouter {
    pub async fn init() -> Result<ApplicationRouter, AppRouterError> {
        Ok(ApplicationRouter {
            // TODO shared data
            data_system: data_system::new(),
            data_updates_a: Arc::new(data_updates::new()),
            form_account: Arc::new(FormAccount::init().await?),
        })
    }
}

// TODO macro derive these things
impl IntoResponse for AppRouterError {
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

impl IntoResponse for ChangePasswordError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl IntoResponse for AccountError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl ApplicationRouter {
    #[rustfmt::skip]
    pub async fn start_app_router(self: Arc<Self>, status: ApplicationStatus) -> Router {
        info!("start_router()");

        let self_a1 = self.clone();

        // TODO don't use default memory storage, use redis or something
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
            // TODO these shouldn't be in root
            .route("/form", get(form_article_create::show_article_create_form))
            .route("/create", post(move |auth, multipart| {
                article::create_article(self_a1.data_updates_a.clone(), auth, multipart)
            }))
            .route("/change-password",
                get(form_change_password::show_change_password)
               .post(form_change_password::handle_change_password),
            )
            .route("/account", get(self.form_account.show_account()))
            .route("/account/update-author", post(self.form_account.handle_update_author_name))
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
            .route("/ping", get("ping success"))
            // serve static files
            .merge(protected_routes)
            // everything already served, user requested for non-existent content
            .fallback(show_404)
            .layer(auth_layer)
            .with_state(status);

        info!("start_router() finished");
        ret
    }
}

async fn auth_middleware(auth_session: AuthSession, req: Request<Body>, next: Next) -> Response {
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
    (StatusCode::NOT_FOUND, Html("404, stránka nenalezená".to_string()))
}
