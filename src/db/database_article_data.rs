use crate::application::form::form_article_data_parser::ArticleUpload;
use crate::data::library;
use crate::data::text_processor::{process_short_text, process_text};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataProcessorError {
    #[error("unknown category: {0}")]
    ArticleProcessor(String),
}

/**
 * create Article database object
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewArticle {
    pub data: ArticleData,
}

/**
 * Article database object
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub uuid: Uuid,
    pub data: ArticleData,
}

/**
 * Use in Templates
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

/**
 * Use in Templates
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticlePublicData {
    pub uuid: String,

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

use std::convert::TryFrom;

impl From<Article> for ArticlePublicData {
    fn from(article: Article) -> Self {
        ArticlePublicData {
            uuid: article.uuid.to_string(),

            author: article.data.author,
            user: article.data.user,
            date: article.data.date,
            date_str: article.data.date_str,
            title: article.data.title,
            text: article.data.text,
            short_text: article.data.short_text,
            mini_text: article.data.mini_text,
            file_base: article.data.file_base,
            image_desc: article.data.image_desc,
            image_50_path: article.data.image_50_path,
            image_288_path: article.data.image_288_path,
            image_440_path: article.data.image_440_path,
            image_820_path: article.data.image_820_path,
            has_video: article.data.has_video,
            video_path: article.data.video_path,
            has_audio: article.data.has_audio,
            audio_path: article.data.audio_path,
            category: article.data.category,
            related_articles: article.data.related_articles,
            is_main: article.data.is_main,
            is_exclusive: article.data.is_exclusive,
            views: article.data.views,
        }
    }
}
