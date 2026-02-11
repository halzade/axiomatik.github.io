use crate::application::form::form_article_data_parser::ArticleUpload;
use crate::data::library;
use crate::data::text_processor::{process_short_text, process_text};
use crate::db::database::SurrealError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use surrealdb::types::{SurrealValue, Uuid};

/*
 * Main Articles on index.html
 */
#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct MainArticleData {
    pub article_file_name: String,
    pub title: String,
    pub is_exclusive: bool,
    pub short_text: String,
    pub image_440_path: String,
    pub image_desc: String,
}

impl MainArticleData {
    // used for Index.html, if there are no articles yet
    pub fn empty() -> MainArticleData {
        MainArticleData {
            article_file_name: "".into(),
            title: "".into(),
            is_exclusive: false,
            short_text: "".into(),
            image_440_path: "".into(),
            image_desc: "".into(),
        }
    }
}

/*
 * Second and Third Article on index.html
 */
#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct TopArticleData {
    pub article_file_name: String,
    pub title: String,
    pub short_text: String,
}

/**
 * Article database object
 */
#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct Article {
    pub uuid: Uuid,

    pub article_file_name: String,

    pub author: String,
    pub user: String, // TODO this should be username

    pub date: DateTime<Utc>,
    pub date_str: String,

    pub title: String,

    pub text: String,
    pub short_text: String,
    pub mini_text: String,

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
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct ShortArticleData {
    pub article_file_name: String,
    pub title: String,
    pub short_text: String,
    pub image_288_path: String,
    pub image_desc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct MiniArticleData {
    pub article_file_name: String,
    pub title: String,
    pub mini_text: String,
    pub image_50_path: String,
    pub image_desc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct AccountArticleData {
    pub article_file_name: String,
    pub title: String,
    pub short_text: String,
    pub image_288_path: String,
    pub image_desc: String,
    pub category: String,
    pub date: DateTime<Utc>,
    pub date_str: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct ArticleViews {
    pub article_file_name: String,
    pub views: u64,
}

impl From<MainArticleData> for TopArticleData {
    fn from(value: MainArticleData) -> Self {
        Self {
            article_file_name: value.article_file_name,
            title: value.title,
            short_text: value.short_text,
        }
    }
}

impl TryFrom<ArticleUpload> for Article {
    type Error = SurrealError;

    fn try_from(data: ArticleUpload) -> Result<Self, Self::Error> {
        let now = Utc::now();
        Ok(Article {
            uuid: Uuid::new(), // TODO
            author: data.author,
            user: data.user,
            date: now,
            date_str: library::formatted_article_date(now),

            title: data.title,
            text: process_text(&data.text_raw),
            short_text: process_short_text(&data.short_text_raw),
            mini_text: process_short_text(&data.mini_text_raw),

            article_file_name: format!("{}.html", data.base_file_name.clone()),
            image_desc: data.image_desc,
            image_50_path: format!("u/{}_image_50.{}", data.base_file_name, data.image_ext),
            image_288_path: format!("u/{}_image_288.{}", data.base_file_name, data.image_ext),
            image_440_path: format!("u/{}_image_440.{}", data.base_file_name, data.image_ext),
            image_820_path: format!("u/{}_image_820.{}", data.base_file_name, data.image_ext),

            has_video: data.has_video,
            video_path: if data.has_video {
                format!("u/{}_video.{}", data.base_file_name, data.video_ext)
            } else {
                "".into()
            },

            has_audio: data.has_audio,
            audio_path: if data.has_audio {
                format!("u/{}_audio.{}", data.base_file_name, data.audio_ext)
            } else {
                "".into()
            },

            category: data.category,
            related_articles: data.related_articles,

            is_main: data.is_main,
            is_exclusive: data.is_exclusive,
        })
    }
}
