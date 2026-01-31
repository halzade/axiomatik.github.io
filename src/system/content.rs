use crate::system::content::ContentError::{InvalidFileName, Io};
use crate::system::system_data::ApplicationLastUpdatesData;
use axum::extract::OriginalUri;
use axum_core::response::IntoResponse;
use http::{Response, StatusCode};
use reqwest::Body;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use askama::Template;
use thiserror::Error;
use tracing::debug;
use ContentError::InvalidPath;
use crate::db::database_article;
use crate::db::database_article::Article;
use crate::library;
use crate::library::CategoryEnum;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("Invalid path: {0}")]
    InvalidPath(&'static str),

    #[error("File not found: {0}")]
    NotFound(String),

    #[error("invalid filename for template {0}")]
    InvalidFileName(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub async fn serve_html_content(
    uri: OriginalUri,
    last_updates_data: &ApplicationLastUpdatesData,
) -> Result<impl IntoResponse, ContentError> {

    // process request URL
    let file_path = safe_file_path(uri)?;

    /*
     * Process the request
     */
    let update_it = last_updates_data.article_update(&file_path);

    if update_it {
        debug!("Updating html content");
        let file_name_base = file_name_base(&file_path);
        let category_enum = CategoryEnum::from_str(&file_name_base)?;
        // let article_data_o = database_article::article_by_file_name(&file_name_base).await?;

        match category_enum {
            CategoryEnum::Index => {
                form_index::render_index().await;
            }
            CategoryEnum::News => {
                form_category::render_template().await;
            }
            CategoryEnum::Finance => {}
            CategoryEnum::Republika => {}
            CategoryEnum::Technologie => {}
            CategoryEnum::Veda => {}
            CategoryEnum::Zahranici => {}
        }
        last_updates_data.article_update_now(&file_path)
    }

    /*
     * serve the content
     */
    let content = read_file_content(&file_path).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(content))
        .unwrap())
}

fn file_name_base(path_buf: &PathBuf) -> Result<impl Template, ContentError> {
    let file_name = path_buf.file_name()
        .ok_or(InvalidPath("No file name"))?;
    let file_name_str = file_name.to_str()
        .ok_or(InvalidPath("Invalid UTF-8 in file name"))?;

    file_name_str.strip_suffix(".html")
        .ok_or(InvalidPath("No .html suffix"))?
        .to_string();
}
match file_name_base.as_str() {
        FINANCE_STR => Ok(form_category::FinanceTemplate),
        INDEX_STR => Ok(form_index::IndexTemplate),
        NEWS_STR => Ok(form_index::NewsTemplate),
        REPUBLIKA_STR => Ok(form_category::RepublikaTemplate),
        TECHNOLOGIE_STR => Ok(form_category::TechnologieTemplate),
        VEDA_STR => Ok(form_category::VedaTemplate),
        ZAHRANICI_STR => Ok(form_category::ZahraniciTemplate),
        _ => Err(InvalidFileName(file_name_base)),
    }
}

fn safe_file_path(uri: OriginalUri) -> Result<PathBuf, ContentError> {
    let requested_url = uri.0.path();
    debug!("requested_url {}", requested_url);

    let path = requested_url.trim_start_matches('/');
    if !path.ends_with(".html") {
        return Err(InvalidPath(requested_url.to_string()));
    }
    Ok(PathBuf::from("web").join(path))
}

async fn read_file_content(file_path: &PathBuf) -> Result<String, ContentError> {
    match fs::read(file_path) {
        Ok(bytes) => Ok(String::from_utf8(bytes)?),
        Err(error) => Err(Io(error)),
    }
}
