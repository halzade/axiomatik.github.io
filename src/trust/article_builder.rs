use crate::data::library::safe_article_file_name;
use crate::db::database_article_data::Article;
use chrono::Utc;
use std::io::Write;
use axum_core::response::Response;
use surrealdb_types::Uuid;
use crate::trust::me::TrustError;
use crate::trust::response_verifier::ResponseVerifier;
use crate::trust::script_base;
use crate::trust::script_base_data::PNG;

pub const BOUNDARY: &str = "---------------------------123456789012345678901234567";

#[derive(Default)]
pub struct ArticleBuilder<'a> {
    title_o: Option<String>,
    author_o: Option<String>,
    category_o: Option<String>,
    text_o: Option<String>,
    short_text_o: Option<String>,
    related_articles_o: Option<String>,
    image_desc_o: Option<String>,
    is_main_o: Option<bool>,
    is_exclusive_o: Option<bool>,
    image_o: Option<(String, Vec<u8>, String)>,
    audio_o: Option<(String, Vec<u8>, String)>,
}

impl<'a> ArticleBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title_o = Some(title.into());
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.author_o = Some(author.into());
        self
    }

    pub fn category(mut self, category: &str) -> Self {
        self.category_o = Some(category.into());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text_o = Some(text.into());
        self
    }

    pub fn short_text(mut self, short_text: &str) -> Self {
        self.short_text_o = Some(short_text.into());
        self
    }

    pub fn related_articles(mut self, related_articles: &str) -> Self {
        self.related_articles_o = Some(related_articles.into());
        self
    }

    pub fn image_desc(mut self, image_desc: &str) -> Self {
        self.image_desc_o = Some(image_desc.into());
        self
    }

    pub fn main(mut self) -> Self {
        self.is_main_o = Some(true);
        self
    }

    pub fn exclusive(mut self) -> Self {
        self.is_exclusive_o = Some(true);
        self
    }

    pub fn image(mut self, filename: &str, data: Vec<u8>, content_type: &str) -> Self {
        self.image_o = Some((filename.into(), data, content_type.into()));
        self
    }
    pub fn audio(mut self, filename: &str, data: Vec<u8>, content_type: &str) -> Self {
        self.audio_o = Some((filename.into(), data, content_type.into()));
        self
    }

    pub fn image_any_png(&self) -> &Self {
        // TODO image data and image desc
        self.image("test.jpg",
                   std::fs::read("tests/data/image_1024.png")?
                   , PNG);
        self.image_desc("test description");
        self
    }
    /*
     * build Article data u8 for new article request
     * And Execute
     */
    pub fn execute(self) -> Result<ResponseVerifier, TrustError>{
        let mut body: Vec<u8> = Vec::new();

        macro_rules! text_part {
            ($name:expr, $val:expr) => {
                write!(
                    body,
                    "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
                    BOUNDARY, $name, $val
                )?;
            };
        }

        if let Some(v) = self.title_o {
            text_part!("title", v);
        }
        if let Some(v) = self.author_o {
            text_part!("author", v);
        }
        if let Some(v) = self.category_o {
            text_part!("category", v);
        }
        if let Some(v) = self.text_o {
            text_part!("text", v);
        }
        if let Some(v) = self.short_text_o {
            text_part!("short_text", v);
        }
        if let Some(v) = self.related_articles_o {
            text_part!("related_articles", v);
        }
        if let Some(v) = self.image_desc_o {
            text_part!("image_desc", v);
        }
        if let Some(v) = self.is_main_o {
            text_part!("is_main", if v { "on" } else { "off" });
        }
        if let Some(v) = self.is_exclusive_o {
            text_part!("is_exclusive", if v { "on" } else { "off" });
        }

        // image
        if let Some((filename, data, content_type)) = self.image_o {
            write!(
                body,
                "--{}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
                BOUNDARY, filename, content_type
            )?;
            body.extend_from_slice(data);
            body.extend_from_slice(b"\r\n");
        }

        // audio
        if let Some((filename, data, content_type)) = self.audio_o {
            write!(
                body,
                "--{}\r\nContent-Disposition: form-data; name=\"audio\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n",
                BOUNDARY, filename, content_type
            )?;
            body.extend_from_slice(data);
            body.extend_from_slice(b"\r\n");
        }

        // TODO do video and write test

        write!(body, "--{}--\r\n", BOUNDARY)?;
        Ok(body)
    }
}

pub fn easy_article(title: &str, author: &str, text: &str) -> Article {
    let now = Utc::now();
    let base = safe_article_file_name(&title.to_string());
    Article {
        uuid: Uuid::new(),
        author: author.to_string(),
        user: author.to_string(),
        date: now,
        date_str: "date".to_string(),
        title: title.to_string(),
        text: text.to_string(),
        short_text: "short text here".to_string(),
        mini_text: "mini text".to_string(),
        article_file_name: format!("{}.html", base.clone()),
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
        views: 0,
    }
}
