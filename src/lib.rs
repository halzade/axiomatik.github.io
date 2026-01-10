pub mod auth;
pub mod configuration;
pub mod db;
pub mod db_tool;
pub mod namedays;

use askama::Template;
use axum::{
    Router,
    body::Body,
    extract::{Form, Multipart, State},
    http::{Request, StatusCode},
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
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "../pages/form.html")]
pub struct FormTemplate {
    pub author_name: String,
}

#[derive(Template)]
#[template(path = "../pages/login.html")]
pub struct LoginTemplate {
    pub error: bool,
}

#[derive(Template)]
#[template(path = "../pages/change_password.html")]
pub struct ChangePasswordTemplate {
    pub error: bool,
    pub username: String,
}

#[derive(Template)]
#[template(path = "../pages/account.html")]
pub struct AccountTemplate {
    pub username: String,
    pub author_name: String,
    pub articles: Vec<db::Article>,
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

#[derive(Deserialize)]
pub struct UpdateAuthorNamePayload {
    pub author_name: String,
}

pub const AUTH_COOKIE: &str = "axiomatik_auth";

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

const CZECH_MONTHS: [&str; 12] = [
    "Leden",
    "Únor",
    "Březen",
    "Duben",
    "Květen",
    "Červen",
    "Červenec",
    "Srpen",
    "Září",
    "Říjen",
    "Listopad",
    "Prosinec",
];

const CZECH_MONTHS_SHORT: [&str; 12] = [
    "leden", "unor", "brezen", "duben", "kveten", "cerven", "cervenec", "srpen", "zari", "rijen",
    "listopad", "prosinec",
];

fn get_czech_month(month: u32, capitalized: bool) -> &'static str {
    let idx = (month - 1) as usize;
    if capitalized {
        CZECH_MONTHS[idx]
    } else {
        CZECH_MONTHS_SHORT[idx]
    }
}

pub fn app(db: Arc<db::Database>) -> Router {
    let protected_routes = Router::new()
        .route("/form", get(show_form))
        .route("/create", post(create_article))
        .route(
            "/change-password",
            get(show_change_password).post(handle_change_password),
        )
        .route("/account", get(show_account))
        .route("/account/update-author", post(handle_update_author_name))
        .layer(middleware::from_fn_with_state(db.clone(), auth_middleware));

    Router::new()
        .route("/", get(|| async { Redirect::to("/index.html") }))
        .route("/login", get(show_login).post(handle_login))
        .merge(protected_routes)
        // serve static content
        // TODO serve only html, css, js
        .fallback(handle_fallback)
        .with_state(db)
}

pub async fn handle_fallback(State(db): State<Arc<db::Database>>, req: Request<Body>) -> Response {
    let path = req.uri().path();
    let file_path = if path == "/" || path.is_empty() {
        "index.html".to_string()
    } else {
        path.trim_start_matches('/').to_string()
    };

    if file_path.ends_with(".html") {
        if let Ok(content) = fs::read_to_string(&file_path) {
            let _ = db.increment_article_views(&file_path).await;
            return Html(content).into_response();
        }
    }

    // Default to serving from ServeDir
    let service = ServeDir::new(".");
    match tower::ServiceExt::oneshot(service, req).await {
        Ok(res) => {
            if res.status() == StatusCode::NOT_FOUND {
                show_404().await.into_response()
            } else {
                res.into_response()
            }
        }
        Err(_) => show_404().await.into_response(),
    }
}

pub async fn show_404() -> impl IntoResponse {
    info!("show_404 called");
    (
        StatusCode::NOT_FOUND,
        Html(fs::read_to_string("404.html").unwrap_or_else(|_| "404 Not Found".to_string())),
    )
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
) -> Response {
    if validate_input_simple(&payload.username).is_err()
        || validate_input_simple(&payload.password).is_err()
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let username = &payload.username;
    match auth::authenticate_user(&db, username, &payload.password).await {
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

pub async fn show_change_password(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value().to_string();
        Html(
            ChangePasswordTemplate {
                error: false,
                username,
            }
            .render()
            .unwrap(),
        )
        .into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

pub async fn handle_change_password(
    State(db): State<Arc<db::Database>>,
    jar: CookieJar,
    Form(payload): Form<ChangePasswordPayload>,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value();
        if validate_input(&payload.new_password).is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match auth::change_password(&db, username, &payload.new_password).await {
            Ok(_) => Redirect::to("/account").into_response(),
            Err(e) => {
                error!("{:?}", e);
                Html(
                    ChangePasswordTemplate {
                        error: true,
                        username: username.to_string(),
                    }
                    .render()
                    .unwrap(),
                )
                .into_response()
            }
        }
    } else {
        Redirect::to("/login").into_response()
    }
}

pub async fn show_form(State(db): State<Arc<db::Database>>, jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        if let Ok(Some(user)) = db.get_user(cookie.value()).await {
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
    Redirect::to("/login").into_response()
}

pub async fn show_account(State(db): State<Arc<db::Database>>, jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        if let Ok(Some(user)) = db.get_user(cookie.value()).await {
            let articles = db
                .get_articles_by_username(&user.username)
                .await
                .unwrap_or_default();

            return Html(
                AccountTemplate {
                    username: user.username,
                    author_name: user.author_name,
                    articles,
                }
                .render()
                .unwrap(),
            )
            .into_response();
        }
    }
    Redirect::to("/login").into_response()
}

pub async fn handle_update_author_name(
    State(db): State<Arc<db::Database>>,
    jar: CookieJar,
    Form(payload): Form<UpdateAuthorNamePayload>,
) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let username = cookie.value();
        if validate_input(&payload.author_name).is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match auth::update_author_name(&db, username, &payload.author_name).await {
            Ok(_) => Redirect::to("/account").into_response(),
            Err(_) => Redirect::to("/account").into_response(), // Simple error handling for now
        }
    } else {
        Redirect::to("/login").into_response()
    }
}

