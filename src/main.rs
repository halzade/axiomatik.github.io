use askama::Template;
use axum::{
    Router,
    extract::Multipart,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
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
    text: String,
    image_path: String,
    video_path: Option<String>,
    category: String,
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
    let mut text = String::new();
    let mut category = String::new();
    let mut image_path = String::new();
    let mut video_path = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        println!("Processing field: {}", name);

        match name.as_str() {
            "title" => title = field.text().await.unwrap(),

            "text" => text = field.text().await.unwrap(),

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

    let template = ArticleTemplate {
        title: title.clone(),
        text,
        image_path,
        video_path,
        category,
    };

    let html_content = template.render().unwrap();

    // Generate a slug-like filename
    let safe_title = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>();
    let file_path = format!("unp/{}.html", safe_title);

    fs::write(&file_path, html_content).unwrap();

    println!("Generated static file at: {}", file_path);

    Redirect::to(&format!("/unp/{}.html", safe_title))
}
