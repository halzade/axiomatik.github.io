use crate::data::processor;
use crate::data::processor::ProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::system::data_system::DataSystem;
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

pub async fn render_news(data_system: &DataSystem) -> Result<(), NewsError> {
    let articles_most_read = database_article::articles_most_read(10).await?;
    let z_republiky = database_article::articles_by_category("republika", 10).await?;
    let ze_zahranici = database_article::articles_by_category("zahranici", 10).await?;
    let technologie = database_article::articles_by_category("technologie", 10).await?;
    let veda = database_article::articles_by_category("veda", 10).await?;
    let finance = database_article::articles_by_category("finance", 10).await?;

    let news = NewsTemplate {
        date: data_system.date(),
        weather: data_system.weather(),
        name_day: data_system.name_day(),
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
