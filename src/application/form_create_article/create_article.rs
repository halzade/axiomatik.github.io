use crate::application::form_create_article::create_article_parser;
use crate::application::form_create_article::create_article_parser::ArticleCreateError;
use crate::application::page_article::article::ArticleError;
use crate::application::page_article::article::ArticleError::CategoryFailed;
use crate::data::audio_processor::AudioProcessorError;
use crate::data::image_processor::ImageProcessorError;
use crate::data::video_processor::VideoProcessorError;
use crate::data::{audio_processor, image_processor, video_processor};
use crate::db::database::SurrealError;
use crate::db::database_article_data::Article;
use crate::db::database_user::SurrealUserError;
use crate::system::router_app::AuthSession;
use crate::system::server::TheState;
use askama::Template;
use axum::extract::{Multipart, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use thiserror::Error;
use tracing::info;
use tracing::log::debug;

#[derive(Error, Debug)]
pub enum FormArticleCreateError {
    #[error("article error")]
    FormArticleError(#[from] ArticleError),

    #[error("article create error")]
    FormArticleCreateError(#[from] ArticleCreateError),

    #[error("image processor error")]
    ImageProcessorError(#[from] ImageProcessorError),

    #[error("audio processor error")]
    AudioProcessorError(#[from] AudioProcessorError),

    #[error("video processor error")]
    VideoProcessorError(#[from] VideoProcessorError),

    #[error("database error")]
    DatabaseError(#[from] SurrealError),

    #[error("surreal user error")]
    FormArticleSurrealUserError(#[from] SurrealUserError),

    #[error("render error")]
    FormArticleRenderError(#[from] askama::Error),
}

#[derive(Template)]
#[template(path = "application/form_create_article/create_article_template.html")]
pub struct FormTemplate {
    pub author_name: String,
}

pub async fn show_article_create_form(
    State(state): State<TheState>,
    auth_session: AuthSession,
) -> Result<Response, FormArticleCreateError> {
    match auth_session.user {
        None => {}
        Some(user) => {
            let user_o = state.dbu.get_user_by_name(&user.username).await?;
            match user_o {
                None => {}
                Some(user) => {
                    return Ok(Html(FormTemplate { author_name: user.author_name }.render()?)
                        .into_response());
                }
            }
        }
    }
    Ok(Redirect::to("/login").into_response())
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
     * store Article data
     */
    state.dba.create_article(article_db).await?;

    /*
     * don't render anything
     * redirect to the new article
     * web router manages render trigger
     */
    Ok(Redirect::to("/account").into_response())
}
