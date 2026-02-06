use crate::system::router_app::{AppRouterError, ApplicationRouter};
use crate::system::router_web::WebRouter;
use axum::Router;
use chrono::{DateTime, TimeDelta, Utc};
use parking_lot::RwLock;
use std::sync::Arc;
use thiserror::Error;
use ApplicationStatus::{Off, Started, Unknown};
use ServerError::{ServerAlreadyStarted, UnknownServerStatus};

pub const AUTH_COOKIE: &str = "axiomatik_auth";

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("handling server status error")]
    ServerStatusError,

    #[error("server already started")]
    ServerAlreadyStarted,

    #[error("unknown server status")]
    UnknownServerStatus,

    #[error("router error")]
    ApplicationRouter(#[from] AppRouterError),
}

#[derive(Clone, Copy, PartialEq)]
pub enum ApplicationStatus {
    Started,
    Off,
    Unknown,
}

pub struct Server {
    status_web: RwLock<ApplicationStatus>,
    status_app: RwLock<ApplicationStatus>,
    router_app: Arc<ApplicationRouter>,
    start_time_app: DateTime<Utc>,
    router_web: Arc<WebRouter>,
    start_time_web: DateTime<Utc>,
}

impl Server {
    pub async fn start_app_router(&self) -> Result<Router, ServerError> {
        // setup status
        let application_status = self.status_app();

        match application_status {
            Started => Err(ServerAlreadyStarted),
            Off => {
                // server is off, start it

                // set up router
                let app = self.router_app.clone().start_app_router(application_status).await;
                Ok(app)
            }
            Unknown => Err(UnknownServerStatus),
        }
    }

    pub async fn start_web_router(&self) -> Result<Router, ServerError> {
        // setup status
        let application_status = self.status_web();

        match application_status {
            Started => Err(ServerAlreadyStarted),
            Off => {
                let web = self.router_web.clone().start_web_router(application_status).await;
                Ok(web)
            }
            Unknown => Err(UnknownServerStatus),
        }
    }

    pub fn is_off(&self) -> bool {
        Off == self.status_app() && Off == self.status_web()
    }

    pub fn run_time_app(&self) -> String {
        let duration = Utc::now().signed_duration_since(self.start_time_app);
        duration_str(duration)
    }

    pub fn run_time_web(&self) -> String {
        let duration = Utc::now().signed_duration_since(self.start_time_web);
        duration_str(duration)
    }

    pub fn status_app(&self) -> ApplicationStatus {
        self.status_app.read().clone()
    }
    pub fn status_web(&self) -> ApplicationStatus {
        self.status_web.read().clone()
    }

    pub fn status_start(&self) -> Result<(), ServerError> {
        *self.status_app.write() = Started;
        *self.status_web.write() = Started;
        Ok(())
    }
}

fn duration_str(duration: TimeDelta) -> String {
    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{}h {}m {}s", hours, minutes, seconds)
}

pub async fn new() -> Result<Server, ServerError> {
    Ok(Server {
        status_web: RwLock::new(Off),
        status_app: RwLock::new(Off),
        router_app: Arc::new(ApplicationRouter::init().await?),
        start_time_app: Utc::now(),
        router_web: Arc::new(WebRouter::new()),
        start_time_web: Utc::now(),
    })
}
