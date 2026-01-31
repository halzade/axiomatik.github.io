use crate::application::account::form_account;
use crate::application::article::form_article_create;
use crate::application::article::form_article_create::FormError;
use crate::application::article::form_article_data_parser::ArticleCreateError;
use crate::application::change_password::form_change_password;
use crate::application::login::form_login;
use crate::db::database_user;
use crate::system::data_system::{DataSystem, DataSystemError};
use crate::system::data_updates::{DataUpdates, DataUpdatesError};
use crate::system::server::{ApplicationStatus, AUTH_COOKIE};
use crate::system::{data_system, data_updates, heartbeat};
use crate::web::index::index;
use crate::web::index::index::IndexError;
use crate::web::search::search;
use axum::body::Body;
use axum::extract::OriginalUri;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, get_service, post};
use axum::{middleware, Router};
use axum_core::extract::Request;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use std::convert::Infallible;
use std::fs;
use thiserror::Error;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{debug, error, info};

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("create article error: {0}")]
    RouterArticleError(#[from] ArticleCreateError),

    #[error("data update error: {0}")]
    RouterDataUpdate(#[from] DataUpdatesError),

    #[error("data update system: {0}")]
    RouterDataSystem(#[from] DataSystemError),

    #[error("form error: {0}")]
    RouterForm(#[from] FormError),

    #[error("index error: {0}")]
    RouterIndexError(#[from] IndexError),

    #[error("response infallible: {0}")]
    RouterInfallible(#[from] Infallible),
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

impl IntoResponse for RouterError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
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
            .route("/*.html", get(|ori_uri: OriginalUri, request| async move {
                    self.serve_html_content(ori_uri, request).await
                }
            ))
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
        ori_uri: OriginalUri,
        request: Request<Body>,
    ) -> Result<Response, RouterError> {
        /*
         * What kind of content is it?
         */
        match &ori_uri {
            OriginalUri(uri) => match uri.path() {
                "/index.html" => {
                    if self.update_index_now() {
                        index::render_index().await?;
                    }
                }
                "/finance.html" => {}
                "/news.html" => {}
                "/republika.html" => {}
                "/technologie.html" => {}
                "/veda.html" => {}
                "/zahranici.html" => {}
                _ => {
                    // forgot some or Article
                }
            },
        }
        let service = ServeFile::new(ori_uri.path().to_string());
        let response = service.oneshot(request).await?;

        Ok(response.into_response())
    }

    fn update_index_now(&self) -> bool {
        let index_invalid = self.data_updates.index_valid();
        if !index_invalid {
            // index invalidated because of a new Article published, render
            return true;
        }

        let my_last_update = self.data_updates.index_updated();
        let weather_last_update = self.data_system.weather_last_update();
        if my_last_update.clone() < weather_last_update {
            // weather changed, render
            return true;
        }

        let date_last_update = self.data_system.date_last_update();
        if my_last_update < date_last_update {
            // date changed, render
            return true;
        }
        false
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
