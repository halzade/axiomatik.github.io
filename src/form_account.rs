use crate::database::Database;
use crate::server::AUTH_COOKIE;
use crate::templates::{AccountTemplate, UpdateAuthorNamePayload};
use askama::Template;
use axiomatik_web::{auth, db};
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use std::sync::Arc;

pub async fn show_account(State(db): State<Arc<db::Database>>, jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        if let Ok(Some(user)) = db.get_user(cookie.value()).await {
            let articles = db
                .get_articles_by_username(&user.username)
                .await
                .unwrap_or_default();

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
    Redirect::to("/login").into_response()
}

pub async fn handle_update_author_name(
    State(db): State<Arc<db::Database>>,
    jar: CookieJar,
    Form(payload): Form<UpdateAuthorNamePayload>,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value();
        if validate_input(&payload.author_name).is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match auth::update_author_name(&db, username, &payload.author_name).await {
            Ok(_) => Redirect::to("/account").into_response(),
            Err(_) => Redirect::to("/account").into_response(), // Simple error handling for now
        }
    } else {
        Redirect::to("/login").into_response()
    }
}

async fn update_author_name(
    db: &Database, // TODO never as parameter
    username: &str,
    author_name: &str,
) -> Result<(), String> {
    match db.get_user(username).await {
        Ok(Some(mut user)) => {
            user.author_name = author_name.to_string();
            db.update_user(user).await.map_err(|e| e.to_string())?;
            Ok(())
        }
        Ok(None) => Err("User not found".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}
