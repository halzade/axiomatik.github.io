use crate::application::form_change_password::change_password::ChangePasswordError::UserNotFound;
use crate::data::text_validator::validate_input_simple;
use crate::db::database::SurrealError;
use crate::db::database_user::SurrealUserError;
use crate::system::router_app::AuthSession;
use crate::system::server::TheState;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChangePasswordError {
    #[error("password too short")]
    PasswordTooShort,

    #[error("password validation failed")]
    PasswordValidation,

    #[error("render error: {0}")]
    Render(#[from] askama::Error),

    #[error("user not found")]
    UserNotFound,

    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("surreal user error: {0}")]
    SurrealUserChangePasswordError(#[from] SurrealUserError),

    #[error("surreal change password error: {0}")]
    SurrealChangePasswordError(#[from] SurrealError),
}

#[derive(Deserialize)]
pub struct ChangePasswordPayload {
    pub new_password: String,
}

#[derive(Template)]
#[template(path = "application/form_change_password/change_password_template.html")]
pub struct ChangePasswordTemplate {
    pub error: bool,
    pub username: String,
}

pub async fn show_change_password(
    auth_session: AuthSession,
) -> Result<Response, ChangePasswordError> {
    if let Some(user) = auth_session.user {
        Ok(Html(ChangePasswordTemplate { error: false, username: user.username }.render()?)
            .into_response())
    } else {
        Ok(Redirect::to("/login").into_response())
    }
}

pub async fn handle_change_password(
    State(state): State<TheState>,
    mut auth_session: AuthSession,
    Form(payload): Form<ChangePasswordPayload>,
) -> Result<Response, ChangePasswordError> {
    if let Some(ref user_auth) = auth_session.user {
        let username = user_auth.username.clone();
        if validate_input_simple(&payload.new_password).is_err() {
            return Err(ChangePasswordError::PasswordTooShort);
        }

        if payload.new_password.len() < 3 {
            return Err(ChangePasswordError::PasswordTooShort);
        }
        state
            .dbu
            .update_user_password(username.clone(), hash(&payload.new_password, DEFAULT_COST)?)
            .await?;

        /*
         * successfully changed the password
         */
        /*
         * internally re-login user to update the session with new user data
         */
        let updated_user_o = state.dbu.get_user_by_name(&username).await?;
        return match updated_user_o {
            None => Err(UserNotFound),
            Some(updated_user) => {
                let _ = auth_session.login(&updated_user).await;
                Ok(Redirect::to("/account").into_response())
            }
        };
    }
    Ok(Redirect::to("/login").into_response())
}
