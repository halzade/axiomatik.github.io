use crate::db::database_article_data::ShortArticleData;
use crate::system::server::TheState;
use askama::Template;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use std::fs;
use thiserror::Error;
use tracing::{debug, info};
use crate::system::router_app::AuthSession;

#[derive(Debug, Error)]
pub enum AdminArticleError {
    #[error("render error: {0}")]
    Render(#[from] askama::Error),

    #[error("database error: {0}")]
    Database(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

#[derive(Template)]
#[template(path = "application/admin_form_article/admin_article_template.html")]
pub struct AdminArticlesTemplate {
    pub articles: Vec<ShortArticleData>,
    pub date: String,
    pub name_day: String,
    pub weather: String,
    pub username: String,
}

pub async fn show_admin_articles(
    auth_session: AuthSession,
    State(state): State<TheState>,
) -> Result<Response, AdminArticleError> {
    debug!("show_admin_articles()");
    let articles = state
        .dba
        .list_all_articles()
        .await
        .map_err(|e| AdminArticleError::Database(e.to_string()))?;

    let username = auth_session
        .user
        .map(|u| u.username)
        .unwrap_or_else(|| "unknown".to_string());

    Ok(Html(
        AdminArticlesTemplate {
            articles,
            date: state.ds.date(),
            name_day: state.ds.name_day(),
            weather: state.ds.weather(),
            username,
        }
        .render()?,
    )
    .into_response())
}

pub async fn handle_delete_article(
    State(state): State<TheState>,
    Path(article_file_name): Path<String>,
) -> Result<Response, AdminArticleError> {
    debug!("handle_delete_article: {}", article_file_name);

    #[rustfmt::skip]
    let article = state.dba.article_by_file_name(&article_file_name).await
        .map_err(|e| AdminArticleError::Database(e.to_string()))?;

    let category = article.category.clone();

    #[rustfmt::skip]
    state.dba.delete_article(&article_file_name).await
        .map_err(|e| AdminArticleError::Database(e.to_string()))?;

    info!("Admin deleted article: {}", article_file_name);

    // delete the html file
    let path = format!("web/{}", article.article_file_name);
    let _ = fs::remove_file(path);

    // delete images
    let _ = fs::remove_file(format!("web/{}", article.image_50_path));
    let _ = fs::remove_file(format!("web/{}", article.image_288_path));
    let _ = fs::remove_file(format!("web/{}", article.image_440_path));
    let _ = fs::remove_file(format!("web/{}", article.image_820_path));

    // delete audio, video
    if article.has_audio {
        let _ = fs::remove_file(format!("web/{}", article.audio_path));
    }
    if article.has_video {
        let _ = fs::remove_file(format!("web/{}", article.video_path));
    }

    // Invalidate
    state.dv.index_invalidate();
    state.dv.news_invalidate();

    match category.as_str() {
        "zahranici" => state.dv.zahranici_invalidate(),
        "republika" => state.dv.republika_invalidate(),
        "finance" => state.dv.finance_invalidate(),
        "technologie" => state.dv.technologie_invalidate(),
        "veda" => state.dv.veda_invalidate(),
        _ => info!("Unknown category for invalidation: {}", category),
    }

    Ok(Redirect::to("/admin_article").into_response())
}
