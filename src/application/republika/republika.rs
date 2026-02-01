use crate::application::article::article::EmbeddedArticleData;
use crate::application::most_read::most_read_articles::ArticlesMostReadTemplate;
use askama::Template;

#[derive(Template)]
#[template(path = "application/republika/republika.html")]
pub struct RepublikaTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: ArticlesMostReadTemplate,
    pub articles: Vec<EmbeddedArticleData>,
}
