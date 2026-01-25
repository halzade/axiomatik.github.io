use crate::server::AUTH_COOKIE;
use crate::{data, database, form_index, library};
use askama::Template;
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use chrono::{Datelike, Local};
use http::StatusCode;
use std::fs;

pub struct ArticleData {
    pub is_main: bool,
    pub is_exclusive: bool,
    pub author: String,
    pub title: String,
    pub text_processed: String,
    pub short_text_processed: String,
    pub image_path: String,
    pub image_description: String,
    pub video_path: Option<String>,
    pub audio_path: Option<String>,
    pub category: String,
    pub category_display: String,
    pub related_articles: String,
}

#[derive(Template)]
#[template(path = "article_template.html")]
pub struct ArticleTemplate {
    pub title: String,
    pub author: String,
    pub date: String,
    pub text: String,
    pub image_path: String,
    pub image_description: String,
    pub video_path: Option<String>,
    pub audio_path: Option<String>,
    pub category: String,
    pub category_display: String,
    pub related_articles: Vec<form_index::IndexCategoryArticleTemplate>,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<form_index::IndexArticleMostRead>,
}

#[derive(Template)]
#[template(path = "../pages/form.html")]
pub struct FormTemplate {
    pub author_name: String,
}

#[derive(Template)]
#[template(path = "category_template.html")]
pub struct CategoryTemplate {
    pub title: String,
    pub date: String,
    pub weather: String,
    pub name_day: String,
    pub articles: String,
}

#[derive(Template)]
#[template(path = "index_category_article_template.html")]
pub struct CategoryArticleTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
    pub is_first: bool,
    pub image_path: String,
    pub image_description: String,
    pub category_name: String,
    pub category_url: String,
}

pub async fn show_form(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let user_o = database::get_user(cookie.value()).await;
        match user_o {
            None => {}
            Some(user) => {
                return Html(
                    FormTemplate {
                        author_name: user.author_name,
                    }
                    .render()
                    .unwrap(),
                )
                .into_response();
            }
        }
    }
    Redirect::to("/login").into_response()
}

