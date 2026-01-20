use askama::Template;

pub struct IndexData {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    // TODO
}

#[derive(Template)]
#[template(path = "index_template.html")]
pub struct IndexTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub main_article_url: String,
    pub main_article_title: String,
    pub main_article_short_text: String,
    pub main_article_image: String,

    pub second_article_url: String,
    pub second_article_title: String,
    pub second_article_short_text: String,

    pub third_article_url: String,
    pub third_article_title: String,
    pub third_article_short_text: String,

    pub z_republiky: String,
    pub ze_zahranici: String,
}

pub async fn render_new_index() {
    // TODO
}
