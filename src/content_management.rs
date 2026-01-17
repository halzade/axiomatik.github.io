use crate::{external, library, name_days};
use chrono::prelude::*;
use std::fs;
use tracing::error;

// TODO
const FILES: [&str; 6] = [
    "index.html",
    "republika.html",
    "zahranici.html",
    "technologie.html",
    "finance.html",
    "veda.html",
];

fn replace_name_day_in_content(content: &str, name_day: &String) -> String {
    let start_tag = "<!-- NAME_DAY -->";
    let end_tag = "<!-- /NAME_DAY -->";
    replace_in_content(start_tag, end_tag, content, name_day)
}

fn replace_weather_in_content(content: &str, weather_string: &String) -> String {
    let start_tag = "<!-- WEATHER -->";
    let end_tag = "<!-- /WEATHER -->";
    replace_in_content(start_tag, end_tag, content, weather_string)
}

fn replace_date_in_content(content: &str, date_string: &String) -> String {
    let start_tag = "<!-- DATE -->";
    let end_tag = "<!-- /DATE -->";
    replace_in_content(start_tag, end_tag, content, date_string)
}

fn replace_in_content(start_tag: &str, end_tag: &str, content: &str, name_day: &String) -> String {
    if let (Some(start), Some(end)) = (content.find(start_tag), content.find(end_tag)) {
        let mut new_content = content[..start + start_tag.len()].to_string();
        new_content.push_str(name_day);
        new_content.push_str(&content[end..]);
        new_content
    } else {
        content.to_string()
    }
}

pub async fn update_all_header_info(now: DateTime<Local>) {
    let formated_date = library::formatted_article_date(now);
    let formated_name_day = name_days::formatted_today_name_date(now);
    let formated_weather = external::fetch_weather().await;

    for file in FILES {
        if let Ok(mut content) = fs::read_to_string(file) {
            let mut changed = false;

            // date
            if !formated_date.is_empty() {
                let next = replace_date_in_content(&content, &formated_date);
                if next != content {
                    content = next;
                    changed = true;
                }
            }

            // name day
            if !formated_name_day.is_empty() {
                let next = replace_name_day_in_content(&content, &formated_name_day);
                if next != content {
                    content = next;
                    changed = true;
                }
            }

            // weather
            if !formated_weather.is_empty() {
                let next = replace_weather_in_content(&content, &formated_weather);
                if next != content {
                    content = next;
                    changed = true;
                }
            }

            if changed {
                if let Err(e) = fs::write(file, content) {
                    error!("Failed to write {}: {}", file, e);
                }
            }
        }
    }
}

pub async fn update_index_weather() {
    let formated_weather = external::fetch_weather().await;

    for file in FILES {
        if let Ok(mut content) = fs::read_to_string(file) {
            let mut changed = false;

            // weather
            if !formated_weather.is_empty() {
                let next = replace_weather_in_content(&content, &formated_weather);
                if next != content {
                    content = next;
                    changed = true;
                }
            }

            if changed {
                if let Err(e) = fs::write(file, content) {
                    error!("Failed to write {}: {}", file, e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_replacement_logic() {
        let content = "<html><!-- WEATHER -->OLD<!-- /WEATHER --></html>";
        let weather_string = "23°C | Praha".into();
        let new_content = replace_weather_in_content(content, &weather_string);
        assert_eq!(
            new_content,
            "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>"
        );
    }

    #[test]
    fn test_no_weather_replacement_if_same() {
        let content = "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>";
        let weather_string = "23°C | Praha".into();
        let new_content = replace_weather_in_content(content, &weather_string);
        assert_eq!(content, new_content);
    }

    #[test]
    fn test_weather_replacement_empty_if_exception() {
        let content = "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>";
        let weather_string = "".into();
        let new_content = replace_weather_in_content(content, &weather_string);
        assert_eq!(
            new_content,
            "<html><!-- WEATHER --><!-- /WEATHER --></html>"
        );
    }

    #[test]
    fn test_name_day_replacement() {
        let content = "<html><!-- NAME_DAY -->OLD<!-- /NAME_DAY --></html>";
        let name_day = "Svátek má Jaroslava".into();
        let new_content = replace_name_day_in_content(content, &name_day);
        assert_eq!(
            new_content,
            "<html><!-- NAME_DAY -->Svátek má Jaroslava<!-- /NAME_DAY --></html>"
        );
    }

    #[test]
    fn test_no_name_day_replacement_if_same() {
        let content = "<html><!-- NAME_DAY -->Svátek má Jaroslava<!-- /NAME_DAY --></html>";
        let name_day = "Svátek má Jaroslava".into();
        let new_content = replace_name_day_in_content(content, &name_day);
        assert_eq!(content, new_content);
    }
}
