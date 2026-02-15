use crate::data::processor;
use crate::data::processor::ProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article::SurrealArticleError;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::db::database_system::SurrealSystemError;
use crate::system::server::TheState;
use askama::Template;
use thiserror::Error;
use TechnologieError::CreateCategoryError;

#[derive(Debug, Error)]
pub enum TechnologieError {
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

const TECHNOLOGIE: &str = "technologie";

#[derive(Template)]
#[template(path = "application/category_technologie/technologie_template.html")]
pub struct TechnologieTemplate<'a> {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<MiniArticleData>,
    pub articles_left: &'a [ShortArticleData],
    pub articles_right: &'a [ShortArticleData],
}

pub async fn render_technologie(state: &TheState) -> Result<(), TechnologieError> {
    let articles = state.dba.articles_by_category(TECHNOLOGIE, 100).await?;
    let articles_most_read: Vec<MiniArticleData> =
        state.dba.most_read_in_category_by_views(TECHNOLOGIE).await?;

    let split = articles.len().div_ceil(2);
    let (articles_left, articles_right) = articles.split_at(split);
    let technologie = TechnologieTemplate {
        date: state.ds.date(),
        weather: state.ds.weather(),
        name_day: state.ds.name_day(),
        articles_most_read,
        articles_left,
        articles_right,
    };
    match technologie.render() {
        Ok(rendered_html) => {
            processor::save_web_file(rendered_html, "technologie.html")?;
            Ok(())
        }
        Err(_) => Err(CreateCategoryError),
    }
}
