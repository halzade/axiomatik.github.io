use askama::Template;

#[derive(Template)]
#[template(path = "application/news/news_template.html")]
pub struct NewsTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub articles_most_read: Vec<ArticleMostRead>,

    pub z_republiky: IndexCategoryTemplate,
    pub ze_zahranici: IndexCategoryTemplate,
    pub technologie: IndexCategoryTemplate,
    pub veda: IndexCategoryTemplate,
    pub finance: IndexCategoryTemplate,
}

pub struct NewsData {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub articles_most_read: Vec<ArticleMostRead>,

    pub z_republiky: IndexCategoryData,
    pub ze_zahranici: IndexCategoryData,
    pub technologie: IndexCategoryData,
    pub veda: IndexCategoryData,
    pub finance: IndexCategoryData,
}
