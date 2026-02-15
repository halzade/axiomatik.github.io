use crate::application::admin_form_article::admin_article;
use crate::application::admin_form_article::admin_article::AdminArticleError;
use crate::application::admin_form_user::admin_user;
use crate::application::admin_form_user::admin_user::AdminUserError;
use crate::application::category_finance::finance::FinanceError;
use crate::application::category_republika::republika::RepublikaError;
use crate::application::category_technologie::technologie::TechnologieError;
use crate::application::category_veda::veda::VedaError;
use crate::application::category_zahranici::zahranici::ZahraniciError;
use crate::application::form_account::account;
use crate::application::form_account::account::AccountError;
use crate::application::form_change_password;
use crate::application::form_change_password::change_password::ChangePasswordError;
use crate::application::form_create_article::create_article;
use crate::application::form_create_article::create_article::FormArticleCreateError;
use crate::application::form_create_article::create_article_parser::ArticleCreateError;
use crate::application::form_login::login;
use crate::application::page_all_news::all_news::NewsError;
use crate::application::page_article::article::ArticleError;
use crate::application::page_index::index::IndexError;
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
use database_user::Role::{Admin, Editor};
use http::StatusCode;
use thiserror::Error;
use tower_http::services::{ServeDir, ServeFile};
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

    #[error("admin article error: {0}")]
    RouterAdminArticleError(#[from] AdminArticleError),

    #[error("admin user error: {0}")]
    RouterAdminUserError(#[from] AdminUserError),

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
    pub const fn init(state: TheState) -> Result<Self, AppRouterError> {
        Ok(Self { state })
    }
}

impl IntoResponse for AppRouterError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl IntoResponse for ArticleError {
    fn into_response(self) -> Response {
        match self {
            Self::ArticleCreate(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::CategoryFailed(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::ImageProcessor(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::AudioProcessor(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::VideoProcessor(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Self::ProcessorError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
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

impl IntoResponse for AdminArticleError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl IntoResponse for AdminUserError {
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
         * protected admin routes
         */
        let admin_article_routes = Router::new()
            .route("/", get(admin_article::show_admin_articles))
            .route("/delete/{article_file_name}", post(admin_article::handle_delete_article))
            .layer(middleware::from_fn(admin_middleware));
        let admin_user_routes = Router::new()
            .route("/", get(admin_user::show_admin_users))
            .route("/create", get(admin_user::show_create_user_form).post(admin_user::handle_create_user))
            .route("/delete/{username}", post(admin_user::handle_delete_user))
            .layer(middleware::from_fn(admin_middleware));

        /*
         * protected routes
         */
        let protected_routes = Router::new()
            .nest("/admin_article", admin_article_routes)
            .nest("/admin_user", admin_user_routes)
            .route("/form", get(create_article::show_article_create_form))
            .route("/create", post(create_article::create_article))
            .route("/change-password",
                get(form_change_password::change_password::show_change_password)
               .post(form_change_password::change_password::handle_change_password),
            )
            .route("/account", get(account::show_account))
            .route("/account/update-author", post(account::handle_update_author_name))
            .route("/health", get(health::handle_health))
            .layer(middleware::from_fn(auth_middleware));

        /*
         * unprotected routes
         */
        let ret = Router::new()
            .route("/login",
                get(login::show_login)
               .post(login::handle_login),
            )
            .route("/ping", get("{\"message\": \"app ping\"}"))

            // static content
            .nest_service("/image", ServeDir::new("web/image"))
            .nest_service("/css", ServeDir::new("web/css"))
            .nest_service("/js", ServeDir::new("web/js"))
            .nest_service("/u", ServeDir::new("web/u"))
            .route_service("/favicon.ico", ServeFile::new("web/favicon.ico"))

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
            if user.role == Editor || user.role == Admin {
                info!("auth_middleware: role={:?}, continue", user.role);
                return next.run(req).await;
            }
            info!("auth_middleware: role NOT Editor/Admin: {:?}", user.role);
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

async fn admin_middleware(auth_session: AuthSession, req: Request<Body>, next: Next) -> Response {
    debug!("admin_middleware");
    match auth_session.user {
        Some(user) => {
            if user.role == Admin {
                info!("admin_middleware: role=Admin, continue");
                return next.run(req).await;
            }
            info!("admin_middleware: role NOT Admin: {:?}", user.role);
            Redirect::to("/login").into_response()
        }
        None => {
            info!("admin_middleware: No user in session");
            Redirect::to("/login").into_response()
        }
    }
}

async fn show_404() -> impl IntoResponse {
    warn!("app router fallback");
    (StatusCode::NOT_FOUND, Html("404; stránka nenalezena".to_string()))
}
