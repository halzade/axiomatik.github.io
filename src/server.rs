use axiomatik_web::{
    create_article, db, handle_change_password, handle_fallback, handle_login, handle_search,
    handle_update_author_name, show_account, show_change_password, show_form, show_login,
};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::sync::Arc;
use axum::body::Body;
use axum::extract::State;
use axum::middleware::Next;
use axum_extra::extract::CookieJar;
use http::Request;

// TODO pub?
pub const AUTH_COOKIE: &str = "axiomatik_auth";

pub fn router(db: Arc<db::Database>) -> Router {
    let protected_routes = Router::new()
        .route("/form", get(show_form))
        .route("/create", post(create_article))
        .route(
            "/change-password",
            get(show_change_password).post(handle_change_password),
        )
        .route("/account", get(show_account))
        .route("/account/update-author", post(handle_update_author_name))
        .layer(middleware::from_fn_with_state(db.clone(), auth_middleware));

    Router::new()
        .route("/", get(|| async { Redirect::to("/index.html") }))
        .route("/login", get(show_login).post(handle_login))
        .route("/search", get(handle_search))
        .merge(protected_routes)

        // serve static content
        // TODO serve only html, css, js
        .fallback(handle_fallback)
        .with_state(db)
}

async fn auth_middleware(
    State(db): State<Arc<db::Database>>,
    jar: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        if let Ok(Some(user)) = db.get_user(cookie.value()).await {
            if user.needs_password_change && req.uri().path() != "/change-password" {
                return Redirect::to("/change-password").into_response();
            }
            if user.role == db::Role::Editor {
                return next.run(req).await;
            }
        }
    }
    Redirect::to("/login").into_response()
}