mod auth;

use askama::Template;
use axum::{
    Router,
    extract::{Multipart, Form},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    http::StatusCode,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use chrono::Datelike;
use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "form.html")]
struct FormTemplate;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[derive(Deserialize)]
struct LoginPayload {
    token: String,
}

const AUTH_COOKIE: &str = "axiomatik_auth";

#[derive(Template)]
#[template(path = "article_template.html")]
struct ArticleTemplate {
    title: String,
    author: String,
    text: String,
    image_path: String,
    video_path: Option<String>,
    category: String,
    category_display: String,
    related_snippets: String,
}

#[derive(Template)]
#[template(path = "snippet_template.html")]
struct SnippetTemplate {
    url: String,
    title: String,
    short_text: String,
}

#[derive(Template)]
#[template(path = "category_template.html")]
struct CategoryTemplate {
    title: String,
}

#[tokio::main]
async fn main() {
    // Ensure uploads directory exists
    fs::create_dir_all("uploads").unwrap();
    fs::create_dir_all("unp").unwrap();
    fs::create_dir_all("snippets").unwrap();

    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/form") }))
        .route("/form", get(show_form))
        .route("/login", get(show_login).post(handle_login))
        .route("/create", post(create_article))
        .nest_service("/unp", ServeDir::new("unp"))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .nest_service("/css", ServeDir::new("css"))
        .nest_service("/js", ServeDir::new("js"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn show_login() -> impl IntoResponse {
    Html(LoginTemplate.render().unwrap())
}

async fn handle_login(jar: CookieJar, Form(payload): Form<LoginPayload>) -> impl IntoResponse {
    if auth::verify_token(&payload.token) {
        let jar = jar.add(Cookie::new(AUTH_COOKIE, "authenticated"));
        (jar, Redirect::to("/form"))
    } else {
        (jar, Redirect::to("/login"))
    }
}

async fn show_form(jar: CookieJar) -> Response {
    if jar.get(AUTH_COOKIE).is_some() {
        Html(FormTemplate.render().unwrap()).into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

async fn create_article(jar: CookieJar, mut multipart: Multipart) -> Response {
    if jar.get(AUTH_COOKIE).is_none() {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    println!("Received create_article request");
    let mut title = String::new();
    let mut author = String::new();
    let mut text = String::new();
    let mut short_text = String::new();
    let mut category = String::new();
    let mut related_articles_input = String::new();
    let mut image_path = String::new();
    let mut video_path = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        println!("Processing field: {}", name);

        match name.as_str() {
            "title" => title = field.text().await.unwrap(),

            "author" => author = field.text().await.unwrap(),

            "text" => {
                let raw_text = field.text().await.unwrap();
                // Process text:
                // 1. Normalize line endings
                // 2. Split by "two empty lines" (3 or more newlines) into containers
                // 3. Within each container, split by "one empty line" (2 newlines) into paragraphs
                let normalized = raw_text.replace("\r\n", "\n");

                let processed = normalized
                    .split("\n\n\n") // Three newlines = two empty lines
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
                text = processed;
            }

            "short_text" => {
                let raw_text = field.text().await.unwrap();
                let normalized = raw_text.replace("\r\n", "\n");
                short_text = normalized
                    .split("\n\n")
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().replace("\n", "<br>\n"))
                    .collect::<Vec<String>>()
                    .join("</p><p>");
            }

            "category" => category = field.text().await.unwrap(),

            "related_articles" => related_articles_input = field.text().await.unwrap(),

            "image" => {
                let file_name = field.file_name().unwrap().to_string();
                let extension = std::path::Path::new(&file_name)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("jpg");
                let new_name = format!("{}.{}", Uuid::new_v4(), extension);
                let data = field.bytes().await.unwrap();
                fs::write(format!("uploads/{}", new_name), data).unwrap();
                image_path = new_name.clone();
                println!("Saved image to uploads/{}", new_name);
            }

            "video" => {
                if let Some(file_name) = field.file_name() {
                    if !file_name.is_empty() {
                        let extension = std::path::Path::new(file_name)
                            .extension()
                            .and_then(|s| s.to_str())
                            .unwrap_or("mp4");
                        let new_name = format!("{}.{}", Uuid::new_v4(), extension);
                        let data = field.bytes().await.unwrap();
                        fs::write(format!("uploads/{}", new_name), data).unwrap();
                        video_path = Some(new_name.clone());
                        println!("Saved video to uploads/{}", new_name);
                    }
                }
            }
            _ => (),
        }
    }

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
        // Look for the snippet in the snippets/ folder
        // The snippet file is named [url].txt, where [url] is the filename like "jeden-tisic-dnu.html"
        let snippet_path = format!("snippets/{}.txt", path);
        if let Ok(snippet_html) = fs::read_to_string(&snippet_path) {
            related_snippets.push_str(&snippet_html);
            related_snippets.push('\n');
        } else {
            println!("Warning: Snippet not found for related article: {}", path);
        }
    }

    let template = ArticleTemplate {
        title: title.clone(),
        author,
        text,
        image_path,
        video_path,
        category: category.clone(),
        category_display: category_display.clone(),
        related_snippets: related_snippets.clone(),
    };

    let html_content = template.render().unwrap();

    // Generate a slug-like filename
    let safe_title = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>();
    let file_path = format!("{}.html", safe_title);

    fs::write(&file_path, html_content).unwrap();
    println!("Generated static file at: {}", file_path);

    // Update category-month-year.html
    let now = chrono::Local::now();
    let czech_months = [
        "leden", "unor", "brezen", "duben", "kveten", "cerven", "cervenec", "srpen", "zari",
        "rijen", "listopad", "prosinec",
    ];
    let month_name = czech_months[(now.month() - 1) as usize];
    let cat_month_year_filename = format!("archive-{}-{}-{}.html", category, month_name, now.year());

    let snippet = SnippetTemplate {
        url: file_path.clone(),
        title: title.clone(),
        short_text,
    }
    .render()
    .unwrap();

    // Save snippet to 'snippets' folder
    let snippet_file_path = format!("snippets/{}.txt", file_path);
    fs::write(snippet_file_path, &snippet).unwrap();
    println!("Saved snippet to snippets/{}.txt", file_path);

    if !std::path::Path::new(&cat_month_year_filename).exists() {
        let cat_template = CategoryTemplate {
            title: format!("{} - {} {}", category_display, month_name, now.year()),
        };
        let mut base_html = cat_template.render().unwrap();
        // Insert snippet into the article-grid section using the comment marker
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

    // Update main category page (e.g., zahranici.html)
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

    // Update related articles with the new article's snippet
    for path in &related_article_paths {
        if let Ok(mut content) = fs::read_to_string(path) {
            if content.contains("<!-- SNIPPETS -->") {
                content = content.replace(
                    "<!-- SNIPPETS -->",
                    &format!("<!-- SNIPPETS -->\n{}", snippet),
                );
                fs::write(path, content).unwrap();
            } else if content.contains("<div class=\"article-grid\">") {
                // fallback if comment marker is missing but grid exists
                content = content.replace(
                    "<div class=\"article-grid\">",
                    &format!("<div class=\"article-grid\">\n{}", snippet),
                );
                fs::write(path, content).unwrap();
            }
        }
    }

    Redirect::to(&format!("/{}.html", safe_title)).into_response()
}
