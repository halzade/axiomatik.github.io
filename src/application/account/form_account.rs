use crate::data::text_validator;
use crate::db::database_article_data::Article;
use crate::db::{database_article, database_user};
use crate::system::router::AuthSession;
use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, error};
use validator::Validate;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("User not found")]
    UserNotFound,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAuthorNamePayload {
    #[validate(
        length(min = 3, max = 26),
        regex(
            path = "&*text_validator::AUTHOR_NAME_REGEX",
            message = "author_name may contain only Latin letters, numbers, spaces, or underscores and must be 3–26 characters long"
        )
    )]
    pub author_name: String,
}

#[derive(Template)]
#[template(path = "application/account/form_account_template.html")]
pub struct AccountTemplate {
    pub username: String,
    pub author_name: String,
    pub articles: Vec<Article>,
}

pub async fn show_account(auth_session: AuthSession) -> Response {
    match auth_session.user {
        None => Redirect::to("/login").into_response(),
        Some(user) => {
            let articles_r = database_article::articles_by_username(&user.username, 100).await;
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
    }
}

pub async fn handle_update_author_name(
    auth_session: AuthSession,
    Form(payload): Form<UpdateAuthorNamePayload>,
) -> Response {
    debug!("handle_update_author_name()");
    match auth_session.user {
        Some(user) => {
            debug!("session user: {}", user.username);

            if let Err(errors) = payload.validate() {
                println!("{:#?}", errors);
                return StatusCode::BAD_REQUEST.into_response();
            }

            debug!("validation ok");
            match update_author_name(&user.username, &payload.author_name).await {
                _ => {
                    debug!("always redirect to account");
                    Redirect::to("/account").into_response()
                }
            }
        }
        None => {
            debug!("failed");
            Redirect::to("/login").into_response()
        }
    }
}

async fn update_author_name(username: &str, author_name: &str) -> Result<(), AccountError> {
    database_user::update_user_author_name(username, author_name)
        .await
        .map_err(|e| {
            error!("Failed to update user: {}", e);
            AccountError::UserNotFound
        })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_validate_author_name() {
        assert!(UpdateAuthorNamePayload{ author_name: "Hello".to_string() }.validate().is_ok());
        assert!(UpdateAuthorNamePayload{ author_name: "ValidName123".to_string() }.validate().is_ok());
        assert!(UpdateAuthorNamePayload{ author_name: "Valid Name".to_string() }.validate().is_ok());
        assert!(UpdateAuthorNamePayload{ author_name: "ĚŠČŘŽÝÁÍÉÓŮÚ".to_string() }.validate().is_ok());
        assert!(UpdateAuthorNamePayload{ author_name: "ěščřžýáíéóůú".to_string() }.validate().is_ok());

        assert!(UpdateAuthorNamePayload{ author_name: "Some  Name".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "Some Name ".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: " Some Name".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: " Some Name".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: " Some   Name ".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "Some_Name".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "Some Name?".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: ".Some Name".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "Some^Name".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "SomeName!".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "!SomeName".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "SN".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "x!".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: ".&$@Z!".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: ".&$@)!".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: ".&$@)P{{{{P{P{{}}}}{}{}}{!".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: " ".to_string() }.validate().is_err());
        assert!(UpdateAuthorNamePayload{ author_name: "    ".to_string() }.validate().is_err());
        
        assert!(UpdateAuthorNamePayload{ author_name: "12345678901234567890123456".to_string() }.validate().is_ok());
        assert!(UpdateAuthorNamePayload{ author_name: "123456789012345678901234567".to_string() }.validate().is_err());
    }
}
