use crate::application::article::article::ArticleError::{ArticleNotFound, RenderArticleError};
use crate::application::form::form_article_data_parser;
use crate::application::form::form_article_data_parser::ArticleCreateError;
use crate::data::audio_processor::AudioProcessorError;
use crate::data::image_processor::ImageProcessorError;
use crate::data::video_processor::VideoProcessorError;
use crate::data::{audio_processor, image_processor, processor, video_processor};
use crate::db::database::SurrealError;
use crate::db::database_article;
use crate::db::database_article_data::{MiniArticleData, NewArticle, ShortArticleData};
use crate::system::data_system::DataSystem;
use crate::system::data_updates::DataUpdates;
use crate::system::router::AuthSession;
use askama::Template;
use axum::extract::Multipart;
use axum::response::{IntoResponse, Redirect};
use std::sync::Arc;
use thiserror::Error;
use ArticleError::CategoryFailed;

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
}

#[derive(Template)]
#[template(path = "application/article/article_template.html")]
pub struct ArticleTemplate {
    pub date: String,
    pub weather: String,
    pub name_day: String,

    pub author: String,

    pub title: String,
    pub text: String,

    pub image_path: String,
    pub image_desc: String,
    pub video_path: Option<String>,
    pub audio_path: Option<String>,

    pub category: String,
    pub category_display: String,

    pub related_articles: Vec<ShortArticleData>,
    pub articles_most_read: Vec<MiniArticleData>,
}

pub async fn create_article(
    data_updates: Arc<DataUpdates>,
    auth_session: AuthSession,
    multipart: Multipart,
) -> Result<impl IntoResponse, ArticleError> {
    // TODO X doubled request on create button

    /*
     * Read request data
     */
    let article_data = form_article_data_parser::article_data(auth_session, multipart).await?;
    let article_url = format!("{}.html", article_data.base_file_name.clone());

    /*
     * Validate
     */

    // TODO X Validate text fields, use validator framework instead

    let article_db = NewArticle::try_from(article_data.clone())?;

    // process data image
    image_processor::process_images(
        &article_data.image_data,
        &article_data.base_file_name,
        &article_data.image_ext,
    )?;

    // process data audio
    if article_data.has_audio {
        // validate_audio_data(&article_data.audio_data)?;
        // validate_audio_extension(&article_data.audio_ext)?;
        audio_processor::process_valid_audio(
            &article_data.audio_data,
            &format!("{}.{}", article_data.base_file_name, article_data.audio_ext),
        )?;
    }

    // process data video
    if article_data.has_video {
        // validate_video_data(&article.video_data)?;
        // validate_video_extension(&article.video_data_ext)?;

        video_processor::process_video(
            &article_data.video_data,
            &format!("{}.{}", article_data.base_file_name, article_data.video_ext),
        )?;
    }

    // invalidate cache
    data_updates.index_invalidate();
    data_updates.news_invalidate();

    match article_data.category.as_str() {
        "zahranici" => data_updates.zahranici_invalidate(),
        "republika" => data_updates.republika_invalidate(),
        "finance" => data_updates.finance_invalidate(),
        "technologie" => data_updates.technologie_invalidate(),
        "veda" => data_updates.veda_invalidate(),
        cat => return Err(CategoryFailed(cat.into())),
    }

    /*
     * Store Article data
     */
    database_article::create_article(article_db).await?;

    /*
     * don't render anything
     * redirect to new article
     * trigger to render from template will be handled by router
     */
    Ok(Redirect::to(&article_url).into_response())
}

/**
 * This will process and store the new Article and related files
 * But wont render any html
 */
pub async fn render_article(
    article_file_name: &str,
    data_system: &DataSystem,
) -> Result<(), ArticleError> {
    let article_o = database_article::article_by_file_name(article_file_name).await?;

    match article_o {
        None => Err(ArticleNotFound),
        Some(article) => {
            let related_articles =
                database_article::related_articles(&article.data.related_articles).await?;
            let articles_most_read = database_article::articles_most_read(3).await?;

            let article_template = ArticleTemplate {
                date: data_system.date(),
                weather: data_system.weather(),
                name_day: data_system.name_day(),

                author: article.data.author,
                title: article.data.title,

                text: article.data.text,

                image_path: article.data.image_820_path,
                image_desc: article.data.image_desc,
                video_path: if article.data.has_video {
                    Some(article.data.video_path)
                } else {
                    None
                },
                audio_path: if article.data.has_audio {
                    Some(article.data.audio_path)
                } else {
                    None
                },
                category: article.data.category.clone(),
                category_display: processor::process_category(article.data.category.as_str())?,
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
    }
}
