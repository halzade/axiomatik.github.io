use chrono::Local;
use std::time::Duration;
use tokio::time;
use tokio::time::{interval, Instant};
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
    tokio::spawn(async move {
        let start = next_midnight_instant();
        let mut interval = time::interval_at(start, Duration::from_secs(24 * 60 * 60));

        loop {
            interval.tick().await;
            info!("midnight event");

            GLOBAL_DATA.update_date();
            GLOBAL_DATA.update_name_day();
        }
    });
}

pub fn weather_worker() {
    tokio::spawn(async move {
        // Every 60 minutes
        let mut interval = interval(Duration::from_secs(60 * 60));
        loop {
            interval.tick().await;

            GLOBAL_DATA.update_weather().await;
        }
    });
}

fn next_midnight_instant() -> Instant {
    let now = Local::now();

    let next_midnight = now
        .date_naive()
        .succ_opt()
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let duration_until = (next_midnight - now.naive_local()).to_std().unwrap();

    Instant::now() + duration_until
}
