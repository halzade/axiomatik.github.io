use crate::{content_management, next_midnight_instant};
use std::time::Duration;
use tokio::time;
use tokio::time::interval;
use tracing::{info, trace};

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

pub fn midnight_worker() {
    info!("schedule midnight worker");
    tokio::spawn(async {
        let start = next_midnight_instant();
        let mut interval = time::interval_at(start, Duration::from_secs(24 * 60 * 60));

        loop {
            interval.tick().await;
            info!("midnight event");
            content_management::update_index_date();
            content_management::update_index_nameday();
        }
    });
}

pub fn weather_worker() {
    tokio::spawn(async {
        // Every 60 minutes
        let mut interval = interval(Duration::from_secs(60 * 60));
        loop {
            interval.tick().await;
            content_management::update_index_weather().await;
        }
    });
}
