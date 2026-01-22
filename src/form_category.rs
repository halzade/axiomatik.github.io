use crate::form_index::IndexCategoryArticleTemplate;
use crate::{database, library};
use askama::Template;

pub struct CategoryArticleData {
    pub url: String,
    pub title: String,
    pub short_text: String,
}

pub struct CategoryData {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles: Vec<CategoryArticleData>,
}

#[derive(Template)]
#[template(path = "category_finance_template.html")]
pub struct FinanceTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "category_republika_template.html")]
pub struct RepublikaTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "category_technologie_template.html")]
pub struct TechnologieTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "category_veda_template.html")]
pub struct VedaTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "category_zahranici_template.html")]
pub struct ZahraniciTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,
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
            })
            .collect();

        CategoryData {
            date: "".to_string(),    // These should ideally be passed or fetched
            weather: "".to_string(), // but for now we follow the pattern
            name_day: "".to_string(),
            articles: articles_data,
        }
    };

    let articles_template: Vec<IndexCategoryArticleTemplate> = category_data
        .articles
        .into_iter()
        .map(|a| IndexCategoryArticleTemplate {
            url: a.url,
            title: a.title,
            short_text: a.short_text,
        })
        .collect();

    let rendered = match category {
        "finance" => FinanceTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        "republika" => RepublikaTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        "technologie" => TechnologieTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        "veda" => VedaTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        "zahranici" => ZahraniciTemplate {
            date: category_data.date,
            weather: category_data.weather,
            name_day: category_data.name_day,
            articles: articles_template,
        }
        .render()
        .unwrap(),
        _ => return,
    };

    let filename = format!("{}.html", category);
    std::fs::write(filename, rendered).unwrap();
}
