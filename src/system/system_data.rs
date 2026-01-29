use crate::feature::{name_days, weather};
use crate::library;
use chrono::{DateTime, Duration, Local};
use std::collections::HashMap;
use std::sync::RwLock;
use tracing::error;

pub struct ApplicationData {
    date: RwLock<String>,
    name_day: RwLock<String>,
    weather: RwLock<String>,
}

type ArticleLastUpdates = HashMap<String, DateTime<Local>>;

pub struct ApplicationArticleData {
    // when was which HTML file last updated?
    updates: RwLock<ArticleLastUpdates>,
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
        let w = weather::fetch_weather().await;
        *self.weather.write().unwrap() = w;
    }
}

impl ApplicationArticleData {
    /*
     * returns true if article/html was last updated over 30 minutes ago.
     * maybe needs a weather update
     */
    pub fn article_update(&self, article_name: &str) -> bool {
        let now = Local::now();

        match self.updates.read() {
            Ok(updates) => {
                let is_present = updates.contains_key(article_name);
                if is_present {
                    match updates.get(article_name) {
                        None => {
                            error!("Article not found: {}", article_name);
                            false
                        }
                        Some(last_update_time) => {
                            // if last updated too old, maybe update the article
                            now.signed_duration_since(*last_update_time) > Duration::minutes(30)
                        }
                    }
                } else {
                    // new article, or server restart
                    self.updates
                        .write()
                        .expect("failed to write")
                        .insert(article_name.to_string(), now);
                    false
                }
            }
            Err(_) => {
                error!("Error while reading article data for {}", article_name);
                false
            }
        }
    }

    /*
     * html was updated from template with new data
     */
    pub fn article_update_now(&self, article_name: &str) {
        match self.updates.write() {
            Ok(mut updates) => {
                updates.insert(article_name.to_string(), Local::now());
            }
            Err(_) => {
                error!("Error while writing article data for {}", article_name);
            }
        }
    }
}

pub fn init_trivial_data() -> ApplicationData {
    ApplicationData {
        date: RwLock::new("Pondělí 1. Ledna 2024".to_string()),
        name_day: RwLock::new("Je Nový rok, státní svátek".to_string()),
        weather: RwLock::new("0°C | Praha".to_string()),
    }
}

pub fn init_trivial_article_data() -> ApplicationArticleData {
    let mut alu = ArticleLastUpdates::new();
    alu.insert("index".to_string(), Local::now());
    ApplicationArticleData {
        updates: RwLock::new(alu),
    }
}

#[cfg(test)]
mod tests {
    use crate::system::system_data::{init_trivial_article_data, init_trivial_data};

    #[test]
    fn test_application_data() {
        let ad = init_trivial_data();
        assert!(!ad.date().is_empty());
        assert!(!ad.name_day().is_empty());
        assert!(!ad.weather().is_empty());
    }

    #[test]
    fn test_application_article_data() {
        let aad = init_trivial_article_data();
        assert!(!aad.article_update("index"));
    }
}
