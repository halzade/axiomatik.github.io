use crate::data::text_validator::validate_input_simple;
use crate::db::database_user::{SurrealUserError, User};
use crate::system::router_app::AuthSession;
use crate::system::server::TheState;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use bcrypt::verify;
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use tracing::log::debug;
use tracing::{error, info, warn};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("user not found")]
    UserNotFound,

    #[error("invalid password")]
    InvalidPassword,

    #[error("surreal user error")]
    AuthSurrealUserError(#[from] SurrealUserError),
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Template)]
#[template(path = "application/login/form_login_template.html")]
pub struct LoginTemplate {
    pub error: bool,
}

pub async fn show_login() -> impl IntoResponse {
    Html(LoginTemplate { error: false }.render().unwrap())
}

pub async fn handle_login(
    mut auth_session: AuthSession,
    Form(payload): Form<LoginPayload>,
) -> Response {
    debug!("Handling login request");
    if validate_input_simple(&payload.username).is_err()
        || validate_input_simple(&payload.password).is_err()
    {
        debug!("...Invalid input");
        return StatusCode::BAD_REQUEST.into_response();
    }

    let credentials = (payload.username.clone(), payload.password.clone());

    match auth_session.authenticate(credentials).await {
        Ok(Some(user)) => {
            if let Err(_) = auth_session.login(&user).await {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }

            info!(user = %user.username, "User logged in successfully");

            if user.needs_password_change {
                debug!("Redirecting to change password");
                Redirect::to("/change-password").into_response()
            } else {
                debug!("Redirecting to account");
                Redirect::to("/account").into_response()
            }
        }
        Ok(None) => {
            warn!(username = %payload.username, "Failed login attempt: Invalid credentials");
            Html(LoginTemplate { error: true }.render().unwrap()).into_response()
        }
        Err(_) => {
            error!(username = %payload.username, "Authentication error");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn authenticate_user(
    state: &TheState,
    username: &str,
    password: &str,
) -> Result<User, AuthError> {
    let user_o = state.dbu.get_user_by_name(username).await?;
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
