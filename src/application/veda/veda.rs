use askama::Template;

#[derive(Template)]
#[template(path = "application/veda/veda_template.html")]
pub struct VedaTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<ArticleMostRead>,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}
