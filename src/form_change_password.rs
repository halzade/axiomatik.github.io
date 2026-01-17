use crate::database::User;
use crate::server::AUTH_COOKIE;
use crate::templates::ChangePasswordPayload;
use crate::validation::validate_input;
use crate::{database, templates};
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use axum_extra::extract::CookieJar;
use bcrypt::{hash, DEFAULT_COST};
use http::StatusCode;
use std::sync::Arc;
use tracing::error;

pub async fn show_change_password(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value().to_string();
        Html(
            templates::ChangePasswordTemplate {
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
        if validate_input(&payload.new_password).is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match change_password(username, &payload.new_password).await {
            Ok(_) => Redirect::to("/account").into_response(),
            Err(e) => {
                error!("{:?}", e);
                Html(
                    templates::ChangePasswordTemplate {
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

async fn change_password(username: &str, new_password: &str) -> Result<(), String> {
    if new_password.len() < 3 {
        return Err("Heslo musí být delší".to_string());
    }

    let user_o = database::get_user(username).await;
    match user_o {
        None => Err("User not found".to_string()),
        Some(mut user) => {
            let password_hash = hash(new_password, DEFAULT_COST).unwrap();
            user.password_hash = password_hash;
            user.needs_password_change = false;
            database::update_user(user).await;
            Ok(())
        }
    }
}
