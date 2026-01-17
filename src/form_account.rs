use crate::database;
use crate::server::AUTH_COOKIE;
use crate::templates::{AccountTemplate, UpdateAuthorNamePayload};
use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use axum_extra::extract::CookieJar;
use http::StatusCode;

pub async fn show_account(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let user_o = database::get_user(cookie.value()).await;
        match user_o {
            None => {
                return Redirect::to("/login").into_response();
            }
            Some(user) => {
                let articles = database::get_articles_by_username(&user.username).await;
                return Html(
                    AccountTemplate {
                        username: user.username,
                        author_name: user.author_name,
                        articles,
                    }
                    .render()
                    .unwrap(),
                )
                .into_response();
            }
        }
    }
    Redirect::to("/login").into_response()
}

pub async fn handle_update_author_name(
    jar: CookieJar,
    Form(payload): Form<UpdateAuthorNamePayload>,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value();
        if validate_input(&payload.author_name).is_err() {
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

async fn update_author_name(username: &str, author_name: &str) -> Result<(), String> {
    match database::get_user(username).await {
        Some(mut user) => {
            user.author_name = author_name.to_string();
            database::update_user(user);
            Ok(())
        }
        None => Err("User not found".to_string()),
    }
}
