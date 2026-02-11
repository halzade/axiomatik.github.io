use crate::trust::data::utils::error;
use crate::trust::me::TrustError;
use axum_core::response::Response;
use http::header::SET_COOKIE;
use http::{HeaderMap, StatusCode};
use parking_lot::RwLock;
use std::sync::Arc;
use tracing::error as log_error;

#[derive(Default, Debug, Clone)]
pub struct ResponseData {
    pub status: Option<StatusCode>,
    pub location: Option<String>,
    pub cookies: Vec<Vec<String>>,
    pub body: Option<String>,
    pub body_contains: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResponseFluent {
    pub(crate) data: Arc<RwLock<ResponseData>>,
}

impl ResponseFluent {
    pub fn new() -> Self {
        Self { data: Arc::new(RwLock::new(ResponseData::default())) }
    }

    pub fn status(&self, status: StatusCode) -> &Self {
        let mut guard = self.data.write();
        guard.status = Some(status);
        self
    }

    pub fn location(&self, location: &str) -> &Self {
        let mut guard = self.data.write();
        guard.location = Some(location.to_string());
        self
    }

    pub fn body(&self, text: &str) -> &Self {
        let mut guard = self.data.write();
        guard.body = Some(text.to_string());
        self
    }

    pub fn body_contains(&self, text: &str) -> &Self {
        let mut guard = self.data.write();
        guard.body_contains.push(text.to_string());
        self
    }

    pub fn cookie(&self, properties: &[&str]) -> &Self {
        let mut guard = self.data.write();
        guard.cookies.push(properties.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn get_data(&self) -> ResponseData {
        self.data.read().clone()
    }
}

pub struct ResponseVerifier {
    pub headers: HeaderMap,
    pub response: Response,
    pub expected: ResponseFluent,
}

impl ResponseVerifier {
    pub fn new(response: Response) -> Self {
        let headers = response.headers().clone();
        Self { headers, response, expected: ResponseFluent::new() }
    }

    pub fn header_location(self, location: &str) -> Self {
        self.expected.location(location);
        self
    }

    pub fn header_cookie(self, properties: &[&str]) -> Self {
        self.expected.cookie(properties);
        self
    }

    pub fn must_see_response(self, status: StatusCode) -> Self {
        self.expected.status(status);
        self
    }

    pub fn body(self, text: &str) -> Self {
        self.expected.body(text);
        self
    }

    pub fn body_contains(self, text: &str) -> Self {
        self.expected.body_contains(text);
        self
    }

    pub async fn verify(self) -> Result<(), TrustError> {
        let mut errors: Vec<String> = Vec::new();
        let expected = self.expected.get_data();

        // status
        let real_status = self.response.status();
        if let Some(exp) = expected.status {
            if exp != real_status {
                errors.push(error("status", exp.to_string(), real_status.as_str()));
            }
        }

        // body
        let real_body = crate::trust::data::utils::response_to_body(self.response).await;
        if let Some(exp) = &expected.body {
            if !real_body.contains(exp) {
                errors.push(error("body", exp.clone(), &real_body));
            }
        }

        for exp in &expected.body_contains {
            if !real_body.contains(exp) {
                errors.push(error("body_contains", exp.clone(), &real_body));
            }
        }

        if !errors.is_empty() {
            tracing::error!("Real body: {}", real_body);
        }

        // location
        if let Some(exp) = &expected.location {
            let real = self.headers.get(http::header::LOCATION);
            match real {
                Some(real_val) => {
                    let real_str = real_val.to_str().unwrap_or("");
                    if exp != real_str {
                        errors.push(error("location", exp.clone(), real_str));
                    }
                }
                None => {
                    errors.push(error("location", exp.clone(), "None"));
                }
            }
        }

        let mut cookies = self
            .headers
            .get_all(SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok());

        // cookies
        for exp_props in &expected.cookies {
            let found = cookies.any(|cookie| exp_props.iter().all(|prop| cookie.contains(prop)));
            if !found {
                errors.push(format!(
                    "No Set-Cookie header contained all required properties: {:?}",
                    exp_props
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            log_error!("Validation error");
            for e in &errors {
                log_error!("{}", e);
            }
            Err(TrustError::Validation(format!("{} incorrect", errors.len())))
        }
    }
}
