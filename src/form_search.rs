use crate::{database, validation};
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

// TODO it's own category
#[derive(Template)]
#[template(path = "category_template.html")]
pub struct SearchTemplate {
    pub title: String,
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

    match articles_o {
        None => {
            info!("No articles found");

            // Sort by match count descending
            let template = SearchTemplate {
                title: format!("Výsledky hledání: {}", query),
            };

            let mut html = template.render().unwrap();
            html = html.replace("<!-- SNIPPETS -->", &"");

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
            };

            let mut html = template.render().unwrap();
            html = html.replace("<!-- SNIPPETS -->", &snippets_html);

            Html(html).into_response()
        }
    }
}
