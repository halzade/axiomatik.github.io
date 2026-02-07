#[derive(Clone, Debug, Default)]
pub struct ArticleData {
    pub title: Option<String>,
    pub text: Option<String>,
}

impl ArticleData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
}
