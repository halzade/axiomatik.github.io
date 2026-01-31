use crate::feature::{name_days, weather};
use crate::library;
use chrono::Local;
use std::sync::RwLock;

pub struct DataSystem {
    date: RwLock<String>,
    name_day: RwLock<String>,
    weather: RwLock<String>,
}

pub fn new() -> DataSystem {
    DataSystem {
        // TODO
        date: RwLock::new(String::new()),
        name_day: RwLock::new(String::new()),
        weather: RwLock::new(String::new()),
    }
}

impl DataSystem {
    
    pub fn date(&self) -> String {
        self.date.read().unwrap().clone()
    }

    pub fn name_day(&self) -> String {
        self.name_day.read().unwrap().clone()
    }

    pub fn weather(&self) -> String {
        self.weather.read().unwrap().clone()
    }

    pub fn update_date(&self) {
        let d = library::formatted_article_date(Local::now());
        *self.date.write().unwrap() = d;
    }

    pub fn update_name_day(&self) {
        let nd = name_days::formatted_today_name_day(Local::now());
        *self.name_day.write().unwrap() = nd;
    }

    pub async fn update_weather(&self) {
        let w = weather::fetch_weather().await;
        *self.weather.write().unwrap() = w;
    }
}
