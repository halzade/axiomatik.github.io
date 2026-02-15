use crate::data::library::safe_article_file_name;
use crate::db::database_article_data::Article;
use chrono::Utc;
use surrealdb_types::Uuid;

pub struct ArticleBuilder {
    title: String,
    author: String,
    text: String,

    short_text: String,
    mini_text: String,
    image_desc: String,
    category: String,

    has_video: bool,
    video_path: String,
    has_audio: bool,
    audio_path: String,

    related_articles: Vec<String>,
    is_main: bool,
    is_exclusive: bool,
}

impl ArticleBuilder {
    pub fn article() -> Self {
        Self {
            title: "".into(),
            author: "".into(),
            text: "".into(),

            short_text: "short text here".to_string(),
            mini_text: "mini text".to_string(),
            image_desc: "desc".to_string(),
            category: "cat".to_string(),

            has_video: false,
            video_path: String::new(),
            has_audio: false,
            audio_path: String::new(),

            related_articles: Vec::new(),
            is_main: false,
            is_exclusive: false,
        }
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = value.into();
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = value.into();
        self
    }
    pub fn short_text(mut self, value: impl Into<String>) -> Self {
        self.short_text = value.into();
        self
    }

    pub fn mini_text(mut self, value: impl Into<String>) -> Self {
        self.mini_text = value.into();
        self
    }

    pub fn image_desc(mut self, value: impl Into<String>) -> Self {
        self.image_desc = value.into();
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = value.into();
        self
    }

    pub fn video(mut self, path: impl Into<String>) -> Self {
        self.has_video = true;
        self.video_path = path.into();
        self
    }

    pub fn audio(mut self, path: impl Into<String>) -> Self {
        self.has_audio = true;
        self.audio_path = path.into();
        self
    }

    pub fn related_articles(mut self, articles: Vec<String>) -> Self {
        self.related_articles = articles;
        self
    }

    pub const fn main(mut self, value: bool) -> Self {
        self.is_main = value;
        self
    }

    pub const fn exclusive(mut self, value: bool) -> Self {
        self.is_exclusive = value;
        self
    }

    pub fn build(self) -> Article {
        let now = Utc::now();
        let base = safe_article_file_name(&self.title);

        Article {
            uuid: Uuid::new(),
            author: self.author.clone(),
            username: self.author,
            date: now,

            title: self.title,
            text: self.text,
            short_text: self.short_text,
            mini_text: self.mini_text,

            article_file_name: format!("{}.html", base),

            image_desc: self.image_desc,

            image_50_path: format!("u/{}_image_50.png", base),
            image_288_path: format!("u/{}_image_288.png", base),
            image_440_path: format!("u/{}_image_440.png", base),
            image_820_path: format!("u/{}_image_820.png", base),

            has_video: self.has_video,
            video_path: self.video_path,
            has_audio: self.has_audio,
            audio_path: self.audio_path,

            category: self.category,
            related_articles: self.related_articles,

            is_main: self.is_main,
            is_exclusive: self.is_exclusive,
        }
    }
}
