use crate::library::get_czech_month_genitive;
use crate::{external, library, name_days};
use axum::response::{Html, IntoResponse};
use chrono::prelude::*;
use std::fs;
use tracing::{error, info};

fn replace_name_day_in_content(content: &str, nameday_string: String) -> String {
    let start_tag = "<!-- NAME_DAY -->";
    let end_tag = "<!-- /NAME_DAY -->";
    replace_in_content(start_tag, end_tag, content, nameday_string)
}

fn replace_weather_in_content(content: &str, weather_string: String) -> String {
    let start_tag = "<!-- WEATHER -->";
    let end_tag = "<!-- /WEATHER -->";
    replace_in_content(start_tag, end_tag, content, weather_string)
}

fn replace_date_in_content(content: &str, date_string: String) -> String {
    let start_tag = "<!-- DATE -->";
    let end_tag = "<!-- /DATE -->";
    replace_in_content(start_tag, end_tag, content, date_string)
}

fn replace_in_content(
    start_tag: &str,
    end_tag: &str,
    content: &str,
    nameday_string: String,
) -> String {
    if let (Some(start), Some(end)) = (content.find(start_tag), content.find(end_tag)) {
        let mut new_content = content[..start + start_tag.len()].to_string();
        new_content.push_str(&*nameday_string);
        new_content.push_str(&content[end..]);
        new_content
    } else {
        content.to_string()
    }
}

pub fn update_all_header_info(now: DateTime<Local>) {

    // TODO
    let files = [
        "index.html",
        "republika.html",
        "zahranici.html",
        "technologie.html",
        "finance.html",
        "veda.html",
    ];

    let formated_date = library::formatted_article_date(now);
    let formated_name_day = name_days::formatted_today_name_date(now);
    let formated_weather = external::fetch_weather().await;

    for file in files {
        if let Ok(mut content) = fs::read_to_string(file) {
            let mut changed = false;

            // date
            if !formated_date.is_empty() {
                let next = replace_date_in_content(&content, formated_date);
                if next != content {
                    content = next;
                    changed = true;
                }
            }

            // nameday
            if !formated_name_day.is_empty() {
                let next = replace_name_day_in_content(&content, formated_name_day);
                if next != content {
                    content = next;
                    changed = true;
                }
            }

            // weather
            if !formated_weather.is_empty() {
                let next = replace_weather_in_content(&content, formated_weather);
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
        let weather_string = "23°C | Praha";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(
            new_content,
            "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>"
        );
    }

    #[test]
    fn test_no_weather_replacement_if_same() {
        let content = "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>";
        let weather_string = "23°C | Praha";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(content, new_content);
    }

    #[test]
    fn test_weather_replacement_empty_if_exception() {
        let content = "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>";
        let weather_string = "";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(
            new_content,
            "<html><!-- WEATHER --><!-- /WEATHER --></html>"
        );
    }

    #[test]
    fn test_nameday_replacement_logic() {
        let content = "<html><!-- NAME_DAY -->OLD<!-- /NAME_DAY --></html>";
        let nameday_string = "Svátek má Jaroslava";
        let new_content = replace_name_day_in_content(content, nameday_string);
        assert_eq!(
            new_content,
            "<html><!-- NAME_DAY -->Svátek má Jaroslava<!-- /NAME_DAY --></html>"
        );
    }

    #[test]
    fn test_no_nameday_replacement_if_same() {
        let content = "<html><!-- NAME_DAY -->Svátek má Jaroslava<!-- /NAME_DAY --></html>";
        let nameday_string = "Svátek má Jaroslava";
        let new_content = replace_name_day_in_content(content, nameday_string);
        assert_eq!(content, new_content);
    }
}
