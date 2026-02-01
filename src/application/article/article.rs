use crate::application::form::form_article_create::FormArticleCreateError;
use crate::application::form::form_article_data_parser;
use crate::application::form::form_article_data_parser::ArticleData;
use crate::data::audio_validator::AudioValidatorError;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
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
    // TODO X article already exists
    // TODO X doubled request on create button

    /*
     * Read request data
     */
    let article_data = form_article_data_parser::article_data(auth_session, multipart).await?;

    /*
     * Create Article, process the data
     */
    let article_url = process_article_create(article_data).await?;

    Ok(Redirect::to(&article_url).into_response())
}

/**
 * This will process and store the new Article and related files
 * But wont render any html
 */
pub async fn process_article_create(article_data: ArticleData) -> Result<String, ArticleError> {
    /*
     * Validate
     */

    // TODO X Validate text fields, use validator framework instead

    if article_data.has_audio {
        // validate_audio_data(&article_data.audio_data)?;
        // validate_audio_extension(&article_data.audio_ext)?;
    }
    if article_data.has_video {
        // validate_video_data(&article.video_data)?;
        // validate_video_extension(&article.video_data_ext)?;
    }


    // process data image
    // process data audio
    // process data video


    /*
     * Prepare Article data
     */

    // let article_db = database_article::create_article();


    /*
     * Index page
     */
    // invalidate index

    /*
     * category page
     */
    // invalidate category page

    /*
     * category page
     */
    // invalidate related articles


    // let html_content = article_template.render().unwrap();

    // Store in DB

    // don't render anything
    Ok("".to_string())
}