pub async fn create_article(jar: CookieJar, multipart: Multipart) -> Response {
    let created_by = if let Some(cookie) = jar.get(AUTH_COOKIE) {
        cookie.value().to_string()
    } else {
        return Redirect::to("/login").into_response();
    };

    // TODO article already exists
    // TODO double click on create button

    /*
     * Read request data
     */
    let article_data_o = crate::form_new_article_data::article_data(multipart).await;

    match article_data_o {
        None => StatusCode::BAD_REQUEST.into_response(),
        Some(article_data) => {
            let now = Local::now();
            let formatted_date = data::date();
            let formatted_weather = data::weather();
            let formatted_name_day = data::name_day();

            let related_articles_vec: Vec<String> = article_data
                .related_articles
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();

            let mut most_read_data = Vec::new();
            for i in 1..=5 {
                most_read_data.push(form_index::IndexArticleMostRead {
                    image_url_50: "images/placeholder_50.png".to_string(),
                    title: format!("Dummy Article {}", i),
                    text: "This is a dummy most read article.".to_string(),
                });
            }

            let article_template = ArticleTemplate {
                title: article_data.title.clone(),
                author: article_data.author.clone(),
                text: article_data.text_processed.clone(),
                image_path: article_data.image_path.clone(),
                image_description: article_data.image_description.clone(),
                video_path: article_data.video_path.clone(),
                audio_path: article_data.audio_path.clone(),
                category: article_data.category.clone(),
                category_display: article_data.category_display.clone(),
                date: formatted_date.clone(),
                weather: formatted_weather.clone(),
                name_day: formatted_name_day.clone(),
                related_articles: vec![], // TODO
                articles_most_read: most_read_data,
            };

            let html_content = article_template.render().unwrap();
            let safe_title = library::save_article_file_name(&article_data.title);
            let file_path = format!("{}.html", safe_title);

            /*
             * Write the Article
             */
            fs::write(&file_path, html_content).unwrap();

            // category
            let month_name = library::get_czech_month(now.month());
            let category_month_year_filename = format!(
                "archive-{}-{}-{}.html",
                article_data.category,
                month_name,
                now.year()
            );

            let category_article = CategoryArticleTemplate {
                url: file_path.clone(),
                title: article_data.title.clone(),
                short_text: article_data.short_text_processed.clone(),
                is_first: false,
                image_path: article_data.image_path.clone(),
                image_description: article_data.image_description.clone(),
                category_name: article_data.category_display.clone(),
                category_url: format!("{}.html", article_data.category),
            }
            .render()
            .unwrap();

            // Store in DB
            let article_db = database::Article {
                author: article_data.author.clone(),
                created_by,
                date: formatted_date.clone(),
                title: article_data.title.clone(),
                text: article_data.text_processed.clone(),
                short_text: article_data.short_text_processed.clone(),
                article_file_name: file_path.clone(),
                image_url: article_data.image_path.clone(),
                image_description: article_data.image_description.clone(),
                video_url: article_data.video_path.clone(),
                audio_url: article_data.audio_path.clone(),
                category: article_data.category.clone(),
                related_articles: article_data.related_articles.clone(),
                is_main: article_data.is_main,
                is_exclusive: article_data.is_exclusive,
                views: 0,
            };

            let _ = database::create_article(article_db).await;

            if !std::path::Path::new(&category_month_year_filename).exists() {
                let cat_template = CategoryTemplate {
                    title: format!(
                        "{} - {} {}",
                        article_data.category_display,
                        month_name,
                        now.year()
                    ),
                    date: formatted_date.clone(),
                    weather: formatted_weather.clone(),
                    name_day: formatted_name_day.clone(),
                    articles: "".to_string(),
                };
                let mut base_html = cat_template.render().unwrap();
                base_html = base_html.replace("", &format!("\n{}", category_article));
                fs::write(&category_month_year_filename, base_html).unwrap();
            } else {
                let mut content = fs::read_to_string(&category_month_year_filename).unwrap();
                content = content.replace("", &format!("\n{}", category_article));
                fs::write(&category_month_year_filename, content).unwrap();
            }

            let main_cat_filename = format!("{}.html", article_data.category);
            if std::path::Path::new(&main_cat_filename).exists() {
                let mut content = fs::read_to_string(&main_cat_filename).unwrap();
                if content.contains("") {
                    content = content.replace("", &format!("\n{}", category_article));
                }
                fs::write(&main_cat_filename, content).unwrap();
            }

            for path in &related_articles_vec {
                if let Ok(mut content) = fs::read_to_string(path) {
                    if content.contains("") {
                        content = content.replace("", &format!("\n{}", category_article));
                        fs::write(path, content).unwrap();
                    }
                }
            }

            let index_data = form_index::IndexData {
                date: formatted_date,
                weather: formatted_weather,
                name_day: formatted_name_day,
                main_article: form_index::IndexArticleTopMainData {
                    url: "".to_string(),
                    title: "".to_string(),
                    is_exclusive: false,
                    short_text: "".to_string(),
                    image_path: "".to_string(),
                    image_description: "".to_string(),
                },
                second_article: form_index::IndexArticleTopData {
                    url: "".to_string(),
                    title: "".to_string(),
                    short_text: "".to_string(),
                },
                third_article: form_index::IndexArticleTopData {
                    url: "".to_string(),
                    title: "".to_string(),
                    short_text: "".to_string(),
                },
                articles_most_read: vec![],
                z_republiky: form_index::IndexCategoryData {
                    category_name: "".to_string(),
                    category_url: "".to_string(),
                    articles: vec![],
                },
                ze_zahranici: form_index::IndexCategoryData {
                    category_name: "".to_string(),
                    category_url: "".to_string(),
                    articles: vec![],
                },
            };

            form_index::render_new_index(Some(index_data)).await;

            Redirect::to(&*file_path).into_response()
        }
    }
}
