use crate::data::text_validator::validate_search_query;
use crate::db::database_article;
use crate::db::database_article_data::ShortArticleData;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use axum::Form;
use http::StatusCode;
use serde::Deserialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum SerchError {}

#[derive(Deserialize)]
pub struct SearchPayload {
    pub q: String,
}

#[derive(Template)]
#[template(path = "application/search/search_template.html")]
pub struct SearchTemplate {
    pub title: String,
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles: Vec<ShortArticleData>,
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

    if let Err(e) = validate_search_query(query) {
        return (StatusCode::BAD_REQUEST, e.to_string()).into_response();
    }

    let search_words: Vec<String> = query
        .split_whitespace()
        .map(|w| w.trim().to_lowercase())
        .filter(|w| !w.is_empty())
        .collect();

    let articles_r = database_article::articles_by_words(search_words, 20).await;

    match articles_r {
        Ok(articles) => {
            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
                date: "".into(),
                weather: "".into(),
                name_day: "".into(),
                articles,
            };

            Html(template.render().unwrap()).into_response()
        }
        Err(_) => {
            error!("error while searching articles");

            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
                date: "".into(),
                weather: "".into(),
                name_day: "".into(),
                articles: Vec::new(),
            };

            Html(template.render().unwrap()).into_response()
        }
    }
}
