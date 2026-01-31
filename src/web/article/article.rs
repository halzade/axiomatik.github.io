use crate::processor::{process_audio, process_images, process_video};
use crate::system::server::AUTH_COOKIE;
use askama::Template;
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use chrono::{Datelike, Local};
use http::StatusCode;
use std::fs;
use crate::db::{database_article, database_user};
use crate::library;
use crate::system::system_data;

pub struct ArticleData {
    pub is_main: bool,
    pub is_exclusive: bool,
    pub author: String,
    pub title: String,
    pub text_processed: String,
    pub short_text_processed: String,

    pub image_description: String,
    pub image_data: Vec<u8>,
    pub video_data: Vec<u8>,
    pub audio_data: Vec<u8>,

    pub category: String,
    pub category_display: String,
    pub related_articles: Vec<String>,

    pub article_file_name: String,
    pub image_path: String,
    pub video_path: Option<String>,
    pub audio_path: Option<String>,
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
    pub related_articles: Vec<IndexCategoryArticleTemplate>,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<ArticleMostRead>,
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

// to Try from for Multipart
// need multipart because of the files
pub async fn create_article(jar: CookieJar, multipart: Multipart) -> Response {

    match article_data_r {
        Ok(article_data) => {
            let now = Local::now();
            let formatted_date = system_data::date();
            let formatted_weather = system_data::weather();
            let formatted_name_day = system_data::name_day();

            let related_articles_vec = article_data.related_articles.clone();

            let mut most_read_data = Vec::new();
            for i in 1..=5 {
                most_read_data.push(form_index::ArticleMostRead {
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

                // TODO
                related_articles: vec![],
                articles_most_read: most_read_data,
            };

            let html_content = article_template.render().unwrap();

            /*
             * Write the Article
             */
            fs::write(article_data.article_file_name.clone(), html_content).unwrap();

            // Process media
            if let Ok(img) = image::load_from_memory(&article_data.image_data) {
                let ext = article_data.image_path.split('.').last().unwrap_or("png");
                let _ = process_images(&img, &article_data.image_path, ext);
            }

            if let Some(video_path) = &article_data.video_path {
                let file_name = video_path.split('/').last().unwrap_or(video_path);
                let _ = process_video(&article_data.video_data, file_name);
            }

            if let Some(audio_path) = &article_data.audio_path {
                let file_name = audio_path.split('/').last().unwrap_or(audio_path);
                let _ = process_audio(&article_data.audio_data, file_name);
            }

            // category
            let month_name = library::get_czech_month(now.month());
            let category_month_year_filename = format!(
                "archive-{}-{}-{}.html",
                article_data.category,
                month_name,
                now.year()
            );

            let category_article = CategoryArticleTemplate {
                url: article_data.article_file_name.clone(),
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
            let article_db = database_article::Article {
                author: article_data.author.clone(),
                created_by,
                date: formatted_date.clone(),
                title: article_data.title.clone(),
                text: article_data.text_processed.clone(),
                short_text: article_data.short_text_processed.clone(),
                article_file_name: article_data.article_file_name.clone(),
                image_url: article_data.image_path.clone(),
                image_description: article_data.image_description.clone(),
                video_url: article_data.video_path.clone(),
                audio_url: article_data.audio_path.clone(),
                category: article_data.category.clone(),
                related_articles: related_articles_vec.clone(),
                is_main: article_data.is_main,
                is_exclusive: article_data.is_exclusive,
                views: 0,
            };

            let _ = database_article::create_article(article_db).await;

            if !std::path::Path::new(&category_month_year_filename).exists() {

                let mut base_html = category_template.render().unwrap();
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

            Redirect::to(&article_data.article_file_name).into_response()
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}
