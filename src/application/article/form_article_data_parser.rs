use crate::library;
use axum::extract::Multipart;
use axum::extract::multipart::Field;
use thiserror::Error;
use tracing::{debug, error, info};
use crate::data::audio_extractor::extract_audio_data;
use crate::data::image_extractor::extract_image_data;
use crate::data::text_extractor::{extract_required_string, extract_required_text};
use crate::data::video_extractor::extract_video_data;
use crate::library::ProcessorError;
use crate::processor::text_extractor::extract_required_string;

#[derive(Debug, Error)]
pub enum ArticleError {

    #[error("Processor error: {0}")]
    Processor(#[from] ProcessorError),

    #[error("Field {0} is required")]
    FieldRequired(String),

    #[error("Unknown field {0}")]
    UnknownField(String),
}

/**
 * Parsed result of new Article /create
 * Contains sanitized raw text and raw data
 * Text will be processed only for the Template
 */
pub struct ArticleData {
    pub is_main: bool,
    pub is_exclusive: bool,

    pub title: String,
    pub text_processed: String,
    pub short_text_processed: String,

    pub image_description: String,
    pub image_data: Vec<u8>,
    pub video_data: Vec<u8>,
    pub audio_data: Vec<u8>,

    pub category: String,
    pub related_articles: Vec<String>,

    pub image_path: String,
    pub video_path: Option<String>,
    pub audio_path: Option<String>,
    pub article_file_name: String,
}

pub async fn article_data(mut multipart: Multipart) -> Result<ArticleData, ArticleError> {
    // required
    let mut title_o = None;
    let mut base_file_name_o = None;
    let mut text_processed_o = None;
    let mut short_text_processed_o = None;
    let mut image_data_o = None;
    let mut image_description_o = None;
    let mut category_o = None;

    // not required
    let mut video_data_o = None;
    let mut audio_data_o = None;
    let mut is_main_o = None;
    let mut is_exclusive_o = None;
    let mut related_articles_o = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap().to_string();
        let content_type = field.content_type().unwrap();

        debug!(
            "Processing field: {}, content_type: {:?}",
            field_name, content_type
        );

        match field_name.as_str() {
            "is_main" => {
                // if present, then required
                let extracted = extract_required_string(field).await?;
                is_main_o = Some(extracted == "on");
            }

            "is_exclusive" => {
                // if present, then required
                let extracted = extract_required_string(field).await?;
                is_exclusive_o = Some(extracted == "on");
            }

            "title" => {
                let extracted = extract_required_string(field).await?;
                title_o = Some(extracted.clone());
                base_file_name_o = Some(library::safe_article_file_name(&extracted));
            }

            "text" => {
                let raw_text = extract_required_text(field).await?;
                text_processed_o = Some(raw_text);
            }

            "short_text" => {
                let raw_text = extract_required_text(field).await?;
                short_text_processed_o = Some(raw_text);
            }

            "category" => {
                let extracted = extract_required_string(field).await?;
                category_o = Some(extracted);
            }

            "related_articles" => {
                let extracted = extract_required_string(field).await?;
                related_articles_o = Some(
                    extracted
                        .lines()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(String::from)
                        .collect(),
                );
            }

            "image_description" => {
                let extracted = extract_required_string(field, "image_description").await?;
                image_description_o = Some(extracted);
            }

            "image" => {
                image_data_o = extract_image_data(field).await;
            }

            "video" => {
                video_data_o = extract_video_data(field).await;
            }
            "audio" => {
                audio_data_o = extract_audio_data(field).await;
            }
            _ => {
                error!("Unknown field: {}", field_name);
                Err(ArticleError::UnknownField(field_name))?
            }
        }
    }

    let base_file_name = base_file_name_o
        .ok_or_else(|| ArticleError::FieldRequired("Title/Base file name".to_string()))?;

    /*
     * process images
     */
    let image_data_bu8;
    let image_extension;
    let image_path;
    match &image_data_o {
        Some(image_data) => {
            image_data_bu8 = image_data.0.clone();
            image_extension = image_data.1.clone();
            image_path = format!("u/{}-image.{}", &base_file_name, image_extension);
        }
        None => {
            error!("Image was required");
            return Err(ArticleError::FieldRequired("Image".to_string()));
        }
    }

    /*
     * process video
     */
    let video_path;
    match &video_data_o {
        Some(video_data) => {
            video_path = Some(format!("u/{}-video.{}", &base_file_name, video_data.1));
        }
        None => {
            info!("video not set");
            video_path = None
        }
    }

    /*
     * process images
     */
    let audio_path;
    match &audio_data_o {
        Some(audio_data) => {
            audio_path = Some(format!("u/{}-audio.{}", &base_file_name, audio_data.1));
        }
        None => {
            info!("audio not set");
            audio_path = None
        }
    }

    Ok(ArticleData {
        is_main: is_main_o.unwrap_or(false),
        is_exclusive: is_exclusive_o.unwrap_or(false),
        title: title_o.ok_or_else(|| ArticleError::FieldRequired("Title".to_string()))?,
        text_processed: text_processed_o
            .ok_or_else(|| ArticleError::FieldRequired("Text".to_string()))?,
        short_text_processed: short_text_processed_o
            .ok_or_else(|| ArticleError::FieldRequired("Short text".to_string()))?,
        image_data: image_data_bu8,
        image_description: image_description_o
            .ok_or_else(|| ArticleError::FieldRequired("Image description".to_string()))?,

        // TODO stupid defaults
        video_data: video_data_o
            .as_ref()
            .map(|(d, _)| d.clone())
            .unwrap_or_default(),
        audio_data: audio_data_o
            .as_ref()
            .map(|(d, _)| d.clone())
            .unwrap_or_default(),
        category: category_o.unwrap(),
        related_articles: related_articles_o
            .ok_or_else(|| ArticleError::FieldRequired("Related articles".to_string()))?,
        image_path,
        video_path,
        audio_path,
        article_file_name: format!("{}.html", base_file_name),
    })
}
