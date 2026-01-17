use crate::form_new_article::ArticleData;
use crate::validation::validate_input;
use axum::extract::Multipart;
use std::fs;
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
        let field_name = field.name().unwrap().to_string();

        match field_name.as_str() {
            "is_main" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return None;
                }
                is_main_o = Some(val == "on");
            }

            "is_exclusive" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return None;
                }
                is_exclusive_o = Some(val == "on");
            }

            "title" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return None;
                }
                title_o = Some(val);
            }

            "author" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return None;
                }
                author_o = Some(val);
            }

            "text" => {
                let raw_text = field.text().await.unwrap();
                if validate_input(&raw_text).is_err() {
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
                if validate_input(&raw_text).is_err() {
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
                if validate_input(&val).is_err() {
                    return None;
                }
                category_o = Some(val);
            }

            "related_articles" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return None;
                }
                let lines = val
                    .lines()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();

                related_articles_o = Some(lines);
            }

            "image_description" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return None;
                }
                image_description_o = Some(val);
            }

            "image" => {
                if let Some(file_name) = field.file_name() {
                    if validate_input(&file_name).is_err() {
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
                                return None;
                            }
                        }
                    }
                }
            }

            "video" => {
                if let Some(file_name) = field.file_name() {
                    if validate_input(&file_name).is_err() {
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
                                return None;
                            }
                        }
                    }
                }
            }

            "audio" => {
                if let Some(file_name) = field.file_name() {
                    if validate_input(&file_name).is_err() {
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
        None => return None,
        Some(category) => match category.as_str() {
            "zahranici" => "zahraničí",
            "republika" => "republika",
            "finance" => "finance",
            "technologie" => "technologie",
            "veda" => "věda",
            _ => return None,
        },
    };

    Some(ArticleData {
        is_main: is_main_o?,
        is_exclusive: is_exclusive_o?,
        author: author_o?,
        title: title_o?,
        text_processed: text_processed_o?,
        short_text_processed: short_text_processed_o?,
        image_path: image_path_o?,
        image_description: image_description_o?,
        video_path: video_path_o,
        audio_path: audio_path_o,
        category: category_o?,
        category_display: category_display.to_string(),
        related_articles: related_articles_o?,
    })
}
