use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::system::server::TheState;
use askama::Template;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("create article error")]
    RouterArticleError,

    #[error("render error")]
    RenderError,
}

/*
 * Main Article
 */
pub struct MainArticleData {
    pub article_file_name: String,
    pub title: String,
    pub is_exclusive: bool,
    pub short_text: String,
    pub image_path: String,
    pub image_desc: String,
}

/*
 * Second and Third Article
 */
pub struct TopArticleData {
    pub article_file_name: String,
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

    pub articles_most_read: Vec<MiniArticleData>,

    pub main_article: MainArticleData,
    pub second_article: TopArticleData,
    pub third_article: TopArticleData,

    pub z_republiky_articles: Vec<ShortArticleData>,
    pub ze_zahranici_articles: Vec<ShortArticleData>,
}

pub async fn render_index(state: &TheState) -> Result<(), IndexError> {
    // TODO fetch data
    let articles_most_read = vec![];
    let z_republiky_articles = vec![];
    let ze_zahranici_articles = vec![];

    let main_article = MainArticleData {
        article_file_name: "".into(),
        title: "".into(),
        is_exclusive: false,
        short_text: "".into(),
        image_path: "".into(),
        image_desc: "".into(),
    };
    let second_article = TopArticleData {
        article_file_name: "".into(),
        title: "".into(),
        short_text: "".into(),
    };
    let third_article = TopArticleData {
        article_file_name: "".into(),
        title: "".into(),
        short_text: "".into(),
    };

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
