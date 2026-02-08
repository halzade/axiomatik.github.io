use crate::trust::data::utils::error;
use crate::trust::me::TrustError;
use axum_core::response::Response;
use http::header::SET_COOKIE;
use http::{HeaderMap, StatusCode};
use tracing::error as log_error;

#[derive(Default)]
pub struct LoginResponseExpected {
    pub status: Option<StatusCode>,
    pub location: Option<String>,
    pub cookies: Vec<Vec<String>>,
    pub body: Option<String>,
}

pub struct LoginResponseVerifier {
    pub headers: HeaderMap,
    pub response: Response,
    pub expected: LoginResponseExpected,
    pub login_cookie: String,
}

impl LoginResponseVerifier {
    pub fn new(response: Response) -> Result<Self, TrustError> {
        let headers = response.headers().clone();

        let login_cookie_hv =
            headers.get(SET_COOKIE).cloned().ok_or_else(|| TrustError::NoCookie)?;
        let login_cookie = login_cookie_hv.to_str()?.to_string();

        Ok(LoginResponseVerifier {
            headers,
            response,
            expected: LoginResponseExpected::default(),
            login_cookie,
        })
    }

    pub fn header_location(mut self, location: &str) -> Self {
        self.expected.location = Some(location.to_string());
        self
    }

    pub fn header_cookie(mut self, properties: &[&str]) -> Self {
        self.expected.cookies.push(properties.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn must_see_response(mut self, status: StatusCode) -> Self {
        self.expected.status = Some(status);
        self
    }

    pub fn body(mut self, text: &str) -> Self {
        self.expected.body = Some(text.to_string());
        self
    }

    pub async fn verify(self) -> Result<String, TrustError> {
        let mut errors: Vec<String> = Vec::new();

        // status
        let real_status = self.response.status();
        if let Some(exp) = self.expected.status {
            if exp != real_status {
                errors.push(error("status", exp.to_string(), real_status.as_str()));
            }
        }

        // body
        let real_body = crate::trust::data::utils::response_to_body(self.response).await;
        if let Some(exp) = &self.expected.body {
            if !real_body.contains(exp) {
                errors.push(error("body", exp.clone(), &real_body));
            }
        }

        if !errors.is_empty() {
            tracing::error!("Real body: {}", real_body);
        }

        // location
        if let Some(exp) = &self.expected.location {
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

        // cookies
        for exp_props in &self.expected.cookies {
            let mut cookies = self
                .headers
                .get_all(http::header::SET_COOKIE)
                .iter()
                .map(|v| v.to_str().expect("Invalid Set-Cookie header"));

            let found = cookies.any(|cookie| exp_props.iter().all(|prop| cookie.contains(prop)));

            if !found {
                errors.push(format!(
                    "No Set-Cookie header contained all required properties: {:?}",
                    exp_props
                ));
            }
        }

        if errors.is_empty() {
            Ok(self.login_cookie.clone())
        } else {
            for e in &errors {
                log_error!("{}", e);
            }
            Err(TrustError::Validation(format!("{} incorrect", errors.len())))
        }
    }
}
