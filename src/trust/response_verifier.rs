use http::StatusCode;

pub struct ResponseVerifier {
    body: Vec<u8>,
}

impl ResponseVerifier {
    pub fn new(body: Vec<u8>) -> Self {
        Self { body }
    }
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
