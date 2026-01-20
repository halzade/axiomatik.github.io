use crate::test_framework::article_builder::BOUNDARY;
use axum::body::Body;
use axum::extract::{multipart::Field, FromRequest, Multipart};
use http::Request;

pub async fn create_multipart_from_body(body: Vec<u8>) -> Multipart {
    let req = Request::builder()
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={}", BOUNDARY),
        )
        .body(Body::from(body))
        .unwrap();

    Multipart::from_request(req, &())
        .await
        .expect("Failed to create Multipart")
}

pub async fn get_first_field(multipart: &mut Multipart) -> Field<'_> {
    multipart
        .next_field()
        .await
        .expect("Failed to get next field")
        .expect("No field found")
}
