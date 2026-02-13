use crate::db::database_system::ArticleStatus;
use parking_lot::RwLock;
use std::sync::Arc;

/**
 * system fluent interface
 */
#[derive(Debug)]
pub struct SystemData {
    pub article_status: Option<ArticleStatus>,
}

#[derive(Clone, Debug)]
pub struct SystemFluent {
    data: Arc<RwLock<SystemData>>,
}

impl Default for SystemFluent {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemFluent {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(SystemData {
                article_status: None,
            })),
        }
    }

    pub fn article_status(&self, status: ArticleStatus) -> &Self {
        let mut guard = self.data.write();
        guard.article_status = Some(status);
        self
    }

    // Safe read access (no poison, no unwrap)
    pub fn get_data(&self) -> SystemData {
        let guard = self.data.read();
        SystemData {
            article_status: guard.article_status,
        }
    }
}
