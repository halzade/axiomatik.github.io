use crate::data::processor;
use crate::data::processor::ProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::system::server::TheState;
use askama::Template;
use thiserror::Error;
use VedaError::CreateCategoryError;

#[derive(Debug, Error)]
pub enum VedaError {
    #[error("create category error")]
    CreateCategoryError,

    #[error("create category processor error {0}")]
    ProcessorError(#[from] ProcessorError),

    #[error("create category database error {0}")]
    DatabaseError(#[from] SurrealError),
}

#[derive(Template)]
#[template(path = "application/veda/veda_template.html")]
pub struct VedaTemplate<'a> {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<MiniArticleData>,
    pub articles_left: &'a [ShortArticleData],
    pub articles_right: &'a [ShortArticleData],
}

pub async fn render_veda(state: &TheState) -> Result<(), VedaError> {
    let articles = state.dba.articles_by_category("veda", 100).await?;
    let articles_most_read = state.dba.articles_most_read(3).await?;

    let split = articles.len() / 3;
    let (articles_left, articles_right) = articles.split_at(split);
    let veda = VedaTemplate {
        date: state.ds.date(),
        weather: state.ds.weather(),
        name_day: state.ds.name_day(),
        articles_most_read,
        articles_left,
        articles_right,
    };
    match veda.render() {
        Ok(rendered_html) => {
            processor::save_web_file(rendered_html, "veda.html")?;
            Ok(())
        }
        Err(_) => Err(CreateCategoryError),
    }
}
