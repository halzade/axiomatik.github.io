use crate::form_new_article::ArticleData;
use crate::utils::{
    extract_audio_data, extract_image_data, extract_required_string, extract_required_text,
    extract_video_data,
};
use anyhow::{anyhow, Error};
use axum::extract::Multipart;
use tracing::{debug, error, info};

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

    let category_display = process_category(&category_o.clone().unwrap())?;

    let base_file_name = base_file_name_o.unwrap();

    let image_data_bu8;
    let image_extension;
    let image_path;
    match image_data_o {
        Some(image_data) => {
            image_data_bu8 = image_data.0;
            image_extension = image_data.1;
            image_path = format!("uploads/{}-image.{}", &base_file_name, image_extension);
        }
        None => {
            error!("Image was required");
            return Err(anyhow!("Image was required"));
        }
    }

    let video_data_bu8;
    let video_extension;
    let video_path;
    match video_data_o {
        Some(video_data) => {
            video_data_bu8 = Some(video_data.0);
            video_extension = Some(video_data.1);
            video_path = Some(format!("uploads/{}-video.{}", &base_file_name, video_extension.unwrap()));
        }
        None => {
            info!("video not set");
            video_data_bu8 = None;
            video_extension = None;
            video_path = None
        }
    }

    let audio_data_bu8;
    let audio_extension;
    let audio_path;
    match audio_data_o {
        Some(audio_data) => {
            audio_data_bu8 = Some(audio_data.0);
            audio_extension = Some(audio_data.1);
            audio_path = Some(format!("uploads/{}-audio.{}", &base_file_name, audio_extension.unwrap()));
        }
        None => {
            info!("audio not set");
            audio_data_bu8 = None;
            audio_extension = None;
            audio_path = None
        }
    }

    Ok(ArticleData {
        is_main: is_main_o.unwrap_or(false),
        is_exclusive: is_exclusive_o.unwrap_or(false),
        author: author_o.unwrap(),
        title: title_o.unwrap(),
        text_processed: text_processed_o.unwrap(),
        short_text_processed: short_text_processed_o.unwrap(),
        image_data: image_data_bu8,
        image_description: image_description_o.unwrap(),
        video_data: video_data_o.map(|(d, _)| d).unwrap_or_default(),
        audio_data: audio_data_o.map(|(d, _)| d).unwrap_or_default(),
        category: category_o.unwrap(),
        category_display: category_display.to_string(),
        related_articles: related_articles_o.unwrap(),
        image_path,
        video_path,
        audio_path,
    })
}

fn process_category(raw_category: &str) -> Result<String, Error> {
    match raw_category {
        "zahranici" => Ok("zahraničí".into()),
        "republika" => Ok("republika".into()),
        "finance" => Ok("finance".into()),
        "technologie" => Ok("technologie".into()),
        "veda" => Ok("věda".into()),
        cat => {
            error!("Unknown category: {}", cat);
            Err(anyhow!(format!("Unknown category: {}", cat)))
        }
    }
}

// TODO
fn process_short_text(raw_text: &str) -> String {
    raw_text
        .replace("\r\n", "\n")
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().replace("\n", "<br>\n"))
        .collect::<Vec<String>>()
        .join("</p><p>")
}

// TODO
fn process_text(raw_text: &str) -> String {
    raw_text
        .replace("\r\n", "\n")
        .split("\n\n\n")
        .filter(|block| !block.trim().is_empty())
        .map(|block| {
            let inner_html = block
                .split("\n\n")
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    if s.starts_with("   ") {
                        format!("<blockquote>{}</blockquote>", s.trim())
                    } else {
                        format!("<p>{}</p>", s.trim().replace("\n", " "))
                    }
                })
                .collect::<Vec<String>>()
                .join("");
            format!("<div class=\"container\">{}</div>", inner_html)
        })
        .collect::<Vec<String>>()
        .join("")
}
