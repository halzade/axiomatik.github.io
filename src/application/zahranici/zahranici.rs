use askama::Template;

#[derive(Template)]
#[template(path = "application/zahranici/zahranici_template.html")]
pub struct ZahraniciTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<ArticleMostRead>,
    pub articles: Vec<CategoryArticleTemplate>,
}