pub async fn create_article(
    State(db): State<Arc<db::Database>>,
    jar: CookieJar,
    mut multipart: Multipart,
) -> Response {
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

    let now = chrono::Local::now();
    let month_name = get_czech_month(now.month(), true);
    let formatted_date = format!("{}. {} {}", now.day(), month_name, now.year());

    // TODO
    if is_exclusive {
        // title = format!("<span class=\"red\">EXKLUZIVNĚ:</span> {}", title);
    }

    let template = ArticleTemplate {
        title: title.clone(),
        author: author.clone(),
        date: formatted_date.clone(),
        text: text_processed,
        image_path: image_path.clone(),
        image_description: image_description.clone(),
        video_path: video_path.clone(),
        category: category.clone(),
        category_display: category_display.clone(),
        related_snippets: related_snippets.clone(),
    };

    let html_content = template.render().unwrap();
    let safe_title = title
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            'á' => 'a',
            'č' => 'c',
            'ď' => 'd',
            'é' => 'e',
            'ě' => 'e',
            'í' => 'i',
            'ň' => 'n',
            'ó' => 'o',
            'ř' => 'r',
            'š' => 's',
            'ť' => 't',
            'ú' => 'u',
            'ů' => 'u',
            'ý' => 'y',
            'ž' => 'z',
            _ => '-',
        })
        .collect::<String>();
    let file_path = format!("{}.html", safe_title);
    fs::write(&file_path, html_content).unwrap();

    let month_name = get_czech_month(now.month(), false);
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

            let mut first_article_content = String::new();
            if let (Some(start), Some(end)) = (
                index_content.find("<!-- FIRST_ARTICLE -->"),
                index_content.find("<!-- /FIRST_ARTICLE -->"),
            ) {
                first_article_content = index_content[start + "<!-- FIRST_ARTICLE -->".len()..end]
                    .trim()
                    .to_string();
            }

            // 2. Prepare new MAIN_ARTICLE
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
                title,
                file_path,
                short_text_processed,
                file_path,
                image_path,
                image_description
            );

            // 3. Update index_content (from back to front to avoid index shifts if we were doing simple replaces,
            // but we'll use a more robust way since we have the full content)

            // Update SECOND_ARTICLE with old FIRST_ARTICLE
            if let (Some(start), Some(end)) = (
                index_content.find("<!-- SECOND_ARTICLE -->"),
                index_content.find("<!-- /SECOND_ARTICLE -->"),
            ) {
                let shifted_second = first_article_content
                    .replace("class=\"first-article\"", "class=\"second-article\"")
                    .replace("class='first-article'", "class='second-article'");

                index_content.replace_range(
                    start + "<!-- SECOND_ARTICLE -->".len()..end,
                    &format!("\n                {}\n                ", shifted_second),
                );
            }

            // Update FIRST_ARTICLE with old MAIN_ARTICLE
            if let (Some(start), Some(end)) = (
                index_content.find("<!-- FIRST_ARTICLE -->"),
                index_content.find("<!-- /FIRST_ARTICLE -->"),
            ) {
                let shifted_first = main_article_content
                    .replace("class=\"main-article-text\"", "class=\"first-article\"")
                    .replace("class='main-article-text'", "class='first-article'")
                    .replace("<h1>", "<h2>")
                    .replace("</h1>", "</h2>");

                // MAIN_ARTICLE might have <img> outside the div, we might need to handle that if we want it in FIRST_ARTICLE
                // But the requirement says "The previous article, which was rendered between MAIN_ARTICLE tags... render between FIRST_ARTICLE"
                // So we just take the whole content.

                index_content.replace_range(
                    start + "<!-- FIRST_ARTICLE -->".len()..end,
                    &format!("\n                {}\n                ", shifted_first),
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
                index_content.replace_range(
                    start + marker_start.len()..end,
                    &new_section_content,
                );
                fs::write("index.html", index_content).unwrap();
            }
        }
    }

    Redirect::to(&format!("/{}.html", safe_title)).into_response()
}

fn validate_input(input: &str) -> Result<(), &'static str> {
    for c in input.chars() {
        if c.is_ascii() {
            let val = c as u32;
            // Allow printable ASCII (32-126) and common whitespace (\n, \r, \t)
            if !(val >= 32 && val <= 126 || c == '\n' || c == '\r' || c == '\t') {
                return Err("Invalid character detected");
            }
        }
        // Non-ASCII (UTF-8) is allowed
    }
    Ok(())
}

fn validate_input_simple(input: &str) -> Result<(), &'static str> {
    for c in input.chars() {
        if !c.is_ascii_alphanumeric() {
            if c != '_' {
                return Err("Incorrect character detected");
            }
        }
    }
    Ok(())
}
