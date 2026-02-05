use crate::system::router_app::ApplicationRouter;
use crate::system::router_web::WebRouter;
use axum::Router;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use std::sync::Arc;
use thiserror::Error;
use ApplicationStatus::{Off, Started, Unknown};
use ServerError::{ServerAlreadyStarted, UnknownServerStatus};

pub const AUTH_COOKIE: &str = "axiomatik_auth";

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Handling server status error")]
    ServerStatusError,

    #[error("Server already started")]
    ServerAlreadyStarted,

    #[error("Unknown server status")]
    UnknownServerStatus,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ApplicationStatus {
    Started,
    Off,
    Unknown,
}

pub struct Server {
    status: RwLock<ApplicationStatus>,
    start_time: DateTime<Utc>,
    router_app: Arc<ApplicationRouter>,
    router_web: Arc<WebRouter>,
}

impl Server {
    pub async fn start_server(&self) -> Result<(Router, Router), ServerError> {
        // setup status
        let application_status = self.status();

        match application_status {
            Started => Err(ServerAlreadyStarted),
            Off => {
                // server is off, start it
                self.status_start()?;

                // set up router
                let app = self
                    .router_app
                    .clone()
                    .start_app_router(application_status)
                    .await;

                let web = self
                    .router_web
                    .clone()
                    .start_web_router(application_status)
                    .await;
                Ok((app, web))
            }
            Unknown => Err(UnknownServerStatus),
        }
    }

    pub fn is_off(&self) -> bool {
        Off == self.status()
    }

    pub fn run_time(&self) -> String {
        let duration = Utc::now().signed_duration_since(self.start_time);

        let total_seconds = duration.num_seconds();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        format!("{}h {}m {}s", hours, minutes, seconds)
    }

    pub fn status(&self) -> ApplicationStatus {
        self.status.read().clone()
    }

    fn status_start(&self) -> Result<(), ServerError> {
        *self.status.write() = Started;
        Ok(())
    }
}

pub fn new() -> Server {
    Server {
        status: RwLock::new(Off),
        start_time: Utc::now(),
        router_app: Arc::new(ApplicationRouter::new()),
        router_web: Arc::new(WebRouter::new()),
    }
}
