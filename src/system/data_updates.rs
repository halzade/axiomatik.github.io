use chrono::{DateTime, Duration, Local};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::ops::Index;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataUpdatesError {
    #[error("index lock")]
    Poisoned,
}

/*
 * when was which HTML file last updated?
 * or invalidated, because of new artile published
 */
pub struct DataUpdates {
    articles_update: RwLock<HashMap<String, DateTime<Local>>>,
    articles_valid: RwLock<HashMap<String, bool>>,
    index: RwLock<DateTime<Local>>,
    news: RwLock<DateTime<Local>>,
    finance: RwLock<DateTime<Local>>,
    republika: RwLock<DateTime<Local>>,
    zahranici: RwLock<DateTime<Local>>,
    technologie: RwLock<DateTime<Local>>,
    veda: RwLock<DateTime<Local>>,
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
        articles_update: RwLock::new(HashMap::new()),
        articles_valid: RwLock::new(HashMap::new()),
        index: RwLock::new(yesterday()),
        news: RwLock::new(yesterday()),
        finance: RwLock::new(yesterday()),
        republika: RwLock::new(yesterday()),
        technologie: RwLock::new(yesterday()),
        veda: RwLock::new(yesterday()),
        zahranici: RwLock::new(yesterday()),
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
    pub fn index_updated(&self) -> DateTime<Local> {
        *self.index.read()
    }

    pub fn news_updated(&self) -> DateTime<Local> {
        *self.news.read()
    }

    pub fn finance_updated(&self) -> DateTime<Local> {
        *self.finance.read()
    }
    pub fn republika_updated(&self) -> DateTime<Local> {
        *self.republika.read()
    }
    pub fn technologie_updated(&self) -> DateTime<Local> {
        *self.technologie.read()
    }
    pub fn veda_updated(&self) -> DateTime<Local> {
        *self.veda.read()
    }
    pub fn zahranici_updated(&self) -> DateTime<Local> {
        *self.zahranici.read()
    }
    pub fn article_updated(&self, file_name: &str) -> DateTime<Local> {
        let ldt_o = self.articles_update.read().get(file_name).cloned();
        match ldt_o {
            None => {
                self.articles_update.write().insert(file_name.to_string(), Local::now());
                
                
            }
            Some(ldt) => {
                ldt
            }
        }
    }

    // TODO
    // article_valids: RwLock::new(HashMap::new()),
    pub fn index_valid(&self) -> bool {
        *self.index_valid.read()
    }

    pub fn news_valid(&self) -> bool {
        *self.news_valid.read()
    }

    pub fn finance_valid(&self) -> bool {
        *self.finance_valid.read()
    }

    pub fn republika_valid(&self) -> bool {
        *self.republika_valid.read()
    }
    pub fn technologie_valid(&self) -> bool {
        *self.technologie_valid.read()
    }

    pub fn veda_valid(&self) -> bool {
        *self.veda_valid.read()
    }
    pub fn zahranici_valid(&self) -> bool {
        *self.zahranici_valid.read()
    }
}

fn yesterday() -> DateTime<Local> {
    // after restart, all content gets updated when requested
    Local::now() - Duration::hours(25)
}

#[cfg(test)]
mod tests {
    
}
