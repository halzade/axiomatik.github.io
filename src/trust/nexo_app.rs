use crate::trust::article_builder::ArticleBuilder;

pub struct NexoApp {}

impl NexoApp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn post_create_article(&self) -> ArticleBuilder {
        ArticleBuilder::new()
    }
}
