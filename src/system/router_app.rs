use crate::application::account::form_account;
use crate::application::account::form_account::AccountError;
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
use crate::db::database::SurrealError;
use crate::db::database_user;
use crate::system::authentication::Backend;
use crate::system::data_system::DataSystemError;
use crate::system::data_updates::DataUpdatesError;
use crate::system::health;
use crate::system::server::TheState;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{middleware, Router};
use axum_core::extract::Request;
use axum_login::AuthManagerLayerBuilder;
use database_user::Role::Editor;
use http::StatusCode;
use thiserror::Error;
use tower_sessions::cookie::SameSite::Strict;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::log::debug;
use tracing::{info, warn};

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

    #[error("change password error: {0}")]
    RouterChangePasswordError(#[from] ChangePasswordError),

    #[error("account error: {0}")]
    RouterAccountError(#[from] AccountError),

    #[error("surreal db error: {0}")]
    SurrealRouter(#[from] SurrealError),
}

pub struct ApplicationRouter {
    state: TheState,
}

impl ApplicationRouter {
    pub fn init(state: TheState) -> Result<ApplicationRouter, AppRouterError> {
        Ok(ApplicationRouter { state })
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
        match self {
            ArticleError::ArticleCreate(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            ArticleError::CategoryFailed(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            ArticleError::ImageProcessor(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            ArticleError::AudioProcessor(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            ArticleError::VideoProcessor(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            ArticleError::ProcessorError(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
        }
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
    pub async fn start_app_router(&self) -> Router {
        info!("start_app_router()");
        let session_layer = SessionManagerLayer::new(MemoryStore::default())
            // true only https
            .with_secure(true)
            .with_http_only(true)
            // creating articles doesn't require any clos site features
            .with_same_site(Strict);

        let auth_layer = AuthManagerLayerBuilder::new(Backend { db_user: self.state.dbu.clone() }, session_layer).build();

        /*
         * protected routes
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
            .route("/health", get(health::handle_health))
            .layer(middleware::from_fn(auth_middleware));

        /*
         * unprotected routes
         */
        let ret = Router::new()
            .route("/login",
                get(form_login::show_login)
               .post(form_login::handle_login),
            )
            .route("/ping", get("{\"message\": \"app ping\"}"))
            // protected routes
            .merge(protected_routes)
            // everything already served, user requested for non-existent content
            .fallback(show_404)
            .layer(auth_layer)
            .with_state(self.state.clone());

        info!("start_router() finished");
        ret
    }
}

async fn auth_middleware(auth_session: AuthSession, req: Request<Body>, next: Next) -> Response {
    debug!("auth_middleware");
    match auth_session.user {
        Some(user) => {
            // change password
            if user.needs_password_change && req.uri().path() != "/change-password" {
                info!("auth_middleware: needs_password_change redirect");
                return Redirect::to("/change-password").into_response();
            }

            // continue
            if user.role == Editor {
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
    warn!("app router fallback");
    (StatusCode::NOT_FOUND, Html("404, stránka nenalezená".to_string()))
}
