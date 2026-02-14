use crate::data::processor;
use crate::db::database_article::SurrealArticleError;
use crate::db::database_article_data::{
    MainArticleData, MiniArticleData, ShortArticleData, TopArticleData,
};
use crate::db::database_system::SurrealSystemError;
use crate::system::server::TheState;
use askama::Template;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("create article error")]
    RouterArticleError,

    #[error("render error")]
    RenderError,

    #[error("surreal article error {0}")]
    SurrealArticle(#[from] SurrealArticleError),

    #[error("surreal system error {0}")]
    SurrealSystem(#[from] SurrealSystemError),
}

/*
 * Index
 */
#[derive(Template)]
#[template(path = "application/page_index/index_template.html")]
pub struct IndexTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub articles_most_read: Vec<MiniArticleData>,

    pub main_article: MainArticleData,
    pub second_article: TopArticleData,
    pub third_article: TopArticleData,

    pub z_republiky_articles: Vec<ShortArticleData>,
    pub ze_zahranici_articles: Vec<ShortArticleData>,
}

pub async fn render_index(state: &TheState) -> Result<(), IndexError> {
    let articles_most_read = state.dba.most_read_by_views().await?;

    let z_republiky_articles = state.dba.articles_by_category("republika", 10).await?;
    let ze_zahranici_articles = state.dba.articles_by_category("zahranici", 10).await?;

    let (mut main_article, second_article, third_article) = state.dba.article_top_three().await?;

    main_article.category_display = processor::process_category(&main_article.category)
        .unwrap_or_else(|_| "".to_string());

    let template = IndexTemplate {
        date: state.ds.date(),
        weather: state.ds.weather(),
        name_day: state.ds.name_day(),
        articles_most_read,
        main_article,
        second_article,
        third_article,
        z_republiky_articles,
        ze_zahranici_articles,
    };

    match template.render() {
        Ok(rendered_html) => {
            processor::save_web_file(rendered_html, "index.html")
                .map_err(|_| IndexError::RenderError)?;
            Ok(())
        }
        Err(_) => Err(IndexError::RenderError),
    }
}
