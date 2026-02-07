use axum_core::response::Response;

pub struct ResponseVerifier{
    response: Response
}


// TODO use methods and delete this
impl ResponseVerifier {
    pub fn new(response: Response) -> Self {

        let (parts, body) = response.into_parts();
        headers: parts.headers
        Self {
            response
        }
    }

    pub fn header_location(self, location: &str) -> ResponseVerifier {
        assert_eq!(self.headers.get(http::header::LOCATION).unwrap(), location);
        self
    }

    pub fn header_cookie(self, properties: &[&str]) -> ResponseVerifier {
        let mut cookies = self
            .headers
            .get_all(http::header::SET_COOKIE)
            .iter()
            .map(|v| v.to_str().expect("Invalid Set-Cookie header"));

        let found = cookies.any(|cookie| properties.iter().all(|prop| cookie.contains(prop)));

        assert!(found, "No Set-Cookie header contained all required properties: {:?}", properties);

        self
    }
}
