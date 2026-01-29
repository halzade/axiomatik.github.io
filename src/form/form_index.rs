use askama::Template;
use std::fs;
use crate::db::database_article;
use crate::library;

pub struct IndexArticleTopMainData {
    pub url: String,
    pub title: String,
    pub is_exclusive: bool,
    pub short_text: String,
    pub image_path: String,
    pub image_description: String,
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
    pub image_path: String,
    pub image_description: String,
    pub category_name: String,
    pub category_url: String,
}

pub struct IndexCategoryData {
    pub category_name: String,
    pub category_url: String,
    pub articles: Vec<IndexCategoryArticleData>,
}

pub struct IndexData {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub main_article: IndexArticleTopMainData,
    pub second_article: IndexArticleTopData,
    pub third_article: IndexArticleTopData,

    pub articles_most_read: Vec<IndexArticleMostRead>,

    pub z_republiky: IndexCategoryData,
    pub ze_zahranici: IndexCategoryData,
}

pub struct NewsData {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub articles_most_read: Vec<IndexArticleMostRead>,

    pub z_republiky: IndexCategoryData,
    pub ze_zahranici: IndexCategoryData,
    pub technologie: IndexCategoryData,
    pub veda: IndexCategoryData,
    pub finance: IndexCategoryData,
}

#[derive(Template)]
#[template(path = "news_template.html")]
pub struct NewsTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub articles_most_read: Vec<IndexArticleMostRead>,

    pub z_republiky: IndexCategoryTemplate,
    pub ze_zahranici: IndexCategoryTemplate,
    pub technologie: IndexCategoryTemplate,
    pub veda: IndexCategoryTemplate,
    pub finance: IndexCategoryTemplate,
}

#[derive(Template)]
#[template(path = "index_template.html")]
pub struct IndexTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub articles_most_read: Vec<IndexArticleMostRead>,
    pub main_article: IndexArticleTopMainTemplate,
    pub second_article: IndexArticleTopTemplate,
    pub third_article: IndexArticleTopTemplate,

    pub z_republiky: IndexCategoryTemplate,
    pub ze_zahranici: IndexCategoryTemplate,
}

#[derive(Template)]
#[template(path = "index_category_template.html")]
pub struct IndexCategoryTemplate {
    pub category_name: String,
    pub category_url: String,
    pub articles: Vec<IndexCategoryArticleTemplate>,
}

#[derive(Template)]
#[template(path = "index_category_article_template.html")]
pub struct IndexCategoryArticleTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
    pub is_first: bool,
    pub image_path: String,
    pub image_description: String,
    pub category_name: String,
    pub category_url: String,
}

#[derive(Template, Clone)]
#[template(path = "index_article_most_read.html")]
pub struct IndexArticleMostRead {
    pub image_url_50: String,
    pub title: String,
    pub text: String,
}

#[derive(Template)]
#[template(path = "index_article_top_main_template.html")]
pub struct IndexArticleTopMainTemplate {
    pub url: String,
    pub title: String,
    pub category_url: String,
    pub category_name: String,
    pub is_exclusive: bool,
    pub short_text: String,
    pub image_path: String,
    pub image_description: String,
}

#[derive(Template)]
#[template(path = "index_article_top_template.html")]
pub struct IndexArticleTopTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
}

