use crate::data::audio_extractor::{extract_audio_data, AudioExtractorError};
use crate::data::image_extractor::{extract_image_data, ImageExtractorError};
use crate::data::library;
use crate::data::text_extractor::{
    extract_required_string, extract_required_text, TextExtractorError,
};
use crate::data::video_extractor::{extract_video_data, VideoExtractorError};
use axum::extract::Multipart;
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]
pub enum ArticleCreateError {
    #[error("text extraction failed: {0}")]
    ArticleTextExtractorError(#[from] TextExtractorError),

    #[error("image extraction failed: {0}")]
    ImageAudioExtractorError(#[from] ImageExtractorError),

    #[error("audio extraction failed: {0}")]
    ArticleAudioExtractorError(#[from] AudioExtractorError),

    #[error("video extraction failed: {0}")]
    ArticleVideoExtractorError(#[from] VideoExtractorError),

    #[error("image was required")]
    ImageRequired,

    #[error("image description was required")]
    ImageDescriptionRequired,

    #[error("Unknown field {0}")]
    UnknownField(String),

    #[error("user required")]
    UserRequired,
}

/**
 * Parsed result of new Article /create
 * Contains sanitized raw text and raw data
 * Text will be processed only for the Template
 */
#[derive(Debug, Clone)]
pub struct ArticleUpload {
    pub is_main: bool,
    pub is_exclusive: bool,
    pub author: String,
    pub username: String,

    pub title: String,
    pub text_raw: String,
    pub short_text_raw: String,
    pub mini_text_raw: String,
    pub category: String,

    pub image_desc: String,
    pub image_ext: String,
    pub image_data: Vec<u8>,

    pub has_video: bool,
    pub video_data: Vec<u8>,
    pub video_ext: String,

    pub has_audio: bool,
    pub audio_data: Vec<u8>,
    pub audio_ext: String,

    pub related_articles: Vec<String>,
    pub base_file_name: String,
}

/*
 * return raw Article data
 */
pub async fn article_data(
    auth_session: crate::system::router_app::AuthSession,
    mut multipart: Multipart,
) -> Result<ArticleUpload, ArticleCreateError> {
    let user = auth_session.user.ok_or(ArticleCreateError::UserRequired)?.username.clone();
    // required
    let mut author = String::new();
    let mut title = String::new();
    let mut base_file_name = String::new();
    let mut text_raw = String::new();
    let mut short_text_raw = String::new();
    let mut mini_text_raw = String::new();
    let mut image_data = Vec::<u8>::new();
    let mut image_data_ext = String::new();
    let mut image_desc = String::new();
    let mut category = String::new();

    // not required
    let mut has_video = false;
    let mut has_audio = false;
    let mut video_data = Vec::<u8>::new();
    let mut video_data_ext = String::new();
    let mut audio_data = Vec::<u8>::new();
    let mut audio_data_ext = String::new();
    let mut is_main = false;
    let mut is_exclusive = false;
    let mut related_articles = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("<unnamed>");
        let content_type = field.content_type().map(|ct| ct).unwrap_or("unknown");

        debug!("Processing: {}, type: {:?}", field_name, content_type);

        match field_name {
            "author" => {
                author = extract_required_text(field).await?;
            }
            "is_main" => {
                // if present, then required
                is_main = extract_required_string(field).await? == "on"
            }

            "is_exclusive" => {
                // if present, then required
                is_exclusive = extract_required_string(field).await? == "on"
            }

            "title" => {
                title = extract_required_string(field).await?;
                base_file_name = library::safe_article_file_name(&title);
            }

            "text" => {
                text_raw = extract_required_text(field).await?;
            }

            "short_text" => {
                short_text_raw = extract_required_text(field).await?;
            }

            "category" => {
                category = extract_required_string(field).await?;
            }

            "related_articles" => {
                related_articles = extract_required_string(field)
                    .await?
                    .lines()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();
            }

            "image_desc" => image_desc = extract_required_string(field).await?,

            "image" => {
                (image_data, image_data_ext) = extract_image_data(field).await?;
            }

            "video" => {
                (video_data, video_data_ext) = extract_video_data(field).await?;
                has_video = true;
            }
            "audio" => {
                (audio_data, audio_data_ext) = extract_audio_data(field).await?;
                has_audio = true;
            }
            "mini_text" => {
                mini_text_raw = extract_required_text(field).await?;
            }
            _ => Err(ArticleCreateError::UnknownField(field_name.to_string()))?,
        }
    }

    let ad = ArticleUpload {
        is_main,
        is_exclusive,
        author,
        username: user,
        title,
        text_raw,
        short_text_raw,
        image_data,
        image_ext: image_data_ext,
        image_desc,
        video_data,
        video_ext: video_data_ext,
        has_audio,
        audio_data,
        category,
        related_articles,
        base_file_name,
        has_video,
        audio_ext: audio_data_ext,
        mini_text_raw,
    };

    Ok(ad)
}
