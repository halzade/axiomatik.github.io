use crate::data::processor;
use crate::data::processor::ProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::system::data_system::DataSystem;
use askama::Template;
use thiserror::Error;
use FinanceError::CreateCategoryError;

#[derive(Debug, Error)]
pub enum FinanceError {
    #[error("create category error")]
    CreateCategoryError,

    #[error("create category processor error {0}")]
    ProcessorError(#[from] ProcessorError),

    #[error("create category database error {0}")]
    DatabaseError(#[from] SurrealError),
}

#[derive(Template)]
#[template(path = "application/finance/finance_template.html")]
pub struct FinanceTemplate<'a> {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<MiniArticleData>,
    pub articles_left: &'a [ShortArticleData],
    pub articles_right: &'a [ShortArticleData],
}

pub async fn render_finance(data_system: &DataSystem) -> Result<(), FinanceError> {
    let articles = database_article::articles_by_category("finance", 100).await?;
    let articles_most_read = database_article::articles_most_read(3).await?;

    let split = articles.len() / 3;
    let (articles_left, articles_right) = articles.split_at(split);
    let finance = FinanceTemplate {
        date: data_system.date(),
        weather: data_system.weather(),
        name_day: data_system.name_day(),
        articles_most_read,
        articles_left,
        articles_right,
    };
    match finance.render() {
        Ok(rendered_html) => {
            processor::save_web_file(rendered_html, "finance.html")?;
            Ok(())
        }
        Err(_) => Err(CreateCategoryError),
    }
}
