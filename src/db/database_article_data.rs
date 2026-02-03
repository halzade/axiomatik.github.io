use crate::application::form::form_article_data_parser::ArticleUpload;
use crate::data::library;
use crate::data::text_processor::{process_short_text, process_text};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataProcessorError {
    #[error("unknown category: {0}")]
    ArticleProcessor(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewArticle {
    pub data: ArticleData,
}

/**
 * Article object used in the Application
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub id: u64,
    pub data: ArticleData,
}

/**
 * Common struct for various Article use
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleData {
    pub author: String,
    pub user: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub date: DateTime<Utc>,
    pub date_str: String,

    pub title: String,

    pub text: String,
    pub short_text: String,
    pub mini_text: String,

    pub file_base: String,

    pub image_desc: String,
    pub image_50_path: String,
    pub image_288_path: String,
    pub image_440_path: String,
    pub image_820_path: String,

    pub has_video: bool,
    pub video_path: String,

    pub has_audio: bool,
    pub audio_path: String,

    pub category: String,
    pub related_articles: Vec<String>,

    pub is_main: bool,
    pub is_exclusive: bool,

    pub views: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortArticleData {
    pub url: String,
    pub title: String,
    pub short_text: String,
    pub image_288_path: String,
    pub image_desc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MiniArticleData {
    pub url: String,
    pub title: String,
    pub mini_text: String,
    pub image_50_path: String,
    pub image_desc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleViews {
    pub file_base: String,
    pub category: String,
    pub views: i64,
}

impl TryFrom<ArticleUpload> for NewArticle {
    type Error = DataProcessorError;

    fn try_from(data: ArticleUpload) -> Result<Self, Self::Error> {
        let now = Utc::now();
        Ok(NewArticle {
            data: ArticleData {
                author: data.author,
                user: data.user,
                date: now,
                date_str: library::formatted_article_date(now),

                title: data.title,
                text: process_text(&data.text_raw),
                short_text: process_short_text(&data.short_text_raw),
                mini_text: process_short_text(&data.mini_text_raw),

                file_base: data.base_file_name.clone(),
                image_desc: data.image_desc,
                image_50_path: format!("web/u/{}_image_50.{}", data.base_file_name, data.image_ext),
                image_288_path: format!(
                    "web/u/{}_image_288.{}",
                    data.base_file_name, data.image_ext
                ),
                image_440_path: format!(
                    "web/u/{}_image_440.{}",
                    data.base_file_name, data.image_ext
                ),
                image_820_path: format!(
                    "web/u/{}_image_820.{}",
                    data.base_file_name, data.image_ext
                ),

                has_video: data.has_video,
                video_path: if data.has_video {
                    "".into()
                } else {
                    format!("web/u/{}_video.{}", data.base_file_name, data.video_ext)
                },

                has_audio: data.has_audio,
                audio_path: if data.has_audio {
                    "".into()
                } else {
                    format!("web/u/{}_audio.{}", data.base_file_name, data.audio_ext)
                },

                category: data.category,
                related_articles: data.related_articles,

                is_main: data.is_main,
                is_exclusive: data.is_exclusive,

                views: 0,
            },
        })
    }
}
