use axum_core::response::Response;
use http::HeaderMap;

pub struct ResponseVerifier {
    pub headers: HeaderMap,
    pub response: Response,
}

impl ResponseVerifier {
    pub fn new(response: Response) -> Self {
        let headers = response.headers().clone();
        Self {
            headers,
            response,
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
