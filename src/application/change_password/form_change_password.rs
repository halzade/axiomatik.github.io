use crate::db::database_user;
use crate::system::server::AUTH_COOKIE;
use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use axum_extra::extract::CookieJar;
use bcrypt::{hash, DEFAULT_COST};
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use tracing::error;
use crate::data::text_validator::validate_input_simple;

#[derive(Debug, Error)]
pub enum ChangePasswordError {
    #[error("Password too short")]
    PasswordTooShort,

    #[error("User not found")]
    UserNotFound,

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

#[derive(Deserialize)]
pub struct ChangePasswordPayload {
    pub new_password: String,
}

#[derive(Template)]
#[template(path = "application/change_password/form_change_password_template.html")]
pub struct ChangePasswordTemplate {
    pub error: bool,
    pub username: String,
}

pub async fn show_change_password(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value().to_string();
        Html(
            ChangePasswordTemplate {
                error: false,
                username,
            }
            .render()
            .unwrap(),
        )
        .into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

pub async fn handle_change_password(
    jar: CookieJar,
    Form(payload): Form<ChangePasswordPayload>,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value();
        if validate_input_simple(&payload.new_password).is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match change_password(username, &payload.new_password).await {
            Ok(_) => Redirect::to("/account").into_response(),
            Err(e) => {
                error!("{:?}", e);
                Html(
                    ChangePasswordTemplate {
                        error: true,
                        username: username.to_string(),
                    }
                    .render()
                    .unwrap(),
                )
                .into_response()
            }
        }
    } else {
        Redirect::to("/login").into_response()
    }
}

async fn change_password(username: &str, new_password: &str) -> Result<(), ChangePasswordError> {
    if new_password.len() < 3 {
        return Err(ChangePasswordError::PasswordTooShort);
    }

    let user_o = database_user::get_user(username).await;
    match user_o {
        None => Err(ChangePasswordError::UserNotFound),
        Some(mut user) => {
            let password_hash = hash(new_password, DEFAULT_COST)?;
            user.password_hash = password_hash;
            user.needs_password_change = false;
            database_user::update_user(user).await;
            Ok(())
        }
    }
}
