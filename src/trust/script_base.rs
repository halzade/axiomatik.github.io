use std::string::ToString;

use crate::trust::article_builder::BOUNDARY;

pub fn serialize(params: &[(&str, &str)]) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer.extend_pairs(params);
    serializer.finish()
}

pub fn content_type_with_boundary() -> String {
    format!("multipart/form-data; boundary={}", BOUNDARY)
}

pub async fn response_to_body(response: axum::response::Response) -> String {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await;
    let body_str = String::from_utf8_lossy(&body_bytes.unwrap()).to_string();
    body_str
}
