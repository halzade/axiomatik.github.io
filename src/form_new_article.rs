use crate::form_new_article_data::article_data;
use crate::server::AUTH_COOKIE;
use crate::{database, external, library, name_days};
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
    // TODO double click

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
                weather: formated_weather,
                name_day: formated_name_day,
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

            if article_data.is_main {
                if let Ok(mut index_content) = fs::read_to_string("index.html") {
                    // 1. Get current contents
                    let mut main_article_content = String::new();
                    if let (Some(start), Some(end)) = (
                        index_content.find("<!-- MAIN_ARTICLE -->"),
                        index_content.find("<!-- /MAIN_ARTICLE -->"),
                    ) {
                        main_article_content = index_content
                            [start + "<!-- MAIN_ARTICLE -->".len()..end]
                            .trim()
                            .to_string();
                    }

                    let mut second_article_content = String::new();
                    if let (Some(start), Some(end)) = (
                        index_content.find("<!-- SECOND_ARTICLE -->"),
                        index_content.find("<!-- /SECOND_ARTICLE -->"),
                    ) {
                        second_article_content = index_content
                            [start + "<!-- SECOND_ARTICLE -->".len()..end]
                            .trim()
                            .to_string();
                    }

                    // 2. Prepare a new MAIN_ARTICLE
                    let title_with_exclusive = if article_data.is_exclusive {
                        format!(
                            r#"<span class="red">EXKLUZIVNĚ:</span> {}"#,
                            article_data.title
                        )
                    } else {
                        article_data.title.clone()
                    };

                    // TODO
                    let new_main_article = format!(
                        r#"
                <div class="main-article-text">
                    <a href="{}"><h1>{}</h1></a>
                    <a href="{}">
                        <p>
                            {}
                        </p>
                    </a>
                </div>
                <a href="{}">
                    <img src="uploads/{}" alt="{}">
                </a>
                "#,
                        file_path,
                        title_with_exclusive,
                        file_path,
                        article_data.short_text_processed.clone(),
                        file_path,
                        article_data.image_path.clone(),
                        article_data.image_description.clone()
                    );

                    // 3. Update index_content
                    // Update THIRD_ARTICLE with old SECOND_ARTICLE
                    if let (Some(start), Some(end)) = (
                        index_content.find("<!-- THIRD_ARTICLE -->"),
                        index_content.find("<!-- /THIRD_ARTICLE -->"),
                    ) {
                        let shifted_third = second_article_content
                            .replace("class=\"first-article\"", "class=\"second-article\"")
                            .replace("class='first-article'", "class='second-article'");

                        index_content.replace_range(
                            start + "<!-- THIRD_ARTICLE -->".len()..end,
                            &format!("\n                {}\n                ", shifted_third),
                        );
                    }

                    // Update SECOND_ARTICLE with old MAIN_ARTICLE
                    if let (Some(start), Some(end)) = (
                        index_content.find("<!-- SECOND_ARTICLE -->"),
                        index_content.find("<!-- /SECOND_ARTICLE -->"),
                    ) {
                        let shifted_second = main_article_content
                            .replace("class=\"main-article-text\"", "class=\"first-article\"")
                            .replace("class='main-article-text'", "class='first-article'")
                            .replace("<h1>", "<h2>")
                            .replace("</h1>", "</h2>")
                            .replace(r#"<span class="red">EXKLUZIVNĚ:</span>"#, "")
                            // If there was an image in MAIN_ARTICLE, it was outside the div.
                            // We need to decide if we keep it or strip it for SECOND/THIRD articles.
                            // Looking at index.html, SECOND_ARTICLE and THIRD_ARTICLE don't seem to have images.
                            // However, shifting the WHOLE content might include the <img> tag if it was there.
                            .split("<a href=")
                            .filter(|s| !s.contains("<img")) // Simple way to strip an image if it's in its own <a> tag
                            .collect::<Vec<&str>>()
                            .join("<a href=");

                        index_content.replace_range(
                            start + "<!-- SECOND_ARTICLE -->".len()..end,
                            &format!("\n                {}\n                ", shifted_second),
                        );
                    }

                    // Update MAIN_ARTICLE
                    if let (Some(start), Some(end)) = (
                        index_content.find("<!-- MAIN_ARTICLE -->"),
                        index_content.find("<!-- /MAIN_ARTICLE -->"),
                    ) {
                        index_content.replace_range(
                            start + "<!-- MAIN_ARTICLE -->".len()..end,
                            &new_main_article,
                        );
                    }

                    fs::write("index.html", index_content).unwrap();
                }
            }

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

            let (marker_start, marker_end) = match article_data.category.as_str() {
                "republika" => ("<!-- Z_REPUBLIKY -->", "<!-- /Z_REPUBLIKY -->"),
                "zahranici" => ("<!-- ZE_ZAHRANICI -->", "<!-- /ZE_ZAHRANICI -->"),
                _ => ("", ""),
            };

            if !marker_start.is_empty() {
                if let Ok(mut index_content) = fs::read_to_string("index.html") {
                    if let (Some(start), Some(end)) = (
                        index_content.find(marker_start),
                        index_content.find(marker_end),
                    ) {
                        let section_content = &index_content[start + marker_start.len()..end];
                        let mut articles: Vec<String> = section_content
                            .split("</article>")
                            .filter(|s| s.contains("<article"))
                            .map(|s| format!("{}</article>", s))
                            .collect();

                        articles.insert(0, format!("\n{}", snippet.trim()));

                        if articles.len() > 10 {
                            articles.truncate(10);
                        }

                        let new_section_content =
                            format!("{}\n                    ", articles.join(""));
                        index_content
                            .replace_range(start + marker_start.len()..end, &new_section_content);
                        fs::write("index.html", index_content).unwrap();
                    }
                }
            }
            Redirect::to(&*file_path).into_response()
        }
    }
}
