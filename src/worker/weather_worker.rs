use crate::system::server::TheState;
use std::time::Duration;
use thiserror::Error;
use tokio::time::interval;
use tracing::info;

#[derive(Debug, Error)]
pub enum WeatherWorkerError {
    #[error("weather error")]
    Weather,
}

pub fn start_weather_worker(state: TheState) -> Result<(), WeatherWorkerError> {
    info!("start weather worker");

    // loop thread
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_mins(60));

        loop {
            interval.tick().await;
            let state_c = state.clone();

            // task thread
            tokio::spawn(async move {
                info!("weather action: update data");
                let changed = state_c.ds.update_weather().await;

                if changed {
                    // weather changed
                    info!("weather action: change");
                    state_c.dv.invalidate_index_and_categories();
                    let _ = state_c.dbs.invalidate_all_article().await;
                    info!("weather action: finished");
                } else {
                    info!("weather action: nothing changed");
                }
            });
        }
    });

    Ok(())
}
