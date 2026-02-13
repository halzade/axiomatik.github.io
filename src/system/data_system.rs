use crate::data::library;
use crate::feature::{name_days, weather};
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataSystemError {
    #[error("index lock")]
    Poisoned,
}

pub struct DataSystem {
    date: RwLock<String>,
    name_day: RwLock<String>,
    weather: RwLock<String>,

    date_last_update: RwLock<DateTime<Utc>>,
    weather_last_update: RwLock<DateTime<Utc>>,
}

pub fn new() -> DataSystem {
    let now = Utc::now();
    DataSystem {
        date: RwLock::new("".into()),
        name_day: RwLock::new("".into()),
        weather: RwLock::new("".into()),
        date_last_update: RwLock::new(now),
        weather_last_update: RwLock::new(now),
    }
}

impl DataSystem {
    pub fn date(&self) -> String {
        self.date.read().clone()
    }

    pub fn name_day(&self) -> String {
        self.name_day.read().clone()
    }

    pub fn weather(&self) -> String {
        self.weather.read().clone()
    }

    pub fn date_last_update(&self) -> DateTime<Utc> {
        *self.date_last_update.read()
    }

    pub fn weather_last_update(&self) -> DateTime<Utc> {
        *self.weather_last_update.read()
    }

    pub fn update_date(&self) {
        let d = library::formatted_article_date(Utc::now());
        *self.date.write() = d;
    }

    pub fn update_name_day(&self) {
        let nd = name_days::formatted_today_name_day(Utc::now());
        *self.name_day.write() = nd;
    }

    pub async fn update_weather(&self) -> bool {
        let previous = self.weather.read().to_string();
        let w = weather::fetch_weather().await;
        
        if previous != w {
            // weather change
            *self.weather.write() = w;
            return true;
        }
        false
    }
}
