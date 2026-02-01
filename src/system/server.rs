use axum::Router;
use chrono::{DateTime, Local};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use ApplicationStatus::{Off, Started, Unknown};
use ServerError::{ServerAlreadyStarted, ServerStatusError, UnknownServerStatus};
use crate::system::router::ApplicationRouter;

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
    start_time: DateTime<Local>,
    router: Arc<ApplicationRouter>,
}

impl Server {
    pub async fn start_server(&self) -> Result<Router, ServerError> {
        // setup status
        let status_c = self.status();

        match status_c {
            Started => Err(ServerAlreadyStarted),
            Off => {
                // server is off, start it
                self.status_start()?;

                // setup router
                Ok(self.router.clone().start_router(status_c).await)
            }
            Unknown => Err(UnknownServerStatus),
        }
    }

    pub fn is_off(&self) -> bool {
        Off == self.status()
    }

    pub fn run_time(&self) -> String {
        let duration = Local::now().signed_duration_since(self.start_time);

        let total_seconds = duration.num_seconds();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        format!("{}h {}m {}s", hours, minutes, seconds)
    }

    pub fn status(&self) -> ApplicationStatus {
        self.status
            .read()
            .map(|guard| guard.clone())
            .unwrap_or(Unknown)
    }

    fn status_start(&self) -> Result<(), ServerError> {
        let mut status = self.status.write().map_err(|_| ServerStatusError)?;
        *status = Started;
        Ok(())
    }
}

pub fn new() -> Server {
    Server {
        status: RwLock::new(Off),
        start_time: Local::now(),
        router: Arc::new(ApplicationRouter::new()),
    }
}
