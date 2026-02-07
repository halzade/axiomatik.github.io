use crate::application::article::article;
use crate::application::article::article::ArticleError;
use crate::application::finance::finance;
use crate::application::finance::finance::FinanceError;
use crate::application::index::index;
use crate::application::index::index::IndexError;
use crate::application::news::news;
use crate::application::news::news::NewsError;
use crate::application::republika::republika;
use crate::application::republika::republika::RepublikaError;
use crate::application::search::search;
use crate::application::technologie::technologie;
use crate::application::technologie::technologie::TechnologieError;
use crate::application::veda::veda;
use crate::application::veda::veda::VedaError;
use crate::application::zahranici::zahranici;
use crate::application::zahranici::zahranici::ZahraniciError;
use crate::system::data_system::DataSystemError;
use crate::system::data_updates::{ArticleStatus, DataUpdatesError};
use crate::system::server::TheState;
use axum::body::Body;
use axum::extract::OriginalUri;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use axum_core::extract::Request;
use http::StatusCode;
use std::convert::Infallible;
use thiserror::Error;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum WebRouterError {
    #[error("data update error: {0}")]
    RouterDataUpdate(#[from] DataUpdatesError),

    #[error("data update system: {0}")]
    RouterDataSystem(#[from] DataSystemError),

    #[error("response infallible: {0}")]
    RouterInfallible(#[from] Infallible),

    #[error("index error: {0}")]
    RouterIndexError(#[from] IndexError),

    #[error("finance error: {0}")]
    RouterFinanceError(#[from] FinanceError),

    #[error("news error: {0}")]
    RouterNewsError(#[from] NewsError),

    #[error("republika error: {0}")]
    RouterRepublikaError(#[from] RepublikaError),

    #[error("technologie error: {0}")]
    RouterTechnologieError(#[from] TechnologieError),

    #[error("veda error: {0}")]
    RouterVedaError(#[from] VedaError),

    #[error("zahranici error: {0}")]
    RouterZahraniciError(#[from] ZahraniciError),

    #[error("article error: {0}")]
    RouterArticleError(#[from] ArticleError),
}

pub struct WebRouter {
    state: TheState,
}

impl WebRouter {
    pub fn init(state: TheState) -> Result<WebRouter, WebRouterError> {
        Ok(WebRouter { state })
    }
}

// TODO macro derive these things
impl IntoResponse for WebRouterError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl WebRouter {
    #[rustfmt::skip]
    pub async fn start_web_router(&self) -> Router {
        info!("start_web_router()");

        let self_a2 = self.clone();

        /*
         * Unprotected routes
         */
        let ret = Router::new()
            .route("/search", get(search::handle_search))
            // serve static files
            .nest_service("/web/", ServeDir::new("/web/"))
            /*
             * catch web requests and maybe updat invalid HTML file
             * redirect the request to the web directory
             */
            .route("/", get(move |ori_uri: OriginalUri, request|
                async move {
                    self_a2.serve_static_content(ori_uri, request).await
                }
            ))
            // everything already served, user requested for non-existent content
            .fallback(show_404)
            .with_state(self.state);

        info!("start_router() finished");
        ret
    }

    /**
     * serve all static files for web
     * 1. HTML file requests
     * - serve a static HTML file if valid
     * - rebuild the HTML file if invalid
     * 2. image, video, CSS, js requests
     * - serve static files
     */
    pub async fn serve_static_content(
        &self,
        ori_uri: OriginalUri,
        request: Request<Body>,
    ) -> Result<Response, WebRouterError> {
        /*
         * request url
         */
        let url = ori_uri.path().to_string();

        /*
         * What kind of content is it?
         */
        if url.starts_with("/css/")
            || url.starts_with("/js/")
            || url.starts_with("/u/")
            || url == "/favicon.ico"
        {
            return serve_this(url, request).await;
        }
        // it is an HTML file request, HTML content may need refresh

        match url.as_str() {
            "/index.html" => {
                if !self.state.dv.index_valid() {
                    self.state.dv.index_validate();

                    index::render_index(&self.state).await?;
                }
                serve_this(url, request).await
            }
            "/finance.html" => {
                if !self.state.dv.finance_valid() {
                    self.state.dv.finance_validate();

                    finance::render_finance(&self.state).await?;
                }
                serve_this(url, request).await
            }
            "/news.html" => {
                if !self.state.dv.news_valid() {
                    self.state.dv.news_validate();

                    news::render_news(&self.state).await?;
                }
                serve_this(url, request).await
            }
            "/republika.html" => {
                if !self.state.dv.republika_valid() {
                    self.state.dv.republika_validate();

                    republika::render_republika(&self.state).await?;
                }
                serve_this(url, request).await
            }
            "/technologie.html" => {
                if !self.state.dv.technologie_valid() {
                    self.state.dv.technologie_validate();

                    technologie::render_technologie(&self.state).await?;
                }
                serve_this(url, request).await
            }
            "/veda.html" => {
                if !self.state.dv.veda_valid() {
                    self.state.dv.veda_validate();

                    veda::render_veda(&self.state).await?;
                }
                serve_this(url, request).await
            }
            "/zahranici.html" => {
                if !self.state.dv.zahranici_valid() {
                    self.state.dv.zahranici_validate();

                    zahranici::render_zahranici(&self.state).await?;
                }
                serve_this(url, request).await
            }
            _ => {
                // 404 or Article
                match self.state.dv.article_valid(&url) {
                    ArticleStatus::Valid => {
                        // no change, serve the file
                        serve_this(url, request).await
                    }
                    ArticleStatus::Invalid => {
                        // article was invalidated, render article HTML
                        // new article was u
                        article::render_article(&url, &self.state).await?;
                        self.state.dv.article_validate(&url);
                        serve_this(url, request).await
                    }
                    ArticleStatus::DoesntExist => {
                        // requested url doesn't exist
                        serve_404().await
                    }
                }
            }
        }
    }
}

async fn serve_this(path: String, request: Request<Body>) -> Result<Response, WebRouterError> {
    Ok(ServeFile::new(format!("web/{}", path)).oneshot(request).await?.into_response())
}

async fn serve_404() -> Result<Response, WebRouterError> {
    Ok((StatusCode::NOT_FOUND, Html("404, stránka nenalezená".to_string())).into_response())
}

async fn show_404() -> impl IntoResponse {
    warn!("router fallback");
    (StatusCode::NOT_FOUND, Html("404, stránka nenalezená".to_string()))
}
