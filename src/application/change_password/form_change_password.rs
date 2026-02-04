use crate::data::text_validator::validate_input_simple;
use crate::db::database_user;
use crate::system::router::AuthSession;
use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;
use thiserror::Error;
use tracing::error;
use crate::application::change_password::form_change_password::ChangePasswordError::UserNotFound;

#[derive(Debug, Error)]
pub enum ChangePasswordError {
    #[error("Password too short")]
    PasswordTooShort,

    #[error("Password validation failed")]
    PasswordValidation,

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

pub async fn show_change_password(auth_session: AuthSession) -> Response {
    if let Some(user) = auth_session.user {
        Html(
            ChangePasswordTemplate {
                error: false,
                username: user.username.clone(),
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
    mut auth_session: AuthSession,
    Form(payload): Form<ChangePasswordPayload>,
) -> Result<Response, ChangePasswordError> {
    if let Some(ref user_auth) = auth_session.user {
        let username = user_auth.username.clone();
        if validate_input_simple(&payload.new_password).is_err() {
            return Err(ChangePasswordError::PasswordTooShort);
        }
        change_password(&username, &payload.new_password).await?;
        /*
         * successfully changed password
         */
        /*
         * internally re-login user to update session with new user data
         */
        let updated_user_o = database_user::get_user_by_name(&username).await?;
        match updated_user_o {
            None => {
                Err(UserNotFound)
            }
            Some(updated_user) => {
                let _ = auth_session.login(&updated_user).await;
                return Ok(Redirect::to("/account").into_response());
            }
        }
        Ok(Html(
            ChangePasswordTemplate { error: true, username }
                .render()
                .unwrap())
            .into_response())
    }
    return Ok(Redirect::to("/login").into_response()););
}

async fn change_password(username: &str, new_password: &str) -> Result<(), ChangePasswordError> {
    if new_password.len() < 3 {
        return Err(ChangePasswordError::PasswordTooShort);
    }
    database_user::update_user_password(username.into(), hash(new_password, DEFAULT_COST)?).await?;
    Ok(())
}
