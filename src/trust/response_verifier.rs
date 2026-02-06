use http::StatusCode;

pub struct ResponseVerifier {}

impl ResponseVerifier {
    pub fn must_see_response(self, status: StatusCode) -> ResponseVerifier {
        todo!();

        self
    }

    pub fn headers_location(self, p0: &str) -> ResponseVerifier {
        todo!();

        self
    }

    pub fn verify(&self) {
        todo!()
    }
}
