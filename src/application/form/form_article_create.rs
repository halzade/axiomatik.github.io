use crate::application::article::article::ArticleError;
use crate::application::form::form_article_data_parser::ArticleCreateError;
use crate::data::audio_processor::AudioProcessorError;
use crate::data::image_processor::ImageProcessorError;
use crate::data::video_processor::VideoProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_user;
use crate::system::server::AUTH_COOKIE;
use askama::Template;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
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
}

#[derive(Template)]
#[template(path = "application/form/form_template.html")]
pub struct FormTemplate {
    pub author_name: String,
}

pub async fn show_article_create_form(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let user_o = database_user::get_user_by_name(cookie.value()).await;
        match user_o {
            None => {}
            Some(user) => {
                return Html(
                    FormTemplate {
                        author_name: user.author_name,
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
