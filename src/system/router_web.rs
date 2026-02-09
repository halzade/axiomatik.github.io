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
use crate::system::data_updates::DataUpdatesError;
use crate::system::server::TheState;
use axum::body::Body;
use axum::extract::{OriginalUri, State};
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
use crate::db::database_system::ArticleStatus;

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

        /*
         * Unprotected routes
         */
        let ret = Router::new()
            .route("/search", get(search::handle_search))
            // serve static directories (nest service)
            .nest_service("/css", ServeDir::new("web/css"))
            .nest_service("/js", ServeDir::new("web/js"))
            .nest_service("/u", ServeDir::new("web/u"))
            // serve static files (route service)
            .route_service("/favicon.ico", ServeFile::new("web/favicon.ico"))
            /*
             * catch web requests and maybe updat invalid HTML file
             * redirect the request to the web directory
             */
            .fallback(get(Self::serve_static_content))
            .with_state(self.state.clone());

        info!("start_router() finished");
        ret
    }

    /**
     * serve HTML file requests
     * - serve a static HTML file if valid
     * - rebuild the HTML file if invalid
     */
    pub async fn serve_static_content(
        state: State<TheState>,
        ori_uri: OriginalUri,
        request: Request<Body>,
    ) -> Result<Response, WebRouterError> {
        // this is an HTML file request, HTML content may need refresh
        info!("serve_static_content()");

        /*
         * request url
         */
        let url = ori_uri.path().to_string();
        info!("url: {}", url);

        match url.as_str() {
            "/index.html" => {
                if !state.dv.index_valid() {
                    state.dv.index_validate();

                    index::render_index(&state).await?;
                }
                serve_this(url, request).await
            }
            "/finance.html" => {
                if !state.dv.finance_valid() {
                    state.dv.finance_validate();

                    finance::render_finance(&state).await?;
                }
                serve_this(url, request).await
            }
            "/news.html" => {
                if !state.dv.news_valid() {
                    state.dv.news_validate();

                    news::render_news(&state).await?;
                }
                serve_this(url, request).await
            }
            "/republika.html" => {
                if !state.dv.republika_valid() {
                    state.dv.republika_validate();

                    republika::render_republika(&state).await?;
                }
                serve_this(url, request).await
            }
            "/technologie.html" => {
                if !state.dv.technologie_valid() {
                    state.dv.technologie_validate();

                    technologie::render_technologie(&state).await?;
                }
                serve_this(url, request).await
            }
            "/veda.html" => {
                if !state.dv.veda_valid() {
                    state.dv.veda_validate();

                    veda::render_veda(&state).await?;
                }
                serve_this(url, request).await
            }
            "/zahranici.html" => {
                if !state.dv.zahranici_valid() {
                    state.dv.zahranici_validate();

                    zahranici::render_zahranici(&state).await?;
                }
                serve_this(url, request).await
            }
            _ => {
                // 404 or Article
                match state.dv.article_valid(&url) {
                    ArticleStatus::Valid => {
                        // no change, serve the file
                        serve_this(url, request).await
                    }
                    ArticleStatus::Invalid => {
                        // article was invalidated, render article HTML
                        // new article was u
                        article::render_article(&url, &state).await?;
                        state.dv.article_validate(&url);
                        serve_this(url, request).await
                    }
                    ArticleStatus::DoesNotExist => {
                        // requested url doesn't exist
                        serve_404().await
                    }
                }
            }
        }
    }
}

async fn serve_this(path: String, request: Request<Body>) -> Result<Response, WebRouterError> {
    info!("serve_this: web/{}", path);
    Ok(ServeFile::new(format!("web/{}", path)).oneshot(request).await?.into_response())
}

async fn serve_404() -> Result<Response, WebRouterError> {
    warn!("web router fallback");
    Ok((StatusCode::NOT_FOUND, Html("404, stránka nenalezená".to_string())).into_response())
}
