use crate::{external, library, name_days};
use chrono::Local;
use std::sync::RwLock;

pub struct ApplicationData {
    date: RwLock<String>,
    name_day: RwLock<String>,
    weather: RwLock<String>,
}

impl ApplicationData {
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
        let w = external::fetch_weather().await;
        *self.weather.write().unwrap() = w;
    }
}

pub async fn init() -> ApplicationData {
    let ad = ApplicationData {
        date: RwLock::new("".to_string()),
        name_day: RwLock::new("".to_string()),
        weather: RwLock::new("".to_string()),
    };

    // TODO start these in parallel
    ad.update_date();
    ad.update_name_day();
    ad.update_weather().await;

    ad
}

pub fn init_trivial() -> ApplicationData {
    ApplicationData {
        date: RwLock::new("Pondělí 1. Ledna 2024".to_string()),

        // TODO
        name_day: RwLock::new("Svátek má Nový rok".to_string()),
        weather: RwLock::new("0°C | Praha".to_string()),
    }
}
