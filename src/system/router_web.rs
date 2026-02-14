use crate::application::category_finance::finance;
use crate::application::category_finance::finance::FinanceError;
use crate::application::category_republika::republika;
use crate::application::category_republika::republika::RepublikaError;
use crate::application::category_technologie::technologie;
use crate::application::category_technologie::technologie::TechnologieError;
use crate::application::category_veda::veda;
use crate::application::category_veda::veda::VedaError;
use crate::application::category_zahranici::zahranici;
use crate::application::category_zahranici::zahranici::ZahraniciError;
use crate::application::page_all_news::all_news;
use crate::application::page_all_news::all_news::NewsError;
use crate::application::page_article::article;
use crate::application::page_article::article::ArticleError;
use crate::application::page_index::index;
use crate::application::page_index::index::IndexError;
use crate::application::page_search::search;
use crate::db::database_system::{ArticleStatus, SurrealSystemError};
use crate::system::data_system::DataSystemError;
use crate::system::data_updates::DataUpdatesError;
use crate::system::server::TheState;
use axum::body::Body;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use axum_core::extract::Request;
use http::StatusCode;
use thiserror::Error;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};
use tracing::log::debug;
use tracing::{info, trace, warn};

#[derive(Debug, Error)]
pub enum WebRouterError {
    #[error("data update error: {0}")]
    RouterDataUpdate(#[from] DataUpdatesError),

    #[error("data update system: {0}")]
    RouterDataSystem(#[from] DataSystemError),

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

    #[error("surreal system error: {0}")]
    SurrealSystem(#[from] SurrealSystemError),
}

pub struct WebRouter {
    state: TheState,
}

impl WebRouter {
    pub const fn init(state: TheState) -> Result<Self, WebRouterError> {
        Ok(Self { state })
    }
}

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
            .nest_service("/image", ServeDir::new("web/image"))
            .nest_service("/css", ServeDir::new("web/css"))
            .nest_service("/js", ServeDir::new("web/js"))
            .nest_service("/u", ServeDir::new("web/u"))
            // serve static files (route service)
            .route_service("/favicon.ico", ServeFile::new("web/favicon.ico"))
            .route("/ping", get("{\"message\": \"web ping\"}"))
            /*
             * catch web requests and maybe update an invalid HTML file
             * redirect the request to the web directory
             */
            .fallback(Self::serve_static_content)
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
        request: Request<Body>,
    ) -> Result<Response, WebRouterError> {
        // this is an HTML file request, HTML content may need refresh
        debug!("serve_static_content()");

        let uri = request.uri().clone();
        let url = uri.path().to_string();

        debug!("url: {}", url);

        match url.as_str() {
            "/index.html" => {
                if !state.dv.index_valid() {
                    state.dv.index_validate();

                    index::render_index(&state).await?;
                }
                serve_this(&url, request).await
            }
            "/" => {
                if !state.dv.index_valid() {
                    state.dv.index_validate();

                    index::render_index(&state).await?;
                }
                serve_this("/index.html", request).await
            }
            "/finance.html" => {
                if !state.dv.finance_valid() {
                    state.dv.finance_validate();

                    finance::render_finance(&state).await?;
                }
                serve_this(&url, request).await
            }
            "/news.html" => {
                if !state.dv.news_valid() {
                    state.dv.news_validate();

                    all_news::render_news(&state).await?;
                }
                serve_this(&url, request).await
            }
            "/republika.html" => {
                if !state.dv.republika_valid() {
                    state.dv.republika_validate();

                    republika::render_republika(&state).await?;
                }
                serve_this(&url, request).await
            }
            "/technologie.html" => {
                if !state.dv.technologie_valid() {
                    state.dv.technologie_validate();

                    technologie::render_technologie(&state).await?;
                }
                serve_this(&url, request).await
            }
            "/veda.html" => {
                if !state.dv.veda_valid() {
                    state.dv.veda_validate();

                    veda::render_veda(&state).await?;
                }
                serve_this(&url, request).await
            }
            "/zahranici.html" => {
                if !state.dv.zahranici_valid() {
                    state.dv.zahranici_validate();

                    zahranici::render_zahranici(&state).await?;
                }
                serve_this(&url, request).await
            }
            _ => {
                // remove the leading slash
                let real_article_name = real_filename(&url);

                // 404 or Article
                match state.dbs.read_article_validity(real_article_name).await? {
                    ArticleStatus::Valid => {
                        debug!("Article valid");

                        // count views
                        state.dba.increase_article_views(real_article_name.to_string()).await?;

                        // no change, serve the file
                        serve_this(&format!("/{}", real_article_name), request).await
                    }
                    ArticleStatus::Invalid => {
                        debug!("Article invalid");

                        // count views
                        state.dba.increase_article_views(real_article_name.to_string()).await?;

                        // article was invalidated, render article HTML
                        article::render_article(real_article_name, &state).await?;
                        state.dbs.validate_article(real_article_name.to_string()).await?;
                        serve_this(&format!("/{}", real_article_name), request).await
                    }
                    ArticleStatus::DoesNotExist => {
                        debug!("Article doesn't exist, give 404");
                        // requested url doesn't exist
                        serve_404().await
                    }
                }
            }
        }
    }
}

fn real_filename(article_file_name: &str) -> &str {
    article_file_name.strip_prefix('/').unwrap_or(article_file_name)
}

async fn serve_this(path: &str, request: Request<Body>) -> Result<Response, WebRouterError> {
    // path already begins with /
    trace!("serve_this: web{}", path);
    let sf_r = ServeFile::new(format!("web{}", path)).oneshot(request).await;
    match sf_r {
        Ok(sf) => Ok(sf.into_response()),
        Err(e) => match e {},
    }
}

async fn serve_404() -> Result<Response, WebRouterError> {
    warn!("web router fallback");
    Ok((StatusCode::NOT_FOUND, Html("404; stránka nenalezena".to_string())).into_response())
}
