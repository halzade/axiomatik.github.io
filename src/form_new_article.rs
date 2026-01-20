use crate::form_new_article_data::article_data;
use crate::server::AUTH_COOKIE;
use crate::{database, external, library, name_days, form_index};
use askama::Template;
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use chrono::Datelike;
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
    pub related_snippets: String,
    pub weather: String,
    pub name_day: String,
}

#[derive(Template)]
#[template(path = "snippet_template.html")]
pub struct SnippetTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
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
    let article_data_o = article_data(multipart).await;

    match article_data_o {
        None => StatusCode::BAD_REQUEST.into_response(),
        Some(article_data) => {
            let related_articles_vec = &article_data
                .related_articles
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();

            let related_article_snippets = library::read_related_articles(&related_articles_vec);

            let now = chrono::Local::now();

            let formated_date = library::formatted_article_date(now);
            let formated_name_day = name_days::formatted_today_name_date(now);
            let formated_weather = external::fetch_weather().await;

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
                related_snippets: related_article_snippets.clone(),
                date: formated_date.clone(),
                weather: formated_weather.clone(),
                name_day: formated_name_day.clone(),
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

            let snippet = SnippetTemplate {
                url: file_path.clone(),
                title: article_data.title.clone(),
                short_text: article_data.short_text_processed.clone(),
            }
            .render()
            .unwrap();

            let snippet_file_path = format!("snippets/{}.txt", file_path);
            fs::write(snippet_file_path, &snippet).unwrap();

            // Store in DB
            let article_db = database::Article {
                author: article_data.author.clone(),
                created_by,
                date: formated_date.clone(),
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
                };
                let mut base_html = cat_template.render().unwrap();
                base_html = base_html.replace(
                    "<!-- SNIPPETS -->",
                    &format!("<!-- SNIPPETS -->\n{}", snippet),
                );
                fs::write(&category_month_year_filename, base_html).unwrap();
            } else {
                let mut content = fs::read_to_string(&category_month_year_filename).unwrap();
                content = content.replace(
                    "<!-- SNIPPETS -->",
                    &format!("<!-- SNIPPETS -->\n{}", snippet),
                );
                fs::write(&category_month_year_filename, content).unwrap();
            }

            let main_cat_filename = format!("{}.html", article_data.category);
            if std::path::Path::new(&main_cat_filename).exists() {
                let mut content = fs::read_to_string(&main_cat_filename).unwrap();
                if content.contains("<!-- SNIPPETS -->") {
                    content = content.replace(
                        "<!-- SNIPPETS -->",
                        &format!("<!-- SNIPPETS -->\n{}", snippet),
                    );
                }
                fs::write(&main_cat_filename, content).unwrap();
            }

            for path in related_articles_vec {
                if let Ok(mut content) = fs::read_to_string(path) {
                    if content.contains("<!-- SNIPPETS -->") {
                        content = content.replace(
                            "<!-- SNIPPETS -->",
                            &format!("<!-- SNIPPETS -->\n{}", snippet),
                        );
                        fs::write(path, content).unwrap();
                    }
                }
            }

            let index_data = form_index::IndexData {
                date: formated_date,
                weather: formated_weather,
                name_day: formated_name_day,
                main_article_url: "".to_string(),
                main_article_title: "".to_string(),
                main_article_short_text: "".to_string(),
                main_article_image: "".to_string(),
                second_article_url: "".to_string(),
                second_article_title: "".to_string(),
                second_article_short_text: "".to_string(),
                third_article_url: "".to_string(),
                third_article_title: "".to_string(),
                third_article_short_text: "".to_string(),
                z_republiky: "".to_string(),
                ze_zahranici: "".to_string(),
            };

            form_index::render_new_index(Some(index_data)).await;

            Redirect::to(&*file_path).into_response()
        }
    }
}
