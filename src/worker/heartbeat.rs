use std::time::Duration;
use tokio::time::interval;
use tracing::{info, trace};

pub fn start_heart_beat() {
    info!("start heart beat");
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            trace!("beat");
        }
    });
}
