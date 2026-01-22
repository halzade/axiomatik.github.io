use crate::{database, external, library, name_days, validation};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use axum::Form;
use http::StatusCode;
use serde::Deserialize;
use std::fs;
use tracing::{info, warn};

#[derive(Deserialize)]
pub struct SearchPayload {
    pub q: String,
}

#[derive(Template)]
#[template(path = "search_template.html")]
pub struct SearchTemplate {
    pub title: String,
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles: String,
}

pub async fn handle_search(Form(payload): Form<SearchPayload>) -> Response {
    let query = payload.q.trim();

    // Validate and sanitize the search query
    if query.chars().count() < 3 || query.chars().count() > 100 {
        return (
            StatusCode::BAD_REQUEST,
            "Search query must be between 3 and 100 characters",
        )
            .into_response();
    }

    if let Err(e) = validation::validate_search_query(query) {
        return (StatusCode::BAD_REQUEST, e).into_response();
    }

    let search_words: Vec<&str> = query
        .split_whitespace()
        .map(|w| w.trim())
        .filter(|w| !w.is_empty())
        .collect();

    let articles_o = database::get_all_articles().await;

    let now = chrono::Local::now();
    let formated_date = library::formatted_article_date(now);
    let formated_name_day = name_days::formatted_today_name_date(now);
    let formated_weather = external::fetch_weather().await;

    match articles_o {
        None => {
            info!("No articles found");

            // Sort by match count descending
            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
                date: formated_date,
                weather: formated_weather,
                name_day: formated_name_day,
                articles: "".to_string(),
            };

            let mut html = template.render().unwrap();
            html = html.replace("", &"");

            Html(html).into_response()
        }
        Some(articles) => {
            let mut matched_results = Vec::new();

            for article in articles {
                let mut match_count = 0;
                let article_text_lower = article.text.to_lowercase();
                for word in &search_words {
                    if article_text_lower.contains(&word.to_lowercase()) {
                        match_count += 1;
                    }
                }

                if match_count > 0 {
                    // Use the article url (article_file_name) to search /snippets/
                    let snippet_path = format!("snippets/{}.txt", article.article_file_name);
                    if let Ok(snippet_content) = fs::read_to_string(snippet_path) {
                        matched_results.push((match_count, snippet_content));
                    } else {
                        warn!(
                            "Snippet not found for article: {}",
                            article.article_file_name
                        );
                    }
                }
            }

            // Sort by match count descending
            matched_results.sort_by(|a, b| b.0.cmp(&a.0));

            let snippets_html: String = matched_results
                .into_iter()
                .map(|(_, content)| content)
                .collect::<Vec<String>>()
                .join("\n");

            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
                date: formated_date,
                weather: formated_weather,
                name_day: formated_name_day,
                articles: snippets_html,
            };

            let mut html = template.render().unwrap();

            // TODO tf ?
            html = html.replace("", &"");

            Html(html).into_response()
        }
    }
}
