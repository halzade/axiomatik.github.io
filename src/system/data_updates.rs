use crate::system::data_updates::DateUpdatesError::Poisoned;
use chrono::{DateTime, Duration, Local};
use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DateUpdatesError {
    #[error("index lock")]
    Poisoned,
}

pub struct DataUpdates {
    // when was which HTML file last updated?
    articles: RwLock<HashMap<String, DateTime<Local>>>,
    index: RwLock<DateTime<Local>>,
    news: RwLock<DateTime<Local>>,
    finance: RwLock<DateTime<Local>>,
    republika: RwLock<DateTime<Local>>,
    technologie: RwLock<DateTime<Local>>,
    veda: RwLock<DateTime<Local>>,
    zahranici: RwLock<DateTime<Local>>,
}

pub fn new() -> DataUpdates {
    DataUpdates {

        // TODO these need to be Mutex

        articles: RwLock::new(HashMap::new()),
        index: RwLock::new(yesterday()),
        news: RwLock::new(yesterday()),
        finance: RwLock::new(yesterday()),
        republika: RwLock::new(yesterday()),
        technologie: RwLock::new(yesterday()),
        veda: RwLock::new(yesterday()),
        zahranici: RwLock::new(yesterday()),
    }
}

impl DataUpdates {

    pub fn index_lag(&self) -> Result<Duration, DateUpdatesError> {
        match self.index.read() {
            Ok(dt) => {
                let now = Local::now();
                Ok(now - *dt)
            }
            Err(_) => {
                Err(Poisoned)
            }
        }
    }
}

fn yesterday() -> DateTime<Local> {
    // after restart, all content gets updated when requested
    Local::now() - Duration::hours(25)
}

#[cfg(test)]
mod tests {}
