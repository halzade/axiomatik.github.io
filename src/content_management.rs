use crate::library::get_czech_month_genitive;
use crate::{library, name_days};
use axum::response::{Html, IntoResponse};
use chrono::Local;
use http::StatusCode;
use std::fs;
use tokio::time::Instant;
use tracing::{error, info};

fn replace_nameday_in_content(content: &str, nameday_string: &str) -> String {
    let start_tag = "<!-- NAME_DAY -->";
    let end_tag = "<!-- /NAME_DAY -->";
    replace_in_content(start_tag, end_tag, content, nameday_string)
}

fn replace_weather_in_content(content: &str, weather_string: &str) -> String {
    let start_tag = "<!-- WEATHER -->";
    let end_tag = "<!-- /WEATHER -->";
    replace_in_content(start_tag, end_tag, content, weather_string)
}

fn replace_date_in_content(content: &str, date_string: &str) -> String {
    let start_tag = "<!-- DATE -->";
    let end_tag = "<!-- /DATE -->";
    replace_in_content(start_tag, end_tag, content, date_string)
}

fn replace_in_content(
    start_tag: &str,
    end_tag: &str,
    content: &str,
    nameday_string: &str,
) -> String {
    if let (Some(start), Some(end)) = (content.find(start_tag), content.find(end_tag)) {
        let mut new_content = content[..start + start_tag.len()].to_string();
        new_content.push_str(nameday_string);
        new_content.push_str(&content[end..]);
        new_content
    } else {
        content.to_string()
    }
}

pub fn update_index_date() {
    let now = Local::now();
    let day_name = library::day_of_week(now);
    let month_name = get_czech_month_genitive(now.month());
    let date_string = format!("{} {}. {} {}", day_name, now.day(), month_name, now.year());

    update_all_header_info(&date_string, "", "");
}

pub fn update_index_nameday() {
    let name = name_days::today_name_day();
    let nameday_string =
        if name.is_empty() || name.contains("No nameday") || name.contains("Invalid") {
            "".to_string()
        } else {
            format!("Svátek má {}", name)
        };

    // TODO
    // update_all_header_info("", &nameday_string, "");
}

fn update_all_header_info(date_str: &str, nameday_str: &str, weather_str: &str) {
    let files = [
        "index.html",
        "republika.html",
        "zahranici.html",
        "technologie.html",
        "finance.html",
        "veda.html",
    ];

    for file in files {
        if let Ok(mut content) = fs::read_to_string(file) {
            let mut changed = false;

            // date
            if !date_str.is_empty() {
                let next = replace_date_in_content(&content, date_str);
                if next != content {
                    content = next;
                    changed = true;
                }
            }

            // nameday
            if !nameday_str.is_empty() {
                let next = replace_nameday_in_content(&content, nameday_str);
                if next != content {
                    content = next;
                    changed = true;
                }
            }

            // weather
            if !weather_str.is_empty() {
                let next = replace_weather_in_content(&content, weather_str);
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
    let url = "https://api.open-meteo.com/v1/forecast?latitude=50.0755&longitude=14.4378&current_weather=true&timezone=Europe/Prague";

    let weather_string = match fetch_weather(url).await {
        Ok(temp) => format!("{:.0}°C | Praha", temp),
        Err(_) => "".to_string(),
    };

    update_all_header_info("", "", &weather_string);
}

fn next_midnight_instant() -> Instant {
    let now = Local::now();

    let next_midnight = now
        .date_naive()
        .succ_opt()
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let duration_until = (next_midnight - now.naive_local()).to_std().unwrap();

    Instant::now() + duration_until
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
        let new_content = replace_nameday_in_content(content, nameday_string);
        assert_eq!(
            new_content,
            "<html><!-- NAME_DAY -->Svátek má Jaroslava<!-- /NAME_DAY --></html>"
        );
    }

    #[test]
    fn test_no_nameday_replacement_if_same() {
        let content = "<html><!-- NAME_DAY -->Svátek má Jaroslava<!-- /NAME_DAY --></html>";
        let nameday_string = "Svátek má Jaroslava";
        let new_content = replace_nameday_in_content(content, nameday_string);
        assert_eq!(content, new_content);
    }
}
