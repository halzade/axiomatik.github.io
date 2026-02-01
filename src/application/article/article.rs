use crate::application::form::form_article_create::FormArticleCreateError;
use crate::application::form::form_article_data_parser;
use crate::data::audio_validator::AudioValidatorError;
use crate::data::{audio_processor, image_processor, video_processor};
use crate::db::database_article;
use crate::db::database_article_data::{Article, MiniArticleData, ShortArticleData};
use crate::system::data_system::DataSystem;
use askama::Template;
use axum::extract::Multipart;
use axum::response::{IntoResponse, Redirect};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArticleError {
    #[error("audio validation error {0}")]
    ArticleAudioArticleError(AudioValidatorError),

    #[error("undefined data type")]
    UndefinedAudioType,

    #[error("unsupported format {0}")]
    UnsupportedAudioType(String),

    #[error("detected empty audio file")]
    DetectedEmptyAudioFile,
}

#[derive(Template)]
#[template(path = "application/article/article_template.html")]
pub struct ArticleTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub author: String,

    pub title: String,
    pub text: String,

    pub image_path: String,
    pub image_desc: String,
    pub video_path: Option<String>,
    pub audio_path: Option<String>,

    pub category: String,
    pub category_display: String,

    pub related_articles: Vec<ShortArticleData>,
    pub articles_most_read: Vec<MiniArticleData>,
}

pub async fn create_article(
    auth_session: crate::system::router::AuthSession,
    multipart: Multipart,
) -> Result<impl IntoResponse, FormArticleCreateError> {
    // TODO X doubled request on create button

    /*
     * Read request data
     */
    let article_data = form_article_data_parser::article_data(auth_session, multipart).await?;
    let article_url = format!("{}.html", article_data.base_file_name.clone());

    /*
     * Validate
     */

    // TODO X Validate text fields, use validator framework instead

    let article_db = Article::try_from(article_data.clone())?;

    // process data image
    image_processor::process_images(
        &article_data.image_data,
        &article_data.base_file_name,
        &article_data.image_ext,
    )?;

    // process data audio
    if article_data.has_audio {
        // validate_audio_data(&article_data.audio_data)?;
        // validate_audio_extension(&article_data.audio_ext)?;
        audio_processor::process_valid_audio(
            &article_data.audio_data,
            &format!("{}.{}", article_data.base_file_name, article_data.audio_ext),
        )?;
    }

    // process data video
    if article_data.has_video {
        // validate_video_data(&article.video_data)?;
        // validate_video_extension(&article.video_data_ext)?;

        video_processor::process_video(
            &article_data.video_data,
            &format!("{}.{}", article_data.base_file_name, article_data.video_ext),
        )?;
    }

    /*
     * Store Article data
     */

    database_article::create_article(article_db)
        .await
        .ok_or(FormArticleCreateError::ArticleCreationInDbFailed)?;

    // invalidate index
    // invalidate category page
    // invalidate related articles

    /*
     * don't render anything
     * redirect to new article
     * trigger to render from template will be handled by router
     */
    Ok(Redirect::to(&article_url).into_response())
}

/**
 * This will process and store the new Article and related files
 * But wont render any html
 */
pub async fn render_article(data_system: &DataSystem) -> Result<String, ArticleError> {
    Ok("".to_string())
}
