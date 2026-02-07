use crate::trust::data::media_data::BOUNDARY;
use axum::response::Response;
use std::string::ToString;
use url::form_urlencoded::Serializer;

pub fn error(title: &str, exp: &str, real: String) -> String {
    format!("{}: expected \"{}\", got \"{}\"", title, exp, real)
}

pub fn serialize(params: &[(&str, &str)]) -> String {
    let mut serializer = Serializer::new(String::new());
    serializer.extend_pairs(params);
    serializer.finish()
}

pub fn content_type_with_boundary() -> String {
    format!("multipart/form-data; boundary={}", BOUNDARY)
}

pub async fn response_to_body(response: Response) -> String {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await;
    let body_str = String::from_utf8_lossy(&body_bytes.unwrap()).to_string();
    body_str
}
