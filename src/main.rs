use askama::Template;
use axum::{
    Router,
    extract::Multipart,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use chrono::Datelike;
use std::fs;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "form.html")]
struct FormTemplate;

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

    let app = Router::new()
        .route("/", get(show_form))
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

async fn show_form() -> impl IntoResponse {
    Html(FormTemplate.render().unwrap())
}

async fn create_article(mut multipart: Multipart) -> impl IntoResponse {
    println!("Received create_article request");
    let mut title = String::new();
    let mut author = String::new();
    let mut text = String::new();
    let mut short_text = String::new();
    let mut category = String::new();
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

    let template = ArticleTemplate {
        title: title.clone(),
        author,
        text,
        image_path,
        video_path,
        category: category.clone(),
        category_display: category_display.clone(),
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
    let cat_month_year_filename = format!("{}-{}-{}.html", category, month_name, now.year());

    let snippet = SnippetTemplate {
        url: file_path.clone(),
        title: title.clone(),
        short_text,
    }
    .render()
    .unwrap();

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

    Redirect::to(&format!("/{}.html", safe_title))
}
