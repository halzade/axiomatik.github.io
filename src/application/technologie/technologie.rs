use askama::Template;
use crate::web::base::ArticleMostRead;

#[derive(Template)]
#[template(path = "category_technologie_template.html")]
pub struct TechnologieTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<ArticleMostRead>,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}
