use crate::{external, library, name_days};
use chrono::Local;
use std::sync::{LazyLock, RwLock};

// TODO don't init trivial
pub static GLOBAL_DATA: LazyLock<ApplicationData> = LazyLock::new(|| init_trivial());

pub fn date() -> String {
    GLOBAL_DATA.date()
}

pub fn name_day() -> String {
    GLOBAL_DATA.name_day()
}

pub fn weather() -> String {
    GLOBAL_DATA.weather()
}

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

pub fn init_trivial() -> ApplicationData {
    ApplicationData {
        date: RwLock::new("Pondělí 1. Ledna 2024".to_string()),
        name_day: RwLock::new("Je Nový rok, státní svátek".to_string()),
        weather: RwLock::new("0°C | Praha".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_data_access() {
        assert!(!date().is_empty());
        assert!(!name_day().is_empty());
        assert!(!weather().is_empty());
    }
}
