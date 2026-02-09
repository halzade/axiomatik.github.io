use crate::data::processor;
use crate::data::processor::ProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article::SurrealArticleError;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::system::server::TheState;
use askama::Template;
use thiserror::Error;
use NewsError::CreateCategoryError;

#[derive(Debug, Error)]
pub enum NewsError {
    #[error("create category error")]
    CreateCategoryError,

    #[error("create category processor error {0}")]
    ProcessorError(#[from] ProcessorError),

    #[error("create category database error {0}")]
    DatabaseError(#[from] SurrealError),

    #[error("create category database error {0}")]
    SurrealArticle(#[from] SurrealArticleError),
}

#[derive(Template)]
#[template(path = "application/news/news_template.html")]
pub struct NewsTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<MiniArticleData>,
    pub z_republiky: Vec<ShortArticleData>,
    pub ze_zahranici: Vec<ShortArticleData>,
    pub technologie: Vec<ShortArticleData>,
    pub veda: Vec<ShortArticleData>,
    pub finance: Vec<ShortArticleData>,
}

pub async fn render_news(state: &TheState) -> Result<(), NewsError> {
    let articles_most_read = state.dba.articles_most_read(10).await?;
    let z_republiky = state.dba.articles_by_category("republika", 10).await?;
    let ze_zahranici = state.dba.articles_by_category("zahranici", 10).await?;
    let technologie = state.dba.articles_by_category("technologie", 10).await?;
    let veda = state.dba.articles_by_category("veda", 10).await?;
    let finance = state.dba.articles_by_category("finance", 10).await?;

    let news = NewsTemplate {
        date: state.ds.date(),
        weather: state.ds.weather(),
        name_day: state.ds.name_day(),
        articles_most_read,
        z_republiky,
        ze_zahranici,
        technologie,
        veda,
        finance,
    };
    match news.render() {
        Ok(rendered_html) => {
            processor::save_web_file(rendered_html, "news.html")?;
            Ok(())
        }
        Err(_) => Err(CreateCategoryError),
    }
}
