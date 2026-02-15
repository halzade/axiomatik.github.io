use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct AdminArticleData {
    pub article_file_name: Option<String>,
}

impl AdminArticleData {
    pub const fn new() -> Self {
        Self { article_file_name: None }
    }
}

#[derive(Clone, Debug)]
pub struct AdminArticleFluent {
    data: Arc<RwLock<AdminArticleData>>,
}

impl Default for AdminArticleFluent {
    fn default() -> Self {
        Self::new()
    }
}

impl AdminArticleFluent {
    pub fn new() -> Self {
        Self { data: Arc::new(RwLock::new(AdminArticleData::new())) }
    }

    pub fn article_file_name(&self, name: &str) -> &Self {
        let mut guard = self.data.write();
        guard.article_file_name = Some(name.to_string());
        self
    }

    pub fn get_data(&self) -> AdminArticleData {
        let guard = self.data.read();
        AdminArticleData {
            article_file_name: guard.article_file_name.clone(),
        }
    }
}
