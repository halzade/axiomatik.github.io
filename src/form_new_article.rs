use crate::{database, library};
use crate::server::AUTH_COOKIE;
use crate::templates::FormTemplate;
use askama::Template;
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use chrono::Datelike;
use http::StatusCode;
use std::fs;
use uuid::Uuid;

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

pub async fn create_article(jar: CookieJar, mut multipart: Multipart) -> Response {
    let created_by = if let Some(cookie) = jar.get(AUTH_COOKIE) {
        cookie.value().to_string()
    } else {
        return Redirect::to("/login").into_response();
    };

    let mut title = String::new();
    let mut author = String::new();
    let mut text_raw = String::new();
    let mut text_processed = String::new();
    let mut short_text_raw = String::new();
    let mut short_text_processed = String::new();
    let mut category = String::new();
    let mut related_articles_input = String::new();
    let mut image_path = String::new();
    let mut image_description = String::new();
    let mut video_path = None;
    let mut audio_path = None;
    let mut is_main = false;
    let mut is_exclusive = false;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "is_main" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                is_main = val == "on";
            }

            "is_exclusive" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                is_exclusive = val == "on";
            }

            "title" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                title = val;
            }

            "author" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                author = val;
            }

            "text" => {
                let raw_text = field.text().await.unwrap();
                if validate_input(&raw_text).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                text_raw = raw_text.clone();
                let normalized = raw_text.replace("\r\n", "\n");
                let processed = normalized
                    .split("\n\n\n")
                    .filter(|block| !block.trim().is_empty())
                    .map(|block| {
                        let inner_html = block
                            .split("\n\n")
                            .filter(|s| !s.trim().is_empty())
                            .map(|s| {
                                if s.starts_with("   ") {
                                    format!("<blockquote>{}</blockquote>", s.trim())
                                } else {
                                    format!("<p>{}</p>", s.trim().replace("\n", " "))
                                }
                            })
                            .collect::<Vec<String>>()
                            .join("");
                        format!("<div class=\"container\">{}</div>", inner_html)
                    })
                    .collect::<Vec<String>>()
                    .join("");
                text_processed = processed;
            }

            "short_text" => {
                let raw_text = field.text().await.unwrap();
                if validate_input(&raw_text).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                short_text_raw = raw_text.clone();
                let normalized = raw_text.replace("\r\n", "\n");
                short_text_processed = normalized
                    .split("\n\n")
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().replace("\n", "<br>\n"))
                    .collect::<Vec<String>>()
                    .join("</p><p>");
            }

            "category" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                category = val;
            }

            "related_articles" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                related_articles_input = val;
            }

            "image_description" => {
                let val = field.text().await.unwrap();
                if validate_input(&val).is_err() {
                    return StatusCode::BAD_REQUEST.into_response();
                }
                image_description = val;
            }

            "image" => {
                if let Some(file_name) = field.file_name() {
                    if validate_input(&file_name).is_err() {
                        return StatusCode::BAD_REQUEST.into_response();
                    }

                    if !file_name.is_empty() {
                        let extension = std::path::Path::new(file_name)
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_lowercase());

                        match extension {
                            Some(ext) if ["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()) => {
                                let new_name = format!("{}.{}", Uuid::new_v4(), ext);
                                let data = field.bytes().await.unwrap();
                                fs::write(format!("uploads/{}", new_name), data).unwrap();
                                image_path = new_name;
                            }
                            _ => {
                                // If extension is missing or not allowed, we just skip it or could return error
                                // For now, let's just not set image_path
                            }
                        }
                    }
                }
            }

            "video" => {
                if let Some(file_name) = field.file_name() {
                    if validate_input(&file_name).is_err() {
                        return StatusCode::BAD_REQUEST.into_response();
                    }

                    if !file_name.is_empty() {
                        let extension = std::path::Path::new(file_name)
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_lowercase());

                        match extension {
                            Some(ext) if ["avi", "mp4", "webm", "mov"].contains(&ext.as_str()) => {
                                let new_name = format!("{}.{}", Uuid::new_v4(), ext);
                                let data = field.bytes().await.unwrap();
                                fs::write(format!("uploads/{}", new_name), data).unwrap();
                                video_path = Some(new_name);
                            }
                            _ => {
                                // If extension is missing or not allowed, skip
                            }
                        }
                    }
                }
            }

            "audio" => {
                if let Some(file_name) = field.file_name() {
                    if validate_input(&file_name).is_err() {
                        return StatusCode::BAD_REQUEST.into_response();
                    }

                    if !file_name.is_empty() {
                        let extension = std::path::Path::new(file_name)
                            .extension()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_lowercase());

                        match extension {
                            Some(ext) if ["mp3", "wav", "ogg", "m4a"].contains(&ext.as_str()) => {
                                let new_name = format!("{}.{}", Uuid::new_v4(), ext);
                                let data = field.bytes().await.unwrap();
                                fs::write(format!("uploads/{}", new_name), data).unwrap();
                                audio_path = Some(new_name);
                            }
                            _ => {
                                // If extension is missing or not allowed, skip
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    // TODO move to library
    let category_display = match category.as_str() {
        "zahranici" => "zahraničí",
        "republika" => "republika",
        "finance" => "finance",
        "technologie" => "technologie",
        "veda" => "věda",
        _ => &category,
    }
    .to_string();

    let mut related_snippets = String::new();
    let related_article_paths: Vec<&str> = related_articles_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for path in &related_article_paths {
        let snippet_path = format!("snippets/{}.txt", path);
        if let Ok(snippet_html) = fs::read_to_string(&snippet_path) {
            related_snippets.push_str(&snippet_html);
            related_snippets.push('\n');
        }
    }

    let now = chrono::Local::now();
    let month_name = get_czech_month(now.month(), true);
    let formatted_date = format!("{}. {} {}", now.day(), month_name, now.year());

    let day_name = library::day_of_week(now.weekday());
    let month_name_genitive = get_czech_month_genitive(now.month());
    let current_date = format!(
        "{} {}. {} {}",
        day_name,
        now.day(),
        month_name_genitive,
        now.year()
    );

    let nameday = {
        let name = name_days::today_name_day();
        if name.is_empty() || name.contains("No nameday") || name.contains("Invalid") {
            "".to_string()
        } else {
            format!("Svátek má {}", name)
        }
    };

    // TODO move to external
    let weather = {
        let url = "https://api.open-meteo.com/v1/forecast?latitude=50.0755&longitude=14.4378&current_weather=true&timezone=Europe/Prague";
        // Simple synchronous fetch for simplicity in this context, or just use a default if it fails
        // Given we are in an async function, we can use reqwest
        match reqwest::get(url).await {
            Ok(resp) => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let temp = json["current_weather"]["temperature"]
                        .as_f64()
                        .unwrap_or(0.0);
                    format!("{:.0}°C | Praha", temp)
                } else {
                    "".to_string()
                }
            }
            Err(_) => "".to_string(),
        }
    };

    let template = ArticleTemplate {
        title: title.clone(),
        author: author.clone(),
        date: formatted_date.clone(),
        text: text_processed,
        image_path: image_path.clone(),
        image_description: image_description.clone(),
        video_path: video_path.clone(),
        audio_path: audio_path.clone(),
        category: category.clone(),
        category_display: category_display.clone(),
        related_snippets: related_snippets.clone(),
        current_date,
        weather,
        nameday,
    };

    let html_content = template.render().unwrap();
    let safe_title = library::save_article_file_name(title);
    let file_path = format!("{}.html", safe_title);
    fs::write(&file_path, html_content).unwrap();

    let month_name = library::get_czech_month(now.month(), false);
    let cat_month_year_filename =
        format!("archive-{}-{}-{}.html", category, month_name, now.year());

    let snippet = SnippetTemplate {
        url: file_path.clone(),
        title: title.clone(),
        short_text: short_text_processed.clone(),
    }
    .render()
    .unwrap();

    if is_main {
        if let Ok(mut index_content) = fs::read_to_string("index.html") {
            // 1. Get current contents
            let mut main_article_content = String::new();
            if let (Some(start), Some(end)) = (
                index_content.find("<!-- MAIN_ARTICLE -->"),
                index_content.find("<!-- /MAIN_ARTICLE -->"),
            ) {
                main_article_content = index_content[start + "<!-- MAIN_ARTICLE -->".len()..end]
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

            // 2. Prepare new MAIN_ARTICLE
            let title_with_exclusive = if is_exclusive {
                format!(r#"<span class="red">EXKLUZIVNĚ:</span> {}"#, title)
            } else {
                title.clone()
            };

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
                short_text_processed,
                file_path,
                image_path,
                image_description
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
                    .filter(|s| !s.contains("<img")) // Simple way to strip image if it's in its own <a> tag
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
    let article_db = db::Article {
        author: author.clone(),
        created_by,
        date: formatted_date,
        title: title.clone(),
        text: text_raw,
        short_text: short_text_raw,
        article_file_name: file_path.clone(),
        image_url: image_path,
        image_description: image_description.clone(),
        video_url: video_path,
        audio_url: audio_path,
        category: category.clone(),
        related_articles: related_articles_input.clone(),
        views: 0,
    };

    let _ = db.create_article(article_db).await;

    if !std::path::Path::new(&cat_month_year_filename).exists() {
        let cat_template = CategoryTemplate {
            title: format!("{} - {} {}", category_display, month_name, now.year()),
        };
        let mut base_html = cat_template.render().unwrap();
        base_html = base_html.replace(
            "<!-- SNIPPETS -->",
            &format!("<!-- SNIPPETS -->\n{}", snippet),
        );
        fs::write(&cat_month_year_filename, base_html).unwrap();
    } else {
        let mut content = fs::read_to_string(&cat_month_year_filename).unwrap();
        content = content.replace(
            "<!-- SNIPPETS -->",
            &format!("<!-- SNIPPETS -->\n{}", snippet),
        );
        fs::write(&cat_month_year_filename, content).unwrap();
    }

    let main_cat_filename = format!("{}.html", category);
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

    for path in &related_article_paths {
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

    let (marker_start, marker_end) = match category.as_str() {
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

                let new_section_content = format!("{}\n                    ", articles.join(""));
                index_content.replace_range(start + marker_start.len()..end, &new_section_content);
                fs::write("index.html", index_content).unwrap();
            }
        }
    }

    Redirect::to(&format!("/{}.html", safe_title)).into_response()
}
