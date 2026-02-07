use crate::trust::me::TrustError;
use axum::body::Bytes;
use axum_core::response::Response;
use http::{HeaderMap, StatusCode};

pub struct ResponseVerifier {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

impl ResponseVerifier {
    pub fn new(body: Vec<u8>) -> Self {
        Self { status: StatusCode::OK, headers: HeaderMap::new(), body: Bytes::from(body) }
    }

    pub async fn new_from_response(response: Response) -> Result<Self, TrustError> {
        let (parts, body) = response.into_parts();
        let body_bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .map_err(|e| TrustError::AxumError(e.to_string()))?;

        Ok(Self { status: parts.status, headers: parts.headers, body: body_bytes })
    }

    pub fn must_see_response(self, status: StatusCode) -> ResponseVerifier {
        assert_eq!(self.status, status);
        self
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

    pub fn verify(&self) {
        // TODO
        // basic verification already done in methods
    }
}

// TODO probably delete
#[cfg(test)]
mod tests {
    use super::*;
    use http::{header, HeaderMap};

    // TODO
    fn verifier_with_cookies(cookies: &[&str]) -> ResponseVerifier {
        let mut headers = HeaderMap::new();
        for cookie in cookies {
            headers.append(header::SET_COOKIE, cookie.parse().unwrap());
        }

        // TODO
        ResponseVerifier { status: Default::default(), headers, body: Default::default() }
    }

    #[test]
    fn header_cookie_passes_when_cookie_contains_all_properties() {
        let verifier =
            verifier_with_cookies(&["session=abc123; HttpOnly; Secure; SameSite=Strict; Path=/"]);

        verifier.header_cookie(&["HttpOnly", "Secure", "SameSite=Strict", "Path=/"]);
    }

    #[test]
    fn header_cookie_passes_when_one_of_multiple_cookies_matches() {
        let verifier = verifier_with_cookies(&[
            "other=foo; Path=/",
            "session=abc123; HttpOnly; Secure; SameSite=Strict; Path=/",
        ]);

        verifier.header_cookie(&["HttpOnly", "Secure", "SameSite=Strict", "Path=/"]);
    }

    #[test]
    #[should_panic(expected = "No Set-Cookie header contained all required properties")]
    fn header_cookie_fails_when_property_missing() {
        let verifier = verifier_with_cookies(&["session=abc123; HttpOnly; Secure; Path=/"]);

        verifier.header_cookie(&[
            "HttpOnly",
            "Secure",
            "SameSite=Strict", // missing
        ]);
    }

    #[test]
    #[should_panic(expected = "No Set-Cookie header contained all required properties")]
    fn header_cookie_fails_when_no_set_cookie_header_present() {
        let headers = HeaderMap::new();
        // TODO
        let verifier =
            ResponseVerifier { status: Default::default(), headers, body: Default::default() };

        verifier.header_cookie(&["HttpOnly"]);
    }
}