pub async fn render_new_index(data: Option<IndexData>) {
    let index_data = if let Some(mut d) = data {
        if d.main_article.url.is_empty() {
            let articles = database_article::get_all_articles().await.unwrap_or_default();
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
                is_exclusive: main_article.map(|a| a.is_exclusive).unwrap_or_default(),
                short_text: main_article
                    .map(|a| a.short_text.clone())
                    .unwrap_or_default(),
                image_path: main_article
                    .map(|a| a.image_url.clone())
                    .unwrap_or_default(),
                image_description: main_article
                    .map(|a| a.image_description.clone())
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
                    image_path: a.image_url.clone(),
                    image_description: a.image_description.clone(),
                    category_name: "Republika".to_string(),
                    category_url: "republika.html".to_string(),
                });
            }

            let mut ze_zahranici_data = Vec::new();
            for a in ze_zahranici_articles {
                ze_zahranici_data.push(IndexCategoryArticleData {
                    url: a.article_file_name.clone(),
                    title: a.title.clone(),
                    short_text: a.short_text.clone(),
                    image_path: a.image_url.clone(),
                    image_description: a.image_description.clone(),
                    category_name: "Zahraničí".to_string(),
                    category_url: "zahranici.html".to_string(),
                });
            }

            d.z_republiky = IndexCategoryData {
                category_name: "Z naší republiky".to_string(),
                category_url: "republika.html".to_string(),
                articles: z_republiky_data,
            };

            d.ze_zahranici = IndexCategoryData {
                category_name: "Ze zahraničí".to_string(),
                category_url: "zahranici.html".to_string(),
                articles: ze_zahranici_data,
            };
        }
        d
    } else {
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
                image_path: a.image_url.clone(),
                image_description: a.image_description.clone(),
                category_name: "Republika".to_string(),
                category_url: "republika.html".to_string(),
            });
        }

        let mut ze_zahranici_data = Vec::new();
        for a in ze_zahranici_articles {
            ze_zahranici_data.push(IndexCategoryArticleData {
                url: a.article_file_name.clone(),
                title: a.title.clone(),
                short_text: a.short_text.clone(),
                image_path: a.image_url.clone(),
                image_description: a.image_description.clone(),
                category_name: "Zahraničí".to_string(),
                category_url: "zahranici.html".to_string(),
            });
        }

        // TODO
        let mut most_read_data = Vec::new();
        for i in 1..=5 {
            most_read_data.push(IndexArticleMostRead {
                image_url_50: "images/placeholder_50.png".to_string(),
                title: format!("Dummy Article {}", i),
                text: "This is a dummy most read article.".to_string(),
            });
        }

        IndexData {
            date: system_data::date(),
            weather: system_data::weather(),
            name_day: system_data::name_day(),
            main_article: IndexArticleTopMainData {
                url: main_article
                    .map(|a| a.article_file_name.clone())
                    .unwrap_or_default(),
                title: main_article.map(|a| a.title.clone()).unwrap_or_default(),
                is_exclusive: main_article.map(|a| a.is_exclusive).unwrap_or_default(),
                short_text: main_article
                    .map(|a| a.short_text.clone())
                    .unwrap_or_default(),
                image_path: main_article
                    .map(|a| a.image_url.clone())
                    .unwrap_or_default(),
                image_description: main_article
                    .map(|a| a.image_description.clone())
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
            articles_most_read: most_read_data,
            z_republiky: IndexCategoryData {
                category_name: "Z naší republiky".to_string(),
                category_url: "republika.html".to_string(),
                articles: z_republiky_data,
            },
            ze_zahranici: IndexCategoryData {
                category_name: "Ze zahraničí".to_string(),
                category_url: "zahranici.html".to_string(),
                articles: ze_zahranici_data,
            },
        }
    };

    let index_template = IndexTemplate {
        date: index_data.date,
        weather: index_data.weather,
        name_day: index_data.name_day,
        articles_most_read: index_data.articles_most_read,

        main_article: IndexArticleTopMainTemplate {
            url: index_data.main_article.url,
            title: index_data.main_article.title,
            category_url: "republika.html".to_string(),
            category_name: "Republika".to_string(),
            is_exclusive: index_data.main_article.is_exclusive,
            short_text: index_data.main_article.short_text,
            image_path: index_data.main_article.image_path,
            image_description: index_data.main_article.image_description,
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
            category_url: index_data.z_republiky.category_url,
            articles: index_data
                .z_republiky
                .articles
                .into_iter()
                .enumerate()
                .map(|(i, a)| IndexCategoryArticleTemplate {
                    url: a.url,
                    title: a.title,
                    short_text: a.short_text,
                    is_first: i == 0,
                    image_path: a.image_path,
                    image_description: a.image_description,
                    category_name: a.category_name,
                    category_url: a.category_url,
                })
                .collect(),
        },
        ze_zahranici: IndexCategoryTemplate {
            category_name: index_data.ze_zahranici.category_name,
            category_url: index_data.ze_zahranici.category_url,
            articles: index_data
                .ze_zahranici
                .articles
                .into_iter()
                .enumerate()
                .map(|(i, a)| IndexCategoryArticleTemplate {
                    url: a.url,
                    title: a.title,
                    short_text: a.short_text,
                    is_first: i == 0,
                    image_path: a.image_path,
                    image_description: a.image_description,
                    category_name: a.category_name,
                    category_url: a.category_url,
                })
                .collect(),
        },
    };

    let html_content = index_template.render().unwrap();
    fs::write("test-index.html", html_content).unwrap();
}
