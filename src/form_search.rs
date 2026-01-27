use crate::{data, database, validation};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use axum::Form;
use http::StatusCode;
use serde::Deserialize;
use tracing::info;

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

#[derive(Template)]
#[template(path = "index_category_article_template.html")]
pub struct CategoryArticleTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
    pub is_first: bool,
    pub image_path: String,
    pub image_description: String,
    pub category_name: String,
    pub category_url: String,
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
        return (StatusCode::BAD_REQUEST, e.to_string()).into_response();
    }

    let search_words: Vec<String> = query
        .split_whitespace()
        .map(|w| w.trim().to_lowercase())
        .filter(|w| !w.is_empty())
        .collect();

    let articles_o = database::get_all_articles().await;

    match articles_o {
        None => {
            info!("No articles found in database");

            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
                date: data::date(),
                weather: data::weather(),
                name_day: data::name_day(),
                articles: "".to_string(),
            };

            Html(template.render().unwrap()).into_response()
        }
        Some(articles) => {
            let mut matched_results: Vec<(i32, String)> = Vec::new();

            for article in articles {
                let mut match_count = 0;
                let title_lower = article.title.to_lowercase();
                let text_lower = article.text.to_lowercase();
                let short_text_lower = article.short_text.to_lowercase();

                for word in &search_words {
                    if title_lower.contains(word) {
                        match_count += 10; // Higher weight for title match
                    }
                    if short_text_lower.contains(word) {
                        match_count += 5;
                    }
                    if text_lower.contains(word) {
                        match_count += 1;
                    }
                }

                if match_count > 0 {
                    let article_html = CategoryArticleTemplate {
                        url: article.article_file_name.clone(),
                        title: article.title.clone(),
                        short_text: article.short_text.clone(),
                        is_first: false,
                        image_path: article.image_url.clone(),
                        image_description: article.image_description.clone(),
                        category_name: article.category.clone(), // or display name if available
                        category_url: format!("{}.html", article.category),
                    }
                    .render()
                    .unwrap();

                    matched_results.push((match_count, article_html));
                }
            }

            // Sort by match count descending
            matched_results.sort_by(|a, b| b.0.cmp(&a.0));

            // TODO into a method?
            let articles_html: String = matched_results
                .into_iter()
                .map(|(_, content)| content)
                .collect::<Vec<String>>()
                .join("\n");

            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
                date: data::date(),
                weather: data::weather(),
                name_day: data::name_day(),
                articles: articles_html,
            };

            Html(template.render().unwrap()).into_response()
        }
    }
}
