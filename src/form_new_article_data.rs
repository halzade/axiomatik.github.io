use crate::form_new_article::ArticleData;
use crate::validation::validate_input;
use axum::extract::Multipart;
use std::fs;
use tracing::error;
use uuid::Uuid;

pub async fn article_data(mut multipart: Multipart) -> Option<ArticleData> {
    let mut title_o = None;
    let mut author_o = None;
    let mut text_processed_o = None;
    let mut short_text_processed_o = None;
    let mut category_o = None;
    let mut related_articles_o = None;
    let mut image_path_o = None;
    let mut image_description_o = None;
    let mut video_path_o = None;
    let mut audio_path_o = None;
    let mut is_main_o = None;
    let mut is_exclusive_o = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or_default().to_string();
        let content_type = field.content_type().map(|c| c.to_string());
        tracing::info!("Processing field: {}, content_type: {:?}", field_name, content_type);

        match field_name.as_str() {
            "is_main" => {
                let val = field.text().await.unwrap();
                if let Err(e) = validate_input(&val) {
                    tracing::error!("is_main validation failed: {}", e);
                    return None;
                }
                is_main_o = Some(val == "on");
            }

            "is_exclusive" => {
                let val = field.text().await.unwrap();
                if let Err(e) = validate_input(&val) {
                    tracing::error!("is_exclusive validation failed: {}", e);
                    return None;
                }
                is_exclusive_o = Some(val == "on");
            }

            "title" => {
                let val = field.text().await.unwrap();
                if let Err(e) = validate_input(&val) {
                    tracing::error!("title validation failed: {}", e);
                    return None;
                }
                title_o = Some(val);
            }

            "author" => {
                let val = field.text().await.unwrap();
                if let Err(e) = validate_input(&val) {
                    tracing::error!("author validation failed: {}", e);
                    return None;
                }
                author_o = Some(val);
            }

            "text" => {
                let raw_text = field.text().await.unwrap();
                if let Err(e) = validate_input(&raw_text) {
                    tracing::error!("text validation failed: {}", e);
                    return None;
                }
                let normalized = raw_text.replace("\r\n", "\n");
                let processed = normalized
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
                    .join("");
                text_processed_o = Some(processed);
            }

            "short_text" => {
                let raw_text = field.text().await.unwrap();
                if let Err(e) = validate_input(&raw_text) {
                    tracing::error!("short_text validation failed: {}", e);
                    return None;
                }
                let normalized = raw_text.replace("\r\n", "\n");
                let normalized_text = normalized
                    .split("\n\n")
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().replace("\n", "<br>\n"))
                    .collect::<Vec<String>>()
                    .join("</p><p>");
                short_text_processed_o = Some(normalized_text);
            }

            "category" => {
                let val = field.text().await.unwrap();
                if let Err(e) = validate_input(&val) {
                    tracing::error!("category validation failed: {}", e);
                    return None;
                }
                category_o = Some(val);
            }

            "related_articles" => {
                let val = field.text().await.unwrap();
                if let Err(e) = validate_input(&val) {
                    tracing::error!("related_articles validation failed: {}", e);
                    return None;
                }
                related_articles_o = Some(val);
            }

            "image_description" => {
                let val = field.text().await.unwrap();
                if let Err(e) = validate_input(&val) {
                    tracing::error!("image_description validation failed: {}", e);
                    return None;
                }
                image_description_o = Some(val);
            }

            "image" => {
                if let Some(file_name) = field.file_name() {
                    if let Err(e) = validate_input(&file_name) {
                        tracing::error!("image filename validation failed: {}", e);
                        return None;
                    }

                    if !file_name.is_empty() {
                        let extension = std::path::Path::new(file_name)
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_lowercase());

                        match extension {
                            Some(ext) if ["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()) => {
                                let new_name = format!("{}.{}", Uuid::new_v4(), ext);

                                // TODO verify data
                                let data = field.bytes().await.unwrap();
                                fs::write(format!("uploads/{}", new_name), data).unwrap();
                                image_path_o = Some(new_name);
                            }
                            _ => {
                                tracing::error!("image invalid extension: {:?}", extension);
                                return None;
                            }
                        }
                    }
                }
            }

            "video" => {
                if let Some(file_name) = field.file_name() {
                    if let Err(e) = validate_input(&file_name) {
                        tracing::error!("video filename validation failed: {}", e);
                        return None;
                    }

                    if !file_name.is_empty() {
                        let extension = std::path::Path::new(file_name)
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_lowercase());

                        match extension {
                            Some(ext) if ["avi", "mp4", "webm", "mov"].contains(&ext.as_str()) => {
                                let new_name = format!("{}.{}", Uuid::new_v4(), ext);
                                let data = field.bytes().await.unwrap();
                                fs::write(format!("uploads/{}", new_name), data).unwrap();
                                video_path_o = Some(new_name);
                            }
                            _ => {
                                tracing::error!("video invalid extension: {:?}", extension);
                                return None;
                            }
                        }
                    }
                }
            }

            "audio" => {
                if let Some(file_name) = field.file_name() {
                    if let Err(e) = validate_input(&file_name) {
                        tracing::error!("audio filename validation failed: {}", e);
                        return None;
                    }

                    if !file_name.is_empty() {
                        let extension = std::path::Path::new(file_name)
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_lowercase());

                        match extension {
                            Some(ext) if ["mp3", "wav", "ogg", "m4a"].contains(&ext.as_str()) => {
                                let new_name = format!("{}.{}", Uuid::new_v4(), ext);
                                let data = field.bytes().await.unwrap();
                                fs::write(format!("uploads/{}", new_name), data).unwrap();
                                audio_path_o = Some(new_name);
                            }
                            _ => {
                                tracing::error!("audio invalid extension: {:?}", extension);
                                return None;
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    // TODO move to library
    let category_display = match &category_o {
        None => {
            tracing::error!("Category is None");
            return None;
        }
        Some(category) => match category.as_str() {
            "zahranici" => "zahraničí",
            "republika" => "republika",
            "finance" => "finance",
            "technologie" => "technologie",
            "veda" => "věda",
            cat => {
                tracing::error!("Unknown category: {}", cat);
                return None;
            }
        },
    };

    let res = (|| {
        Some(ArticleData {
            is_main: is_main_o?,
            is_exclusive: is_exclusive_o?,
            author: author_o.as_ref()?.clone(),
            title: title_o.as_ref()?.clone(),
            text_processed: text_processed_o.as_ref()?.clone(),
            short_text_processed: short_text_processed_o.as_ref()?.clone(),
            image_path: image_path_o.as_ref()?.clone(),
            image_description: image_description_o.as_ref()?.clone(),
            video_path: video_path_o.clone(),
            audio_path: audio_path_o.clone(),
            category: category_o.as_ref()?.clone(),
            category_display: category_display.to_string(),
            related_articles: related_articles_o.as_ref()?.clone(),
        })
    })();
    if res.is_none() {
        error!("ArticleData construction failed: is_main={:?}, is_exclusive={:?}, author={:?}, title={:?}, text={:?}, short_text={:?}, image_path={:?}, image_description={:?}, category={:?}, related_articles={:?}",
            is_main_o, is_exclusive_o, author_o, title_o, text_processed_o.is_some(), short_text_processed_o.is_some(), image_path_o, image_description_o, category_o, related_articles_o);
    }
    res
}
