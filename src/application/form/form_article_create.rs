use crate::application::article::article::ArticleError;
use crate::application::form::form_article_data_parser::ArticleCreateError;
use crate::data::audio_processor::AudioProcessorError;
use crate::data::image_processor::ImageProcessorError;
use crate::data::video_processor::VideoProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_user::SurrealUserError;
use crate::system::router_app::AuthSession;
use crate::system::server::TheState;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormArticleCreateError {
    #[error("article error")]
    FormArticleError(#[from] ArticleError),

    #[error("article create error")]
    FormArticleCreateError(#[from] ArticleCreateError),

    #[error("image processor error")]
    ImageProcessorError(#[from] ImageProcessorError),

    #[error("audio processor error")]
    AudioProcessorError(#[from] AudioProcessorError),

    #[error("video processor error")]
    VideoProcessorError(#[from] VideoProcessorError),

    #[error("database error")]
    DatabaseError(#[from] SurrealError),

    #[error("surreal user error")]
    FormArticleSurrealUserError(#[from] SurrealUserError),

    #[error("render error")]
    FormArticleRenderError(#[from] askama::Error),
}

#[derive(Template)]
#[template(path = "application/form/form_template.html")]
pub struct FormTemplate {
    pub author_name: String,
}

pub async fn show_article_create_form(
    State(state): State<TheState>,
    auth_session: AuthSession,
) -> Result<Response, FormArticleCreateError> {
    match auth_session.user {
        None => {}
        Some(user) => {
            let user_o = state.dbu.get_user_by_name(&user.username).await?;
            match user_o {
                None => {}
                Some(user) => {
                    return Ok(Html(
                        FormTemplate {
                            author_name: user.author_name,
                        }
                        .render()?,
                    )
                    .into_response());
                }
            }
        }
    }
    Ok(Redirect::to("/login").into_response())
}
