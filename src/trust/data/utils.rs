use crate::trust::data::media_data::BOUNDARY;
use axum::response::Response;
use std::string::ToString;

pub fn error(title: &str, exp: String, real: &str) -> String {
    format!("{}: \"{}\", was \"{}\"", title, exp, real)
}

pub fn content_type_with_boundary() -> String {
    format!("multipart/form-data; boundary={}", BOUNDARY)
}

pub async fn response_to_body(response: Response) -> String {
    let body_bytes = match axum::body::to_bytes(response.into_body(), usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Failed to read response body: {}", e);
            return String::new();
        }
    };
    String::from_utf8_lossy(&body_bytes).to_string()
}
