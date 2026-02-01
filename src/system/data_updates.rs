use parking_lot::RwLock;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataUpdatesError {
    #[error("index lock")]
    Poisoned,
}

/*
 * when was which HTML file last updated?
 * or invalidated because of a new article published
 */
pub struct DataUpdates {
    articles_valid: RwLock<HashMap<String, bool>>,
    index_valid: RwLock<bool>,
    news_valid: RwLock<bool>,
    finance_valid: RwLock<bool>,
    republika_valid: RwLock<bool>,
    technologie_valid: RwLock<bool>,
    veda_valid: RwLock<bool>,
    zahranici_valid: RwLock<bool>,
}

pub fn new() -> DataUpdates {
    DataUpdates {
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

impl DataUpdates {
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
    pub fn article_valid(&self, file_name: &str) -> bool {
        let valid_o = self.articles_valid.read().get(file_name).cloned();
        match valid_o {
            None => {
                // no record, new Article or restart
                self.articles_valid
                    .write()
                    .insert(file_name.to_string(), false);
                false
            }
            Some(valid) => valid,
        }
    }

    pub fn article_validate(&self, file_name: &str) {
        self.article_set(file_name, true);
    }

    pub fn article_invalidate(&self, file_name: &str) {
        self.article_set(file_name, false);
    }

    fn article_set(&self, file_name: &str, value: bool) {
        self.articles_valid
            .write()
            .insert(file_name.to_string(), value);
    }
}

#[cfg(test)]
mod tests {
    // TODO X
}
