use askama::Template;
use crate::web::base::ArticleMostRead;

#[derive(Template)]
#[template(path = "category_zahranici_template.html")]
pub struct ZahraniciTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<ArticleMostRead>,
    pub articles: Vec<CategoryArticleTemplate>,
}
