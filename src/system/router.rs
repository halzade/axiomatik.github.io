use crate::application::account::form_account;
use crate::application::article::form_article_create;
use crate::application::article::form_article_data_parser::ArticleCreateError;
use crate::application::change_password::form_change_password;
use crate::application::login::form_login;
use crate::db::database_user;
use crate::system::data_system::DataSystem;
use crate::system::data_updates::DataUpdates;
use crate::system::server::{ApplicationStatus, AUTH_COOKIE};
use crate::system::{data_system, data_updates, heartbeat};
use crate::web::search::search;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, get_service, post};
use axum::{middleware, Router};
use axum_extra::extract::CookieJar;
use http::{Request, StatusCode};
use std::fs;
use std::sync::Arc;
use axum::extract::OriginalUri;
use thiserror::Error;
use tokio::sync::RwLock;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{debug, error, info};

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("create article error: {0}")]
    RouterArticleError(#[from] ArticleCreateError),
}

pub struct ApplicationRouter {
    data_system: DataSystem,
    data_updates: DataUpdates,
}

pub fn new() -> ApplicationRouter {
    ApplicationRouter {
        data_system: data_system::new(),
        data_updates: data_updates::new(),
    }
}
impl ApplicationRouter {

    #[rustfmt::skip]
    pub async fn start_router(&self, status: ApplicationStatus) -> Router {
        info!("start_router()");

        /*
         * Protected routes
         */
        let protected_routes = Router::new()
            .route("/form", get(form_article_create::show_form))
            .route("/create", post(form_article_create::create_article))
            .route("/change-password",
                 get(form_change_password::show_change_password)
                .post(form_change_password::handle_change_password),
            )
            .route("/account", get(form_account::show_account))
            .route("/account/update-author", post(form_account::handle_update_author_name))
            .route("/heartbeat", get(heartbeat::handle_heartbeat))
            /*
             * Authorization
             */
            .route_layer(
                middleware::from_fn_with_state(
                status.clone(),
                auth_middleware,
            ))
            .with_state(status.clone());

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
            .route("/*.html", get(self.serve_html_content()))
            // web app
            .merge(protected_routes)
            // everything already served, user requested for non-existent content
            .fallback(show_404)
            .with_state(status);

        info!("start_router() finished");
        ret
    }

    pub async fn serve_html_content(
        &self,
        ouri: OriginalUri,
    ) -> Result<impl IntoResponse, RouterError> {
        /*
         * What kind of content is it?
         */
        let mut is_index = false;
        let mut is_finance = false;
        let mut is_republika = false;
        let mut is_veda = false;
        let mut is_technologie = false;
        let mut is_zahranici = false;
        let mut is_news = false;
        let mut is_article = false;

        match ouri {
            OriginalUri(url) => match url.path() {
                "/index.html" => is_index = true,
                "/finance.html" => is_finance = true,
                "/news.html" => is_news = true,
                "/republika.html" => is_republika = true,
                "/technologie.html" => is_technologie = true,
                "/veda.html" => is_veda = true,
                "/zahranici.html" => is_zahranici = true,
                // it probably is an Article
                _ => is_article = true,
            },
        }

        if is_index {
            let d = self.data_updates.index_lag()?;

        }



        /*
         * serve the content
         */

    }
}
async fn auth_middleware(jar: CookieJar, req: Request<Body>, next: Next) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let user_o = database_user::get_user(cookie.value()).await;
        match user_o {
            None => {
                // error -> login
                error!("User not found");
                return Redirect::to("/login").into_response();
            }
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
