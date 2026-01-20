use crate::{database, external, library, name_days};
use askama::Template;
use std::fs;

pub struct IndexData {
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

#[derive(Template)]
#[template(path = "index_template.html")]
pub struct IndexTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    // TODO use template instead
    pub main_article_url: String,
    pub main_article_title: String,
    pub main_article_short_text: String,
    pub main_article_image: String,

    // TODO use template instead
    pub second_article_url: String,
    pub second_article_title: String,
    pub second_article_short_text: String,

    // TODO use template instead
    pub third_article_url: String,
    pub third_article_title: String,
    pub third_article_short_text: String,

    // TODO use template instead
    pub z_republiky: String,

    // TODO use template instead
    pub ze_zahranici: String,
}

#[derive(Template)]
#[template(path = "index_category.html")]
pub struct IndexCategoryTemplate {
    pub category_name: String,
    pub articles: String,
}

#[derive(Template)]
#[template(path = "index_category_article.html")]
pub struct IndexCategoryArticleTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
}

#[derive(Template)]
#[template(path = "index_article_top_main.html")]
pub struct IndexArticleTopMainTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
    pub image_path: String,
}

#[derive(Template)]
#[template(path = "index_article_top.html")]
pub struct IndexArticleTopTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
}

pub async fn render_new_index(data: Option<IndexData>) {
    let index_data = if let Some(mut d) = data {
        if d.main_article_url.is_empty() {
            let articles = database::get_all_articles().await.unwrap_or_default();
            let mut main_articles: Vec<_> = articles.iter().filter(|a| a.is_main).collect();
            // Sort by date descending
            main_articles.sort_by(|a, b| {
                let da = library::save_article_file_name(&a.title);
                let db = library::save_article_file_name(&b.title);
                b.date.cmp(&a.date).then(db.cmp(&da))
            });

            let main_article = main_articles.get(0);
            let second_article = main_articles.get(1);
            let third_article = main_articles.get(2);

            d.main_article_url = main_article
                .map(|a| a.article_file_name.clone())
                .unwrap_or_default();
            d.main_article_title = main_article.map(|a| a.title.clone()).unwrap_or_default();
            d.main_article_short_text = main_article
                .map(|a| a.short_text.clone())
                .unwrap_or_default();
            d.main_article_image = main_article
                .map(|a| a.image_url.clone())
                .unwrap_or_default();

            d.second_article_url = second_article
                .map(|a| a.article_file_name.clone())
                .unwrap_or_default();
            d.second_article_title = second_article.map(|a| a.title.clone()).unwrap_or_default();
            d.second_article_short_text = second_article
                .map(|a| a.short_text.clone())
                .unwrap_or_default();

            d.third_article_url = third_article
                .map(|a| a.article_file_name.clone())
                .unwrap_or_default();
            d.third_article_title = third_article.map(|a| a.title.clone()).unwrap_or_default();
            d.third_article_short_text = third_article
                .map(|a| a.short_text.clone())
                .unwrap_or_default();

            let mut z_republiky_articles: Vec<_> = articles
                .iter()
                .filter(|a| a.category == "republika")
                .collect();
            z_republiky_articles.sort_by(|a, b| {
                let da = library::save_article_file_name(&a.title);
                let db = library::save_article_file_name(&b.title);
                b.date.cmp(&a.date).then(db.cmp(&da))
            });
            let z_republiky_articles = z_republiky_articles.into_iter().take(6);

            let mut ze_zahranici_articles: Vec<_> = articles
                .iter()
                .filter(|a| a.category == "zahranici")
                .collect();
            ze_zahranici_articles.sort_by(|a, b| {
                let da = library::save_article_file_name(&a.title);
                let db = library::save_article_file_name(&b.title);
                b.date.cmp(&a.date).then(db.cmp(&da))
            });
            let ze_zahranici_articles = ze_zahranici_articles.into_iter().take(6);

            let mut z_republiky_html = String::new();
            for a in z_republiky_articles {
                z_republiky_html.push_str(
                    &IndexCategoryArticleTemplate {
                        url: a.article_file_name.clone(),
                        title: a.title.clone(),
                        short_text: a.short_text.clone(),
                    }
                    .render()
                    .unwrap(),
                );
            }

            let mut ze_zahranici_html = String::new();
            for a in ze_zahranici_articles {
                ze_zahranici_html.push_str(
                    &IndexCategoryArticleTemplate {
                        url: a.article_file_name.clone(),
                        title: a.title.clone(),
                        short_text: a.short_text.clone(),
                    }
                    .render()
                    .unwrap(),
                );
            }

            d.z_republiky = IndexCategoryTemplate {
                category_name: "Z naší republiky".to_string(),
                articles: z_republiky_html,
            }
            .render()
            .unwrap();

            d.ze_zahranici = IndexCategoryTemplate {
                category_name: "Ze zahraničí".to_string(),
                articles: ze_zahranici_html,
            }
            .render()
            .unwrap();
        }
        d
    } else {
        let now = chrono::Local::now();

        let date = library::formatted_article_date(now);
        let name_day = name_days::formatted_today_name_date(now);
        let weather = external::fetch_weather().await;

        // TODO get 3 articles
        let articles = database::get_all_articles().await.unwrap_or_default();
        let mut main_articles: Vec<_> = articles.iter().filter(|a| a.is_main).collect();
        // Sort by date descending
        main_articles.sort_by(|a, b| {
            let da = library::save_article_file_name(&a.title);
            let db = library::save_article_file_name(&b.title);
            b.date.cmp(&a.date).then(db.cmp(&da))
        });

        let main_article = main_articles.get(0);
        let second_article = main_articles.get(1);
        let third_article = main_articles.get(2);

        let mut z_republiky_articles: Vec<_> = articles
            .iter()
            .filter(|a| a.category == "republika")
            .collect();
        z_republiky_articles.sort_by(|a, b| {
            let da = library::save_article_file_name(&a.title);
            let db = library::save_article_file_name(&b.title);
            b.date.cmp(&a.date).then(db.cmp(&da))
        });
        let z_republiky_articles = z_republiky_articles.into_iter().take(6);

        let mut ze_zahranici_articles: Vec<_> = articles
            .iter()
            .filter(|a| a.category == "zahranici")
            .collect();
        ze_zahranici_articles.sort_by(|a, b| {
            let da = library::save_article_file_name(&a.title);
            let db = library::save_article_file_name(&b.title);
            b.date.cmp(&a.date).then(db.cmp(&da))
        });
        let ze_zahranici_articles = ze_zahranici_articles.into_iter().take(6);

        let mut z_republiky_html = String::new();
        for a in z_republiky_articles {
            z_republiky_html.push_str(
                &IndexCategoryArticleTemplate {
                    url: a.article_file_name.clone(),
                    title: a.title.clone(),
                    short_text: a.short_text.clone(),
                }
                .render()
                .unwrap(),
            );
        }

        let mut ze_zahranici_html = String::new();
        for a in ze_zahranici_articles {
            ze_zahranici_html.push_str(
                &IndexCategoryArticleTemplate {
                    url: a.article_file_name.clone(),
                    title: a.title.clone(),
                    short_text: a.short_text.clone(),
                }
                .render()
                .unwrap(),
            );
        }

        IndexData {
            date,
            weather,
            name_day,
            main_article_url: main_article
                .map(|a| a.article_file_name.clone())
                .unwrap_or_default(),
            main_article_title: main_article.map(|a| a.title.clone()).unwrap_or_default(),
            main_article_short_text: main_article
                .map(|a| a.short_text.clone())
                .unwrap_or_default(),
            main_article_image: main_article
                .map(|a| a.image_url.clone())
                .unwrap_or_default(),
            second_article_url: second_article
                .map(|a| a.article_file_name.clone())
                .unwrap_or_default(),
            second_article_title: second_article.map(|a| a.title.clone()).unwrap_or_default(),
            second_article_short_text: second_article
                .map(|a| a.short_text.clone())
                .unwrap_or_default(),
            third_article_url: third_article
                .map(|a| a.article_file_name.clone())
                .unwrap_or_default(),
            third_article_title: third_article.map(|a| a.title.clone()).unwrap_or_default(),
            third_article_short_text: third_article
                .map(|a| a.short_text.clone())
                .unwrap_or_default(),
            z_republiky: IndexCategoryTemplate {
                category_name: "Z naší republiky".to_string(),
                articles: z_republiky_html,
            }
            .render()
            .unwrap(),
            ze_zahranici: IndexCategoryTemplate {
                category_name: "Ze zahraničí".to_string(),
                articles: ze_zahranici_html,
            }
            .render()
            .unwrap(),
        }
    };

    let index_template = IndexTemplate {
        date: index_data.date,
        weather: index_data.weather,
        name_day: index_data.name_day,

        main_article_url: IndexArticleTopMainTemplate {
            url: index_data.main_article_url,
            title: index_data.main_article_title,
            short_text: index_data.main_article_short_text,
            image_path: index_data.main_article_image,
        }
        .render()
        .unwrap(),
        main_article_title: "".to_string(),
        main_article_short_text: "".to_string(),
        main_article_image: "".to_string(),

        second_article_url: IndexArticleTopTemplate {
            url: index_data.second_article_url,
            title: index_data.second_article_title,
            short_text: index_data.second_article_short_text,
        }
        .render()
        .unwrap(),
        second_article_title: "".to_string(),
        second_article_short_text: "".to_string(),

        third_article_url: IndexArticleTopTemplate {
            url: index_data.third_article_url,
            title: index_data.third_article_title,
            short_text: index_data.third_article_short_text,
        }
        .render()
        .unwrap(),
        third_article_title: "".to_string(),
        third_article_short_text: "".to_string(),

        z_republiky: index_data.z_republiky,
        ze_zahranici: index_data.ze_zahranici,
    };

    let html_content = index_template.render().unwrap();
    fs::write("index.html", html_content).unwrap();
}
