use crate::application::most_read::most_read_articles::ArticlesMostReadTemplate;
use askama::Template;
use thiserror::Error;
use crate::db::database_article::ShortArticleData;

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("create article error")]
    RouterArticleError,
}

/*
 * Main Article
 */
pub struct MainArticleData {
    pub url: String,
    pub title: String,
    pub is_exclusive: bool,
    pub short_text: String,
    pub image_path: String,
    pub image_description: String,
}

/*
 * Second and Third Article
 */
pub struct TopArticleData {
    pub url: String,
    pub title: String,
    pub short_text: String,
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

    pub articles_most_read: ArticlesMostReadTemplate,

    pub main_article: MainArticleData,
    pub second_article: TopArticleData,
    pub third_article: TopArticleData,

    pub z_republiky_articles: Vec<ShortArticleData>,
    pub ze_zahranici_articles: Vec<ShortArticleData>,
}

pub async fn render_index() -> Result<(), IndexError> {
    
    
    
}

pub async fn compose_index_from_data() {}
