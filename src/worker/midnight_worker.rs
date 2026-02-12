use crate::data::time::to_prague_time;
use chrono::{Duration as ChronoDuration, Timelike, Utc};
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;
use tracing::{info, trace};

#[derive(Debug, Error)]
pub enum MidnightWorkerError {
    #[error("midnight error")]
    Midnight,
}

pub fn start_midnight_worker() -> Result<(), MidnightWorkerError> {
    info!("start midnight worker");
    tokio::spawn(async move {
        loop {
            let now_utc = Utc::now();
            let now_prague = to_prague_time(now_utc);

            // Calculate next midnight in Prague
            let next_midnight_prague = (now_prague + ChronoDuration::days(1))
                .with_hour(0)
                .and_then(|t| t.with_minute(0))
                .and_then(|t| t.with_second(0))
                .and_then(|t| t.with_nanosecond(0));

            if let Some(next_midnight) = next_midnight_prague {
                let duration_to_midnight = next_midnight.signed_duration_since(now_prague);
                let wait_secs = duration_to_midnight.num_seconds() as u64;

                info!("waiting {} seconds until midnight in Prague", wait_secs);
                sleep(Duration::from_secs(wait_secs)).await;

                // Perform the midnight action
                trace!("midnight action");



                sleep(Duration::from_secs(1)).await;
            } else {
                // This should not happen
                sleep(Duration::from_secs(60)).await;
            }
        }
    });

    Ok(())
}
