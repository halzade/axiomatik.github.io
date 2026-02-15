use crate::data::processor;
use crate::data::processor::ProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::db::database_system::SurrealSystemError;
use crate::system::server::TheState;
use askama::Template;
use thiserror::Error;
use VedaError::CreateCategoryError;
use crate::db::database_article::SurrealArticleError;

#[derive(Debug, Error)]
pub enum VedaError {
    #[error("create category error")]
    CreateCategoryError,

    #[error("create category processor error {0}")]
    ProcessorError(#[from] ProcessorError),

    #[error("create category database error {0}")]
    DatabaseError(#[from] SurrealError),

    #[error("create category database error {0}")]
    SurrealArticle(#[from] SurrealArticleError),
    
    #[error("create category database system error {0}")]
    SurrealSystem(#[from] SurrealSystemError),
}

#[derive(Template)]
#[template(path = "application/category_veda/veda_template.html")]
pub struct VedaTemplate<'a> {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<MiniArticleData>,
    pub articles_left: &'a [ShortArticleData],
    pub articles_right: &'a [ShortArticleData],
}

const VEDA: &str = "veda";

pub async fn render_veda(state: &TheState) -> Result<(), VedaError> {
    let articles = state.dba.articles_by_category(VEDA, 100).await?;
    let articles_most_read: Vec<MiniArticleData> = state.dba.most_read_in_category_by_views(VEDA).await?;

    let split = articles.len().div_ceil(2);
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
