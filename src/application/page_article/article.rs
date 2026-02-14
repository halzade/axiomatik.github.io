use crate::application::form_create_article::create_article_parser;
use crate::application::form_create_article::create_article_parser::ArticleCreateError;
use crate::application::page_article::article::ArticleError::RenderArticleError;
use crate::data::audio_processor::AudioProcessorError;
use crate::data::image_processor::ImageProcessorError;
use crate::data::video_processor::VideoProcessorError;
use crate::data::{audio_processor, image_processor, processor, video_processor};
use crate::db::database::SurrealError;
use crate::db::database_article::SurrealArticleError;
use crate::db::database_article_data::{Article, MiniArticleData, ShortArticleData};
use crate::db::database_system::SurrealSystemError;
use crate::system::router_app::AuthSession;
use crate::system::server::TheState;
use askama::Template;
use axum::extract::Multipart;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use thiserror::Error;
use tracing::info;
use tracing::log::debug;
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

pub async fn create_article(
    State(state): State<TheState>,
    auth_session: AuthSession,
    multipart: Multipart,
) -> Result<impl IntoResponse, ArticleError> {
    // TODO XX doubled request on create button

    /*
     * Read request data
     */
    let article_data = create_article_parser::article_data(auth_session, multipart).await?;
    let article_file_name = format!("{}.html", article_data.base_file_name.clone());

    /*
     * Validate
     */

    // TODO XX Validate text fields, use validator framework instead

    let article_db = Article::try_from(article_data.clone())?;

    info!("is main {}", article_db.is_main);
    info!("is excl {}", article_db.is_exclusive);
    info!("file name {}", article_file_name.clone());

    debug!("process images");
    // process data image
    image_processor::process_images(
        &article_data.image_data,
        &article_data.base_file_name,
        &article_data.image_ext,
    )?;
    debug!("process images done");

    // process data audio
    if article_data.has_audio {
        debug!("process audio");
        // validate_audio_data(&article_data.audio_data)?;
        // validate_audio_extension(&article_data.audio_ext)?;
        audio_processor::process_valid_audio(
            &article_data.audio_data,
            &format!("{}.{}", article_data.base_file_name, article_data.audio_ext),
        )?;
        debug!("process audio done");
    }

    // process data video
    if article_data.has_video {
        debug!("process video");
        // validate_video_data(&article.video_data)?;
        // validate_video_extension(&article.video_data_ext)?;

        video_processor::process_video(
            &article_data.video_data,
            &format!("{}.{}", article_data.base_file_name, article_data.video_ext),
        )?;
        debug!("process video done");
    }

    // create article record
    debug!("create db record");
    state.dbs.create_article_record(article_file_name.clone()).await?;
    debug!("article record created: {}", article_file_name);

    // invalidate cache
    state.dv.index_invalidate();
    state.dv.news_invalidate();

    // invalidate related articles
    for related_article in &article_data.related_articles {
        info!("invalidate related article {}", related_article);
        state.dbs.invalidate_article(related_article.clone()).await?;

        // add bidirectional relationship
        state.dba.add_related_article(related_article.clone(), article_file_name.clone()).await?;
    }

    match article_data.category.as_str() {
        "zahranici" => state.dv.zahranici_invalidate(),
        "republika" => state.dv.republika_invalidate(),
        "finance" => state.dv.finance_invalidate(),
        "technologie" => state.dv.technologie_invalidate(),
        "veda" => state.dv.veda_invalidate(),
        cat => return Err(CategoryFailed(cat.into())),
    }

    /*
     * Store Article data
     */
    state.dba.create_article(article_db).await?;

    /*
     * don't render anything
     * redirect to the new article
     * trigger to render from template will be handled by router
     */
    Ok(Redirect::to(&absolute_web_path(state, &article_file_name)).into_response())
}

pub fn absolute_web_path(state: TheState, relative_path: &str) -> String {
    format!("https://{}:{}/{}", state.config.host_hame, state.config.port.web, relative_path)
}

/**
 * This will process and store the new Article and related files
 * But won't render any HTML
 */
pub async fn render_article(article_file_name: &str, state: &TheState) -> Result<(), ArticleError> {
    let article = state.dba.article_by_file_name(article_file_name).await?;

    let related_articles = state.dba.related_articles(article.related_articles).await?;
    let articles_most_read = state.dba.most_read_by_views().await?;

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
        category_display: processor::process_category(article.category.as_str())?,
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
