use crate::application::article::article::ShortArticleData;
use crate::application::most_read::most_read_articles::ArticlesMostReadTemplate;
use crate::db::database_article;
use askama::Template;

#[derive(Template)]
#[template(path = "category_finance_template.html")]
pub struct FinanceTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: ArticlesMostReadTemplate,
    pub articles: Vec<ShortArticleData>,
}

pub fn render() {
    let articles = database_article::articles_by_category().await;
}
