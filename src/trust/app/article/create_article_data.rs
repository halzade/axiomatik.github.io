use parking_lot::RwLock;
use std::sync::Arc;

/**
 * article fluent interface
 */
#[derive(Debug)]
struct ArticleData {
    pub title: Option<String>,
    pub text: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ArticleFluent {
    data: Arc<RwLock<ArticleData>>,
}

impl ArticleFluent {
    pub fn new() -> Self {
        Self { data: Arc::new(RwLock::new(ArticleData { title: None, text: None })) }
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

    // Safe read access (no poison, no unwrap)
    pub fn get_data(&self) -> ArticleData {
        let guard = self.data.read();
        ArticleData { title: guard.title.clone(), text: guard.text.clone() }
    }
}
