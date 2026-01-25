use crate::form_index::{IndexArticleMostRead, IndexCategoryArticleTemplate};
use crate::{data, database, library};
use askama::Template;

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

#[derive(Template)]
#[template(path = "category_finance_template.html")]
pub struct FinanceTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<IndexArticleMostRead>,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "category_republika_template.html")]
pub struct RepublikaTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<IndexArticleMostRead>,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "category_technologie_template.html")]
pub struct TechnologieTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<IndexArticleMostRead>,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "category_veda_template.html")]
pub struct VedaTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<IndexArticleMostRead>,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "category_zahranici_template.html")]
pub struct ZahraniciTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<IndexArticleMostRead>,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

pub async fn render_template(category: &str, data: Option<CategoryData>) {
    let category_data = if let Some(d) = data {
        d
    } else {
        let articles = database::get_all_articles().await.unwrap_or_default();
        let mut category_articles: Vec<_> =
            articles.iter().filter(|a| a.category == category).collect();

        // Sort by date descending
        category_articles.sort_by(|a, b| {
            let da = library::save_article_file_name(&a.title);
            let db = library::save_article_file_name(&b.title);
            b.date.cmp(&a.date).then(db.cmp(&da))
        });

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
            date: data::date(),
            weather: data::weather(),
            name_day: data::name_day(),
            category_name: category.to_string(),
            category_url: format!("{}.html", category),
            articles: articles_data,
        }
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
        most_read_data.push(IndexArticleMostRead {
            image_url_50: "images/placeholder_50.png".to_string(),
            title: format!("Dummy Article {}", i),
            text: "This is a dummy most read article.".to_string(),
        });
    }

    let rendered = match category {
        "finance" => FinanceTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles_most_read: most_read_data,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        "republika" => RepublikaTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles_most_read: most_read_data,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        "technologie" => TechnologieTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles_most_read: most_read_data,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        "veda" => VedaTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles_most_read: most_read_data,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        "zahranici" => ZahraniciTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles_most_read: most_read_data,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        _ => return,
    };

    let filename = format!("{}.html", category);
    std::fs::write(filename, rendered).unwrap();
}
