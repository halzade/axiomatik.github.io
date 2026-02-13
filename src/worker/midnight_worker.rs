use crate::data::time::to_prague_time;
use crate::system::server::TheState;
use chrono::{Duration as ChronoDuration, Timelike, Utc};
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;
use tracing::info;

#[derive(Debug, Error)]
pub enum MidnightWorkerError {
    #[error("midnight error")]
    Midnight,
}

fn calculate_next_midnight_delay() -> Duration {
    let now_utc = Utc::now();
    let now_prague = to_prague_time(now_utc);

    // Calculate next midnight in Prague
    let next_midnight_prague = (now_prague + ChronoDuration::days(1))
        .with_hour(0)
        .and_then(|t| t.with_minute(0))
        .and_then(|t| t.with_second(0))
        .and_then(|t| t.with_nanosecond(0));

    next_midnight_prague.map_or_else(|| Duration::from_secs(60), |next_midnight| {
        let duration_to_midnight = next_midnight.signed_duration_since(now_prague);
        let wait_secs = duration_to_midnight.num_seconds() as u64;
        Duration::from_secs(wait_secs)
    })
}

pub fn start_midnight_worker(state: TheState) -> Result<(), MidnightWorkerError> {
    info!("start midnight worker");
    tokio::spawn(async move {
        loop {
            let wait_duration = calculate_next_midnight_delay();

            info!("waiting {} seconds until midnight in Prague", wait_duration.as_secs());
            sleep(wait_duration).await;

            let state_c = state.clone();
            tokio::spawn(async move {
                info!("midnight action: update data");
                state_c.ds.update_date();
                state_c.ds.update_name_day();

                info!("midnight action: invalidate everything");
                state_c.dv.invalidate_index_and_categories();
                let _ = state_c.dbs.invalidate_all_article().await;
                info!("midnight action: finished");
            });

            sleep(Duration::from_secs(2)).await;
        }
    });

    Ok(())
}
