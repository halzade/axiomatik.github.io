use askama::Template;
use crate::db::database_article;
use crate::web::base::ArticleMostRead;

#[derive(Template)]
#[template(path = "category_finance_template.html")]
pub struct FinanceTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<ArticleMostRead>,
    pub articles: Vec<CategoryArticleTemplate>,
}

pub fn render() {
    let articles = database_article::articles_by_category().await;

}