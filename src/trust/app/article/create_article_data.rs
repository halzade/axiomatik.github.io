use parking_lot::RwLock;
use std::sync::Arc;

/**
 * article fluent interface
 */
#[derive(Debug, Clone)]
pub struct ArticleData {
    pub title: Option<String>,
    pub text: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub short_text: Option<String>,
    pub mini_text: Option<String>,
    pub is_main: bool,
    pub is_exclusive: bool,
    pub image_data: Option<Vec<u8>>,
    pub image_ext: Option<String>,
    pub image_desc: Option<String>,
    pub audio_data: Option<Vec<u8>>,
    pub audio_ext: Option<String>,
    pub video_data: Option<Vec<u8>>,
    pub video_ext: Option<String>,
    pub related_articles: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ArticleFluent {
    pub(crate) data: Arc<RwLock<ArticleData>>,
}

impl ArticleData {
    pub fn new() -> Self {
        Self {
            title: None,
            text: None,
            author: None,
            category: None,
            short_text: None,
            mini_text: None,
            is_main: false,
            is_exclusive: false,
            image_data: None,
            image_ext: None,
            image_desc: None,
            audio_data: None,
            audio_ext: None,
            video_data: None,
            video_ext: None,
            related_articles: Vec::new(),
        }
    }
}

impl ArticleFluent {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(ArticleData::new())),
        }
    }

    pub fn title(&self, title: &str) -> &Self {
        let mut guard = self.data.write();
        guard.title = Some(title.to_string());
        self
    }

    pub fn text(&self, text: &str) -> &Self {
        let mut guard = self.data.write();
        guard.text = Some(text.to_string());
        self
    }

    pub fn author(&self, author: &str) -> &Self {
        let mut guard = self.data.write();
        guard.author = Some(author.to_string());
        self
    }

    pub fn category(&self, category: &str) -> &Self {
        let mut guard = self.data.write();
        guard.category = Some(category.to_string());
        self
    }

    pub fn short_text(&self, short_text: &str) -> &Self {
        let mut guard = self.data.write();
        guard.short_text = Some(short_text.to_string());
        self
    }

    pub fn mini_text(&self, mini_text: &str) -> &Self {
        let mut guard = self.data.write();
        guard.mini_text = Some(mini_text.to_string());
        self
    }

    pub fn is_main(&self, is_main: bool) -> &Self {
        let mut guard = self.data.write();
        guard.is_main = is_main;
        self
    }

    pub fn is_exclusive(&self, is_exclusive: bool) -> &Self {
        let mut guard = self.data.write();
        guard.is_exclusive = is_exclusive;
        self
    }

    pub fn image(&self, data: Vec<u8>, ext: &str) -> &Self {
        let mut guard = self.data.write();
        guard.image_data = Some(data);
        guard.image_ext = Some(ext.to_string());
        self
    }

    pub fn image_desc(&self, desc: &str) -> &Self {
        let mut guard = self.data.write();
        guard.image_desc = Some(desc.to_string());
        self
    }

    pub fn audio(&self, data: Vec<u8>, ext: &str) -> &Self {
        let mut guard = self.data.write();
        guard.audio_data = Some(data);
        guard.audio_ext = Some(ext.to_string());
        self
    }

    pub fn video(&self, data: Vec<u8>, ext: &str) -> &Self {
        let mut guard = self.data.write();
        guard.video_data = Some(data);
        guard.video_ext = Some(ext.to_string());
        self
    }

    pub fn related_articles(&self, related: &str) -> &Self {
        let mut guard = self.data.write();
        guard.related_articles.push(related.to_string());
        self
    }

    // Safe read access (no poison, no unwrap)
    pub fn get_data(&self) -> ArticleData {
        let guard = self.data.read();
        ArticleData {
            title: guard.title.clone(),
            text: guard.text.clone(),
            author: guard.author.clone(),
            category: guard.category.clone(),
            short_text: guard.short_text.clone(),
            mini_text: guard.mini_text.clone(),
            is_main: guard.is_main,
            is_exclusive: guard.is_exclusive,
            image_data: guard.image_data.clone(),
            image_ext: guard.image_ext.clone(),
            image_desc: guard.image_desc.clone(),
            audio_data: guard.audio_data.clone(),
            audio_ext: guard.audio_ext.clone(),
            video_data: guard.video_data.clone(),
            video_ext: guard.video_ext.clone(),
            related_articles: guard.related_articles.clone(),
        }
    }
}
