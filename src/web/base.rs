use askama::Template;

#[derive(Template, Clone)]
#[template(path = "article_most_read.html")]
pub struct ArticleMostRead {
    pub image_url_50: String,
    pub title: String,
    pub text: String,
}

pub struct CategoryArticleData {
    pub url: String,
    pub title: String,
    pub short_text: String,
    pub image_path: String,
    pub image_description: String,
}

pub struct CategoryData {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub category_name: String,
    pub category_url: String,
    pub articles: Vec<CategoryArticleData>,
}


pub async fn render_template(category: &str, data: Option<CategoryData>) {
    let mut category_articles: Vec<_> =
        articles.iter().filter(|a| a.category == category).collect();

    let articles_data = category_articles
        .into_iter()
        .map(|a| CategoryArticleData {
            url: a.article_file_name.clone(),
            title: a.title.clone(),
            short_text: a.short_text.clone(),
            image_path: a.image_url.clone(),
            image_description: a.image_description.clone(),
        })
        .collect();

    CategoryData {
        date: system_data::date(),
        weather: system_data::weather(),
        name_day: system_data::name_day(),
        category_name: category.to_string(),
        category_url: format!("{}.html", category),
        articles: articles_data,
    };


    let articles_template: Vec<IndexCategoryArticleTemplate> = category_data
        .articles
        .into_iter()
        .enumerate()
        .map(|(i, a)| IndexCategoryArticleTemplate {
            url: a.url,
            title: a.title,
            short_text: a.short_text,
            is_first: i < 2, // First two articles are "first"
            image_path: a.image_path,
            image_description: a.image_description,
            category_name: category_data.category_name.clone(),
            category_url: category_data.category_url.clone(),
        })
        .collect();

    // TODO: fetch actual most read articles
    let mut most_read_data = Vec::new();
    for i in 1..=5 {
        most_read_data.push(ArticleMostRead {
            image_url_50: "images/placeholder_50.png".to_string(),
            title: format!("Dummy Article {}", i),
            text: "This is a dummy most read article.".to_string(),
        });
    }

    let filename = format!("{}.html", category);
    std::fs::write(filename, rendered).unwrap();
}
