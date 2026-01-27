use crate::database;
use crate::database::User;
use crate::server::AUTH_COOKIE;
use crate::validation::validate_input_simple;
use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use bcrypt::verify;
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid password")]
    InvalidPassword,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Template)]
#[template(path = "../pages/login.html")]
pub struct LoginTemplate {
    pub error: bool,
}

pub async fn show_login() -> impl IntoResponse {
    Html(LoginTemplate { error: false }.render().unwrap())
}

pub async fn handle_login(jar: CookieJar, Form(payload): Form<LoginPayload>) -> Response {
    if validate_input_simple(&payload.username).is_err()
        || validate_input_simple(&payload.password).is_err()
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let username = &payload.username;
    match authenticate_user(username, &payload.password).await {
        Ok(user) => {
            info!(user = %user.username, "User logged in successfully");

            let jar = jar.add(
                Cookie::build((AUTH_COOKIE, user.username))
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Strict)
                    .path("/")
                    .build(),
            );

            if user.needs_password_change {
                (jar, Redirect::to("/change-password")).into_response()
            } else {
                (jar, Redirect::to("/account")).into_response()
            }
        }
        Err(e) => {
            warn!(username = %payload.username, error = %e, "Failed login attempt");
            (jar, Html(LoginTemplate { error: true }.render().unwrap())).into_response()
        }
    }
}

pub async fn authenticate_user(username: &str, password: &str) -> Result<User, AuthError> {
    let user_o = database::get_user(username).await;
    match user_o {
        None => Err(AuthError::UserNotFound),
        Some(user) => {
            if verify(password, &user.password_hash).unwrap_or(false) {
                Ok(user)
            } else {
                Err(AuthError::InvalidPassword)
            }
        }
    }
}
