use crate::{database, external, library, name_days};
use askama::Template;
use std::fs;

pub struct IndexArticleTopMainData {
    pub url: String,
    pub title: String,
    pub short_text: String,
    pub image_path: String,
}

pub struct IndexArticleTopData {
    pub url: String,
    pub title: String,
    pub short_text: String,
}

pub struct IndexCategoryArticleData {
    pub url: String,
    pub title: String,
    pub short_text: String,
}

pub struct IndexCategoryData {
    pub category_name: String,
    pub articles: Vec<IndexCategoryArticleData>,
}

pub struct IndexData {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub main_article: IndexArticleTopMainData,
    pub second_article: IndexArticleTopData,
    pub third_article: IndexArticleTopData,

    pub z_republiky: IndexCategoryData,
    pub ze_zahranici: IndexCategoryData,
}

#[derive(Template)]
#[template(path = "index_template.html")]
pub struct IndexTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub main_article: IndexArticleTopMainTemplate,
    pub second_article: IndexArticleTopTemplate,
    pub third_article: IndexArticleTopTemplate,

    pub z_republiky: IndexCategoryTemplate,
    pub ze_zahranici: IndexCategoryTemplate,
}

#[derive(Template)]
#[template(path = "index_category.html")]
pub struct IndexCategoryTemplate {
    pub category_name: String,
    pub articles: Vec<IndexCategoryArticleTemplate>,
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
        if d.main_article.url.is_empty() {
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

            d.main_article = IndexArticleTopMainData {
                url: main_article
                    .map(|a| a.article_file_name.clone())
                    .unwrap_or_default(),
                title: main_article.map(|a| a.title.clone()).unwrap_or_default(),
                short_text: main_article
                    .map(|a| a.short_text.clone())
                    .unwrap_or_default(),
                image_path: main_article
                    .map(|a| a.image_url.clone())
                    .unwrap_or_default(),
            };

            d.second_article = IndexArticleTopData {
                url: second_article
                    .map(|a| a.article_file_name.clone())
                    .unwrap_or_default(),
                title: second_article.map(|a| a.title.clone()).unwrap_or_default(),
                short_text: second_article
                    .map(|a| a.short_text.clone())
                    .unwrap_or_default(),
            };

            d.third_article = IndexArticleTopData {
                url: third_article
                    .map(|a| a.article_file_name.clone())
                    .unwrap_or_default(),
                title: third_article.map(|a| a.title.clone()).unwrap_or_default(),
                short_text: third_article
                    .map(|a| a.short_text.clone())
                    .unwrap_or_default(),
            };

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

            let mut z_republiky_data = Vec::new();
            for a in z_republiky_articles {
                z_republiky_data.push(IndexCategoryArticleData {
                    url: a.article_file_name.clone(),
                    title: a.title.clone(),
                    short_text: a.short_text.clone(),
                });
            }

            let mut ze_zahranici_data = Vec::new();
            for a in ze_zahranici_articles {
                ze_zahranici_data.push(IndexCategoryArticleData {
                    url: a.article_file_name.clone(),
                    title: a.title.clone(),
                    short_text: a.short_text.clone(),
                });
            }

            d.z_republiky = IndexCategoryData {
                category_name: "Z naší republiky".to_string(),
                articles: z_republiky_data,
            };

            d.ze_zahranici = IndexCategoryData {
                category_name: "Ze zahraničí".to_string(),
                articles: ze_zahranici_data,
            };
        }
        d
    } else {
        let now = chrono::Local::now();

        let date = library::formatted_article_date(now);
        let name_day = name_days::formatted_today_name_date(now);
        let weather = external::fetch_weather().await;

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

        let mut z_republiky_data = Vec::new();
        for a in z_republiky_articles {
            z_republiky_data.push(IndexCategoryArticleData {
                url: a.article_file_name.clone(),
                title: a.title.clone(),
                short_text: a.short_text.clone(),
            });
        }

        let mut ze_zahranici_data = Vec::new();
        for a in ze_zahranici_articles {
            ze_zahranici_data.push(IndexCategoryArticleData {
                url: a.article_file_name.clone(),
                title: a.title.clone(),
                short_text: a.short_text.clone(),
            });
        }

        IndexData {
            date,
            weather,
            name_day,
            main_article: IndexArticleTopMainData {
                url: main_article
                    .map(|a| a.article_file_name.clone())
                    .unwrap_or_default(),
                title: main_article.map(|a| a.title.clone()).unwrap_or_default(),
                short_text: main_article
                    .map(|a| a.short_text.clone())
                    .unwrap_or_default(),
                image_path: main_article
                    .map(|a| a.image_url.clone())
                    .unwrap_or_default(),
            },
            second_article: IndexArticleTopData {
                url: second_article
                    .map(|a| a.article_file_name.clone())
                    .unwrap_or_default(),
                title: second_article.map(|a| a.title.clone()).unwrap_or_default(),
                short_text: second_article
                    .map(|a| a.short_text.clone())
                    .unwrap_or_default(),
            },
            third_article: IndexArticleTopData {
                url: third_article
                    .map(|a| a.article_file_name.clone())
                    .unwrap_or_default(),
                title: third_article.map(|a| a.title.clone()).unwrap_or_default(),
                short_text: third_article
                    .map(|a| a.short_text.clone())
                    .unwrap_or_default(),
            },
            z_republiky: IndexCategoryData {
                category_name: "Z naší republiky".to_string(),
                articles: z_republiky_data,
            },
            ze_zahranici: IndexCategoryData {
                category_name: "Ze zahraničí".to_string(),
                articles: ze_zahranici_data,
            },
        }
    };

    let index_template = IndexTemplate {
        date: index_data.date,
        weather: index_data.weather,
        name_day: index_data.name_day,

        main_article: IndexArticleTopMainTemplate {
            url: index_data.main_article.url,
            title: index_data.main_article.title,
            short_text: index_data.main_article.short_text,
            image_path: index_data.main_article.image_path,
        },

        second_article: IndexArticleTopTemplate {
            url: index_data.second_article.url,
            title: index_data.second_article.title,
            short_text: index_data.second_article.short_text,
        },

        third_article: IndexArticleTopTemplate {
            url: index_data.third_article.url,
            title: index_data.third_article.title,
            short_text: index_data.third_article.short_text,
        },

        z_republiky: IndexCategoryTemplate {
            category_name: index_data.z_republiky.category_name,
            articles: index_data
                .z_republiky
                .articles
                .into_iter()
                .map(|a| IndexCategoryArticleTemplate {
                    url: a.url,
                    title: a.title,
                    short_text: a.short_text,
                })
                .collect(),
        },
        ze_zahranici: IndexCategoryTemplate {
            category_name: index_data.ze_zahranici.category_name,
            articles: index_data
                .ze_zahranici
                .articles
                .into_iter()
                .map(|a| IndexCategoryArticleTemplate {
                    url: a.url,
                    title: a.title,
                    short_text: a.short_text,
                })
                .collect(),
        },
    };

    let html_content = index_template.render().unwrap();
    fs::write("index.html", html_content).unwrap();
}
