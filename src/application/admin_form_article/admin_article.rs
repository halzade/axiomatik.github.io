use crate::db::database_article_data::ShortArticleData;
use crate::system::server::TheState;
use askama::Template;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use thiserror::Error;
use tracing::{debug, info};

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
}

pub async fn show_admin_articles(
    State(state): State<TheState>,
) -> Result<Response, AdminArticleError> {
    debug!("show_admin_articles()");
    let articles = state
        .dba
        .list_all_articles()
        .await
        .map_err(|e| AdminArticleError::Database(e.to_string()))?;

    Ok(Html(
        AdminArticlesTemplate {
            articles,
            date: state.ds.date(),
            name_day: state.ds.name_day(),
            weather: state.ds.weather(),
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

    state
        .dba
        .delete_article(&article_file_name)
        .await
        .map_err(|e| AdminArticleError::Database(e.to_string()))?;
    info!("Admin deleted article: {}", article_file_name);

    // TODO delete the html file

    // TODO delete images, audio, video

    // Invalidate caches
    state.dv.index_invalidate();
    state.dv.news_invalidate();
    // Invalidate categories as well, to be sure
    state.dv.zahranici_invalidate();
    state.dv.republika_invalidate();
    state.dv.finance_invalidate();
    state.dv.technologie_invalidate();
    state.dv.veda_invalidate();

    Ok(Redirect::to("/admin/articles").into_response())
}
