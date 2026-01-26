use crate::form_new_article::ArticleData;
use crate::utils::{
    extract_audio_data, extract_image_data, extract_required_string, extract_required_text,
    extract_video_data,
};
use anyhow::{anyhow, Error};
use axum::extract::Multipart;
use tracing::{debug, error, info};
use processor::process_text;
use crate::processor;
use crate::processor::{process_category, process_short_text};

pub async fn article_data(mut multipart: Multipart) -> Result<ArticleData, Error> {
    // required
    let mut title_o = None;
    let mut base_file_name_o = None;
    let mut author_o = None;
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
        // TODO don't default
        let field_name = field.name().unwrap_or_default().to_string();
        let content_type = field.content_type().map(|c| c.to_string());

        debug!(
            "Processing field: {}, content_type: {:?}",
            field_name, content_type
        );

        match field_name.as_str() {
            "is_main" => {
                // if present, then required
                let extracted = extract_required_string(field, "is_main").await?;
                is_main_o = Some(extracted == "on");
            }

            "is_exclusive" => {
                // if present, then required
                let extracted = extract_required_string(field, "is_exclusive").await?;
                is_exclusive_o = Some(extracted == "on");
            }

            "title" => {
                let extracted = extract_required_string(field, "title").await?;
                title_o = Some(extracted.clone());
                base_file_name_o = Some(extracted);
            }

            "author" => {
                // TODO use data from DB instead
                let extracted = extract_required_string(field, "author").await?;
                author_o = Some(extracted);
            }

            "text" => {
                let raw_text = extract_required_text(field, "text").await?;
                text_processed_o = Some(process_text(&raw_text));
            }

            "short_text" => {
                let raw_text = extract_required_text(field, "short_text").await?;
                short_text_processed_o = Some(process_short_text(&raw_text));
            }

            "category" => {
                let extracted = extract_required_string(field, "category").await?;
                category_o = Some(extracted);
            }

            "related_articles" => {
                let extracted = extract_required_string(field, "related_articles").await?;
                related_articles_o = Some(extracted);
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
            }
        }
    }

    let category_o = category_o.ok_or_else(|| anyhow!("Category is required"))?;
    let category_display = process_category(&category_o)?;

    let base_file_name = base_file_name_o.ok_or_else(|| anyhow!("Title/Base file name is required"))?;

    let image_data_bu8;
    let image_extension;
    let image_path;
    match &image_data_o {
        Some(image_data) => {
            image_data_bu8 = image_data.0.clone();
            image_extension = image_data.1.clone();
            image_path = format!("uploads/{}-image.{}", &base_file_name, image_extension);
        }
        None => {
            error!("Image was required");
            return Err(anyhow!("Image was required"));
        }
    }

    let video_path;
    match &video_data_o {
        Some(video_data) => {
            video_path = Some(format!("uploads/{}-video.{}", &base_file_name, video_data.1));
        }
        None => {
            info!("video not set");
            video_path = None
        }
    }

    let audio_path;
    match &audio_data_o {
        Some(audio_data) => {
            audio_path = Some(format!("uploads/{}-audio.{}", &base_file_name, audio_data.1));
        }
        None => {
            info!("audio not set");
            audio_path = None
        }
    }

    Ok(ArticleData {
        is_main: is_main_o.unwrap_or(false),
        is_exclusive: is_exclusive_o.unwrap_or(false),
        author: author_o.ok_or_else(|| anyhow!("Author is required"))?,
        title: title_o.ok_or_else(|| anyhow!("Title is required"))?,
        text_processed: text_processed_o.ok_or_else(|| anyhow!("Text is required"))?,
        short_text_processed: short_text_processed_o.ok_or_else(|| anyhow!("Short text is required"))?,
        image_data: image_data_bu8,
        image_description: image_description_o.ok_or_else(|| anyhow!("Image description is required"))?,
        video_data: video_data_o.as_ref().map(|(d, _)| d.clone()).unwrap_or_default(),
        audio_data: audio_data_o.as_ref().map(|(d, _)| d.clone()).unwrap_or_default(),
        category: category_o,
        category_display: category_display.to_string(),
        related_articles: related_articles_o.ok_or_else(|| anyhow!("Related articles field is required"))?,
        image_path,
        video_path,
        audio_path,
    })
}
