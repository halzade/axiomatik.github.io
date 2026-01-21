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

// TODO

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

           // TODO

            if changed {
                if let Err(e) = fs::write(file, content) {
                    error!("Failed to write {}: {}", file, e);
                }
            }
        }
    }
}

pub async fn update_index_weather() {
    // TODO
}

