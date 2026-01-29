use axum::extract::OriginalUri;
use axum_core::response::IntoResponse;
use tracing::debug;

pub async fn serve_html_content(uri: OriginalUri) -> impl IntoResponse {
    let requested_url = uri.0.path();
    debug!("requested_url {}", requested_url);

    // TODO
}
