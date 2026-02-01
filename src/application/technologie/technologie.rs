use askama::Template;

#[derive(Template)]
#[template(path = "application/technologie/technologie_template.html")]
pub struct TechnologieTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<ArticleMostRead>,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}
