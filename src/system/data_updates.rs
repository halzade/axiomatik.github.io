use crate::system::data_updates::DataUpdatesError::Poisoned;
use chrono::{DateTime, Duration, Local};
use std::collections::HashMap;
use std::ops::Index;
use std::sync::RwLock;
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
    pub fn index_updated(&self) -> Result<DateTime<Local>, DataUpdatesError> {
        match self.index.read() {
            Ok(dt) => Ok(*dt),
            Err(_) => Err(Poisoned),
        }
    }

    pub fn news_updated(&self) -> Result<DateTime<Local>, DataUpdatesError> {
        match self.news.read() {
            Ok(dt) => Ok(*dt),
            Err(_) => Err(Poisoned),
        }
    }

    pub fn finance_updated(&self) -> Result<DateTime<Local>, DataUpdatesError> {
        match self.finance.read() {
            Ok(dt) => Ok(*dt),
            Err(_) => Err(Poisoned),
        }
    }
    pub fn republika_updated(&self) -> Result<DateTime<Local>, DataUpdatesError> {
        match self.republika.read() {
            Ok(dt) => Ok(*dt),
            Err(_) => Err(Poisoned),
        }
    }
    pub fn technologie_updated(&self) -> Result<DateTime<Local>, DataUpdatesError> {
        match self.technologie.read() {
            Ok(dt) => Ok(*dt),
            Err(_) => Err(Poisoned),
        }
    }
    pub fn veda_updated(&self) -> Result<DateTime<Local>, DataUpdatesError> {
        match self.veda.read() {
            Ok(dt) => Ok(*dt),
            Err(_) => Err(Poisoned),
        }
    }
    pub fn zahranici_updated(&self) -> Result<DateTime<Local>, DataUpdatesError> {
        match self.zahranici.read() {
            Ok(dt) => Ok(*dt),
            Err(_) => Err(Poisoned),
        }
    }
    pub fn article_updated(&self, file_name: &str) -> Result<DateTime<Local>, DataUpdatesError> {
        match self.article_updates.read() {
            Ok(hm) => {
                match hm.get(file_name) {
                    None => {
                        // record not found
                        match self.article_updates.write() {
                            Ok(mut hm) => {
                                hm.insert(file_name.into(), Local::now());
                            }
                            Err(_) => {}
                        }
                    }
                    Some(dtl) => {
                        // record found
                        Ok(*dtl)
                    }
                }
            }
            Err(_) => Err(Poisoned),
        }
    }

    // TODO
    // article_valids: RwLock::new(HashMap::new()),

    pub fn index_valid(&self) -> Result<bool, DataUpdatesError> {
        match self.index_valid.read() {
            Ok(b) => Ok(*b),
            Err(_) => Err(Poisoned),
        }
    }

    pub fn news_valid(&self) -> Result<bool, DataUpdatesError> {
        match self.news_valid.read() {
            Ok(b) => Ok(*b),
            Err(_) => Err(Poisoned),
        }
    }

    pub fn finance_valid(&self) -> Result<bool, DataUpdatesError> {
        match self.finance_valid.read() {
            Ok(b) => Ok(*b),
            Err(_) => Err(Poisoned),
        }
    }

    pub fn republika_valid(&self) -> Result<bool, DataUpdatesError> {
        match self.republika_valid.read() {
            Ok(b) => Ok(*b),
            Err(_) => Err(Poisoned),
        }
    }
    pub fn technologie_valid(&self) -> Result<bool, DataUpdatesError> {
        match self.technologie_valid.read() {
            Ok(b) => Ok(*b),
            Err(_) => Err(Poisoned),
        }
    }
    pub fn veda_valid(&self) -> Result<bool, DataUpdatesError> {
        match self.veda_valid.read() {
            Ok(b) => Ok(*b),
            Err(_) => Err(Poisoned),
        }
    }
    pub fn zahranici_valid(&self) -> Result<bool, DataUpdatesError> {
        match self.zahranici_valid.read() {
            Ok(b) => Ok(*b),
            Err(_) => Err(Poisoned),
        }
    }
}


fn yesterday() -> DateTime<Local> {
    // after restart, all content gets updated when requested
    Local::now() - Duration::hours(25)
}

#[cfg(test)]
mod tests {
    use crate::system::data_updates::{time_check_date, time_check_hour};
    use chrono::{Duration, Local};

    #[test]
    fn test_time_check_hour() {
        assert_eq!(false, time_check_hour(Local::now() - Duration::minutes(1)));
        assert_eq!(false, time_check_hour(Local::now() - Duration::minutes(30)));
        assert_eq!(false, time_check_hour(Local::now() - Duration::minutes(59)));

        assert_eq!(true, time_check_hour(Local::now() - Duration::hours(1)));
        assert_eq!(true, time_check_hour(Local::now() - Duration::hours(2)));
    }

    #[test]
    fn test_time_check_date() {
        assert_eq!(false, time_check_date(Local::now() - Duration::hours(6)));
        assert_eq!(false, time_check_date(Local::now() - Duration::hours(12)));
        assert_eq!(false, time_check_date(Local::now() - Duration::hours(23)));

        assert_eq!(true, time_check_date(Local::now() - Duration::days(1)));
        assert_eq!(true, time_check_date(Local::now() - Duration::days(2)));
    }
}
