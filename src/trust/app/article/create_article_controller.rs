use crate::trust::app::article::create_article_data::{ArticleFluent, ArticleData};
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::me::TrustError;
use axum::Router;
use std::sync::Arc;
use http::{header, Request};
use axum::body::Body;
use tower::ServiceExt;
use crate::trust::data::utils::content_type_with_boundary;
use crate::trust::data::media_data::{BOUNDARY, PNG};

#[derive(Debug, Clone)]
pub struct CreateArticleController {
    app_router: Arc<Router>,
    input: ArticleFluent,
    user_cookie: Arc<parking_lot::RwLock<Option<String>>>,
}

impl CreateArticleController {
    pub fn new(app_router: Arc<Router>) -> Self {
        Self {
            app_router,
            input: ArticleFluent::new(),
            user_cookie: Arc::new(parking_lot::RwLock::new(None)),
        }
    }

    pub fn set_cookie(&self, cookie: Option<String>) {
        *self.user_cookie.write() = cookie;
    }

    pub fn title(&self, title: &str) -> &Self {
        self.input.title(title);
        self
    }

    pub fn text(&self, text: &str) -> &Self {
        self.input.text(text);
        self
    }

    pub fn author(&self, author: &str) -> &Self {
        self.input.author(author);
        self
    }

    pub fn category(&self, category: &str) -> &Self {
        self.input.category(category);
        self
    }

    pub fn short_text(&self, short_text: &str) -> &Self {
        self.input.short_text(short_text);
        self
    }

    pub fn mini_text(&self, mini_text: &str) -> &Self {
        self.input.mini_text(mini_text);
        self
    }

    pub fn is_main(&self, is_main: bool) -> &Self {
        self.input.is_main(is_main);
        self
    }

    pub fn is_exclusive(&self, is_exclusive: bool) -> &Self {
        self.input.is_exclusive(is_exclusive);
        self
    }

    pub fn image(&self, data: Vec<u8>, ext: &str) -> &Self {
        self.input.image(data, ext);
        self
    }

    pub fn image_desc(&self, desc: &str) -> &Self {
        self.input.image_desc(desc);
        self
    }

    pub fn related_articles(&self, related: &str) -> &Self {
        self.input.related_articles(related);
        self
    }

    pub fn image_any_png(&self) -> Result<&Self, TrustError> {
        let image_data = std::fs::read("web/images/placeholder_1024.png")?;
        self.image(image_data, PNG);
        if self.input.get_data().image_desc.is_none() {
            self.image_desc("any png description");
        }
        Ok(self)
    }

    pub async fn execute(&self) -> Result<ResponseVerifier, TrustError> {
        let data = self.input.get_data();
        let body = self.build_multipart_body(data);
        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        let request = Request::builder()
            .method("POST")
            .uri("/create")
            .header(header::CONTENT_TYPE, content_type_with_boundary())
            .header(header::COOKIE, cookie)
            .body(Body::from(body))?;

        let response = self
            .app_router
            .as_ref()
            .clone()
            .oneshot(request)
            .await?;

        Ok(ResponseVerifier::new(response))
    }

    fn build_multipart_body(&self, data: ArticleData) -> Vec<u8> {
        let mut body = Vec::new();

        self.add_field(&mut body, "title", &data.title.unwrap_or_default());
        self.add_field(&mut body, "text", &data.text.unwrap_or_default());
        self.add_field(&mut body, "author", &data.author.unwrap_or_default());
        self.add_field(&mut body, "category", &data.category.unwrap_or_default());
        self.add_field(&mut body, "short_text", &data.short_text.unwrap_or_default());
        self.add_field(&mut body, "mini_text", &data.mini_text.unwrap_or_default());
        self.add_field(&mut body, "image_desc", &data.image_desc.unwrap_or_default());

        if data.is_main {
            self.add_field(&mut body, "is_main", "on");
        }
        if data.is_exclusive {
            self.add_field(&mut body, "is_exclusive", "on");
        }

        let related = data.related_articles.join("\n");
        self.add_field(&mut body, "related_articles", &related);

        if let Some(image_data) = data.image_data {
            let ext = data.image_ext.unwrap_or_else(|| "png".to_string());
            let mime = if ext == "jpg" || ext == "jpeg" { "image/jpeg" } else { "image/png" };
            self.add_file(&mut body, "image", &format!("image.{}", ext), mime, &image_data);
        }

        if let Some(audio_data) = data.audio_data {
            let ext = data.audio_ext.unwrap_or_else(|| "mp3".to_string());
            self.add_file(&mut body, "audio", &format!("audio.{}", ext), "audio/mpeg", &audio_data);
        }

        if let Some(video_data) = data.video_data {
            let ext = data.video_ext.unwrap_or_else(|| "mp4".to_string());
            self.add_file(&mut body, "video", &format!("video.{}", ext), "video/mp4", &video_data);
        }

        body.extend_from_slice(format!("--{}--\r\n", BOUNDARY).as_bytes());
        body
    }

    fn add_field(&self, body: &mut Vec<u8>, name: &str, value: &str) {
        body.extend_from_slice(format!("--{}\r\n", BOUNDARY).as_bytes());
        body.extend_from_slice(
            format!("Content-Disposition: form-data; name=\"{}\"\r\n", name).as_bytes(),
        );
        body.extend_from_slice(b"Content-Type: text/plain\r\n\r\n");
        body.extend_from_slice(format!("{}\r\n", value).as_bytes());
    }

    fn add_file(&self, body: &mut Vec<u8>, name: &str, filename: &str, mime: &str, data: &[u8]) {
        body.extend_from_slice(format!("--{}\r\n", BOUNDARY).as_bytes());
        body.extend_from_slice(
            format!(
                "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                name, filename
            )
            .as_bytes(),
        );
        body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", mime).as_bytes());
        body.extend_from_slice(data);
        body.extend_from_slice(b"\r\n");
    }
}
