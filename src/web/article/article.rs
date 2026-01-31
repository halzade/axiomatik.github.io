use crate::application::article::form_article_data_parser::ArticleData;
use crate::data::audio_validator::{validate_audio_data, validate_audio_extension, AudioValidatorError};
use crate::db::{database_article, database_user};
use crate::library;
use crate::system::server::AUTH_COOKIE;
use crate::system::data_updates;
use crate::web::base::ArticleMostRead;
use crate::web::search::search::CategoryArticleTemplate;
use askama::Template;
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use chrono::{Datelike, Local};
use http::StatusCode;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArticleError {
    #[error("audio validation error {0}")]
    ArticleAudioArticleError(AudioValidatorError),

    #[error("undefined data type")]
    UndefinedAudioType,

    #[error("unsupported format {0}")]
    UnsupportedAudioType(String),

    #[error("detected empty audio file")]
    DetectedEmptyAudioFile,
}


#[derive(Template)]
#[template(path = "web/article/article_template.html")]
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
    pub related_articles: Vec<CategoryArticleTemplate>,
    pub weather: String,
    pub name_day: String,
    pub articles_most_read: Vec<ArticleMostRead>,
}

/**
 * This will process store the new Article and related files
 * But wont render any html
 */
pub async fn process_article_create(article_data: ArticleData) -> Result<String, ArticleError> {
    /*
     * Validate
     */

    // TODO Validate text fileds

    if article_data.has_audio {
        validate_audio_data(&article_data.audio_data)?;
        validate_audio_extension(&article_data.audio_data_ext)?;
    }
    if article_data.has_video {
        // validate_video_data(&article.video_data)?;
        // validate_video_extension(&article.video_data_ext)?;
    }

    /*
     * Prepare Article data
     */
    let article_template = article_template(&article_data);
    let article_db = article_db(&article_data);

    // process data image
    // process data audio
    // process data video

    /*
     * Index page
     */
    // invalidate index

    /*
     * category page
     */
    // invalidate category page

    /*
     * category page
     */
    // invalidate related articles


    let html_content = article_template.render().unwrap();

    // Store in DB

    // don't render anything
}

pub async fn render_article_create(article_url: String) -> Result<String, ArticleError> {

}


fn article_template(article_data: &ArticleData) -> ArticleTemplate {
    ArticleTemplate {
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
}

fn article_db(article_data: &ArticleData) -> database_article::Article {

    // TODO TryInto ?

    // TODO  database_article::Article  should be probably the same object as ArticleData

    database_article::Article {
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
    }
}