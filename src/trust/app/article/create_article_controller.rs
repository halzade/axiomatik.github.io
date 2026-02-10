use crate::trust::app::article::create_article_data::{ArticleData, ArticleFluent};
use crate::trust::data::media_data::BOUNDARY;
use crate::trust::data::response_verifier::ResponseVerifier;
use crate::trust::data::utils::content_type_with_boundary;
use crate::trust::me::TrustError;
use axum::body::Body;
use axum::Router;
use http::{header, Request};
use std::io::Write;
use std::sync::Arc;
use tower::ServiceExt;

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
        let image_data = std::fs::read("tests/data/image_1024.png")?;
        self.image(image_data, "png");
        if self.input.get_data().image_desc.is_none() {
            self.image_desc("any png description");
        }
        Ok(self)
    }

    pub async fn execute(&self) -> Result<ResponseVerifier, TrustError> {
        let data = self.input.get_data();
        let body = self.build_multipart_body(data)?;
        let cookie = self.user_cookie.read().clone().unwrap_or_default();

        let response = self
            .app_router
            .as_ref()
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/create")
                    .header(header::CONTENT_TYPE, content_type_with_boundary())
                    .header(header::COOKIE, cookie)
                    .body(Body::from(body))?,
            )
            .await?;

        // Clear input after execution if successful
        if response.status().is_success() || response.status().is_redirection() {
            *self.input.data.write() = ArticleData::new();
        }

        Ok(ResponseVerifier::new(response))
    }

    fn build_multipart_body(&self, data: ArticleData) -> Result<Vec<u8>, TrustError> {
        let mut body = Vec::new();

        let title = data.title.clone().unwrap_or_default();
        let author = data.author.clone().unwrap_or_default();
        let text = data.text.clone().unwrap_or_default();
        let category = data.category.clone().unwrap_or_default();
        let short_text = data.short_text.clone().unwrap_or_else(|| text.clone());
        let mini_text = data.mini_text.clone().unwrap_or_else(|| short_text.clone());

        self.add_field(&mut body, "title", &title)?;
        self.add_field(&mut body, "author", &author)?;
        self.add_field(&mut body, "category", &category)?;
        self.add_field(&mut body, "text", &text)?;
        self.add_field(&mut body, "short_text", &short_text)?;
        self.add_field(&mut body, "mini_text", &mini_text)?;

        if data.is_main {
            self.add_field(&mut body, "is_main", "on")?;
        }
        if data.is_exclusive {
            self.add_field(&mut body, "is_exclusive", "on")?;
        }

        if let Some(image_desc) = data.image_desc {
            self.add_field(&mut body, "image_desc", &image_desc)?;
        }

        let related = data.related_articles.join("\n");
        if !related.is_empty() {
            self.add_field(&mut body, "related_articles", &related)?;
        }

        if let Some(image_data) = data.image_data {
            let ext = data.image_ext.unwrap_or_else(|| "png".to_string());
            let mime = if ext == "jpg" || ext == "jpeg" { "image/jpeg" } else { "image/png" };
            self.add_file(&mut body, "image", &format!("image.{}", ext), mime, &image_data)?;
        }

        if let Some(audio_data) = data.audio_data {
            let ext = data.audio_ext.unwrap_or_else(|| "mp3".to_string());
            self.add_file(
                &mut body,
                "audio",
                &format!("audio.{}", ext),
                "audio/mpeg",
                &audio_data,
            )?;
        }

        if let Some(video_data) = data.video_data {
            let ext = data.video_ext.unwrap_or_else(|| "mp4".to_string());
            self.add_file(&mut body, "video", &format!("video.{}", ext), "video/mp4", &video_data)?;
        }

        write!(body, "--{}--\r\n", BOUNDARY)?;
        Ok(body)
    }

    fn add_field(&self, body: &mut Vec<u8>, name: &str, value: &str) -> Result<(), TrustError> {
        write!(
            body,
            "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
            BOUNDARY, name, value
        )?;
        Ok(())
    }

    fn add_file(
        &self,
        body: &mut Vec<u8>,
        name: &str,
        filename: &str,
        mime: &str,
        data: &[u8],
    ) -> Result<(), TrustError> {
        write!(body, "--{}\r\n", BOUNDARY)?;
        write!(
            body,
            "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
            name, filename
        )?;
        write!(body, "Content-Type: {}\r\n\r\n", mime)?;
        body.extend_from_slice(data);
        write!(body, "\r\n")?;
        Ok(())
    }
}
