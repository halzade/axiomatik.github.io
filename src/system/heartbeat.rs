use crate::system::server::TheState;
use std::time::Duration;
use axum::extract::State;
use axum::Json;
use axum_core::response::IntoResponse;
use serde::Serialize;
use tokio::time::interval;
use tracing::{info, trace};

#[derive(Serialize)]
struct Heartbeat {
    status: &'static str,
    uptime_seconds: u64,
    db: &'static str, // placeholder for database check
}

pub fn heart_beat() {
    info!("start heart beat");
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            trace!("beat");
        }
    });
}

// TODO
pub async fn handle_heartbeat(State(state): State<TheState>) -> impl IntoResponse {
    Json(Heartbeat {
        status: "ok",
        uptime_seconds: 100,
        db: "ok",
    })
}
