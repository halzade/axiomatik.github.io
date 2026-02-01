use askama::Template;

#[derive(Template, Clone)]
#[template(path = "article_most_read.html")]
pub struct ArticlesMostReadTemplate {
    pub image_url_50: String,
    pub title: String,
    pub text: String,
}
