use crate::application::form_create_article::create_article_parser::ArticleCreateError;
use crate::application::page_article::article::ArticleError::RenderArticleError;
use crate::data::audio_processor::AudioProcessorError;
use crate::data::image_processor::ImageProcessorError;
use crate::data::processor;
use crate::data::video_processor::VideoProcessorError;
use crate::db::database::SurrealError;
use crate::db::database_article::SurrealArticleError;
use crate::db::database_article_data::{MiniArticleData, ShortArticleData};
use crate::db::database_system::SurrealSystemError;
use crate::system::server::TheState;
use askama::Template;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArticleError {
    #[error("category unknown {0}")]
    CategoryFailed(String),

    #[error("article creation failed: {0}")]
    ArticleCreate(#[from] ArticleCreateError),

    #[error("image processing failed: {0}")]
    ImageProcessor(#[from] ImageProcessorError),

    #[error("audio processing failed: {0}")]
    AudioProcessor(#[from] AudioProcessorError),

    #[error("video processing failed: {0}")]
    VideoProcessor(#[from] VideoProcessorError),

    #[error("database error: {0}")]
    DatabaseError(#[from] SurrealError),

    #[error("processor error: {0}")]
    ProcessorError(#[from] processor::ProcessorError),

    #[error("failed to create article in db")]
    ArticleCreationInDbFailed,

    #[error("failed to render article")]
    RenderArticleError,

    #[error("failed not found")]
    ArticleNotFound,

    #[error("surreal article error {0}")]
    SurrealArticle(#[from] SurrealArticleError),

    #[error("surreal system error {0}")]
    SurrealSystem(#[from] SurrealSystemError),
}

#[derive(Template)]
#[template(path = "application/page_article/article_template.html")]
pub struct ArticleTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub author: String,

    pub title: String,
    pub text: String,

    pub image_820_path: String,
    pub image_desc: String,
    pub video_path: Option<String>,
    pub audio_path: Option<String>,

    pub category: String,
    pub category_display: String,

    pub related_articles: Vec<ShortArticleData>,
    pub articles_most_read: Vec<MiniArticleData>,
}

/**
 * This will process and store the new Article and related files
 * But won't render any HTML
 */
pub async fn render_article(article_file_name: &str, state: &TheState) -> Result<(), ArticleError> {
    let article = state.dba.article_by_file_name(article_file_name).await?;

    let related_articles = state.dba.related_articles(article.related_articles).await?;

    let category = article.category.clone();
    let articles_most_read = state.dba.most_read_in_category_by_views(&category).await?;

    let article_template = ArticleTemplate {
        date: state.ds.date(),
        weather: state.ds.weather(),
        name_day: state.ds.name_day(),

        author: article.author,
        title: article.title,

        text: article.text,

        image_820_path: article.image_820_path,
        image_desc: article.image_desc,
        video_path: if article.has_video { Some(article.video_path) } else { None },
        audio_path: if article.has_audio { Some(article.audio_path) } else { None },
        category: article.category.clone(),
        category_display: processor::process_category(article.category.as_str()),
        related_articles,
        articles_most_read,
    };
    match article_template.render() {
        Ok(rendered_html) => {
            processor::save_web_file(rendered_html, article_file_name)?;
            Ok(())
        }
        Err(_) => Err(RenderArticleError),
    }
}
