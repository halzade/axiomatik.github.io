use crate::{
    database, form_account, form_change_password, form_login, form_new_article, form_search,
};
use axum::body::Body;
use axum::handler::HandlerWithoutStateExt;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{middleware, Router};
use axum_extra::extract::CookieJar;
use http::{Request, StatusCode};
use std::fs;
use std::sync::LazyLock;
use tokio::signal;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::{debug, error, info};

pub const AUTH_COOKIE: &str = "axiomatik_auth";

pub static APP_STATUS: LazyLock<Mutex<ApplicationStatus>> =
    LazyLock::new(|| Mutex::new(ApplicationStatus::new()));

#[derive(Clone, Copy, PartialEq)]
pub enum ApplicationStatus {
    Started,
    Off,
}

impl ApplicationStatus {
    pub fn new() -> Self {
        Self::Off
    }
}

pub async fn is_started() -> bool {
    APP_STATUS.lock().await.eq(&ApplicationStatus::Started)
}

async fn start() {
    *APP_STATUS.lock().await = ApplicationStatus::Started
}

pub async fn start_router() -> Router {
    info!("start_router()");

    let status = *APP_STATUS.lock().await;

    if is_started().await {
        info!("Application already started");
        info!("Shutting down gracefully...");
        signal::ctrl_c().await.ok();
    }

    start().await;

    /*
     * Protected routes
     */
    let protected_routes = Router::new()
        .route("/form", get(form_new_article::show_form))
        .route("/create", post(form_new_article::create_article))
        .route(
            "/change-password",
            get(form_change_password::show_change_password)
                .post(form_change_password::handle_change_password),
        )
        .route("/account", get(form_account::show_account))
        .route(
            "/account/update-author",
            post(form_account::handle_update_author_name),
        )
        /*
         * Authentication
         */
        .route_layer(middleware::from_fn_with_state(
            status.clone(),
            auth_middleware,
        ))
        .with_state(status.clone());

    /*
     * Unprotected routes
     */
    let ret = Router::new()
        .route("/", get(|| async { Redirect::to("/index.html") }))
        .route(
            "/login",
            get(form_login::show_login).post(form_login::handle_login),
        )
        .route("/search", get(form_search::handle_search))
        // serve static content
        .nest_service("/css", ServeDir::new("web/css"))
        .nest_service("/js", ServeDir::new("web/js"))
        .nest_service("/uploads", ServeDir::new("web/uploads"))
        // web app
        .merge(protected_routes)
        // non existent content
        .fallback_service(ServeDir::new("web").not_found_service(show_404.into_service()))
        .with_state(status);

    info!("start_router() finished");
    ret
}

async fn auth_middleware(jar: CookieJar, req: Request<Body>, next: Next) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let user_o = database::get_user(cookie.value()).await;
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
                if user.role == database::Role::Editor {
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

    match fs::read_to_string("web/404.html") {
        Ok(content) => (StatusCode::NOT_FOUND, Html(content)),
        Err(err) => {
            debug!("Failed to read 404.html: {err}");
            (StatusCode::NOT_FOUND, Html("404 Not Found".to_string()))
        }
    }
}
