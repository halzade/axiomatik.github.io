use crate::feature::{name_days, weather};
use crate::library;
use chrono::{DateTime, Local};
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

    date_last_update: RwLock<DateTime<Local>>,
    weather_last_update: RwLock<DateTime<Local>>,
}

pub fn new() -> DataSystem {
    DataSystem {
        // TODO
        date: RwLock::new(String::new()),
        name_day: RwLock::new(String::new()),
        weather: RwLock::new(String::new()),

        date_last_update: RwLock::new(Local::now()),
        weather_last_update: RwLock::new(Local::now()),
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

    pub fn date_last_update(&self) -> DateTime<Local> {
        self.date_last_update.read().clone()
    }

    pub fn weather_last_update(&self) -> DateTime<Local> {
        self.weather_last_update.read().clone()
    }

    pub fn update_date(&self) {
        let d = library::formatted_article_date(Local::now());
        *self.date.write() = d;
    }

    pub fn update_name_day(&self) {
        let nd = name_days::formatted_today_name_day(Local::now());
        *self.name_day.write() = nd;
    }

    pub async fn update_weather(&self) {
        let w = weather::fetch_weather().await;
        *self.weather.write() = w;
    }
}
