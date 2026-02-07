use parking_lot::RwLock;
use std::collections::HashMap;
use thiserror::Error;
use ArticleStatus::{DoesntExist, Invalid, Valid};

#[derive(Error, Debug)]
pub enum DataUpdatesError {
    #[error("index lock")]
    Poisoned,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArticleStatus {
    Valid,
    Invalid,
    DoesntExist,
}

/*
 * when was which HTML file last updated?
 * or invalidated because of a new article published
 */
pub struct DataValidHtml {
    articles_valid: RwLock<HashMap<String, ArticleStatus>>,
    index_valid: RwLock<bool>,
    news_valid: RwLock<bool>,
    finance_valid: RwLock<bool>,
    republika_valid: RwLock<bool>,
    technologie_valid: RwLock<bool>,
    veda_valid: RwLock<bool>,
    zahranici_valid: RwLock<bool>,
}

pub fn new() -> DataValidHtml {
    DataValidHtml {
        articles_valid: RwLock::new(HashMap::new()),
        index_valid: RwLock::new(false),
        news_valid: RwLock::new(false),
        finance_valid: RwLock::new(false),
        republika_valid: RwLock::new(false),
        technologie_valid: RwLock::new(false),
        veda_valid: RwLock::new(false),
        zahranici_valid: RwLock::new(false),
    }
}

impl DataValidHtml {
    // index
    pub fn index_valid(&self) -> bool {
        *self.index_valid.read()
    }
    pub fn index_validate(&self) {
        *self.index_valid.write() = true;
    }
    pub fn index_invalidate(&self) {
        *self.index_valid.write() = false;
    }

    // republika
    pub fn republika_valid(&self) -> bool {
        *self.republika_valid.read()
    }
    pub fn republika_validate(&self) {
        *self.republika_valid.write() = true;
    }
    pub fn republika_invalidate(&self) {
        *self.republika_valid.write() = false;
    }

    // news
    pub fn news_valid(&self) -> bool {
        *self.news_valid.read()
    }
    pub fn news_validate(&self) {
        *self.news_valid.write() = true;
    }
    pub fn news_invalidate(&self) {
        *self.news_valid.write() = false;
    }

    // finance
    pub fn finance_valid(&self) -> bool {
        *self.finance_valid.read()
    }
    pub fn finance_validate(&self) {
        *self.finance_valid.write() = true;
    }
    pub fn finance_invalidate(&self) {
        *self.finance_valid.write() = false;
    }

    // technologie
    pub fn technologie_valid(&self) -> bool {
        *self.technologie_valid.read()
    }
    pub fn technologie_validate(&self) {
        *self.technologie_valid.write() = true;
    }
    pub fn technologie_invalidate(&self) {
        *self.technologie_valid.write() = false;
    }

    // veda
    pub fn veda_valid(&self) -> bool {
        *self.veda_valid.read()
    }
    pub fn veda_validate(&self) {
        *self.veda_valid.write() = true;
    }
    pub fn veda_invalidate(&self) {
        *self.veda_valid.write() = false;
    }

    // zahranici
    pub fn zahranici_valid(&self) -> bool {
        *self.zahranici_valid.read()
    }
    pub fn zahranici_validate(&self) {
        *self.zahranici_valid.write() = true;
    }
    pub fn zahranici_invalidate(&self) {
        *self.zahranici_valid.write() = false;
    }

    // articles
    pub fn article_valid(&self, file_name: &str) -> ArticleStatus {
        let read_guard = self.articles_valid.read();
        match read_guard.get(file_name) {
            Some(status) => *status,
            None => DoesntExist,
        }
    }

    pub fn article_validate(&self, file_name: &str) {
        self.article_set(file_name, Valid);
    }

    pub fn article_invalidate(&self, file_name: &str) {
        self.article_set(file_name, Invalid);
    }

    fn article_set(&self, file_name: &str, value: ArticleStatus) {
        self.articles_valid
            .write()
            .insert(file_name.to_string(), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let du = new();
        assert!(!du.index_valid());
        assert!(!du.news_valid());
        assert!(!du.finance_valid());
        assert!(!du.republika_valid());
        assert!(!du.technologie_valid());
        assert!(!du.veda_valid());
        assert!(!du.zahranici_valid());
    }

    #[test]
    fn test_index_validation() {
        let du = new();
        assert!(!du.index_valid());
        du.index_validate();
        assert!(du.index_valid());
        du.index_invalidate();
        assert!(!du.index_valid());
    }

    #[test]
    fn test_article_validation() {
        let du = new();
        let name = "test_article.html";

        assert_eq!(du.article_valid(name), DoesntExist);

        // Validate
        du.article_validate(name);
        match du.article_valid(name) {
            Valid => (),
            _ => panic!("Should be valid after validation"),
        }

        // Invalidate
        du.article_invalidate(name);
        match du.article_valid(name) {
            Invalid => (),
            _ => panic!("Should be invalid after invalidation"),
        }
    }

    #[test]
    fn test_multiple_articles() {
        let du = new();
        du.article_validate("a");
        du.article_invalidate("b");

        match du.article_valid("a") {
            Valid => (),
            _ => panic!("a should be valid"),
        }
        match du.article_valid("b") {
            Invalid => (),
            _ => panic!("b should be invalid"),
        }
        match du.article_valid("c") {
            ArticleStatus::DoesntExist => (),
            _ => panic!("c should not exist"),
        }
    }
}
