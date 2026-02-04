use crate::data::processor;
use crate::data::processor::ProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::system::data_system::DataSystem;
use askama::Template;
use thiserror::Error;
use ZahraniciError::CreateCategoryError;

#[derive(Debug, Error)]
pub enum ZahraniciError {
    #[error("create category error")]
    CreateCategoryError,

    #[error("create category processor error {0}")]
    ProcessorError(#[from] ProcessorError),

    #[error("create category database error {0}")]
    DatabaseError(#[from] SurrealError),
}

#[derive(Template)]
#[template(path = "application/zahranici/zahranici_template.html")]
pub struct ZahraniciTemplate<'a> {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<MiniArticleData>,
    pub articles_left: &'a [ShortArticleData],
    pub articles_right: &'a [ShortArticleData],
}

pub async fn render_zahranici(data_system: &DataSystem) -> Result<(), ZahraniciError> {
    let articles = database_article::articles_by_category("zahranici", 100).await?;
    let articles_most_read = database_article::articles_most_read(3).await?;

    let split = articles.len() / 3;
    let (articles_left, articles_right) = articles.split_at(split);
    let zahranici = ZahraniciTemplate {
        date: data_system.date(),
        weather: data_system.weather(),
        name_day: data_system.name_day(),
        articles_most_read,
        articles_left,
        articles_right,
    };
    match zahranici.render() {
        Ok(rendered_html) => {
            processor::save_web_file(rendered_html, "zahranici.html")?;
            Ok(())
        }
        Err(_) => Err(CreateCategoryError),
    }
}
