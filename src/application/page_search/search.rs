use crate::data::text_validator::validate_search_query;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::system::server::TheState;
use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Response};
use axum::Form;
use http::StatusCode;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
pub struct SearchPayload {
    pub q: String,
}

#[derive(Template)]
#[template(path = "application/page_search/search_template.html")]
pub struct SearchTemplate {
    pub title: String,
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<MiniArticleData>,
    pub articles: Vec<ShortArticleData>,
}

pub async fn handle_search(
    State(state): State<TheState>,
    Form(payload): Form<SearchPayload>,
) -> Response {
    let query = payload.q.trim();

    // Validate and sanitize the search query
    if query.chars().count() < 3 || query.chars().count() > 100 {
        return (StatusCode::BAD_REQUEST, "Search query must be between 3 and 100 characters")
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

    let articles_r = state.dba.articles_by_words(search_words, 20).await;
    let articles_most_read_r = state.dba.most_read_by_views().await;

    let articles_most_read_use;
    match articles_most_read_r {
        Ok(articles_most_read) => {
            articles_most_read_use = articles_most_read;
        }
        Err(e) => {
            error!("error while getting most read articles: {}", e);
            articles_most_read_use = Vec::new();
        }
    }

    match articles_r {
        Ok(articles) => {
            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
                date: state.ds.date(),
                weather: state.ds.weather(),
                name_day: state.ds.name_day(),
                articles,
                articles_most_read: articles_most_read_use,
            };

            template.render().map_or_else(
                |_| StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                |html| Html(html).into_response(),
            )
        }
        Err(_) => {
            error!("error while searching articles");

            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
                date: state.ds.date(),
                weather: state.ds.weather(),
                name_day: state.ds.name_day(),
                articles: Vec::new(),
                articles_most_read: articles_most_read_use,
            };

            template.render().map_or_else(
                |_| StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                |html| Html(html).into_response(),
            )
        }
    }
}
