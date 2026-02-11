use crate::db::database_article::SurrealArticleError;
use crate::db::database_article_data::{
    MainArticleData, MiniArticleData, ShortArticleData, TopArticleData,
};
use crate::system::server::TheState;
use askama::Template;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("create article error")]
    RouterArticleError,

    #[error("render error")]
    RenderError,

    #[error("surreal article error")]
    SurrealArticle(#[from] SurrealArticleError),
}

/*
 * Index
 */
#[derive(Template)]
#[template(path = "application/index/index_template.html")]
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
    
    let articles_most_read = state.dbs.most_read_by_views(4).await?;
    let articles_most_read = state.dba.articles_most_read(articles_most_read).await?;

    let z_republiky_articles = state.dba.articles_by_category("republika", 10).await?;
    let ze_zahranici_articles = state.dba.articles_by_category("zahranici", 10).await?;

    let (main_article, second_article, third_article) = state.dba.article_top_three().await?;

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
            crate::data::processor::save_web_file(rendered_html, "index.html")
                .map_err(|_| IndexError::RenderError)?;
            Ok(())
        }
        Err(_) => Err(IndexError::RenderError),
    }
}
