use crate::{
    database, form_account, form_change_password, form_login, form_new_article, form_search,
};
use axum::body::Body;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{middleware, Router};
use axum_extra::extract::CookieJar;
use http::{Request, StatusCode};
use std::fs;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::{debug, error};

// TODO
// TODO
// TODO
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

pub async fn started() -> bool {
    APP_STATUS.lock().await.eq(&ApplicationStatus::Started)
}

pub async fn start() {
    *APP_STATUS.lock().await = ApplicationStatus::Started
}

pub async fn router() -> Router {
    let status = *APP_STATUS.lock().await;
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
    Router::new()
        .route("/", get(|| async { Redirect::to("/index.html") }))
        .route(
            "/login",
            get(form_login::show_login).post(form_login::handle_login),
        )
        .route("/search", get(form_search::handle_search))
        .merge(protected_routes)
        // serve static content
        .fallback(handle_fallback)
        .with_state(status)
}

// TODO
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

async fn handle_fallback(req: Request<Body>) -> Response {
    let path = req.uri().path();
    let file_path = if path == "/" || path.is_empty() {
        "index.html".to_string()
    } else {
        path.trim_start_matches('/').to_string()
    };

    // TODO
    if file_path.ends_with(".html") {
        if let Ok(content) = fs::read_to_string(&file_path) {
            return Html(content).into_response();
        }
    }

    // Default to serving from ServeDir
    // TODO serve only css and js
    let service = ServeDir::new(".");
    match tower::ServiceExt::oneshot(service, req).await {
        Ok(res) => {
            if res.status() == StatusCode::NOT_FOUND {
                show_404().await.into_response()
            } else {
                res.into_response()
            }
        }
        Err(_) => show_404().await.into_response(),
    }
}

async fn show_404() -> impl IntoResponse {
    debug!("show_404 called");
    (
        StatusCode::NOT_FOUND,
        Html(fs::read_to_string("404.html").unwrap_or_else(|_| "404 Not Found".to_string())),
    )
}
