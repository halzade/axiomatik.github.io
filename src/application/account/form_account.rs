use crate::data::text_validator;
use crate::db::database_article_data::AccountArticleData;
use crate::system::router_app::AuthSession;
use crate::system::server::TheState;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use tracing::debug;
use validator::Validate;
use crate::db::database_article::SurrealArticleError;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("user not found")]
    AccountUserNotFound,

    #[error("render error")]
    AccountRender(#[from] askama::Error),

    #[error("surreal account error")]
    AccountSurreal(#[from] SurrealArticleError),
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
    pub articles: Vec<AccountArticleData>,
}

pub async fn show_account(
    State(state): State<TheState>,
    auth_session: AuthSession,
) -> Result<Response, AccountError> {
    debug!("show_account()");
    match auth_session.user {
        None => {
            debug!("show_account: no user in session");
            Ok(Redirect::to("/login").into_response())
        }
        Some(user) => {
            debug!("show_account: user={}", user.username);
            let account_articles = state.dba.articles_by_username(&user.username, 100).await.map_err(|e| {
                debug!("show_account: articles_by_username error: {:?}", e);
                AccountError::AccountSurreal(e)
            })?;
            debug!("show_account: found {} articles", account_articles.len());
            Ok(Html(
                AccountTemplate {
                    username: user.username,
                    author_name: user.author_name,
                    articles: account_articles,
                }
                .render()?,
            )
            .into_response())
        }
    }
}

pub async fn handle_update_author_name(
    State(state): State<TheState>,
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
            let _ = state.dbu.update_user_author_name(&user.username, &payload.author_name).await;
            debug!("always redirect to account");
            Redirect::to("/account").into_response()
        }
        None => {
            debug!("failed");
            Redirect::to("/login").into_response()
        }
    }
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
