use crate::data::library;
use crate::data::library::safe_article_file_name;
use crate::data::text_processor::{process_short_text, process_text};
use crate::db::database::SurrealError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use surrealdb::types::{SurrealValue, Uuid};
use crate::application::form_create_article::create_article_parser::ArticleUpload;
/*
 * Main Articles on index.html
 */
#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct MainArticleData {
    pub article_file_name: String,
    pub title: String,
    pub is_exclusive: bool,
    pub short_text: String,
    pub category: String,
    pub image_440_path: String,
    pub image_desc: String,
}

impl MainArticleData {
    // used for index.html if there are no articles yet
    pub fn empty() -> Self {
        Self {
            article_file_name: "".into(),
            title: "".into(),
            is_exclusive: false,
            short_text: "".into(),
            category: "".into(),
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
    pub username: String,

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
    pub date_str: String,
    pub category: String,
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
        Ok(Self {
            uuid: Uuid::new(), // TODO
            author: data.author,
            username: data.username,
            date: now,
            date_str: library::formatted_article_date(now),

            title: data.title,
            text: process_text(&data.text_raw),
            short_text: process_short_text(&data.short_text_raw),
            mini_text: process_short_text(&data.mini_text_raw),

            // everything should be relative to web/
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

pub fn easy_article(title: &str, author: &str, text: &str) -> Article {
    let now = Utc::now();
    let base = safe_article_file_name(title);
    Article {
        uuid: Uuid::new(),
        author: author.to_string(),
        username: author.to_string(),
        date: now,
        date_str: "date".to_string(),
        title: title.to_string(),
        text: text.to_string(),
        short_text: "short text here".to_string(),
        mini_text: "mini text".to_string(),
        article_file_name: format!("{}.html", base),
        image_desc: "desc".to_string(),
        image_50_path: format!("{}_image_50.jpg", base),
        image_288_path: format!("{}_image_288.jpg", base),
        image_440_path: format!("{}_image_440.jpg", base),
        image_820_path: format!("{}_image_820.jpg", base),
        has_video: false,
        video_path: "".to_string(),
        has_audio: false,
        audio_path: "".to_string(),
        category: "cat".to_string(),
        related_articles: vec![],
        is_main: false,
        is_exclusive: false,
    }
}
