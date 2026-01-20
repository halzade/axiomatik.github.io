use crate::form_new_article::ArticleData;
use crate::validation::{
    extract_text_field, save_file_field, ALLOWED_EXTENSIONS_AUDIO,
    ALLOWED_EXTENSIONS_IMAGE, ALLOWED_EXTENSIONS_VIDEO,
};
use axum::extract::Multipart;
use tracing::{debug, error};

pub async fn article_data(mut multipart: Multipart) -> Option<ArticleData> {
    // required
    let mut title_o = None;
    let mut author_o = None;
    let mut text_processed_o = None;
    let mut short_text_processed_o = None;
    let mut image_path_o = None;
    let mut image_description_o = None;
    let mut category_o = None;

    // not required
    let mut video_path_o = None;
    let mut audio_path_o = None;
    let mut is_main_o = None;
    let mut is_exclusive_o = None;

    // not required
    let mut related_articles_o = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or_default().to_string();
        let content_type = field.content_type().map(|c| c.to_string());
        debug!(
            "Processing field: {}, content_type: {:?}",
            field_name, content_type
        );

        match field_name.as_str() {
            "is_main" => {
                let val = extract_text_field(field, "is_main", false).await?;
                is_main_o = Some(val == "on");
            }

            "is_exclusive" => {
                let val = extract_text_field(field, "is_exclusive", false).await?;
                is_exclusive_o = Some(val == "on");
            }

            "title" => {
                title_o = extract_text_field(field, "title", true).await;
            }

            "author" => {
                author_o = extract_text_field(field, "author", true).await;
            }

            "text" => {
                let raw_text = extract_text_field(field, "text", true).await?;
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
                let raw_text = extract_text_field(field, "short_text", true).await?;
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
                category_o = extract_text_field(field, "category", true).await;
            }

            "related_articles" => {
                related_articles_o = extract_text_field(field, "related_articles", false).await;
            }

            "image_description" => {
                image_description_o = extract_text_field(field, "image_description", true).await;
            }

            "image" => {
                if let Some(path) = save_file_field(field, "image", ALLOWED_EXTENSIONS_IMAGE).await
                {
                    image_path_o = Some(path);
                }
            }

            "video" => {
                if let Some(path) = save_file_field(field, "video", ALLOWED_EXTENSIONS_VIDEO).await
                {
                    video_path_o = Some(path);
                }
            }

            "audio" => {
                if let Some(path) = save_file_field(field, "audio", ALLOWED_EXTENSIONS_AUDIO).await
                {
                    audio_path_o = Some(path);
                }
            }
            _ => (),
        }
    }

    // TODO move to library
    let category_display = match &category_o {
        None => {
            error!("Category is None");
            return None;
        }
        Some(category) => match category.as_str() {
            "zahranici" => "zahraničí",
            "republika" => "republika",
            "finance" => "finance",
            "technologie" => "technologie",
            "veda" => "věda",
            cat => {
                error!("Unknown category: {}", cat);
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
        error!(
            "ArticleData construction failed: \
        is_main={:?}, \
        is_exclusive={:?}, \
        author={:?}, \
        title={:?}, \
        text={:?}, \
        short_text={:?}, \
        image_path={:?}, \
        image_description={:?}, \
        category={:?}, \
        related_articles={:?}",
            is_main_o,
            is_exclusive_o,
            author_o,
            title_o,
            text_processed_o,
            short_text_processed_o,
            image_path_o,
            image_description_o,
            category_o,
            related_articles_o
        );
    }
    res
}
