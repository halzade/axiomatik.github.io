use crate::system::server::TheState;
use axum::extract::State;
use axum::Json;
use axum_core::response::IntoResponse;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: String,
    uptime_str: String,
    db: String,
}

pub async fn handle_health(State(state): State<TheState>) -> impl IntoResponse {
    // uptime in seconds since start_time captured in TheState
    let now = Utc::now();
    let duration = now - state.start_time;

    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    let uptime_str = format!("{}d {}h {}m {}s", days, hours, minutes, seconds);

    // Check DB by attempting a lightweight read; if DB responds, it's ok
    let db_status_r = state.dbs.health().await;
    match db_status_r {
        Ok(db_status) => Json(Health { status: "ok".into(), uptime_str, db: db_status }),
        Err(e) => Json(Health { status: "ok".into(), uptime_str, db: e.to_string() }),
    }
}
