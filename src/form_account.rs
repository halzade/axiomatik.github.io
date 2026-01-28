use crate::database;
use crate::database::Article;
use crate::server::AUTH_COOKIE;
use crate::validation::validate_input_simple;
use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("User not found")]
    UserNotFound,
}

#[derive(Deserialize)]
pub struct UpdateAuthorNamePayload {
    pub author_name: String,
}

#[derive(Template)]
#[template(path = "../pages/account.html")]
pub struct AccountTemplate {
    pub username: String,
    pub author_name: String,
    pub articles: Vec<Article>,
}

pub async fn show_account(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let user_o = database::get_user(cookie.value()).await;
        return match user_o {
            None => Redirect::to("/login").into_response(),
            Some(user) => {
                let articles_r = database::articles_by_username(&user.username).await;
                let articles = articles_r.unwrap_or_else(|e| {
                    error!("Failed to fetch articles for user {}: {}", user.username, e);
                    Vec::new()
                });

                Html(
                    AccountTemplate {
                        username: user.username,
                        author_name: user.author_name,
                        articles,
                    }
                    .render()
                    .unwrap(),
                )
                .into_response()
            }
        };
    }
    Redirect::to("/login").into_response()
}

pub async fn handle_update_author_name(
    jar: CookieJar,
    Form(payload): Form<UpdateAuthorNamePayload>,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value();
        if validate_input_simple(&payload.author_name).is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match update_author_name(username, &payload.author_name).await {
            Ok(_) => Redirect::to("/account").into_response(),
            Err(_) => Redirect::to("/account").into_response(), // Simple error handling for now
        }
    } else {
        Redirect::to("/login").into_response()
    }
}

async fn update_author_name(username: &str, author_name: &str) -> Result<(), AccountError> {
    match database::get_user(username).await {
        Some(mut user) => {
            user.author_name = author_name.to_string();
            database::update_user(user).await;
            Ok(())
        }
        None => Err(AccountError::UserNotFound),
    }
}
