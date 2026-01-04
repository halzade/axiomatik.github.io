pub mod auth;
pub mod db;

use askama::Template;
use axum::{
    Router,
    body::Body,
    extract::{Form, Multipart, State},
    http::{Request},
    middleware::{self, Next},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use chrono::Datelike;
use serde::Deserialize;
use std::fs;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "../pages/form.html")]
pub struct FormTemplate;

#[derive(Template)]
#[template(path = "../pages/login.html")]
pub struct LoginTemplate {
    pub error: bool,
}

#[derive(Template)]
#[template(path = "../pages/change_password.html")]
pub struct ChangePasswordTemplate {
    pub error: bool,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordPayload {
    pub new_password: String,
}

pub const AUTH_COOKIE: &str = "axiomatik_auth";

#[derive(Template)]
#[template(path = "article_template.html")]
pub struct ArticleTemplate {
    pub title: String,
    pub author: String,
    pub text: String,
    pub image_path: String,
    pub video_path: Option<String>,
    pub category: String,
    pub category_display: String,
    pub related_snippets: String,
}

#[derive(Template)]
#[template(path = "snippet_template.html")]
pub struct SnippetTemplate {
    pub url: String,
    pub title: String,
    pub short_text: String,
}

#[derive(Template)]
#[template(path = "category_template.html")]
pub struct CategoryTemplate {
    pub title: String,
}

pub fn app(db: Arc<db::Database>) -> Router {
    let protected_routes = Router::new()
        .route("/form", get(show_form))
        .route("/create", post(create_article))
        .route("/change-password", get(show_change_password).post(handle_change_password))
        .layer(middleware::from_fn_with_state(db.clone(), auth_middleware));

    Router::new()
        .route("/", get(|| async { Redirect::to("/form") }))
        .route("/login", get(show_login).post(handle_login))
        .merge(protected_routes)
        .nest_service("/uploads", ServeDir::new("uploads"))
        .nest_service("/css", ServeDir::new("css"))
        .nest_service("/js", ServeDir::new("js"))
        .with_state(db)
}

async fn auth_middleware(
    State(db): State<Arc<db::Database>>,
    jar: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        if let Ok(Some(user)) = db.get_user(cookie.value()).await {
            if user.needs_password_change && req.uri().path() != "/change-password" {
                return Redirect::to("/change-password").into_response();
            }
            if user.role == db::Role::Editor {
                return next.run(req).await;
            }
        }
    }
    Redirect::to("/login").into_response()
}

pub async fn show_login() -> impl IntoResponse {
    Html(LoginTemplate { error: false }.render().unwrap())
}

pub async fn handle_login(
    State(db): State<Arc<db::Database>>,
    jar: CookieJar,
    Form(payload): Form<LoginPayload>,
) -> impl IntoResponse {
    match auth::authenticate_user(&db, &payload.username, &payload.password).await {
        Ok(user) => {
            info!(user = %user.username, "User logged in successfully");
            let jar = jar.add(Cookie::new(AUTH_COOKIE, user.username));
            if user.needs_password_change {
                (jar, Redirect::to("/change-password")).into_response()
            } else {
                (jar, Redirect::to("/form")).into_response()
            }
        }
        Err(e) => {
            warn!(username = %payload.username, error = %e, "Failed login attempt");
            (jar, Html(LoginTemplate { error: true }.render().unwrap())).into_response()
        }
    }
}

pub async fn show_change_password() -> Response {
    Html(ChangePasswordTemplate { error: false }.render().unwrap()).into_response()
}

pub async fn handle_change_password(
    State(db): State<Arc<db::Database>>,
    jar: CookieJar,
    Form(payload): Form<ChangePasswordPayload>,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value();
        match auth::change_password(&db, username, &payload.new_password).await {
            Ok(_) => Redirect::to("/form").into_response(),
            Err(_) => {
                Html(ChangePasswordTemplate { error: true }.render().unwrap()).into_response()
            }
        }
    } else {
        Redirect::to("/login").into_response()
    }
}

pub async fn show_form() -> Response {
    Html(FormTemplate.render().unwrap()).into_response()
}

pub async fn create_article(
    mut multipart: Multipart,
) -> Response {
    let mut title = String::new();
    let mut author = String::new();
    let mut text = String::new();
    let mut short_text = String::new();
    let mut category = String::new();
    let mut related_articles_input = String::new();
    let mut image_path = String::new();
    let mut video_path = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "title" => title = field.text().await.unwrap(),
            "author" => author = field.text().await.unwrap(),
            "text" => {
                let raw_text = field.text().await.unwrap();
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
                if let Some(file_name) = field.file_name() {
                    let extension = std::path::Path::new(file_name)
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("jpg");
                    let new_name = format!("{}.{}", Uuid::new_v4(), extension);
                    let data = field.bytes().await.unwrap();
                    fs::write(format!("uploads/{}", new_name), data).unwrap();
                    image_path = new_name;
                }
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
                        video_path = Some(new_name);
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
        let snippet_path = format!("snippets/{}.txt", path);
        if let Ok(snippet_html) = fs::read_to_string(&snippet_path) {
            related_snippets.push_str(&snippet_html);
            related_snippets.push('\n');
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
    let safe_title = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>();
    let file_path = format!("{}.html", safe_title);
    fs::write(&file_path, html_content).unwrap();

    let now = chrono::Local::now();
    let czech_months = [
        "leden", "unor", "brezen", "duben", "kveten", "cerven", "cervenec", "srpen", "zari",
        "rijen", "listopad", "prosinec",
    ];
    let month_name = czech_months[(now.month() - 1) as usize];
    let cat_month_year_filename =
        format!("archive-{}-{}-{}.html", category, month_name, now.year());

    let snippet = SnippetTemplate {
        url: file_path.clone(),
        title: title.clone(),
        short_text,
    }
    .render()
    .unwrap();

    let snippet_file_path = format!("snippets/{}.txt", file_path);
    fs::write(snippet_file_path, &snippet).unwrap();

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

    Redirect::to(&format!("/{}.html", safe_title)).into_response()
}
